---
name: zed-bootstrap
description: Bootstrap a fresh Windows machine for building this Zed fork from source. Use when the user wants to set up a new development machine, "install dev dependencies", "bootstrap the environment", or runs `/zed-bootstrap`. Installs rustup + cargo, VS BuildTools 2022 with MSVC + Windows SDK + Spectre-mitigated libs + ASAN, CMake, and configures git long paths. Idempotent — skips what is already installed.
---

# Zed Windows Bootstrap

Brings a fresh Windows machine to the point where `cargo run --bin zed -- --foreground` succeeds from this repository.

## Scope

Windows only. Linux/macOS host? Stop and tell the user this skill is Windows-specific.

## Steps

Execute these steps in order. After each install, verify before continuing. Do **not** mark the bootstrap successful unless the final sanity check passes.

### 0. Confirm platform

Run:
```
powershell -Command "[System.Environment]::OSVersion.Platform"
```
Expected: `Win32NT`. Anything else: stop and report platform mismatch.

### 1. winget available?

Run:
```
powershell -Command "winget --version"
```
If missing: tell the user to install **App Installer** from the Microsoft Store, then re-run this skill. Do not attempt to install winget yourself.

### 2. Git long paths

Run:
```
git config --global core.longpaths true
```
Idempotent. No verification needed.

### 3. Rust toolchain (rustup + cargo)

Probe first:
```
ls "/c/Users/luca.manuel/.cargo/bin/cargo.exe" 2>/dev/null && echo PRESENT
```
Replace `luca.manuel` with `$env:USERNAME`-equivalent if needed: use `ls "$HOME/.cargo/bin/cargo.exe"`.

If absent:
```
powershell -Command "winget install --id Rustlang.Rustup -e --accept-source-agreements --accept-package-agreements"
```

After install, prime the toolchain (downloads stable components on first call):
```
"$HOME/.cargo/bin/cargo.exe" --version
```
Expected: a `cargo X.Y.Z` line. If the path doesn't resolve, the user must open a fresh shell to pick up the new PATH; tell them and stop.

### 4. CMake

Probe first:
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
Expected: a `cmake version X.Y.Z` line.

### 5. Visual Studio Build Tools 2022

Probe for the install:
```
ls "/c/Program Files (x86)/Microsoft Visual Studio/2022/BuildTools/VC/Tools/MSVC" 2>/dev/null
```

If absent, install with the required workload + components in one shot:
```
powershell -Command "winget install --id Microsoft.VisualStudio.2022.BuildTools -e --accept-source-agreements --accept-package-agreements --override '--quiet --wait --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended --add Microsoft.VisualStudio.Component.Windows11SDK.22621 --add Microsoft.VisualStudio.Component.VC.Runtimes.x86.x64.Spectre --add Microsoft.VisualStudio.Component.VC.ASAN'"
```
This download is multi-GB and takes several minutes. Run it as a background command (Bash `run_in_background: true`) and wait for the completion notification — never poll.

If Build Tools is already present but the Spectre libs are missing (probe in step 6), you cannot add them silently from a non-elevated shell. Tell the user to:
1. Open Start, search for "Visual Studio Installer", launch it (UAC will elevate).
2. Click **Modify** on Visual Studio Build Tools 2022.
3. Tab **Individual components**, search `Spectre`.
4. Check **MSVC v143 - VS 2022 C++ x64/x86 Spectre-mitigated libs (Latest)**.
5. Click **Modify**, wait for completion.

Then resume from step 6.

### 6. Spectre libs sanity check

```
find "/c/Program Files (x86)/Microsoft Visual Studio/2022/BuildTools/VC/Tools/MSVC" -maxdepth 5 -type d -name "spectre" 2>/dev/null | head -3
```
Expected: at least one path under `lib/spectre`. Empty result → the user must add Spectre libs via the VS Installer GUI (see step 5 fallback).

### 7. Final build sanity check

From the repository root, run a small targeted check that exercises every install touched above:
```
PATH="/c/Program Files/CMake/bin:$PATH" "$HOME/.cargo/bin/cargo.exe" check -p database_viewer 2>&1 | tail -5
```
Expected last line: `Finished` something like `dev profile [unoptimized + debuginfo] target(s) in Xs`.

If it fails:
- Linker error mentioning `link.exe` extra operand → MSVC linker not on PATH inside this shell. The cargo invocation above sets PATH for cmake; MSVC's `link.exe` is auto-located by the `cc` crate via the registry, so this should be fine. If it isn't, tell the user to run from a "Developer PowerShell for VS 2022" or restart their shell after the BuildTools install.
- `cmake` not found → step 4 didn't take; re-run.
- `Spectre libs` warning → step 5/6 fallback; user must add via GUI.

### 8. Report

When all checks pass, report:
- What was newly installed (vs. already present).
- The output of `cargo --version`, `cmake --version`, and the final `cargo check` last line.

Tell the user how to launch Zed once for verification:
```
cd <repo-root>
$env:Path = "C:\Program Files\CMake\bin;" + $env:Path
cargo run --bin zed -- --foreground
```

## Notes

- Steps are idempotent — the skill is safe to re-run after a failure.
- Never run `rustup self uninstall`, never delete `target/`, never modify the user's existing toolchain channels.
- If disk space is the failure mode (`No space left on device`, `disk full`), stop and tell the user. The Zed `target/` directory plus dependencies needs roughly **80 GB** of free disk for comfortable development.
- The skill does not push, pull, or alter the git working tree beyond the single `git config --global` line in step 2.
