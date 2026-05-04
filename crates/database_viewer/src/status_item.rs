use gpui::{Context, IntoElement, ParentElement, Render, Window, div};
use ui::{IconButton, IconName, IconSize, Tooltip, prelude::*};
use workspace::{StatusItemView, item::ItemHandle};

use crate::Open;

pub struct DatabaseViewerStatusItem;

impl DatabaseViewerStatusItem {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DatabaseViewerStatusItem {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for DatabaseViewerStatusItem {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child(
            IconButton::new("database-viewer-status", IconName::DatabaseZap)
                .icon_size(IconSize::Small)
                .tooltip(|_window, cx| Tooltip::for_action("Open Database Viewer", &Open, cx))
                .on_click(|_, window, cx| {
                    window.dispatch_action(Box::new(Open), cx);
                }),
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
    }
}
