use std::collections::{BTreeMap, BTreeSet};

use fluent_syntax::ast;

use super::{Bundles, EMBEDDED_BUNDLES, FluentArgs, Locale};

struct CatalogPattern {
    filename: &'static str,
    pattern: ast::Pattern<&'static str>,
}

#[derive(Default)]
struct PatternDetails<'a> {
    variables: BTreeSet<String>,
    patterns: Vec<&'a ast::Pattern<&'static str>>,
}

fn catalog(locale: Locale) -> BTreeMap<String, CatalogPattern> {
    let mut catalog = BTreeMap::new();
    for &(entry_locale, filename, content) in EMBEDDED_BUNDLES {
        if entry_locale != locale.as_bcp47() {
            continue;
        }
        let resource = fluent_syntax::parser::parse(content)
            .unwrap_or_else(|(_, errors)| panic!("{entry_locale}/{filename}: {errors:?}"));
        for entry in resource.body {
            if let ast::Entry::Message(message) = entry {
                if let Some(pattern) = message.value {
                    assert!(
                        catalog
                            .insert(
                                message.id.name.to_owned(),
                                CatalogPattern { filename, pattern }
                            )
                            .is_none(),
                        "duplicate message {entry_locale}/{}",
                        message.id.name
                    );
                }
                for attribute in message.attributes {
                    let key = format!("{}.{}", message.id.name, attribute.id.name);
                    assert!(
                        catalog
                            .insert(
                                key.clone(),
                                CatalogPattern {
                                    filename,
                                    pattern: attribute.value
                                }
                            )
                            .is_none(),
                        "duplicate attribute {entry_locale}/{key}"
                    );
                }
            }
        }
    }
    assert!(!catalog.is_empty(), "{locale} catalog must not be empty");
    catalog
}

fn pattern_details<'a>(pattern: &'a ast::Pattern<&'static str>) -> PatternDetails<'a> {
    let mut details = PatternDetails::default();
    collect_pattern(pattern, &mut details);
    details
}

fn collect_pattern<'a>(pattern: &'a ast::Pattern<&'static str>, details: &mut PatternDetails<'a>) {
    details.patterns.push(pattern);
    for element in &pattern.elements {
        if let ast::PatternElement::Placeable { expression } = element {
            collect_expression(expression, details);
        }
    }
}

fn collect_expression<'a>(
    expression: &'a ast::Expression<&'static str>,
    details: &mut PatternDetails<'a>,
) {
    match expression {
        ast::Expression::Inline(inline) => collect_inline(inline, details),
        ast::Expression::Select { selector, variants } => {
            collect_inline(selector, details);
            for variant in variants {
                collect_pattern(&variant.value, details);
            }
        }
    }
}

fn collect_inline<'a>(
    inline: &'a ast::InlineExpression<&'static str>,
    details: &mut PatternDetails<'a>,
) {
    match inline {
        ast::InlineExpression::VariableReference { id } => {
            details.variables.insert(id.name.to_owned());
        }
        ast::InlineExpression::Placeable { expression } => collect_expression(expression, details),
        ast::InlineExpression::FunctionReference { arguments, .. } => {
            collect_arguments(arguments, details);
        }
        ast::InlineExpression::TermReference { arguments, .. } => {
            if let Some(arguments) = arguments {
                collect_arguments(arguments, details);
            }
        }
        ast::InlineExpression::StringLiteral { .. }
        | ast::InlineExpression::NumberLiteral { .. }
        | ast::InlineExpression::MessageReference { .. } => {}
    }
}

fn collect_arguments<'a>(
    arguments: &'a ast::CallArguments<&'static str>,
    details: &mut PatternDetails<'a>,
) {
    for argument in &arguments.positional {
        collect_inline(argument, details);
    }
    for argument in &arguments.named {
        collect_inline(&argument.value, details);
    }
}

#[test]
fn embedded_messages_and_attributes_render_without_fluent_errors() {
    let bundles = Bundles::load().expect("production bundles must load");
    let mut failures = Vec::new();
    for locale in [Locale::En, Locale::ZhCn] {
        let catalog = catalog(locale);
        let variables: BTreeSet<String> = catalog
            .values()
            .flat_map(|entry| pattern_details(&entry.pattern).variables)
            .collect();
        let locale_bundle = bundles
            .locale_bundle(locale)
            .expect("supported locale must load");
        for number in [1, 2] {
            let mut args = FluentArgs::new();
            for variable in &variables {
                args.set(variable.as_str(), number);
            }
            for (key, entry) in &catalog {
                // Check each select branch as well as the complete message, including branches
                // that this dummy selector value would otherwise leave unvisited.
                for pattern in pattern_details(&entry.pattern).patterns {
                    let mut errors = Vec::new();
                    let _ = locale_bundle
                        .bundle
                        .format_pattern(pattern, Some(&args), &mut errors);
                    if !errors.is_empty() {
                        failures.push(format!("{locale}/{}:{key}: {errors:?}", entry.filename));
                    }
                }
            }
        }
    }
    assert!(
        failures.is_empty(),
        "Fluent format errors:\n{}",
        failures.join("\n")
    );
}

#[test]
fn embedded_message_and_attribute_variables_match_between_languages() {
    let en = catalog(Locale::En);
    let zh_cn = catalog(Locale::ZhCn);
    assert_eq!(
        en.keys().collect::<BTreeSet<_>>(),
        zh_cn.keys().collect::<BTreeSet<_>>(),
        "message values and attributes must exist in both languages"
    );
    let mut failures = Vec::new();
    for (key, entry) in &en {
        let en_variables = pattern_details(&entry.pattern).variables;
        let zh_variables = pattern_details(&zh_cn[key].pattern).variables;
        if en_variables != zh_variables {
            failures.push(format!(
                "{key}: en={en_variables:?}, zh-CN={zh_variables:?}"
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "Fluent variable mismatches:\n{}",
        failures.join("\n")
    );
}
