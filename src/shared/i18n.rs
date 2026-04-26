use std::collections::HashMap;

use fluent_bundle::{FluentArgs, FluentBundle, FluentResource, FluentValue};
use unic_langid::LanguageIdentifier;

pub fn normalize_locale(locale: &str) -> &'static str {
    let value = locale.trim().to_ascii_lowercase();
    match value.as_str() {
        "ru" | "ru-ru" => "ru",
        "en" | "en-us" | "en-gb" => "en",
        _ => "en",
    }
}

fn parse_locale(locale: &str) -> LanguageIdentifier {
    normalize_locale(locale)
        .parse()
        .unwrap_or_else(|_| "en".parse().expect("hardcoded 'en' locale must be valid"))
}

fn load_bundle(locale: &str) -> Option<FluentBundle<FluentResource>> {
    let normalized = normalize_locale(locale);
    let lang = parse_locale(normalized);
    let mut bundle = FluentBundle::new(vec![lang]);
    let source = match normalized {
        "ru" => include_str!("../../locales/ru.ftl"),
        _ => include_str!("../../locales/en.ftl"),
    };
    let resource = FluentResource::try_new(source.to_string()).ok()?;
    bundle.add_resource(resource).ok()?;
    Some(bundle)
}

pub fn tr(locale: &str, key: &str) -> String {
    tr_with_args(locale, key, &HashMap::new())
}

pub fn tr_with_args(locale: &str, key: &str, args: &HashMap<&str, String>) -> String {
    // Fallback chain: requested locale -> English -> key itself.
    for candidate in [normalize_locale(locale), "en"] {
        if let Some(bundle) = load_bundle(candidate) {
            if let Some(message) = bundle.get_message(key) {
                if let Some(pattern) = message.value() {
                    let mut errors = Vec::new();
                    let mut fluent_args = FluentArgs::new();
                    for (name, value) in args {
                        fluent_args.set(*name, FluentValue::from(value.as_str()));
                    }
                    return bundle
                        .format_pattern(pattern, Some(&fluent_args), &mut errors)
                        .to_string();
                }
            }
        }
    }
    key.to_string()
}
