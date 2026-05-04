---
name: zed-bootstrap
description: Bootstrap a fresh machine for building this Zed fork from source. Auto-detects platform (Windows, Linux, macOS) and installs the required toolchain. Use when the user wants to set up a new development machine, "install dev dependencies", "bootstrap the environment", or runs `/zed-bootstrap`. Idempotent — skips what is already installed.
---

# Zed Cross-Platform Bootstrap

Brings a fresh machine to the point where `cargo run --bin zed` succeeds from this repository.

## Step 0 — Detect platform

Run:
```
uname -s 2>/dev/null || powershell -Command "[System.Environment]::OSVersion.Platform"
```

Branch on the output:
- `Linux` → use the **Linux** section.
- `Darwin` → use the **macOS** section.
- `Win32NT` (or empty `uname` + the PowerShell line returns `Win32NT`) → use the **Windows** section.
- Anything else (e.g., `FreeBSD`, `MINGW`/`MSYS`/`CYGWIN`): stop and tell the user the platform is unsupported by this skill.

Run only the section for the detected platform. Then run **Final sanity check** (common to all platforms).

---

## Linux

### L1. rustup + cargo

Probe:
```
command -v cargo && cargo --version
```
If missing:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
```
After install, ensure `$HOME/.cargo/env` is sourced (the installer prints this). Confirm:
```
"$HOME/.cargo/bin/cargo" --version
```

### L2. System libraries

The repo ships `script/linux` which handles apt, dnf/yum, zypper, pacman, xbps, and emerge distros. Run it from the repo root:
```
./script/linux
```
This will `sudo` for the package manager. Tell the user a password prompt will appear; do **not** background this command on their behalf.

### L3. Done — skip to Final sanity check.

---

## macOS

### M1. Xcode Command Line Tools

Probe:
```
xcode-select -p
```
If it errors:
```
xcode-select --install
```
A GUI dialog appears. Tell the user to click **Install** and wait. Re-probe before continuing.

### M2. Homebrew

Probe:
```
command -v brew
```
If missing:
```
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```
Apple Silicon: ensure `/opt/homebrew/bin` is on PATH (the installer prints the snippet to add). Intel: `/usr/local/bin`.

### M3. cmake

Probe:
```
command -v cmake && cmake --version
```
If missing:
```
brew install cmake
```

### M4. rustup + cargo

Probe:
```
command -v cargo && cargo --version
```
If missing:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
```
After install:
```
"$HOME/.cargo/bin/cargo" --version
```

### M5. Done — skip to Final sanity check.

---

## Windows

### W1. winget available?

```
powershell -Command "winget --version"
```
If missing: tell the user to install **App Installer** from the Microsoft Store, then re-run this skill. Do not attempt to install winget yourself.

### W2. Git long paths

```
git config --global core.longpaths true
```
Idempotent. No verification needed.

### W3. Rust toolchain

Probe:
```
ls "$HOME/.cargo/bin/cargo.exe" 2>/dev/null && echo PRESENT
```
If absent:
```
powershell -Command "winget install --id Rustlang.Rustup -e --accept-source-agreements --accept-package-agreements"
```
Prime the toolchain (downloads stable components on first call):
```
"$HOME/.cargo/bin/cargo.exe" --version
```
If `$HOME` doesn't resolve in the shell, the user must open a fresh terminal to pick up the new PATH; tell them and stop.

### W4. CMake

Probe:
```
ls "/c/Program Files/CMake/bin/cmake.exe" 2>/dev/null && echo PRESENT
```
If absent:
```
powershell -Command "winget install --id Kitware.CMake -e --accept-source-agreements --accept-package-agreements"
```
Verify:
```
"/c/Program Files/CMake/bin/cmake.exe" --version
```

### W5. Visual Studio Build Tools 2022

Probe:
```
ls "/c/Program Files (x86)/Microsoft Visual Studio/2022/BuildTools/VC/Tools/MSVC" 2>/dev/null
```
If absent, install with the required workload + components in one shot. This is multi-GB and takes several minutes — run as a background command (Bash `run_in_background: true`) and wait for the completion notification rather than polling:
```
powershell -Command "winget install --id Microsoft.VisualStudio.2022.BuildTools -e --accept-source-agreements --accept-package-agreements --override '--quiet --wait --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended --add Microsoft.VisualStudio.Component.Windows11SDK.22621 --add Microsoft.VisualStudio.Component.VC.Runtimes.x86.x64.Spectre --add Microsoft.VisualStudio.Component.VC.ASAN'"
```

If Build Tools is already present but Spectre libs are missing (probe in step W6), you cannot add them silently from a non-elevated shell. Tell the user to:
1. Open Start, search for "Visual Studio Installer", launch it (UAC will elevate).
2. Click **Modify** on Visual Studio Build Tools 2022.
3. Tab **Individual components**, search `Spectre`.
4. Check **MSVC v143 - VS 2022 C++ x64/x86 Spectre-mitigated libs (Latest)**.
5. Click **Modify**, wait for completion.

Then resume from W6.

### W6. Spectre libs sanity check

```
find "/c/Program Files (x86)/Microsoft Visual Studio/2022/BuildTools/VC/Tools/MSVC" -maxdepth 5 -type d -name "spectre" 2>/dev/null | head -3
```
Expected: at least one path under `lib/spectre`. Empty result → user must add Spectre libs via the VS Installer GUI (W5 fallback).

---

## Final sanity check (all platforms)

From the repository root, run a small targeted check:

**Linux / macOS:**
```
"$HOME/.cargo/bin/cargo" check -p database_viewer 2>&1 | tail -5
```

**Windows:**
```
PATH="/c/Program Files/CMake/bin:$PATH" "$HOME/.cargo/bin/cargo.exe" check -p database_viewer 2>&1 | tail -5
```

Expected last line: `Finished 'dev' profile [unoptimized + debuginfo] target(s) in Xs`.

Common failure modes:
- Linker error mentioning `link.exe` extra operand (Windows): MSVC linker not on PATH inside this shell. Tell the user to restart their terminal or run from a "Developer PowerShell for VS 2022".
- `cmake` not found: re-run the cmake install step.
- `Spectre libs` warning (Windows): W5/W6 fallback; user must add via GUI.
- `cc` linker error mentioning `aws_lc_sys` on Linux: the user is missing one of the system libs from `script/linux`. Re-run `./script/linux` or check the docs section linked in `docs/src/development/linux.md`.
- `dispatch.h not found` on macOS: Xcode CLI tools incomplete. Run `sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer`, then `cargo clean` and retry.

## Final report

When the sanity check passes, report:
- What was newly installed vs. already present.
- Output of `cargo --version` and `cmake --version` (Linux/Windows; macOS reads cmake from brew).
- The final `cargo check` last line.

Then tell the user how to launch Zed for the smoke test:

**Linux / macOS:**
```
cargo run --bin zed
```

**Windows:**
```
cd <repo-root>
$env:Path = "C:\Program Files\CMake\bin;" + $env:Path
cargo run --bin zed -- --foreground
```

## Notes

- Steps are idempotent — the skill is safe to re-run after a failure.
- Never run `rustup self uninstall`, never delete `target/`, never modify the user's existing toolchain channels.
- Disk space: the Zed `target/` directory plus dependencies needs roughly **40 GB on Linux/macOS** and **80 GB on Windows** (debug profile). Stop and warn the user if `df -h .` (POSIX) or `Get-PSDrive C` (Windows) shows less than that available.
- This skill modifies the user's machine state but does not push, pull, or alter the git working tree beyond a single `git config --global core.longpaths true` on Windows.
