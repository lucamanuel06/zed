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
pub fn init(_cx: &mut App) {
    // Action registration and status bar wiring are added in later tasks.
}
