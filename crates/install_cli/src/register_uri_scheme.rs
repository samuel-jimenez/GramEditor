use client::GRAM_URL_SCHEME;
use gpui::{AsyncApp, actions};

actions!(
    cli,
    [
        /// Registers the gram:// URL scheme handler.
        RegisterUriScheme
    ]
);

pub async fn register_uri_scheme(cx: &AsyncApp) -> anyhow::Result<()> {
    cx.update(|cx| cx.register_url_scheme(GRAM_URL_SCHEME))?
        .await
}
