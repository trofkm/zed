pub mod time_spent;

use editor::{scroll::Autoscroll, Editor};
use gpui::{div, prelude::*, AnyWindowHandle, AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView, Render, SharedString, Styled, Subscription, View, ViewContext, VisualContext, Focusable};
use settings::Settings;
use text::{Bias, Point};
use theme::ActiveTheme;
use ui::{h_flex, prelude::*, v_flex, Label};
use util::paths::FILE_ROW_COLUMN_DELIMITER;
use workspace::ModalView;


pub struct WakatimeView {
    time: Option<String>,
}

impl ModalView for WakatimeView {}

impl WakatimeView {
    fn register(editor: &mut Editor, cx: &mut ViewContext<Editor>) {
        let handle = cx.view().downgrade();
        editor
            .register_action(move |_: &editor::actions::ToggleGoToLine, cx| {
                let Some(editor) = handle.upgrade() else { return };
                let Some(workspace) = editor.read(cx).workspace() else { return };
                
                workspace.update(cx, |workspace, cx|{
                    workspace.toggle_modal(cx, move |cx| WakatimeView::new(editor, cx));
                });
            })
            .detach();
    }
    pub fn new(active_editor: View<Editor>, cx: &mut ViewContext<Editor>) -> Self {
        let editor = active_editor.read(cx);
        let cursor = editor.selections.last::<Point>(cx).head();

        let line = cursor.row + 1;
        let column = cursor.column + 1;
    }
}
