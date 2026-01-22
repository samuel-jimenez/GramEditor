use std::sync::Arc;

use editor::EditorEvent;
use gpui::{EventEmitter, FocusHandle, Focusable, ListState, WeakEntity, actions};
use language::{BinaryStatus, LanguageRegistry};
use project::LspStore;
use ui::{
    ActiveTheme as _, App, Context, IntoElement, Render, Window, WithScrollbar as _, prelude::*,
};
use workspace::{Item, ItemHandle as _, Workspace};

use crate::lsp_button::{LanguageServerBinaryStatus, LanguageServers};

actions!(
    lsp,
    [
        /// Opens the language server configuration view.
        OpenLanguageServerConfig
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(LspConfigView::register).detach();
}

pub(crate) struct LspConfigView {
    workspace: WeakEntity<Workspace>,
    language_registry: Arc<LanguageRegistry>,
    lsp_store: WeakEntity<LspStore>,
    language_servers: LanguageServers,
    focus_handle: FocusHandle,
    list_state: ListState,
}

impl LspConfigView {
    pub fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _cx: &mut Context<Workspace>,
    ) {
        workspace.register_action(move |workspace, _: &OpenLanguageServerConfig, window, cx| {
            LspConfigView::open(workspace, window, cx);
            cx.notify();
        });
    }

    pub(crate) fn open(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<'_, Workspace>,
    ) {
        let language_registry = workspace.project().read(cx).languages().clone();
        let lsp_store = workspace.project().read(cx).lsp_store();
        let mut language_servers = LanguageServers::default();
        for (_, status) in lsp_store.read(cx).language_server_statuses() {
            language_servers.binary_statuses.insert(
                status.name.clone(),
                LanguageServerBinaryStatus {
                    status: BinaryStatus::None,
                    message: None,
                },
            );
        }
        let workspace_handle = workspace.weak_handle();
        if let Some(existing) = workspace.item_of_type::<LspConfigView>(cx) {
            let is_active = workspace
                .active_item(cx)
                .is_some_and(|item| item.item_id() == existing.item_id());
            workspace.activate_item(&existing, true, !is_active, window, cx);
        } else {
            let view = cx.new(|cx| {
                LspConfigView::new(
                    language_registry,
                    lsp_store.downgrade(),
                    language_servers,
                    workspace_handle,
                    window,
                    cx,
                )
            });
            workspace.add_item_to_active_pane(Box::new(view), None, true, window, cx);
        }
    }

    fn new(
        language_registry: Arc<LanguageRegistry>,
        lsp_store: WeakEntity<LspStore>,
        language_servers: LanguageServers,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let list_state = ListState::new(0, gpui::ListAlignment::Top, px(1000.));
        let focus_handle = cx.focus_handle();
        cx.on_focus_in(&focus_handle, window, Self::focus_in)
            .detach();
        let this = Self {
            workspace,
            language_registry,
            lsp_store,
            language_servers,
            focus_handle,
            list_state,
        };
        this
    }

    fn focus_in(&mut self, window: &mut Window, _cx: &mut Context<Self>) {
        if self.focus_handle.is_focused(window) {}
    }
}

impl EventEmitter<EditorEvent> for LspConfigView {}

impl Render for LspConfigView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        // get languages in current project
        //let languages = self.language_registry.language_names();
        v_flex()
            .id("LspConfigView")
            .key_context("LspConfigView")
            .track_focus(&self.focus_handle(cx))
            .size_full()
            .p_4()
            .gap_2()
            .bg(theme.colors().editor_background)
            .vertical_scrollbar_for(&self.list_state, window, cx)
    }
}

impl Focusable for LspConfigView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for LspConfigView {
    type Event = EditorEvent;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        SharedString::from("Language Server Configuration")
    }
}
