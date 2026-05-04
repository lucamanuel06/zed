# Database Viewer — Core Scaffold Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Land a working `database_viewer` crate in the Zed fork that opens a full-pane workspace tab via a status bar icon, command palette, and `database_viewer::Open` action — with stub modules ready for sub-projects 2–6.

**Architecture:** New native Rust crate `crates/database_viewer/`. The view implements `workspace::Item` so it opens as an editor-area tab (singleton enforced by the action handler, like `acp_tools::AcpTools`). A `StatusItemView` button in the right side of the bottom status bar dispatches the open action. Initialization registers the action and adds the status item via `cx.observe_new::<Workspace>` (same pattern used by `acp_tools::init`).

**Tech Stack:** Rust, GPUI, Zed `workspace` crate, Zed `ui` crate (`Icon`, `IconButton`, `IconName::DatabaseZap`).

**Spec:** `docs/superpowers/specs/2026-05-04-database-viewer-scaffold-design.md`

---

## File Structure

**Created:**

| Path | Responsibility |
|------|----------------|
| `crates/database_viewer/Cargo.toml` | Crate manifest, deps on `gpui`, `ui`, `workspace`. |
| `crates/database_viewer/src/database_viewer.rs` | Lib root: `init(cx)`, action declaration, public re-exports. |
| `crates/database_viewer/src/view.rs` | `DatabaseViewerView` — `workspace::Item` + `Render` + `Focusable`. |
| `crates/database_viewer/src/status_item.rs` | `DatabaseViewerStatusItem` — `StatusItemView` rendering an icon button. |
| `crates/database_viewer/src/connections/mod.rs` | Empty stub for sub-project 2. |
| `crates/database_viewer/src/schema/mod.rs` | Empty stub for sub-project 3. |
| `crates/database_viewer/src/query/mod.rs` | Empty stub for sub-project 4. |
| `crates/database_viewer/src/results/mod.rs` | Empty stub for sub-project 5. |
| `crates/database_viewer/src/drivers/mod.rs` | Empty stub for sub-project 6. |

**Modified:**

| Path | Change |
|------|--------|
| `Cargo.toml` (workspace) | Add `"crates/database_viewer"` to `members` (alphabetical, after `csv_preview`); add `database_viewer = { path = "crates/database_viewer" }` to `[workspace.dependencies]` (alphabetical, after `csv_preview`). |
| `crates/zed/Cargo.toml` | Add `database_viewer.workspace = true` under `[dependencies]` (alphabetical position). |
| `crates/zed/src/main.rs` | Add `database_viewer::init(cx);` next to `acp_tools::init(cx);` (line 688). |
| `crates/zed/src/zed.rs` | In `initialize_workspace`, construct `DatabaseViewerStatusItem` and add it to the status bar's right side, alongside the existing `image_info` etc. (around line 591). |

---

## Task 1: Create the crate manifest and lib root with `init` placeholder

**Files:**
- Create: `crates/database_viewer/Cargo.toml`
- Create: `crates/database_viewer/src/database_viewer.rs`

- [ ] **Step 1: Write `crates/database_viewer/Cargo.toml`**

```toml
[package]
name = "database_viewer"
version = "0.1.0"
edition.workspace = true
publish.workspace = true
license = "GPL-3.0-or-later"

[lints]
workspace = true

[lib]
path = "src/database_viewer.rs"
doctest = false

[dependencies]
anyhow.workspace = true
gpui.workspace = true
serde.workspace = true
ui.workspace = true
workspace.workspace = true
```

- [ ] **Step 2: Write `crates/database_viewer/src/database_viewer.rs` (initial — stubs only)**

```rust
mod connections;
mod drivers;
mod query;
mod results;
mod schema;
mod status_item;
mod view;

use gpui::{App, actions};

pub use status_item::DatabaseViewerStatusItem;
pub use view::DatabaseViewerView;

actions!(
    database_viewer,
    [
        /// Open the Database Viewer in a new editor-area tab,
        /// or focus the existing one if it is already open.
        Open
    ]
);

/// Initializes the database viewer feature. Call once during app startup.
pub fn init(_cx: &mut App) {
    // Action registration and status bar wiring are added in later tasks.
}
```

- [ ] **Step 3: Add stub module files**

Each of these is a single empty file:

```rust
// crates/database_viewer/src/connections/mod.rs
//! Connection storage and credential handling. Implemented in sub-project 2.
```

```rust
// crates/database_viewer/src/schema/mod.rs
//! Schema browser tree. Implemented in sub-project 3.
```

```rust
// crates/database_viewer/src/query/mod.rs
//! SQL query editor. Implemented in sub-project 4.
```

```rust
// crates/database_viewer/src/results/mod.rs
//! Query results grid. Implemented in sub-project 5.
```

```rust
// crates/database_viewer/src/drivers/mod.rs
//! Database driver abstraction. Implemented in sub-project 6.
```

- [ ] **Step 4: Add placeholder `view.rs` and `status_item.rs` so the lib compiles**

```rust
// crates/database_viewer/src/view.rs
//! Filled in by Task 3.
```

```rust
// crates/database_viewer/src/status_item.rs
//! Filled in by Task 4.
```

- [ ] **Step 5: Wire the crate into the workspace `Cargo.toml`**

In the repo-root `Cargo.toml`:

In the `members` array, add a new line after `"crates/csv_preview",` (around line 45):

```toml
    "crates/csv_preview",
    "crates/database_viewer",
    "crates/dap",
```

In `[workspace.dependencies]`, add a new line after the `csv_preview` entry (around line 309):

```toml
csv_preview = { path = "crates/csv_preview"}
database_viewer = { path = "crates/database_viewer" }
dap = { path = "crates/dap" }
```

- [ ] **Step 6: Verify the new crate compiles standalone**

Run: `cargo check -p database_viewer`
Expected: `Finished` with no errors. The temporary `pub use` of `DatabaseViewerStatusItem` / `DatabaseViewerView` referencing not-yet-defined items will fail; if so, comment out those re-exports for now and uncomment in the task that defines each type.

If the `pub use` lines do fail, replace `database_viewer.rs` for now with:

```rust
mod connections;
mod drivers;
mod query;
mod results;
mod schema;
mod status_item;
mod view;

use gpui::{App, actions};

actions!(
    database_viewer,
    [
        /// Open the Database Viewer in a new editor-area tab,
        /// or focus the existing one if it is already open.
        Open
    ]
);

/// Initializes the database viewer feature. Call once during app startup.
pub fn init(_cx: &mut App) {}
```

Then re-run `cargo check -p database_viewer`. Expected: PASS.

- [ ] **Step 7: Commit**

```bash
git add Cargo.toml crates/database_viewer/
git commit -m "database_viewer: scaffold crate and stub modules"
```

---

## Task 2: Add `database_viewer` dependency to the `zed` binary crate

**Files:**
- Modify: `crates/zed/Cargo.toml`

- [ ] **Step 1: Add the dependency**

Open `crates/zed/Cargo.toml`, find the `[dependencies]` table, and insert (alphabetical position — between `dap_adapters` / `db_kvp` style entries, just before `db.workspace = true`):

```toml
database_viewer.workspace = true
```

If unsure of exact alphabetical placement, place it next to other `d*`-prefixed workspace deps. Cargo does not require alphabetical order; the codebase keeps it for readability.

- [ ] **Step 2: Verify the binary crate still compiles**

Run: `cargo check -p zed`
Expected: PASS. The dep is unused so far; Rust will not warn about an unused workspace dependency.

- [ ] **Step 3: Commit**

```bash
git add crates/zed/Cargo.toml
git commit -m "zed: depend on database_viewer crate"
```

---

## Task 3: Implement `DatabaseViewerView` (workspace `Item`)

**Files:**
- Modify: `crates/database_viewer/src/view.rs`

Reference patterns: `crates/agent_ui/src/agent_registry_ui.rs:642-668` (`AgentRegistryPage`) for an `Item` impl that opens as a tab; `crates/csv_preview/src/csv_preview.rs:248` (`CsvPreviewView`).

- [ ] **Step 1: Replace `view.rs` with the full implementation**

```rust
use gpui::{
    App, Context, Entity, EventEmitter, FocusHandle, Focusable, IntoElement, ParentElement, Render,
    SharedString, Styled, Window, div,
};
use ui::{Color, Icon, IconName, prelude::*};
use workspace::item::{Item, ItemEvent};

pub struct DatabaseViewerView {
    focus_handle: FocusHandle,
}

impl DatabaseViewerView {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl EventEmitter<ItemEvent> for DatabaseViewerView {}

impl Focusable for DatabaseViewerView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for DatabaseViewerView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .child(
                Label::new("Database Viewer — no connections")
                    .color(Color::Muted),
            )
    }
}

impl Item for DatabaseViewerView {
    type Event = ItemEvent;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Database Viewer".into()
    }

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::DatabaseZap))
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Database Viewer Page Opened")
    }

    fn show_toolbar(&self) -> bool {
        false
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<workspace::WorkspaceId>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Entity<Self>>
    where
        Self: Sized,
    {
        Some(cx.new(|cx| Self::new(cx)))
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(workspace::item::ItemEvent)) {
        f(*event);
    }
}
```

- [ ] **Step 2: Re-enable the `pub use` in `database_viewer.rs`**

Edit `crates/database_viewer/src/database_viewer.rs`. Replace the line `// pub use view::DatabaseViewerView;` (or add it back if it was removed in Task 1 Step 6) so the file ends up with:

```rust
mod connections;
mod drivers;
mod query;
mod results;
mod schema;
mod status_item;
mod view;

use gpui::{App, actions};

pub use view::DatabaseViewerView;

actions!(
    database_viewer,
    [
        /// Open the Database Viewer in a new editor-area tab,
        /// or focus the existing one if it is already open.
        Open
    ]
);

/// Initializes the database viewer feature. Call once during app startup.
pub fn init(_cx: &mut App) {}
```

- [ ] **Step 3: Verify the crate compiles**

Run: `cargo check -p database_viewer`
Expected: PASS.

If signatures differ from the actual `workspace::item::Item` trait in this Zed checkout (Zed evolves), open `crates/workspace/src/item.rs`, find `pub trait Item`, and adjust:
- `tab_content_text` parameter list
- `tab_icon` parameter list
- `to_item_events` closure parameter type

Re-run `cargo check -p database_viewer` until PASS.

- [ ] **Step 4: Commit**

```bash
git add crates/database_viewer/src/view.rs crates/database_viewer/src/database_viewer.rs
git commit -m "database_viewer: add empty workspace Item view"
```

---

## Task 4: Implement `DatabaseViewerStatusItem`

**Files:**
- Modify: `crates/database_viewer/src/status_item.rs`

Reference pattern: `crates/language_selector/src/active_buffer_language.rs:13-89` (the `ActiveBufferLanguage` status item).

- [ ] **Step 1: Replace `status_item.rs`**

```rust
use gpui::{
    App, Context, IntoElement, ParentElement, Render, Styled, WeakEntity, Window, div,
};
use ui::{IconButton, IconName, IconSize, Tooltip, prelude::*};
use workspace::{StatusItemView, Workspace, item::ItemHandle};

use crate::Open;

pub struct DatabaseViewerStatusItem {
    workspace: WeakEntity<Workspace>,
}

impl DatabaseViewerStatusItem {
    pub fn new(workspace: &Workspace) -> Self {
        Self {
            workspace: workspace.weak_handle(),
        }
    }
}

impl Render for DatabaseViewerStatusItem {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let workspace = self.workspace.clone();
        div().child(
            IconButton::new("database-viewer-status", IconName::DatabaseZap)
                .icon_size(IconSize::Small)
                .tooltip(|_window, cx| Tooltip::for_action("Open Database Viewer", &Open, cx))
                .on_click(cx.listener(move |_, _, window, cx| {
                    if let Some(workspace) = workspace.upgrade() {
                        workspace.update(cx, |workspace, cx| {
                            workspace.dispatch_action(Box::new(Open), window, cx);
                        });
                    }
                })),
        )
    }
}

impl StatusItemView for DatabaseViewerStatusItem {
    fn set_active_pane_item(
        &mut self,
        _active_pane_item: Option<&dyn ItemHandle>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        // The button is always visible regardless of the active pane item.
    }
}
```

- [ ] **Step 2: Re-enable the `pub use` in `database_viewer.rs`**

Edit `crates/database_viewer/src/database_viewer.rs` so the top reads:

```rust
mod connections;
mod drivers;
mod query;
mod results;
mod schema;
mod status_item;
mod view;

use gpui::{App, actions};

pub use status_item::DatabaseViewerStatusItem;
pub use view::DatabaseViewerView;
```

(The `actions!` and `init` blocks below stay unchanged.)

- [ ] **Step 3: Verify the crate compiles**

Run: `cargo check -p database_viewer`
Expected: PASS.

If `IconButton` / `Tooltip::for_action` signatures differ in this checkout, open `crates/ui/src/components/button/icon_button.rs` and `crates/ui/src/components/tooltip.rs` to adapt.

- [ ] **Step 4: Commit**

```bash
git add crates/database_viewer/src/status_item.rs crates/database_viewer/src/database_viewer.rs
git commit -m "database_viewer: add status bar icon button"
```

---

## Task 5: Wire the `Open` action handler in `init`

**Files:**
- Modify: `crates/database_viewer/src/database_viewer.rs`

Reference pattern: `crates/acp_tools/src/acp_tools.rs:29-49` (`acp_tools::init`).

- [ ] **Step 1: Replace the body of `init` with the action registration**

Edit `crates/database_viewer/src/database_viewer.rs`. Add the imports and replace `init`:

```rust
mod connections;
mod drivers;
mod query;
mod results;
mod schema;
mod status_item;
mod view;

use gpui::{App, Context, actions};
use workspace::Workspace;

pub use status_item::DatabaseViewerStatusItem;
pub use view::DatabaseViewerView;

actions!(
    database_viewer,
    [
        /// Open the Database Viewer in a new editor-area tab,
        /// or focus the existing one if it is already open.
        Open
    ]
);

/// Initializes the database viewer feature. Call once during app startup.
pub fn init(cx: &mut App) {
    cx.observe_new(
        |workspace: &mut Workspace, _window, _cx: &mut Context<Workspace>| {
            workspace.register_action(open_database_viewer);
        },
    )
    .detach();
}

fn open_database_viewer(
    workspace: &mut Workspace,
    _: &Open,
    window: &mut gpui::Window,
    cx: &mut Context<Workspace>,
) {
    if let Some(existing) = workspace.item_of_type::<DatabaseViewerView>(cx) {
        workspace.activate_item(&existing, true, true, window, cx);
        return;
    }
    let view = cx.new(|cx| DatabaseViewerView::new(cx));
    workspace.add_item_to_active_pane(Box::new(view), None, true, window, cx);
}
```

- [ ] **Step 2: Verify the crate compiles**

Run: `cargo check -p database_viewer`
Expected: PASS.

- [ ] **Step 3: Commit**

```bash
git add crates/database_viewer/src/database_viewer.rs
git commit -m "database_viewer: register Open action with singleton focus"
```

---

## Task 6: Wire `init` into the `zed` binary

**Files:**
- Modify: `crates/zed/src/main.rs`

- [ ] **Step 1: Add the `init` call**

Open `crates/zed/src/main.rs` and find the line `acp_tools::init(cx);` (currently around line 688). Add immediately below it:

```rust
        acp_tools::init(cx);
        database_viewer::init(cx);
```

- [ ] **Step 2: Verify the binary still compiles**

Run: `cargo check -p zed`
Expected: PASS.

- [ ] **Step 3: Commit**

```bash
git add crates/zed/src/main.rs
git commit -m "zed: initialize database_viewer at startup"
```

---

## Task 7: Wire the status bar item into `initialize_workspace`

**Files:**
- Modify: `crates/zed/src/zed.rs`

- [ ] **Step 1: Construct the status item**

Open `crates/zed/src/zed.rs`. Find the block where existing right-side status items are constructed, just before the `workspace.status_bar().update(cx, |status_bar, cx| { ... })` call (around line 575). Add right after the `merge_conflict_indicator` line:

```rust
        let database_viewer_status =
            cx.new(|_| database_viewer::DatabaseViewerStatusItem::new(workspace));
```

- [ ] **Step 2: Add it to the status bar's right items**

Inside the `workspace.status_bar().update(cx, |status_bar, cx| { ... })` block, after the `status_bar.add_right_item(image_info, window, cx);` line (around line 591), add:

```rust
            status_bar.add_right_item(database_viewer_status, window, cx);
```

- [ ] **Step 3: Verify the binary still compiles**

Run: `cargo check -p zed`
Expected: PASS.

- [ ] **Step 4: Commit**

```bash
git add crates/zed/src/zed.rs
git commit -m "zed: show database viewer button in status bar"
```

---

## Task 8: Full release-mode build sanity check

**Files:** None modified.

- [ ] **Step 1: Build the workspace**

Run: `cargo build -p zed`
Expected: PASS. (May take many minutes; acceptable.)

If any unrelated warnings appear in `crates/database_viewer/`, fix them. Treat any error in `database_viewer` or its callsites in `crates/zed/` as blocking.

- [ ] **Step 2: No commit needed unless fixes were made**

If fixes were necessary, commit them:

```bash
git add -u
git commit -m "database_viewer: fix build warnings/errors uncovered by full build"
```

---

## Task 9: Manual smoke test

**Files:** None modified.

- [ ] **Step 1: Launch Zed from the build output**

Run (PowerShell):

```powershell
cargo run --bin zed -- --foreground
```

Expected: Zed window opens with the project of your choice (or a blank workspace).

- [ ] **Step 2: Verify the status bar icon**

Look at the bottom right of the Zed window. A small "database with zap" icon (`IconName::DatabaseZap`) must be present alongside the existing right-side status items.

Hover the icon. Tooltip must read: `Open Database Viewer`.

- [ ] **Step 3: Open the viewer via the status bar icon**

Click the icon.
Expected: A new editor-area tab labeled `Database Viewer` opens and becomes active. The tab shows the placeholder text `Database Viewer — no connections` centered in the pane. The tab icon is the same database icon.

- [ ] **Step 4: Verify singleton behavior**

Click the icon again.
Expected: The existing tab is re-focused. No second `Database Viewer` tab is created.

Close the tab. Click the icon.
Expected: A new `Database Viewer` tab opens.

- [ ] **Step 5: Open via the command palette**

Open the command palette (`Cmd-Shift-P` / `Ctrl-Shift-P`). Type `database viewer`.
Expected: An entry `database viewer: open` is shown.

Run it.
Expected: Same singleton open/focus behavior as Step 3/4.

- [ ] **Step 6: Document the result**

If every check passes, no further action.

If any check fails, halt and create a fix-up task before declaring the scaffold done. Do not paper over failures.

- [ ] **Step 7: No commit**

Smoke testing produces no diff.

---

## Self-Review Notes

The plan covers every section of the spec:

- Crate layout (Spec §Architecture) — Task 1.
- `DatabaseViewerView` with `Item`, `Render`, `Focusable`, `EventEmitter` (Spec §Components 1) — Task 3.
- `DatabaseViewerStatusItem` with `StatusItemView` (Spec §Components 2) — Task 4.
- `database_viewer::Open` action and singleton focus (Spec §Components 3) — Task 5.
- Status bar wiring (Spec §Components 4) — Task 7.
- `init(cx)` and `main.rs` wiring (Spec §Components 5) — Tasks 5, 6.
- Cargo dependencies and workspace registration (Spec §Cargo dependencies) — Tasks 1, 2.
- Manual smoke test (Spec §Testing) — Task 9.
- No keybind by default (Spec §Keybind) — implicit (no keymap edits).
- Acceptance criteria 1–6 (Spec §Acceptance criteria) — Tasks 6 (cargo check zed), 8 (full build), 9 (all five runtime checks).

Type names used consistently across tasks: `DatabaseViewerView`, `DatabaseViewerStatusItem`, `Open` (action), `database_viewer::init`. No placeholders. Each step is a discrete file edit, command, or commit.
