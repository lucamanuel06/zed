# Database Viewer — Core Panel Scaffold (Sub-project 1)

**Date:** 2026-05-04
**Branch:** `feature/database-extension`
**Fork:** `https://github.com/lucamanuel06/zed`
**Upstream:** `https://github.com/zed-industries/zed`

## Context

The user wants a database viewer in Zed comparable to the VS Code extension `cweijan.dbclient-jdbc`: a full GUI with connection tree, schema browser, SQL editor, and results grid, supporting all major databases (PostgreSQL, MySQL, SQLite, MSSQL, Oracle, MongoDB, Redis, etc.).

Zed's WASM extension API does **not** expose UI primitives (panels, status bar items, custom views). This feature must be implemented as a native Rust crate inside a Zed fork. Long-term goal: contribute a panel/UI extension API upstream, then port the database viewer to a publishable extension. That is out of scope for this spec.

## Decomposition (overall feature)

The full database viewer is broken into independent sub-projects, each with its own spec/plan/cycle:

1. **Core panel scaffold** — empty workspace `Item` view + status bar icon + action wiring. *(this spec)*
2. Connection manager — settings.json + OS keychain for passwords.
3. Schema browser — tree of databases → schemas → tables → columns.
4. Query editor — embed Zed `Editor` for SQL with run action.
5. Results grid — paginated, sortable table view of query output.
6. Driver layer — abstraction + per-DB implementations (PostgreSQL, MySQL, SQLite first; MSSQL/Oracle/Mongo/Redis later).
7. Data editing — inline cell edit, INSERT/UPDATE/DELETE.
8. Export/import — CSV/JSON.
9. Upstream panel-extension API — Zed core PR.
10. Port to extension — depends on (9).

This spec covers **only sub-project 1**.

## Goal

Deliver a working scaffold: empty Database Viewer as a full-pane workspace `Item` (editor-area tab), reachable via a bottom status bar icon, an action `database_viewer::Open`, and the command palette. Module stubs exist for sub-projects 2–6 so future work lands cleanly.

No database logic, no connection storage, no UI beyond a placeholder.

## Architecture

New crate `crates/database_viewer/`:

```
crates/database_viewer/
  Cargo.toml
  src/
    database_viewer.rs   # lib root, init(cx), action registration
    view.rs              # DatabaseViewerView (impl workspace::Item, Render, Focusable, EventEmitter)
    status_item.rs       # DatabaseViewerStatusItem (impl StatusItemView)
    connections/mod.rs   # stub for sub-project 2
    schema/mod.rs        # stub for sub-project 3
    query/mod.rs         # stub for sub-project 4
    results/mod.rs       # stub for sub-project 5
    drivers/mod.rs       # stub for sub-project 6
```

This matches the existing Zed convention where each panel/feature is its own crate (`git_ui`, `project_panel`, `terminal_view`, `agent_ui`).

## Components

### 1. `DatabaseViewerView`

A GPUI `Entity` implementing:

- `Focusable`
- `EventEmitter<()>` (placeholder; future events added per sub-project)
- `workspace::Item`:
  - `tab_content_text(_, _) -> SharedString` returns `"Database Viewer"`
  - `tab_icon(_, _) -> Option<Icon>` returns `Some(Icon::new(IconName::DatabaseZap))`
  - `clone_on_split(_, _, _) -> Option<Entity<Self>>` returns a new empty instance
- `Render`: placeholder centered text `"Database Viewer — no connections"`

Singleton behavior (one Database Viewer tab per workspace) is enforced by the `Open` action handler, not by an `Item` trait method — see "Action" below.

### 2. `DatabaseViewerStatusItem`

Implements `workspace::StatusItemView`. Renders an `IconButton` with `IconName::DatabaseZap` and tooltip `"Open Database Viewer"`. On click, dispatches `database_viewer::Open` to the active workspace.

### 3. Action

```rust
actions!(database_viewer, [Open]);
```

Handler `open_database_viewer(workspace: &mut Workspace, _: &Open, window, cx)`:

- Looks for an existing `DatabaseViewerView` item in the workspace; if found, activates it.
- Otherwise creates a new `DatabaseViewerView` entity and calls `workspace.add_item_to_active_pane(...)`.

The action is registered for every new `Workspace` via `cx.observe_new::<Workspace>(|workspace, _, _| workspace.register_action(open_database_viewer))`.

### 4. Status bar wiring

In the same `observe_new::<Workspace>` hook, the status item is created and added to the right side of the status bar:

```rust
let status_item = cx.new(|cx| DatabaseViewerStatusItem::new(workspace.weak_handle(), cx));
workspace.status_bar().update(cx, |bar, cx| {
    bar.add_right_item(status_item, window, cx);
});
```

### 5. `init(cx)` and main wiring

- `database_viewer::init(cx: &mut App)` performs both registrations above.
- Called from `crates/zed/src/zed.rs` during startup, alongside other panel inits.
- `database_viewer` added to workspace `Cargo.toml` members and to `crates/zed/Cargo.toml` dependencies.

## Data flow

None in this phase. State is an empty struct on `DatabaseViewerView`. Subsequent sub-projects fill it.

## Cargo dependencies

```toml
[dependencies]
gpui.workspace = true
ui.workspace = true
workspace.workspace = true
serde.workspace = true
anyhow.workspace = true
```

`zed_actions` only if shared actions are needed; otherwise the local `actions!` macro is sufficient.

## Error handling

Nothing fails in this phase. `Open` is idempotent: it focuses an existing tab if present, otherwise creates one.

## Testing

- **Manual smoke test:** run `zed --foreground` (or platform equivalent), confirm:
  - Database icon visible in the right side of the bottom status bar.
  - Clicking the icon opens a new editor-area tab titled `Database Viewer` with the placeholder text.
  - The tab icon matches the status bar icon.
  - Action `database_viewer: Open` appears in the command palette and opens/focuses the tab.
  - Opening twice focuses the existing tab rather than creating a duplicate.
- **No unit tests this phase** — the scaffold is UI-only with no logic to assert. The first real test suite arrives in sub-project 6 (driver layer).

## Keybind

No default keybind. Avoids conflicts with user setups. The action is discoverable via the command palette and the user can bind it themselves. A default may be proposed once usage settles.

## Out of scope (deferred)

- Any database connection logic.
- Persistent storage of connections (sub-project 2: settings.json + OS keychain).
- Schema/query/results UI (sub-projects 3–5).
- Driver crates (sub-project 6).
- Publishable extension form (sub-projects 9–10).
- Default keybind.

## Acceptance criteria

1. `cargo check -p database_viewer` succeeds.
2. `cargo check -p zed` succeeds.
3. Status bar icon appears at app startup.
4. Click on icon opens the Database Viewer tab once; subsequent clicks focus the existing tab.
5. `database_viewer: Open` is invokable from the command palette with the same behavior.
6. Tab icon and status bar icon both render `IconName::DatabaseZap`.
