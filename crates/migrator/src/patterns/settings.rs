pub const SETTINGS_ROOT_KEY_VALUE_PATTERN: &str = r#"(document
    (object
        (pair
            key: (string (string_content) @name)
            value: (_)  @value
        )
    )
)"#;

pub const SETTINGS_NESTED_KEY_VALUE_PATTERN: &str = r#"(document
    (object
        (pair
            key: (string (string_content) @parent_key)
            value: (object
                (pair
                    key: (string (string_content) @setting_name)
                    value: (_) @setting_value
                )
            )
        )
    )
)"#;

/// Migrate language settings,
/// calls `migrate_fn` with the top level object as well as all language settings under the "languages" key
/// Fails early if `migrate_fn` returns an error at any point
pub fn migrate_language_setting(
    value: &mut serde_json::Value,
    migrate_fn: fn(&mut serde_json::Value, path: &[&str]) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    migrate_fn(value, &[])?;
    let languages = value
        .as_object_mut()
        .and_then(|obj| obj.get_mut("languages"))
        .and_then(|languages| languages.as_object_mut());
    if let Some(languages) = languages {
        for (language_name, language) in languages.iter_mut() {
            let path = vec!["languages", language_name];
            migrate_fn(language, &path)?;
        }
    }
    Ok(())
}
