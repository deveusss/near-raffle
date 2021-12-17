use crate::*;
pub type TicketNumber = i64;
pub type TicketId = i32;

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Ticket {
    id: TicketId,
    numbers: Vec<TicketNumber>,
    owner_id: Option<AccountId>,
    is_winning_ticket: bool,
}
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct RaffleTicket {
    winning_tickets: LookupSet<Ticket>,
    available_tickets: UnorderedSet<Ticket>,
    sold_tickets: UnorderedMap<AccountId, TicketNumber>,
    tokens_per_ticket: u64,
}

impl RaffleTicket {
    pub fn new(tokens_per_ticket: u64, number_of_predefined: i16) -> Self {
        let mut raffle = RaffleTicket {
            available_tickets: UnorderedSet::new(StorageKey::Available),
            winning_tickets: LookupSet::new(StorageKey::Winning),
            sold_tickets: UnorderedMap::new(StorageKey::Sold),
            tokens_per_ticket,
        };
        raffle.add_tickets(number_of_predefined, false);
        raffle.add_tickets_as_winning(1);
        raffle
    }
    fn add_tickets_as_winning(&mut self, number_of_predefined: i16) {
        self.add_tickets(number_of_predefined, true);
    }

    fn add_tickets(&mut self, number_of_predefined: i16, is_winning_ticket: bool) {
        for _ in 1..number_of_predefined {
            let ticket = self.new_ticket(false, None);
            self.available_tickets.insert(&ticket);
            if is_winning_ticket {
                self.winning_tickets.insert(&ticket);
            }
        }
    }
    fn new_ticket(&mut self, is_winning_ticket: bool, owner_id: Option<AccountId>) -> Ticket {
        Ticket {
            id: (self.available_tickets.len() as i32) + 1,
            owner_id: owner_id,
            numbers: self.generate_ticket_numbers(),
            is_winning_ticket: is_winning_ticket,
        }
    }
    fn generate_ticket_numbers(&self) -> Vec<i64> {
        let step = Uniform::new(100, 1000);
        let mut rng = rand::thread_rng();
        let numbers: Vec<_> = step.sample_iter(&mut rng).take(5).collect();
        return numbers;
    }
    pub fn buy(&mut self, prize_tokens: u64) -> u128 {
        assert!(
            prize_tokens >= self.tokens_per_ticket,
            "Invalid prize amount"
        );
        if self.available_tickets.len() < self.tokens_per_ticket {
            log!("No more tickets available");
        }
0
    }
}
