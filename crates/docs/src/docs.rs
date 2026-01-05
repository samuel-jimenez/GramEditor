use std::{borrow::Cow, sync::Arc};

use app_actions::{OpenDocs, OpenDocsAt};
use assets::Docs;
use editor::EditorEvent;
use gpui::{
    Entity, EventEmitter, FocusHandle, Focusable, KeyContext, ScrollHandle, StyleRefinement,
    TextStyleRefinement, WeakEntity,
};
use language::LanguageRegistry;
use markdown::{Markdown, MarkdownElement, MarkdownStyle};
use settings::Settings as _;
use theme::ThemeSettings;
use ui::{ScrollAxes, Tooltip, WithScrollbar as _, prelude::*};
use workspace::{Item, ItemHandle, Workspace};

pub fn init(cx: &mut App) {
    cx.observe_new(DocumentationView::register).detach();
}

pub(crate) struct DocumentationView {
    focus_handle: FocusHandle,
    scroll_handle: ScrollHandle,
    markdown: Entity<Markdown>,
    language_registry: Arc<LanguageRegistry>,
}

impl EventEmitter<EditorEvent> for DocumentationView {}

impl Render for DocumentationView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let buffer_size = settings.buffer_font_size(cx);
        let ui_font_family = settings.ui_font.family.clone();
        let ui_font_fallbacks = settings.ui_font.fallbacks.clone();
        let ui_font_features = settings.ui_font.features.clone();
        let buffer_font_family = settings.buffer_font.family.clone();
        let buffer_font_features = settings.buffer_font.features.clone();
        let buffer_font_fallbacks = settings.buffer_font.fallbacks.clone();
        let mut base_text_style = window.text_style();
        base_text_style.refine(&TextStyleRefinement {
            font_family: Some(ui_font_family),
            font_features: Some(ui_font_features),
            font_fallbacks: ui_font_fallbacks,
            color: Some(cx.theme().colors().editor_foreground),
            ..Default::default()
        });
        let line_height = (2 * buffer_size).round();

        let markdown_style = MarkdownStyle {
            base_text_style,
            code_block: StyleRefinement::default().my(rems(1.)).font_buffer(cx),
            inline_code: TextStyleRefinement {
                background_color: Some(cx.theme().colors().editor_background.opacity(0.5)),
                font_family: Some(buffer_font_family),
                font_features: Some(buffer_font_features),
                font_fallbacks: buffer_font_fallbacks,
                ..Default::default()
            },
            rule_color: cx.theme().colors().border,
            block_quote_border_color: Color::Muted.color(cx),
            block_quote: TextStyleRefinement {
                color: Some(Color::Muted.color(cx)),
                ..Default::default()
            },
            link: TextStyleRefinement {
                color: Some(Color::Accent.color(cx)),
                underline: Some(gpui::UnderlineStyle {
                    thickness: px(1.),
                    color: Some(Color::Accent.color(cx)),
                    wavy: false,
                }),
                ..Default::default()
            },
            height_is_multiple_of_line_height: true,
            syntax: cx.theme().syntax().clone(),
            selection_background_color: cx.theme().colors().element_selection_background,
            ..Default::default()
        };
        let theme = cx.theme();

        let child = v_flex()
            .id("documentation-markdown-view")
            .max_w(Rems(48.0))
            .size_full()
            .gap_1_5()
            .child(
                MarkdownElement::new(self.markdown.clone(), markdown_style)
                    .code_block_renderer(markdown::CodeBlockRenderer::Default {
                        copy_button: true,
                        copy_button_on_hover: true,
                        border: true,
                    })
                    .on_url_click(open_doc_url),
            )
            .track_scroll(&self.scroll_handle);

        v_flex()
            .id("DocumentationView")
            .track_focus(&self.focus_handle(cx))
            .text_size(buffer_size)
            .line_height(line_height)
            .size_full()
            .p_4()
            .gap_2()
            .bg(theme.colors().editor_background)
            .child(
                v_flex().gap_2().child(
                    h_flex()
                        .gap_4()
                        .child(
                            IconButton::new("doc-view-toc", IconName::Library)
                                .tooltip(Tooltip::text("Table of Contents"))
                                .on_click(move |_, window, cx| {
                                    open_doc_url("gram://docs/SUMMARY.md".into(), window, cx);
                                }),
                        )
                        .child(
                            IconButton::new("doc-view-back", IconName::ArrowLeft)
                                .tooltip(Tooltip::text("Back"))
                                .on_click(|_, _window, _cx| {}),
                        )
                        .child(
                            IconButton::new("doc-view-forward", IconName::ArrowRight)
                                .tooltip(Tooltip::text("Forward"))
                                .on_click(|_, _window, _cx| {}),
                        )
                        .child(
                            h_flex()
                                .key_context({
                                    let mut context = KeyContext::new_with_defaults();
                                    context.add("BufferSearchBar");
                                    context
                                })
                                .size_full()
                                .h_8()
                                .pl_2()
                                .pr_1()
                                .py_1()
                                .border_1()
                                .border_color(theme.colors().border)
                                .rounded_md()
                                .child(div()),
                        ),
                ),
            )
            .child(
                v_flex()
                    .size_full()
                    .p_2()
                    .gap_4()
                    .child(child)
                    .overflow_hidden()
                    .custom_scrollbars(
                        ui::Scrollbars::new(ScrollAxes::Both)
                            .tracked_scroll_handle(&self.scroll_handle)
                            .with_track_along(ScrollAxes::Both, theme.colors().panel_background)
                            .tracked_entity(cx.entity_id())
                            .notify_content(),
                        window,
                        cx,
                    ),
            )
    }
}

pub fn open_doc_url(url: SharedString, window: &mut Window, cx: &mut App) {
    let url = url.to_string();
    let url = url
        .strip_prefix("./")
        .map(|url| "gram://docs/".to_owned() + url)
        .unwrap_or(url);
    let url = if url.starts_with("gram://docs/") && !url.ends_with(".md") {
        url + ".md"
    } else {
        url
    };
    log::info!("{}", url);
    if url.starts_with("gram://docs/") {
        window.dispatch_action(Box::new(OpenDocsAt { path: url }), cx);
    } else {
        cx.open_url(&url);
    }
}

impl DocumentationView {
    pub fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _cx: &mut Context<Workspace>,
    ) {
        workspace
            .register_action(move |workspace, _: &OpenDocs, window, cx| {
                DocumentationView::open_documentation_page(workspace, None, window, cx);
                cx.notify();
            })
            .register_action(
                move |workspace, OpenDocsAt { path }: &OpenDocsAt, window, cx| {
                    DocumentationView::open_documentation_page(
                        workspace,
                        Some(path.into()),
                        window,
                        cx,
                    );
                    cx.notify();
                },
            );
    }

    fn new(
        text: SharedString,
        _workspace: WeakEntity<Workspace>,
        window: &mut Window,
        language_registry: Arc<LanguageRegistry>,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        cx.on_focus_in(&focus_handle, window, Self::focus_in)
            .detach();

        let markdown = cx.new(|cx| Markdown::new(text, Some(language_registry.clone()), None, cx));

        let this = Self {
            focus_handle,
            markdown,
            language_registry,
            scroll_handle: ScrollHandle::new(),
        };
        this
    }

    fn focus_in(&mut self, window: &mut Window, _cx: &mut Context<Self>) {
        if self.focus_handle.is_focused(window) {}
    }

    fn open_documentation_page(
        workspace: &mut Workspace,
        path: Option<String>,
        window: &mut Window,
        cx: &mut Context<'_, Workspace>,
    ) {
        let language_registry = workspace.project().read(cx).languages().clone();
        let workspace_handle = workspace.weak_handle();
        let path = path.unwrap_or("SUMMARY.md".into());
        let path = path.strip_prefix("gram://docs/").unwrap_or(&path);
        if let Some(text) = get_docs(&path) {
            if let Some(existing) = workspace.item_of_type::<DocumentationView>(cx) {
                let is_active = workspace
                    .active_item(cx)
                    .is_some_and(|item| item.item_id() == existing.item_id());

                existing.update(cx, |view, cx| view.update_text(text.into(), cx));
                workspace.activate_item(&existing, true, !is_active, window, cx);
            } else {
                let view = cx.new(|cx| {
                    DocumentationView::new(
                        text.into(),
                        workspace_handle,
                        window,
                        language_registry,
                        cx,
                    )
                });
                workspace.add_item_to_active_pane(Box::new(view), None, true, window, cx);
            }
        }
    }

    pub(crate) fn update_text(&mut self, text: SharedString, cx: &mut Context<Self>) {
        self.markdown =
            cx.new(|cx| Markdown::new(text, Some(self.language_registry.clone()), None, cx));
        cx.notify();
    }
}

fn get_docs<'b>(path: &str) -> Option<Cow<'b, str>> {
    if let Some(text) = Docs::get(&path) {
        Some(match text.data {
            Cow::Borrowed(bytes) => Cow::Borrowed(std::str::from_utf8(bytes).unwrap()),
            Cow::Owned(bytes) => Cow::Owned(String::from_utf8(bytes).unwrap()),
        })
    } else {
        None
    }
}

impl Focusable for DocumentationView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for DocumentationView {
    type Event = EditorEvent;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Documentation".into()
    }
}
