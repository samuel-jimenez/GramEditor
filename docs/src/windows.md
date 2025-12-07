# Tehanu on Windows

## Installing Tehanu

To install, you can build Tehanu [from source](https://tehanu.liten.app/docs/development/windows).

## Uninstall

- Installed via installer: Use `Settings` → `Apps` → `Installed apps`, search for Tehanu, and click Uninstall.
- Built from source: Remove the build output directory you created (e.g., your target/install folder).

Your settings and extensions live in your user profile. When uninstalling, you can choose to keep or remove them.

## Remote Development (SSH)

Tehanu supports remote development on Windows through both SSH and WSL. You can connect to remote servers via SSH or work with files inside WSL distributions directly from Tehanu.

For detailed instructions on setting up and using remote development features, including SSH configuration, WSL setup, and troubleshooting, see the [Remote Development documentation](./remote-development.md).

## Troubleshooting

### Tehanu fails to start or shows a blank window

- Check that your hardware and operating system version are compatible with Tehanu. See our [installation guide](./installation.md) for more information.
- Update your GPU drivers from your GPU vendor (Intel/AMD/NVIDIA/Qualcomm).
- Ensure hardware acceleration is enabled in Windows and not blocked by third‑party software.
- Try launching Tehanu with no extensions or custom settings to isolate conflicts.

### Terminal issues

If activation scripts don’t run, update to the latest version and verify your shell profile files are not exiting early. For Git operations, confirm Git Bash or PowerShell is available and on PATH.

### SSH remoting problems

When prompted for credentials, use the graphical askpass dialog. If it doesn’t appear, check for credential manager conflicts and that GUI prompts aren’t blocked by your terminal.

### Graphics issues

#### Tehanu fails to open / degraded performance

Tehanu requires a DirectX 11 compatible GPU to run. If Tehanu fails to open, your GPU may not meet the minimum requirements.

To check if your GPU supports DirectX 11, run the following command:

```
dxdiag
```

This will open the DirectX Diagnostic Tool, which shows the DirectX version your GPU supports under `System` → `System Information` → `DirectX Version`.

If you're running Tehanu inside a virtual machine, it will use the emulated adapter provided by your VM. While Tehanu will work in this environment, performance may be degraded.
