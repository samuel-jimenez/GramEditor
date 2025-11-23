mod keymap;
mod settings;

pub(crate) use keymap::{
    KEYMAP_ACTION_ARRAY_ARGUMENT_AS_OBJECT_PATTERN, KEYMAP_ACTION_STRING_PATTERN,
};

pub(crate) use settings::{
    SETTINGS_NESTED_KEY_VALUE_PATTERN, SETTINGS_ROOT_KEY_VALUE_PATTERN, migrate_language_setting,
};
