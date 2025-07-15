//! Rule Management
//!
//! This module provides a system for defining and managing rules
//! that can be applied to various parts of the application.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::agent::Action;

// FIX: Added the missing fields that the logic depends on.
#[derive(Debug, Clone)]
pub struct Rule {
    pub name: String,
    pub description: String,
    pub target: String,
    pub action: Action,
}

pub fn load_rules() -> Vec<Rule> {
    // Placeholder - load rules from a file or define them here
    vec![]
}

pub fn find_rule_for_target(rules: &[Rule], target: &str) -> Option<Action> {
    for rule in rules {
        if target.contains(&rule.target) {
            return Some(rule.action.clone());
        }
    }
    None
}

pub struct RuleSet {
    rules: HashMap<String, Rule>,
}

impl RuleSet {
    pub fn new() -> Self {
        Self { rules: HashMap::new() }
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.insert(rule.name.clone(), rule);
    }

    pub fn get_rule(&self, name: &str) -> Option<&Rule> {
        self.rules.get(name)
    }

    pub fn evaluate(&self, target: &str) -> Action {
        for rule in self.rules.values() {
            if target.contains(&rule.target) {
                return rule.action.clone();
            }
        }
        Action::Allow
    }
}

impl Default for RuleSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_set() {
        let mut rule_set = RuleSet::new();
        let rule = Rule {
            name: "test_rule".to_string(),
            description: "A test rule".to_string(),
            action: Action::Deny,
            target: "danger".to_string(),
        };
        rule_set.add_rule(rule);

        assert_eq!(rule_set.evaluate("this is a dangerous string"), Action::Deny);
        assert_eq!(rule_set.evaluate("this is a safe string"), Action::Allow);
    }
}
