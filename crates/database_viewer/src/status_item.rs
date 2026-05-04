use gpui::{Context, IntoElement, ParentElement, Render, WeakEntity, Window, div};
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
    }
}
