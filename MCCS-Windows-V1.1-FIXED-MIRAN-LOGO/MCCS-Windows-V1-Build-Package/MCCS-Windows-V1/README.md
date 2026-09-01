# MCCS Windows V1

Windows desktop client for the live Miran Commercial Control System.

## Architecture
The desktop app opens the production MCCS web application:
https://miran-commercial-control-system.vercel.app/

No Supabase, Google Drive, password, API key, or database secret is embedded in this package.
The website remains the single live MCCS application, so future website upgrades appear in the desktop client automatically.

## Outputs
GitHub Actions builds:
- MCCS-Windows-V1-x64-Setup.exe
- MCCS-Windows-V1-x86-Setup.exe

## GitHub build
Upload the complete `MCCS-Windows-V1` folder to the repository root.
Then open GitHub > Actions > Build MCCS Windows V1 > Run workflow.
After completion, download the x64 and x86 artifacts.

## Important
These installers are unsigned unless a Windows code-signing certificate is added to the workflow.
Windows SmartScreen may therefore warn users before installation.
