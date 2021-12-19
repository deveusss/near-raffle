use crate::*;
pub type TicketNumber = i32;
pub type TicketId = u128;

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Ticket {
    id: TicketId,
    numbers: Vec<TicketNumber>,
    owner_id: Option<AccountId>,
    is_winning_ticket: bool,
}
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct RaffleTicket {
    winning_tickets: LookupMap<TicketId,Ticket>,
    available_tickets: LookupMap<TicketId,Ticket>,
    sold_tickets: UnorderedMap<AccountId, Ticket>,
    prizes_per_ticket: Balance,
    total_available:u128,
}

impl RaffleTicket {
    pub fn new(prizes_per_ticket: Balance, number_of_predefined: i16) -> Self {
        let mut raffle = RaffleTicket {
            available_tickets: LookupMap::new(StorageKey::Available),
            winning_tickets: LookupMap::new(StorageKey::Winning),
            sold_tickets: UnorderedMap::new(StorageKey::Sold),
            prizes_per_ticket,
            total_available:0
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
            self.available_tickets.insert(&ticket.id,&ticket);
            self.total_available+=1;
            if is_winning_ticket {
                self.winning_tickets.insert(&ticket.id,&ticket);
            }
        }
    }
    fn new_ticket(&mut self, is_winning_ticket: bool, owner_id: Option<AccountId>) -> Ticket {
        Ticket {
            id: self.total_available + 1,
            owner_id: owner_id,
            numbers: self.generate_ticket_numbers(),
            is_winning_ticket: is_winning_ticket,
        }
    }
    fn generate_ticket_numbers(&self) -> Vec<TicketNumber> {
    let numbers: Vec<_> = (0..5).map(|_| rand_range(100,1000)).collect();
        return numbers;
    }
    pub fn buy_prize(&mut self,buyer_id:AccountId, prize_tokens: Balance) -> Result<Balance,&str> {
        if prize_tokens >= self.prizes_per_ticket.into(){
            return Err("Invalid prize amount");
        }
        if self.total_available < self.prizes_per_ticket {
           return Err("No prize tickets available");
        }
        let mut refund=prize_tokens%self.prizes_per_ticket;
        let buy_count=prize_tokens/self.prizes_per_ticket;
        for t in 1..buy_count{
            if self.total_available<1{
                let left=(buy_count-t)*self.prizes_per_ticket;
                refund=refund+left;
                break
            }
            else{
                let key=self.total_available-1;
                let ticket=self.available_tickets.get(&key).expect("Ticket not found");
                self.sold_tickets.insert(&buyer_id, &ticket);
                self.available_tickets.remove(&key);
            }
        }
        Ok(refund)
    }
}
