//! Merchant tagging — associates a merchant identifier with a spending category.

#![no_std]

use soroban_sdk::{contracttype, panic_with_error, symbol_short, Address, Env, Symbol};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum MerchantTagError {
    InvalidMerchant = 1,
    Unauthorized = 2,
}

impl From<MerchantTagError> for soroban_sdk::Error {
    fn from(e: MerchantTagError) -> Self {
        soroban_sdk::Error::from_contract_error(e as u32)
    }
}

#[contracttype]
#[derive(Clone)]
enum TagKey { MerchantCategory(Symbol) }

/// Tags a merchant ID with a spending category.
pub fn tag_merchant(env: &Env, caller: &Address, merchant_id: Symbol, category: Symbol) {
    caller.require_auth();
    env.storage().persistent().set(&TagKey::MerchantCategory(merchant_id.clone()), &category);
    env.events().publish((symbol_short!("merchant"), symbol_short!("tagged")), (merchant_id, category));
}

/// Returns the category tag for a merchant, if one exists.
pub fn get_merchant_category(env: &Env, merchant_id: Symbol) -> Option<Symbol> {
    env.storage().persistent().get(&TagKey::MerchantCategory(merchant_id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env, symbol_short};

    #[test]
    fn tag_and_retrieve_merchant() {
        let env = Env::default();
        env.mock_all_auths();
        let caller = Address::generate(&env);
        tag_merchant(&env, &caller, symbol_short!("starbucks"), symbol_short!("food"));
        let cat = get_merchant_category(&env, symbol_short!("starbucks"));
        assert_eq!(cat, Some(symbol_short!("food")));
    }
}