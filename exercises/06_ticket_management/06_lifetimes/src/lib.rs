use ticket_fields::{TicketDescription, TicketTitle};

// TODO: Implement the `IntoIterator` trait for `&TicketStore` so that the test compiles and passes.
#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl<'a> IntoIterator for &'a TicketStore {
    type Item = &'a Ticket;
    type IntoIter = TicketStoreIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        TicketStoreIterator {
            ticket_store_ref: self,
            index: 0,
        }
    }
}

pub struct TicketStoreIterator<'a> {
    ticket_store_ref: &'a TicketStore,
    index: usize,
}

impl<'a> Iterator for TicketStoreIterator<'a> {
    type Item = &'a Ticket;
    fn next(&mut self) -> Option<&'a Ticket> {
        let result = match self.ticket_store_ref.tickets.get(self.index) {
            Some(ticket) => ticket,
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
        }
    }

    pub fn add_ticket(&mut self, ticket: Ticket) {
        self.tickets.push(ticket);
    }

    pub fn iter(&self) -> std::slice::Iter<Ticket> {
        self.tickets.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn add_ticket() {
        let mut store = TicketStore::new();

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::ToDo,
        };
        store.add_ticket(ticket);

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::InProgress,
        };
        store.add_ticket(ticket);

        let tickets: Vec<&Ticket> = store.iter().collect();
        let tickets2: Vec<&Ticket> = (&store).into_iter().collect();
        assert_eq!(tickets, tickets2);
    }
}
