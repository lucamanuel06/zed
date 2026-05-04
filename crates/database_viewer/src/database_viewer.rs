mod connections;
mod drivers;
mod query;
mod results;
mod schema;
mod status_item;
mod view;

use gpui::{App, AppContext, Context, Window, actions};
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
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    if let Some(existing) = workspace.item_of_type::<DatabaseViewerView>(cx) {
        workspace.activate_item(&existing, true, true, window, cx);
        return;
    }
    let view = cx.new(|cx| DatabaseViewerView::new(cx));
    workspace.add_item_to_active_pane(Box::new(view), None, true, window, cx);
}
