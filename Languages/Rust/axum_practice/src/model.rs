//! Simplistic Model layers
//!  (with mock store layer)

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::ctx::Ctx;
use crate::{CustomErr, Result};

// region:  --- Ticket types
//serialize: convert to json
//deserialize; convert json to rust
#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub cid: u64, // creator user_id
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}

// endregion: -- Ticket TYpes

// region: ---Model conttroller
//have the store embedded in it; usually a db connection or sqlx or orm
#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

// Constructor
impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }
}

// CRUD IMplementation

impl ModelController {
    pub(crate) async fn create_ticket(
        &self,
        ctx: Ctx,
        ticket_fc: TicketForCreate,
    ) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().await;

        let id = store.len() as u64;

        let ticket = Ticket {
            id,
            cid: ctx.user_id,
            title: ticket_fc.title,
        };

        store.push(Some(ticket.clone()));
        Ok(ticket)
    }

    pub(crate) async fn list_tickets(&self, _ctx: Ctx) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().await;

        let tickets = store.iter().filter_map(|t| t.clone()).collect();

        Ok(tickets)
    }

    pub(crate) async fn delete_ticket(&self, _ctx: Ctx, id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().await;

        let ticket = store.get_mut(id as usize).and_then(|t| t.take());

        ticket.ok_or(CustomErr::TicketDeleteFailIdNotFound { id })
    }
}

// endregion: -- ModelController
