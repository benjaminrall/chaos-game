mod default;
mod no_repeats;
mod star;

use std::collections::{HashMap, VecDeque};
use std::sync::OnceLock;
use crate::types::Vertex;

// Type alias for the rule function
pub type RuleFn = fn(previous_points: &VecDeque<&Vertex>, new_point: &Vertex) -> bool;

/// Struct holding all information needed to create a new rule.
pub struct Rule {
    pub name: &'static str,
    pub function: RuleFn,
    pub history: usize,
}

// Inventory collection for all RuleRegistration instances
inventory::collect!(Rule);

/// Constructs a mapping for each rule using their names.
fn get_rule_map() -> &'static HashMap<&'static str, &'static Rule> {
    static RULE_MAP: OnceLock<HashMap<&'static str, &Rule>> = OnceLock::new();
    RULE_MAP.get_or_init(|| {
        inventory::iter::<Rule>.into_iter().map(|reg| (reg.name, reg)).collect()
    })
}

// Looks up a rule by its registered name and returns its function.
pub fn get_rule_by_name(name: &str) -> Option<&&Rule> {
    get_rule_map().get(name)
}