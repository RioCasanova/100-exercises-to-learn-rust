use crate::store::TicketStore;
use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

// Refer to the tests to understand the expected schema.
pub enum Command {
    Insert {
        draft: data::TicketDraft,
        response_sender: Sender<store::TicketId>,
    },
    Get {
        id: store::TicketId,
        response_sender: Sender<data::Ticket>,
    },
}

pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: handle incoming commands as expected.
pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_sender,
            }) => {
                let ticket_id = store.add_ticket(draft);
                response_sender.send(ticket_id).expect("error")
            }
            Ok(Command::Get {
                id,
                response_sender,
            }) => {
         //       Retrieve the ticket by its ID
                match store.get(id) {
                    Some(ticket) => {
                        // Send the ticket back
                        response_sender
                            .send(ticket.clone())
                            .expect("error sending ticket");
                    }
                    None => {
                        eprintln!("Ticket not found with ID: {:?}", id);
                    }
                }
                
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
