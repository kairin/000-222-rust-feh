#!/usr/bin/env bash
# Install ~/.local/bin/sudo-askpass for GUI sudo prompts.
# Shell config (SUDO_ASKPASS in ~/.bashrc.d/) comes from the 000-0-dotfiles repo.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ASKPASS="$HOME/.local/bin/sudo-askpass"

mkdir -p "$HOME/.local/bin"

install -m 0755 "$ROOT/scripts/sudo-askpass-bin.sh" "$ASKPASS"

echo "Installed: $ASKPASS"
echo ""
echo "The project .envrc (see .envrc.example) sets SUDO_ASKPASS to this helper when direnv loads."
echo "Test: SUDO_ASKPASS=$ASKPASS sudo -A true"
echo "Agent/cmdline: SUDO_ASKPASS=$ASKPASS sudo -A apt install feh"
