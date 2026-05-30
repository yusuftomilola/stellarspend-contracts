//! Budget templates — predefined category allocations users can apply.

#![no_std]

use soroban_sdk::{contracttype, symbol_short, Env, Map, Symbol};

/// A named budget template mapping category names to percentage allocations.
/// All values must sum to 100.
#[contracttype]
#[derive(Clone, Debug)]
pub struct BudgetTemplate {
    pub name: Symbol,
    /// category -> percentage (0-100)
    pub allocations: Map<Symbol, u32>,
}

/// Returns the built-in "essentials" template (50/30/20 rule).
pub fn essentials_template(env: &Env) -> BudgetTemplate {
    let mut alloc = Map::new(env);
    alloc.set(symbol_short!("needs"), 50u32);
    alloc.set(symbol_short!("wants"), 30u32);
    alloc.set(symbol_short!("savings"), 20u32);
    BudgetTemplate { name: symbol_short!("essential"), allocations: alloc }
}

/// Validates that all allocation percentages sum to exactly 100.
pub fn validate_template(template: &BudgetTemplate) -> bool {
    let total: u32 = template.allocations.values().iter().sum();
    total == 100
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn essentials_template_sums_to_100() {
        let env = Env::default();
        let t = essentials_template(&env);
        assert!(validate_template(&t));
    }

    #[test]
    fn invalid_template_fails_validation() {
        let env = Env::default();
        let mut alloc = soroban_sdk::Map::new(&env);
        alloc.set(symbol_short!("a"), 60u32);
        let t = BudgetTemplate { name: symbol_short!("bad"), allocations: alloc };
        assert!(!validate_template(&t));
    }
}