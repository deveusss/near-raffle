use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::collections::LookupSet;
use near_sdk::collections::UnorderedMap;
use near_sdk::collections::UnorderedSet;
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::{env, log, near_bindgen, AccountId, Balance, PanicOnDefault, PromiseOrValue};
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use crate::*;


pub type TicketNumber = i64;
pub type TicketId = i32;

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Ticket {
    id: TicketId,
    numbers: Vec<TicketNumber>,
    owner_id: Option<AccountId>,
}
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct RaffleTicket {
    winning_tickets: LookupSet<Ticket>,
    available_tickets: UnorderedSet<Ticket>,
    sold_tickets: UnorderedMap<AccountId, TicketNumber>,
    tokens_per_ticket:i32,
}

impl RaffleTicket {
    pub fn new(tokens_per_ticket:i32,number_of_predefined: i16)->Self{
        let mut tk=RaffleTicket{
            available_tickets:UnorderedSet::new(StorageKey::Available),
            winning_tickets:LookupSet::new(StorageKey::Winning),
            sold_tickets:UnorderedMap::new(StorageKey::Winning),
            tokens_per_ticket
        };
        tk.initialize(number_of_predefined);
        tk
    }
    fn initialize(&mut self, number_of_predefined: i16) {
        for n in 1..number_of_predefined {
            let ticket=self.new_ticket(false,None);
            self.available_tickets.insert(&ticket);
        }
    }
    fn new_ticket(&mut self,is_winning_ticket:bool, owner_id: Option<AccountId>) -> Ticket {
        Ticket {
            id: (self.available_tickets.len() as i32) + 1,
            owner_id: owner_id,
            numbers: self.generate_ticket_numbers(),
        }
    }
    fn generate_ticket_numbers(&self) -> Vec<i64> {
        let step = Uniform::new(100, 1000);
        let mut rng = rand::thread_rng();
        let numbers: Vec<_> = step.sample_iter(&mut rng).take(5).collect();
        return numbers;
    }
}
