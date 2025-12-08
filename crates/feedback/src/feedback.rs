use app_actions::feedback::{EmailGram, FileBugReport, RequestFeature};
use gpui::{App, ClipboardItem, PromptLevel, actions};
use system_specs::{CopySystemSpecsIntoClipboard, SystemSpecs};
use util::ResultExt;
use workspace::Workspace;

actions!(
    gram,
    [
        /// Opens the Gram repository on GitHub.
        OpenGramRepo,
    ]
);

const GRAM_REPO_URL: &str = "https://codeberg.org/krig/gram";

const REQUEST_FEATURE_URL: &str = "https://codeberg.org/krig/gram";

fn file_bug_report_url(_specs: &SystemSpecs) -> String {
    "https://codeberg.org/krig/gram".into()
}

fn email_gram_url(specs: &SystemSpecs) -> String {
    format!(
        concat!("mailto:k@ziran.se", "?", "body={}"),
        email_body(specs)
    )
}

fn email_body(specs: &SystemSpecs) -> String {
    let body = format!("\n\nSystem Information:\n\n{}", specs);
    urlencoding::encode(&body).to_string()
}

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace
            .register_action(|_, _: &CopySystemSpecsIntoClipboard, window, cx| {
                let specs = SystemSpecs::new(window, cx);

                cx.spawn_in(window, async move |_, cx| {
                    let specs = specs.await.to_string();

                    cx.update(|_, cx| {
                        cx.write_to_clipboard(ClipboardItem::new_string(specs.clone()))
                    })
                    .log_err();

                    cx.prompt(
                        PromptLevel::Info,
                        "Copied into clipboard",
                        Some(&specs),
                        &["OK"],
                    )
                    .await
                })
                .detach();
            })
            .register_action(|_, _: &RequestFeature, _, cx| {
                cx.open_url(REQUEST_FEATURE_URL);
            })
            .register_action(move |_, _: &FileBugReport, window, cx| {
                let specs = SystemSpecs::new(window, cx);
                cx.spawn_in(window, async move |_, cx| {
                    let specs = specs.await;
                    cx.update(|_, cx| {
                        cx.open_url(&file_bug_report_url(&specs));
                    })
                    .log_err();
                })
                .detach();
            })
            .register_action(move |_, _: &EmailGram, window, cx| {
                let specs = SystemSpecs::new(window, cx);
                cx.spawn_in(window, async move |_, cx| {
                    let specs = specs.await;
                    cx.update(|_, cx| {
                        cx.open_url(&email_gram_url(&specs));
                    })
                    .log_err();
                })
                .detach();
            })
            .register_action(move |_, _: &OpenGramRepo, _, cx| {
                cx.open_url(GRAM_REPO_URL);
            });
    })
    .detach();
}
