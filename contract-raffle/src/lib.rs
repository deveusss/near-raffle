mod internal;
mod raffleticket;

use crate::raffleticket::RaffleTicket;
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::collections::LookupSet;
use near_sdk::collections::UnorderedMap;
use near_sdk::collections::UnorderedSet;

use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::Gas;
use near_sdk::PromiseOrValue;
use near_sdk::{env, log, near_bindgen, AccountId, Balance, BorshStorageKey, PanicOnDefault};
use std::convert::{AsRef, From, TryFrom};
const BASE_GAS: u64 = 5_000_000_000_000;
const PROMISE_CALL: u64 = 5_000_000_000_000;
const GAS_FOR_FT_ON_TRANSFER: Gas = BASE_GAS + PROMISE_CALL;

use internal::*;

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Available,
    Winning,
    Sold,
}

enum RaffleInstruction {
    Unknown,
    BuyPrize,
}

impl From<String> for RaffleInstruction {
    fn from(item: String) -> Self {
        match &item[..] {
            "buy_prize" => RaffleInstruction::BuyPrize,
            _ => RaffleInstruction::Unknown,
        }
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]

pub struct RaffleContract {
    ticket: RaffleTicket,
    fungible_token_account_id: AccountId,
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
        log!(
            "in {} tokens from @{} ft_on_transfer, msg = {}",
            amount.0,
            sender_id.as_ref(),
            msg
        );
        match RaffleInstruction::from(msg) {
            RaffleInstruction::BuyPrize => {
                let result = self.ticket.buy_prize(sender_id.into(), amount.into());
                match result {
                    Ok(s) => PromiseOrValue::Value(s.into()),
                    Err(e) => {
                        log!(e);
                        PromiseOrValue::Value(amount)
                    }
                }
            }
            _ => {
                log!("Invalid instruction for raffle call");
                PromiseOrValue::Value(amount)
            }
        }
    }
}

#[near_bindgen]
impl RaffleContract {
    #[init]
    pub fn new(
        fungible_token_account_id: AccountId,
        tokens_per_ticket: Balance,
        number_of_predefined: i16,
    ) -> Self {
        assert_initialized();
        Self {
            ticket: RaffleTicket::new(tokens_per_ticket, number_of_predefined),
            fungible_token_account_id,
        }
    }


    pub fn total_tickets(&self)->u128{
      self.ticket.total_available
    }
    
    pub fn reset(&mut self) {
        self.ticket.reset();
    }
}
