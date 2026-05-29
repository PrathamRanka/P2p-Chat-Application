#!/usr/bin/env bash
set -euo pipefail

echo "Building release..."
cargo build --release

EXE="$(pwd)/target/release/iroh-gossip-cli"
echo
echo "Release binary: $EXE"
echo
echo "On DEVICE A (create room and get ticket):"
echo "    $EXE create"
echo
echo "On DEVICE B (join using ticket printed by DEVICE A):"
echo "    $EXE join <TICKET>"
echo
echo "Notes:"
echo "- Run the create command on device A, copy the printed ticket string to device B." 
echo "- If you want to run both locally, open two shells and run the respective commands."
echo "- For CI-style automation, consider adding a non-interactive 'smoke-test' subcommand."
