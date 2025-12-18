use fluent::{FluentArgs, FluentResource};
use fluent_bundle::bundle::FluentBundle;
use std::collections::HashMap;
use unic_langid::LanguageIdentifier;

/// Supported languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    En,
    Ru,
}

impl Language {
    pub fn from_code(code: &str) -> Self {
        match code.to_lowercase().as_str() {
            "ru" => Language::Ru,
            _ => Language::En,
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Language::En => "en",
            Language::Ru => "ru",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Language::En => "English",
            Language::Ru => "Русский",
        }
    }

    pub fn flag(&self) -> &'static str {
        match self {
            Language::En => "🇬🇧",
            Language::Ru => "🇷🇺",
        }
    }
}

type Bundle = FluentBundle<FluentResource, intl_memoizer::concurrent::IntlLangMemoizer>;

/// Internationalization manager
pub struct I18n {
    bundles: HashMap<Language, Bundle>,
}

impl I18n {
    /// Create new I18n instance with embedded translations
    pub fn new() -> Self {
        let mut bundles = HashMap::new();

        // Load English
        let en_ftl = include_str!("../../locales/en/main.ftl");
        if let Some(bundle) = Self::create_bundle("en", en_ftl) {
            bundles.insert(Language::En, bundle);
        }

        // Load Russian
        let ru_ftl = include_str!("../../locales/ru/main.ftl");
        if let Some(bundle) = Self::create_bundle("ru", ru_ftl) {
            bundles.insert(Language::Ru, bundle);
        }

        Self { bundles }
    }

    fn create_bundle(lang_code: &str, ftl_content: &str) -> Option<Bundle> {
        let langid: LanguageIdentifier = lang_code.parse().ok()?;
        let resource = FluentResource::try_new(ftl_content.to_string()).ok()?;

        let mut bundle: Bundle = FluentBundle::new_concurrent(vec![langid]);
        bundle.add_resource(resource).ok()?;

        Some(bundle)
    }

    /// Get translated message by key
    pub fn get(&self, lang: Language, key: &str) -> String {
        self.get_with_args(lang, key, None)
    }

    /// Get translated message with arguments
    pub fn get_with_args(&self, lang: Language, key: &str, args: Option<&FluentArgs>) -> String {
        // Try requested language, fallback to English
        for try_lang in [lang, Language::En] {
            if let Some(bundle) = self.bundles.get(&try_lang) {
                if let Some(msg) = bundle.get_message(key) {
                    if let Some(pattern) = msg.value() {
                        let mut errors = vec![];
                        let result = bundle.format_pattern(pattern, args, &mut errors);
                        if errors.is_empty() {
                            return result.to_string();
                        }
                    }
                }
            }
        }

        // Fallback to key itself
        tracing::warn!("Missing translation for key: {}", key);
        key.to_string()
    }

    /// Convenience method to get message for user's language code
    pub fn t(&self, lang_code: &str, key: &str) -> String {
        self.get(Language::from_code(lang_code), key)
    }

    /// Convenience method with args
    pub fn t_args(&self, lang_code: &str, key: &str, args: &FluentArgs) -> String {
        self.get_with_args(Language::from_code(lang_code), key, Some(args))
    }
}

impl Default for I18n {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper macro to create FluentArgs
#[macro_export]
macro_rules! fluent_args {
    ($($key:expr => $value:expr),* $(,)?) => {{
        let mut args = fluent::FluentArgs::new();
        $(
            args.set($key, $value);
        )*
        args
    }};
}
