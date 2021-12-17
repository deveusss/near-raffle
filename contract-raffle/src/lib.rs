mod raffleticket;
mod internal;

use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::Gas;
use near_sdk::PromiseOrValue;
use crate::raffleticket::{RaffleTicket};

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::{env,log, near_bindgen, AccountId, Balance, BorshStorageKey, PanicOnDefault};

const BASE_GAS: u64 = 5_000_000_000_000;
const PROMISE_CALL: u64 = 5_000_000_000_000;
const GAS_FOR_FT_ON_TRANSFER: Gas = BASE_GAS + PROMISE_CALL;

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Available,
    Winning,
    Sold,
}

enum RaffleFunction{
    BuyPrize,
}



#[near_bindgen]
pub struct RaffleContract {
    ticket: RaffleTicket,
    fungible_token_account_id:AccountId,
}

#[near_bindgen]
impl FungibleTokenReceiver for RaffleContract {
    /// If given `msg: "take-my-money", immediately returns U128::From(0)
    /// Otherwise, makes a cross-contract call to own `value_please` function, passing `msg`
    /// value_please will attempt to parse `msg` as an integer and return a U128 version of it
    fn ft_on_transfer(
        &mut self,
        sender_id: ValidAccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        // Verifying that we were called by fungible token contract that we expect.
        assert!(
            env::predecessor_account_id() == self.fungible_token_account_id,
            "Only supports the one fungible token contract"
        );
        log!("in {} tokens from @{} ft_on_transfer, msg = {}", amount.0, sender_id.as_ref(), msg);
        match msg.as_str() {
            "take-my-money" => PromiseOrValue::Value(U128::from(0)),
            _ => {
                let prepaid_gas = env::prepaid_gas();
                let account_id = env::current_account_id();
                PromiseOrValue::Value(U128::from(0))
            }
        }
    }
}

#[near_bindgen]
impl RaffleContract {
    #[init]
    pub fn new(fungible_token_account_id: AccountId, tokens_per_ticket: i32, number_of_predefined: i16) -> Self {
        Self {
            ticket: RaffleTicket::new(tokens_per_ticket, number_of_predefined),
            fungible_token_account_id
        }
    }
}

