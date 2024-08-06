use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, SendError, SyncSender};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, SendError<Command>> {
        println!("draft: {:?}", draft);
        let (response_sender, response_receiver) = std::sync::mpsc::sync_channel(1);
        self.send(Command::Insert {
            draft,
            response_channel: response_sender,
        })
        .unwrap();
        Ok(response_receiver.recv().unwrap())
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, SendError<Command>> {
        println!("id: {:?}", id);
        let (response_sender, response_receiver) = std::sync::mpsc::sync_channel(1);
        let _ = self.send(Command::Get {
            id,
            response_channel: response_sender,
        });
        Ok(response_receiver.recv().unwrap())
    }
    fn send(&self, command: Command) -> Result<(), SendError<Command>> {
        self.sender.send(command)
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = std::sync::mpsc::sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender }
}

pub enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: SyncSender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: SyncSender<Option<Ticket>>,
    },
}

fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                let _ = response_channel.send(ticket.cloned());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
