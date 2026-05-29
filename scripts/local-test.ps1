<#
Simple helper script to build the release and print commands to run two local nodes.
This script does NOT attempt to automate interactive chat steps; it builds the binary
and shows the commands you can run on two machines (or two consoles) to verify connectivity.
#>
Set-StrictMode -Version Latest
Write-Host "Building release..."
Push-Location (Split-Path -Parent $MyInvocation.MyCommand.Path)
Push-Location ..
cargo build --release
Pop-Location

$exe = Join-Path (Resolve-Path ..) "target\release\iroh-gossip-cli.exe"
Write-Host "\nRelease binary:" $exe

Write-Host "\nOn DEVICE A (create room and get ticket):"
Write-Host "    `"$exe`" create"

Write-Host "\nOn DEVICE B (join using ticket printed by DEVICE A):"
Write-Host "    `"$exe`" join <TICKET>"

Write-Host "\nNotes:"
Write-Host "- Run the create command on device A, copy the printed ticket string to device B."
Write-Host "- If you want to run both locally, open two terminals and run the respective commands."
Write-Host "- For CI-style automation, consider adding a non-interactive 'smoke-test' subcommand."
