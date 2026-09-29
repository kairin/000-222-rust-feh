#!/usr/bin/env python3
"""Validate and seed review boards through the installed kanban CLI only."""
import argparse
import json
import re
import subprocess
import tempfile
from collections import defaultdict
from pathlib import Path

HERE = Path(__file__).resolve().parent
CONTROL = Path('/home/kkk/.cline/kanban-review-workspaces/rust-feh-control')
SETTINGS = dict(providerId='openai-codex', modelId='gpt-6-luna')
PHASES = {
    '01': ('Contract and existing-evidence audit',
           'Trace the named entry points to state/files/process effects. Map requirements '
           'and every listed scenario to actual assertions. Identify dead paths, missing '
           'assertions, early-return skips and ownership overlaps. Return a scenario-to-test '
           'matrix and exact safe execution plan; do not change product code.'),
    '02': ('Normal-path verification',
           'Execute the normal cases using disposable fixtures after reviewing 01. Assert '
           'observable results, output bytes/dimensions or state transitions, not merely '
           'exit status. Reuse a real public entry point. Record what existing tests do '
           'and do not prove; missing automation may require a separately scoped harness.'),
    '03': ('Boundary, failure and interaction verification',
           'Exercise the adverse cases deterministically, including shared-state interactions. '
           'Use controlled event ordering or task-owned fake executables where appropriate. '
           'Real external integration remains separate from simulated tests. Record a '
           'reproducer before classifying a defect. Do not fix suspected defects in this card.'),
    '04': ('Measurement and evidence-based disposition',
           'Use 01-03 evidence to characterize the named metric with repeated comparable '
           'runs. For non-performance scopes reconcile coverage instead. Classify verified '
           'behavior, confirmed defect, measured opportunity, documentation mismatch or '
           'blocked/unverified. Propose only bounded finding-backed remediation cards. '
           'Do not claim board verification while required scenarios remain unexecuted.'),
}


def run(args, cwd=None):
    # Node's immediate process.exit can truncate pipe-backed console output.
    # Regular files retain complete CLI JSON, including large master boards.
    with tempfile.TemporaryFile(mode='w+', encoding='utf-8') as out, \
            tempfile.TemporaryFile(mode='w+', encoding='utf-8') as err:
        result = subprocess.run(args, cwd=cwd, stdout=out, stderr=err, timeout=120)
        out.seek(0)
        err.seek(0)
        stdout, stderr = out.read(), err.read()
    if result.returncode:
        raise RuntimeError(f'{args[:4]} failed ({result.returncode}): {stderr[-1800:]} {stdout[-800:]}')
    return stdout


def cli(args):
    result = json.loads(run(args))
    if not result.get('ok'):
        raise RuntimeError(f'Kanban operation failed: {result}')
    return result


def validate_manifest(data):
    if not re.fullmatch(r'[0-9a-f]{40}', data['baseline']):
        raise ValueError('baseline must be an exact commit')
    ids = set()
    for b in data['boards']:
        if b['id'] in ids:
            raise ValueError(f'duplicate board: {b["id"]}')
        if not re.fullmatch(r'[A-Z]+-[0-9]{2}', b['id']):
            raise ValueError(f'invalid board ID: {b["id"]}')
        ids.add(b['id'])
        for field in ['group', 'title', 'sources', 'symbols', 'tests', 'cases', 'acceptance']:
            if not b.get(field):
                raise ValueError(f'{b["id"]}: missing {field}')
        for source in b['sources']:
            if not Path(source).is_absolute():
                raise ValueError(f'{b["id"]}: source must be absolute')
            if not Path(source).is_file():
                raise ValueError(f'{b["id"]}: missing source {source}')
    for b in data['boards']:
        if set(b['related']) - ids:
            raise ValueError(f'{b["id"]}: unresolved relationship')


def task_key(prompt):
    match = re.search(r'^Review-ID: ([A-Z0-9-]+)$', prompt, re.MULTILINE)
    return match.group(1) if match else None


def existing_by_key(tasks):
    result = {}
    for task in tasks:
        key = task_key(task.get('prompt', ''))
        if key in result:
            raise ValueError(f'duplicate task marker: {key}')
        if key:
            result[key] = task
    return result


def create_args(project, baseline, title, prompt, effort='high'):
    return ['kanban', 'task', 'create', '--project-path', project,
            '--base-ref', baseline, '--title', title, '--prompt', prompt,
            '--agent-id', 'cline', '--cline-provider', SETTINGS['providerId'],
            '--cline-model', SETTINGS['modelId'], '--cline-reasoning-effort', effort,
            '--start-in-plan-mode', 'true', '--auto-review-enabled', 'false']


def board_path(data, board):
    return str(Path(data['workspace_root']) / ('rust-feh-' + board['id'].lower()))


def ensure_worktree(data, project):
    path = Path(project)
    if not path.exists():
        path.parent.mkdir(parents=True, exist_ok=True)
        run(['git', '-C', data['project'], 'worktree', 'add', '--detach', project, data['baseline']])
    actual = run(['git', '-C', project, 'rev-parse', '--show-toplevel']).strip()
    head = run(['git', '-C', project, 'rev-parse', 'HEAD']).strip()
    if actual != project or head != data['baseline']:
        raise ValueError(f'Existing workspace identity/baseline mismatch: {project}')
    # Do not reset, clean, copy ignored files, alter branches, or rewrite existing work.


def common(data, key, project):
    return f'''Review-ID: {key}
Campaign: rust-feh existing-implementation hardening (maintainer approved 2026-09-27)
Native board project path: {project}
Master project path: {data['project']}
Baseline commit: {data['baseline']}
Required runner: Cline / openai-codex / gpt-6-luna; no silent fallback.
Mode: begin PLAN; do not auto-start other tasks, auto-commit, push or open PRs.

Read /home/kkk/Apps/rust-feh/.agents/hardening/README.md and this entire card.
That campaign's approved Luna routing supersedes historical Haiku/Sonnet/Opus
dispatch roles only. Architecture, no-network product, no-new-dependency and safety
rules remain. Native links are NOT grouping: none are authorized here.

Resolve your actual task checkout with `git rev-parse --show-toplevel`, compare HEAD
with the baseline, inspect dirty state, and read its AGENTS.md and applicable spec.
Canonical source paths below are references: map their suffix under the master
root into YOUR checkout for tests/edits. Never edit the original or board-root
checkout. Existing user changes in the original working tree are excluded from
this committed baseline; SETUP-01 must disposition them. Do not copy/stash/reset them.

Evidence destination: {CONTROL}/evidence/{key}/
In PLAN, return findings in the session; report/fixture writes and test execution
need authorized Act mode. Once authorized, only this evidence directory and
task-owned temporary/build locations may be written. No product changes in review
cards. Missing testability is a bounded harness proposal, not permission to refactor.
Resolve CARGO_HOME/RUSTUP_HOME to existing absolute paths before isolating HOME/XDG.
Use task-owned output/cache/config paths and synthetic images. Never source secrets,
use personal image folders/caches, alter the personal clipboard, install tools,
mount storage, or run global pkill. Kill only children you own. Desktop, clipboard,
real-tool and NAS checks require qualified capability, not just a DISPLAY variable.

Record selected test names/counts; zero matches is NOT a pass. For each scenario
report executed-pass, executed-fail, skipped, blocked, or N/A with reason. Capture
exact commands, exit codes, stdout/stderr, revision, fixture recipe and environment.
Test discovery below does not prove all scenarios are automated. Do not convert
early-return/skipped tests or protocol-only tests into GUI success.
Time-box one card; if its scope exceeds one coherent outcome, return two concrete
child-card proposals and stop instead of expanding files or spawning workers.
Before handoff: include evidence paths, findings, remaining uncertainty, exact
candidate revision (if any), and next bounded action. An agent finishing is not a
verified board. Never clean a task worktree before evidence is durably retained.
'''


def implementation_cards(data, b):
    project = board_path(data, b)
    for suffix, (phase_title, procedure) in PHASES.items():
        key = f'{b["id"]}-{suffix}'
        prereqs = ['SETUP-01', 'SETUP-02', 'SETUP-03', 'SETUP-04']
        if suffix != '01':
            prereqs.append(f'{b["id"]}-01')
        if suffix == '04':
            prereqs.extend([f'{b["id"]}-02', f'{b["id"]}-03'])
        commands = '\n'.join('- ' + x.strip() for x in b['tests'])
        paths = '\n'.join('- ' + x for x in b['sources'])
        prompts = common(data, key, project) + f'''
Implementation: {b['id']} — {b['title']}
Functional group: {b['group']}
Task: {phase_title}
Prerequisite evidence (manual scheduling, not auto-start edges): {', '.join(prereqs)}.
If prerequisites/capabilities are missing, report the exact blocker and do not run checks.
Capabilities to qualify: {', '.join(b['requires'])}.
Related boards (not dependencies): {', '.join(b['related']) or 'none'}.
Conflict/write territory: no production writes; future changes to these sources
must be serialized with their other owners, especially main.rs/ui_logic.rs/image_proc.rs.

Source/spec/test starting points:
{paths}
Named symbols: {', '.join(b['symbols'])}

Normal scenarios:
''' + '\n'.join('- ' + x for x in b['normal']) + '\nAdverse/interaction scenarios:\n' + '\n'.join('- ' + x for x in b['adverse']) + f'''

Procedure for this card: {procedure}
Implementation acceptance: {b['acceptance']}
Measurement/coverage focus: {b['metric']}

Candidate discovery/baseline commands (run from resolved task checkout only;
first inspect --list and skip paths, satisfy isolation/capabilities):
{commands}

Phase-specific deliverable:
- 01: source/requirement/scenario/assertion matrix, safe fixture and test recipe.
- 02: normal-case observed outputs, assertion coverage and exact unexecuted gaps.
- 03: adverse-case evidence and deterministic reproducers or explicit limitations.
- 04: repeated comparable measurements or coverage reconciliation, board disposition,
  and only finding-backed remediation proposals (title, scope, test and acceptance).
Deliver only this card's phase. Do not run all four phases in one session.
Record blocked required scenarios even when existing tests pass. Existing accepted
tradeoffs need evidence and maintainer disposition, not automatic code changes.
'''
        yield dict(key=key, title=f'[{key}] {phase_title}: {b["title"]}', prompt=prompts,
                   effort='high' if suffix in ('03', '04') else 'medium')


def assert_card(task, expected, baseline):
    problems = []
    if task.get('prompt', '').strip() != expected['prompt'].strip():
        problems.append('prompt drift')
    for k, v in [('agentId', 'cline'), ('baseRef', baseline), ('startInPlanMode', True),
                 ('autoReviewEnabled', False)]:
        if task.get(k) != v:
            problems.append(k)
    settings = dict(SETTINGS, reasoningEffort=expected['effort'])
    if task.get('clineSettings') != settings:
        problems.append('Luna settings')
    if problems:
        raise ValueError(f'{expected["key"]}: unexpected existing card {problems}; refusing overwrite')


def seed_project(data, project, expected, verify_only=False):
    try:
        listing = cli(['kanban', 'task', 'list', '--project-path', project])
    except RuntimeError as error:
        # list intentionally requires registration; create performs supported
        # registration. Never swallow transport, parsing or permission failures.
        if verify_only or f'Project {project} is not added to Kanban yet.' not in str(error):
            raise
        listing = dict(workspacePath=project, tasks=[], dependencies=[])
    if listing.get('workspacePath') != project:
        raise ValueError(f'Workspace aliasing detected: {project}')
    if listing.get('dependencies'):
        raise ValueError(f'Unexpected executable dependency links in {project}')
    existing = existing_by_key(listing['tasks'])
    records = []
    for spec in expected:
        old = existing.get(spec['key'])
        if old:
            assert_card(old, spec, data['baseline'])
            record = old
        elif verify_only:
            raise ValueError(f'Missing persisted card: {spec["key"]}')
        else:
            record = cli(create_args(project, data['baseline'], spec['title'], spec['prompt'], spec['effort']))['task']
            assert_card(record, spec, data['baseline'])
            existing[spec['key']] = record
        records.append(dict(key=spec['key'], id=record['id'], title=spec['title'], project=project))
    return records


def master_cards(data):
    setup = [
        ('SETUP-01', 'Baseline and existing-change disposition',
         'Read the before-setup.json fingerprint and original git diff without modifying it. '
         'Confirm committed baseline and enumerate every excluded local candidate, including '
         'aspect_ratio. Record whether each later review covers baseline or an approved candidate. '
         'Acceptance: reproducible baseline, zero discarded user bytes, exact delta disposition.'),
        ('SETUP-02', 'Qualified Luna session and board/worktree pilot',
         'One worker only. Verify actual provider/model resolves to gpt-6-luna, task mode, '
         'checkout root and HEAD. Run a bounded existing merge_converted test in isolated '
         'authorized Act mode; record test count and evidence. Verify evidence survives session '
         'completion and no peers auto-start. No fallback. Acceptance: actual launch and tool '
         'execution receipt, not merely saved model strings. This task is pending, not already done.'),
        ('SETUP-03', 'Safe fixtures and test-environment qualification',
         'Inventory tool paths and versions, existing test skip paths and unsafe side effects. '
         'Define task-owned HOME/XDG/cache/temp and preserve resolved Rust toolchain homes. '
         'Provide real-image vs metadata fixtures, fake executable recipe and safe child-PID '
         'cleanup. No private clipboard or real NAS means those tiers stay blocked. '
         'Acceptance: repeatable isolated recipes and explicit capability matrix, no personal data writes.'),
        ('SETUP-04', 'Review dispatch readiness and write ownership',
         'Depends on SETUP-01..03. Record a time/budget bound and at most two non-overlapping '
         'review workers; serialize display and benchmark sessions and shared-file changes. '
         'Check all prerequisites, define absolute evidence locations and task branch base. '
         'Acceptance: one bounded ready wave; no worker fan-out, product edits or native links '
         'from this planning card. Coordinator launches approved workers separately.'),
        ('SETUP-05', 'Independent board-structure and coverage audit',
         'Read manifest.json and INDEX.md. Check distinct native workspace identities, explicit '
         'Luna overrides, zero native edges, self-contained prompts and all relationships. '
         'Map active Rust functions, GUI entry points, requirements and tests to primary board '
         'owners; identify omissions. Acceptance: coverage matrix with explicit exclusions, '
         'not a claim that the seed inventory itself proves exhaustive review.'),
    ]
    for key, title, instruction in setup:
        yield dict(key=key, title=f'[{key}] {title}', effort='high',
                   prompt=common(data, key, data['project']) + '\nBounded objective and acceptance:\n' + instruction +
                   f'\nRead-only inputs: {HERE}/manifest.json; {HERE}/INDEX.md; {CONTROL}/before-setup.json.\n'
                   'Commands: git rev-parse HEAD; git status --short; '
                   'cargo test --lib merge_converted -- --list (discover before authorized execution).\n')
    groups = defaultdict(list)
    for b in data['boards']:
        groups[b['group']].append(b)
    for n, (group, boards) in enumerate(groups.items(), 1):
        key = f'GROUP-{n:02}'
        links = '\n'.join(f'- {b["id"]}: {b["title"]}\n  Project: {board_path(data,b)}\n'
                          f'  Board: http://127.0.0.1:3484/{Path(board_path(data,b)).name}' for b in boards)
        prompt = common(data, key, data['project']) + f'''\nGroup: {group}
Objective: maintain a source-backed rollup for these implementation boards, not
perform all their work. Read their evidence after SETUP-01..05 and identify duplicate
ownership, uncovered behavior and cross-board blockers. No auto-start/links allowed.
Acceptance: each member has an evidence-backed disposition or explicit pending/
blocked status; do not mark the group verified because its index is complete.
Members and native board navigation:
{links}
Output: rollup with exact task IDs/evidence, no product edits.
'''
        yield dict(key=key, title=f'[{key}] {group} — implementation board index', prompt=prompt, effort='medium')
    journeys = [
        ('JOURNEY-01', 'Folder switch to correct staged image', ['NAV-01','NAV-02','SCAN-06','SCAN-07','LIST-05','STAGE-02'],
         'Rapidly switch A to B while A scans/decodes; only B populates navigation/list/stage and old work stops.'),
        ('JOURNEY-02', 'Filter/sort through feh return and scroll', ['LIST-01','LIST-02','VIEW-01','VIEW-04','NAV-03','LIST-05'],
         'Launch from filtered sorted list, navigate across folders, close viewer during rescan; correct image lands and scrolls.'),
        ('JOURNEY-03', 'Pinned operation through safe output and inventory', ['INSP-04','ACT-03','IMAGE-01','BATCH-04','SCAN-08'],
         'Pin A, select B, process A during converted refresh; correct input/output/original and live inventory remain consistent.'),
        ('JOURNEY-04', 'Batch cancellation and partial-result consistency', ['BATCH-01','BATCH-04','OBS-01','LIST-03'],
         'Cancel mixed-success batch after folder/filter changes; output files, counters, list and busy state agree.'),
        ('JOURNEY-05', 'Cache hit preserves output-policy safety', ['CACHE-02','CACHE-03','ACT-03','IMAGE-03'],
         'Compare true hit and miss under each output policy with known source bytes; originals/backups/output match contract.'),
        ('JOURNEY-06', 'Missing tool, recheck and recovery', ['CAP-01','CAP-03','INSP-02','VIEW-01','OBS-01'],
         'Lose tool mid-session, observe accurate error/guidance, restore controlled PATH and recheck; functionality and folds recover.'),
    ]
    for key,title,members,scenario in journeys:
        prompt = common(data,key,data['project']) + f'''\nJourney: {title}
Required implementation evidence: {', '.join(members)} and SETUP-01..04.
Scenario and acceptance: {scenario}
Create explicit steps/fixtures and assert end-to-end observable state. Use isolated
desktop/control only when qualified; protocol-only or mocked evidence cannot close
a GUI journey. No product edits. Record precise blocker if required member evidence
or capabilities are missing. Return logs, actual UI observations, preserved-byte
checks where applicable and a cross-board defect owner. No native dependency edges.
Source and tests: read each member's manifest sources and selected-test commands in
/home/kkk/Apps/rust-feh/.agents/hardening/manifest.json; copy the exact subset into
the execution receipt. Do not run the entire product test suite blindly.
'''
        yield dict(key=key,title=f'[{key}] {title}',prompt=prompt,effort='high')


def write_index(data, records):
    lines = ['# rust-feh — Cline hardening boards', '',
             'Task status lives in Cline; this is generated navigation, not a status board.', '',
             f'Baseline: `{data["baseline"]}`. All cards explicitly select Cline / openai-codex / gpt-6-luna.',
             'Cards begin in plan mode, auto-review disabled. No workers or native dependency links are started.', '',
             'Open master: `cline --cwd /home/kkk/Apps/rust-feh --kanban`', '',
             'Browser: http://127.0.0.1:3484/rust-feh', '',
             'Start with SETUP-01, then a single SETUP-02 qualification; all implementation execution requires SETUP-01..04.', '',
             '## Implementation boards', '', '| ID | Group | Implementation | Native workspace |',
             '|---|---|---|---|']
    for b in data['boards']:
        p=board_path(data,b)
        lines.append(f'| {b["id"]} | {b["group"]} | {b["title"]} | `{p}` |')
    lines += ['', '## Exact persisted card IDs', '', '| Review ID | Cline card ID | Native project |', '|---|---|---|']
    lines += [f'| {r["key"]} | {r["id"]} | `{r["project"]}` |' for r in records]
    (HERE/'INDEX.md').write_text('\n'.join(lines)+'\n')


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--seed',action='store_true')
    parser.add_argument('--verify',action='store_true')
    parser.add_argument('--only',help='Pilot a single implementation board, e.g. SCAN-08')
    args=parser.parse_args()
    from catalog import manifest
    baseline=json.loads((CONTROL/'before-setup.json').read_text())['baseline']
    data=manifest(baseline)
    validate_manifest(data)
    print(f'Validated {len(data["boards"])} implementation definitions',flush=True)
    if not (args.seed or args.verify):
        return
    if args.seed and args.verify:
        parser.error('choose --seed or --verify')
    (HERE/'manifest.json').write_text(json.dumps(data,indent=2)+'\n')
    selected=[b for b in data['boards'] if not args.only or b['id']==args.only]
    if args.only and not selected:
        raise ValueError('Unknown board ID')
    records=[]
    for b in selected:
        project=board_path(data,b)
        if args.seed:
            ensure_worktree(data,project)
        records.extend(seed_project(data,project,list(implementation_cards(data,b)),args.verify))
        print(f'{"Verified" if args.verify else "Seeded"} {b["id"]}: {project}',flush=True)
    if not args.only:
        records.extend(seed_project(data,data['project'],list(master_cards(data)),args.verify))
        write_index(data,records)
        CONTROL.mkdir(parents=True,exist_ok=True)
        receipt=dict(baseline=baseline,implementationBoards=len(selected),totalBoards=len(selected)+1,
                     cards=len(records),records=records,workersStarted=0,nativeDependencyLinks=0,
                     qualification='pending actual Luna session; not claimed by board configuration')
        (CONTROL/('verified.json' if args.verify else 'created.json')).write_text(json.dumps(receipt,indent=2)+'\n')
        print(json.dumps({k:v for k,v in receipt.items() if k!='records'}),flush=True)


if __name__=='__main__':
    main()
