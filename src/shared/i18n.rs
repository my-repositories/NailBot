use fluent_bundle::{FluentBundle, FluentResource};
use unic_langid::LanguageIdentifier;

pub fn tr_hello(locale: &str, name: &str) -> String {
    let lang: LanguageIdentifier = locale.parse().unwrap_or_else(|_| "en".parse().expect("valid lang"));
    let mut bundle = FluentBundle::new(vec![lang]);
    let resource = FluentResource::try_new("hello = Hello, { $name }!".to_string()).expect("static resource");
    let _ = bundle.add_resource(resource);
    let message = bundle.get_message("hello").expect("message exists");
    let pattern = message.value().expect("pattern exists");
    let mut errors = vec![];
    let mut args = fluent_bundle::FluentArgs::new();
    args.set("name", name);
    bundle.format_pattern(pattern, Some(&args), &mut errors).to_string()
}
