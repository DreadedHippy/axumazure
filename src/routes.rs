use axum::{Router, routing::{post, get}};

use crate::{handlers::{handler_create_ticket, handler_get_tickets, handler_get_single_ticket, handler_update_ticket, handler_delete_ticket}, schemas::AppState};

pub fn routes_ticket(app_state: AppState) -> Router {
	Router::new()
		.route("/",
			post(handler_create_ticket)
			.get(handler_get_tickets))
		.route("/:id",
			get(handler_get_single_ticket)
			.patch(handler_update_ticket)
			.delete(handler_delete_ticket))
		.with_state(app_state)
}