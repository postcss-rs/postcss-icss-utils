use recursive_parser::parser::RuleOrAtRuleOrDecl;
use regex::{Captures, Regex};
use std::{borrow::Cow, collections::HashMap};

pub mod macro_utils;

pub fn replace_value_symbols<'a>(
    value: &'a str,
    replacements: HashMap<&'a str, &'a str>,
) -> String {
    let reg = regex!(r#"[$]?[\w-]+"#);
    reg.replace_all(value, |cap: &Captures| {
        if let Some(rep) = replacements.get(&cap[0]) {
            rep.to_string()
        } else {
            cap[0].to_string()
        }
    })
    .to_string()
}

pub fn replace_symbols<'a>(
    node: &'a mut RuleOrAtRuleOrDecl<'a>,
    replacements: HashMap<&'a str, &'a str>,
) {
    match node {
        RuleOrAtRuleOrDecl::Rule(rule) => {
            if !rule.selector.is_empty() {
                let value = replace_value_symbols(&rule.selector, replacements);
                rule.selector = Cow::Owned(value);
            }
        }
        RuleOrAtRuleOrDecl::AtRule(at_rule) => {
            if !at_rule.params.is_empty() {
                let value = replace_value_symbols(&at_rule.params, replacements);
                at_rule.params = Cow::Owned(value);
            }
        }
        RuleOrAtRuleOrDecl::Declaration(decl) => {
            if !decl.value.is_empty() {
                let value = replace_value_symbols(&decl.value, replacements);
                decl.value = Cow::Owned(value);
            }
        },
    }
}
