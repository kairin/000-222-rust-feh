"""Safety tests for the local-only Cline board seeder (standard library)."""
import copy
import unittest
import json
from pathlib import Path
from unittest.mock import patch

from kanban_setup import create_args, existing_by_key, task_key, validate_manifest, seed_project, assert_card, run


def sample():
    repo = Path(__file__).resolve().parents[2]
    return {
        "baseline": "a" * 40,
        "project": str(repo),
        "workspace_root": "/home/kkk/.cline/kanban-review-workspaces/rust-feh",
        "boards": [{
            "id": "SCAN-01", "group": "Discovery", "title": "Native classification",
            "sources": [str(repo / "src/scanner.rs")],
            "symbols": ["scan_images"], "tests": ["cargo test --lib scanner::tests"],
            "cases": ["empty directory", "supported extension"],
            "acceptance": "Only supported entries are emitted; no pixel decode during listing.",
            "related": [], "requires": ["fixture"],
        }],
    }


class SetupTests(unittest.TestCase):
    def test_manifest_accepts_explicit_scope(self):
        validate_manifest(sample())

    def test_duplicate_board_rejected(self):
        manifest = sample()
        manifest["boards"].append(copy.deepcopy(manifest["boards"][0]))
        with self.assertRaisesRegex(ValueError, "duplicate"):
            validate_manifest(manifest)

    def test_unresolved_relationship_rejected(self):
        manifest = sample()
        manifest["boards"][0]["related"] = ["UNKNOWN"]
        with self.assertRaisesRegex(ValueError, "relationship"):
            validate_manifest(manifest)

    def test_relative_source_rejected(self):
        manifest = sample()
        manifest["boards"][0]["sources"] = ["src/scanner.rs"]
        with self.assertRaisesRegex(ValueError, "absolute"):
            validate_manifest(manifest)

    def test_missing_acceptance_rejected(self):
        manifest = sample()
        manifest["boards"][0]["acceptance"] = ""
        with self.assertRaisesRegex(ValueError, "acceptance"):
            validate_manifest(manifest)

    def test_creation_pins_luna_and_disables_automatic_execution(self):
        args = create_args("/repo", "a" * 40, "Title", "Prompt", "high")
        self.assertEqual(args[:3], ["kanban", "task", "create"])
        for option, value in [("--agent-id", "cline"),
                              ("--cline-provider", "openai-codex"),
                              ("--cline-model", "gpt-6-luna"),
                              ("--start-in-plan-mode", "true"),
                              ("--auto-review-enabled", "false")]:
            self.assertEqual(args[args.index(option) + 1], value)
        self.assertNotIn("start", args)
        self.assertNotIn("link", args)

    def test_idempotency_uses_prompt_marker_not_title(self):
        tasks = [{"id": "abc", "prompt": "Review-ID: SCAN-01-01\nContract"}]
        self.assertEqual(existing_by_key(tasks)["SCAN-01-01"]["id"], "abc")
        self.assertEqual(task_key(tasks[0]["prompt"]), "SCAN-01-01")
        self.assertIsNone(task_key("an unrelated existing task"))

    def test_duplicate_marker_is_not_silently_overwritten(self):
        tasks = [{"id": "a", "prompt": "Review-ID: SCAN-01-01\n"},
                 {"id": "b", "prompt": "Review-ID: SCAN-01-01\n"}]
        with self.assertRaisesRegex(ValueError, "duplicate"):
            existing_by_key(tasks)

    def test_seed_twice_creates_once_via_cli_and_never_starts(self):
        tasks, calls = [], []

        def fake_cli(args):
            calls.append(args)
            if args[2] == "list":
                return dict(ok=True, workspacePath="/repo", tasks=tasks, dependencies=[])
            self.assertEqual(args[2], "create")
            get = lambda name: args[args.index(name) + 1]
            task = dict(id="native-1", prompt=get("--prompt"), baseRef=get("--base-ref"),
                        agentId=get("--agent-id"), startInPlanMode=True, autoReviewEnabled=False,
                        clineSettings=dict(providerId=get("--cline-provider"),
                                           modelId=get("--cline-model"),
                                           reasoningEffort=get("--cline-reasoning-effort")))
            tasks.append(task)
            return dict(ok=True, task=task)

        expected = [dict(key="SCAN-01-01", title="Contract", effort="high",
                         prompt="Review-ID: SCAN-01-01\nContract")]
        with patch("kanban_setup.cli", side_effect=fake_cli):
            first = seed_project(sample(), "/repo", expected)
            second = seed_project(sample(), "/repo", expected)
        self.assertEqual(first, second)
        self.assertEqual([c[2] for c in calls], ["list", "create", "list"])

    def test_verify_fails_for_missing_card_without_mutating(self):
        with patch("kanban_setup.cli", return_value=dict(
                ok=True, workspacePath="/repo", tasks=[], dependencies=[])) as mocked:
            with self.assertRaisesRegex(ValueError, "Missing persisted"):
                seed_project(sample(), "/repo", [dict(key="SCAN-01-01")], True)
            self.assertEqual(mocked.call_count, 1)

    def test_first_create_registers_missing_workspace(self):
        spec = dict(key="SCAN-01-01", title="Contract", effort="high",
                    prompt="Review-ID: SCAN-01-01\nContract")
        task = dict(id="native-1", prompt=spec["prompt"], baseRef="a" * 40,
                    agentId="cline", startInPlanMode=True, autoReviewEnabled=False,
                    clineSettings=dict(providerId="openai-codex", modelId="gpt-6-luna",
                                       reasoningEffort="high"))
        with patch("kanban_setup.cli", side_effect=[
                RuntimeError("Project /repo is not added to Kanban yet."),
                dict(ok=True, task=task)]) as mocked:
            records = seed_project(sample(), "/repo", [spec])
            self.assertEqual(records[0]["id"], "native-1")
            self.assertEqual(mocked.call_args_list[1].args[0][2], "create")

    def test_unrelated_listing_error_never_creates_cards(self):
        with patch("kanban_setup.cli", side_effect=RuntimeError("connection refused")) as mocked:
            with self.assertRaisesRegex(RuntimeError, "connection refused"):
                seed_project(sample(), "/repo", [dict(key="SCAN-01-01")])
            self.assertEqual(mocked.call_count, 1)

    def test_cli_outer_whitespace_normalization_is_not_prompt_drift(self):
        expected = dict(key="SCAN-01-01", effort="high", prompt="Review-ID: SCAN-01-01\nContract\n")
        actual = dict(prompt=expected["prompt"].strip(), baseRef="a" * 40,
                      agentId="cline", startInPlanMode=True, autoReviewEnabled=False,
                      clineSettings=dict(providerId="openai-codex", modelId="gpt-6-luna",
                                         reasoningEffort="high"))
        assert_card(actual, expected, "a" * 40)

    def test_large_node_output_survives_immediate_cli_exit(self):
        # The installed CLI exits immediately after printing JSON. Pipe-backed
        # stdout can lose buffered bytes even when its exit status is zero.
        output = run(["node", "-e",
                      "console.log(JSON.stringify({payload:'x'.repeat(262144)})); process.exit(0)"])
        self.assertEqual(json.loads(output)["payload"], "x" * 262144)


if __name__ == "__main__":
    unittest.main()
