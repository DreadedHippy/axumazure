use axum::{Json, extract::{Path, State}};

use crate::schemas::{CustomResponse, Ticket, CustomError, TicketForCreate, CustomResponseData, AppState, TicketForEdit};

pub async fn handler_create_ticket(State(app_state): State<AppState>, Json(ticket_info): Json<TicketForCreate>) -> Result<Json<CustomResponse<Ticket>>, CustomError> {
	// let ticket = Ticket {
	// 	id: 0,
	// 	title: ticket_info.title
	// };

	let ticket = app_state.create_ticket(ticket_info).await.map_err(|_| {
		CustomError::TicketCreateError
	})?;

	let response = CustomResponse {
		status: true,
		message: Some(String::from("Ticket created")),
		data: Some(CustomResponseData::Item(ticket))
	};


	Ok(Json(response))
}

pub async fn handler_get_tickets(State(app_state): State<AppState>) -> Result<Json<CustomResponse<Ticket>>, CustomError> {
	// let tickets: Vec<Ticket> = vec![
	// 	Ticket {
	// 		id: 0,
	// 		title: String::from("Ticket 0")
	// 	},
	// 	Ticket {
	// 		id: 1,
	// 		title: String::from("Ticket 1")
	// 	},
	// 	Ticket {
	// 		id: 2,
	// 		title: String::from("Ticket 2")
	// 	},
	// 	Ticket {
	// 		id: 3,
	// 		title: String::from("Ticket 3")
	// 	}
	// ];

	let tickets = app_state.get_tickets().await.map_err(|_| {
		CustomError::TicketGetManyError
	})?;

	let response = CustomResponse {
		status: true,
		message: Some(String::from("Tickets retrieved")),
		data: Some(CustomResponseData::Collection(tickets))
	};


	Ok(Json(response))
}


pub async fn handler_get_single_ticket(State(app_state): State<AppState>, Path(id): Path<i64>) -> Result<Json<CustomResponse<Ticket>>, CustomError> {

	let ticket = app_state.get_single_ticket(id).await.map_err(|_| {
		CustomError::TicketGetOneError
	})?;

	let response = CustomResponse {
		status: true,
		message: Some(String::from("Ticket retrieved")),
		data: Some(CustomResponseData::Item(ticket))
	};


	Ok(Json(response))
}

pub async fn handler_update_ticket(State(app_state): State<AppState>, Path(id): Path<i64>, Json(ticket_info) : Json<TicketForEdit>) -> Result<Json<CustomResponse<Ticket>>, CustomError> {
	// let ticket = Ticket {
	// 	id,
	// 	title: ticket_info.title
	// };
	
	let ticket = app_state.update_ticket(id, ticket_info).await.map_err(|_| {
		CustomError::TicketUpdateError
	})?;

	let response = CustomResponse {
		status: true,
		message: Some(String::from("Ticket updated")),
		data: Some(CustomResponseData::Item(ticket))
	};


	Ok(Json(response))
}

pub async fn handler_delete_ticket(State(app_state): State<AppState>, Path(ticket_id) : Path<i64>) -> Result<Json<CustomResponse<Ticket>>, CustomError> {
	let _ = app_state.delete_ticket(ticket_id).await.map_err(|_| {
		CustomError::TicketDeleteError
	})?;
	
	let response = CustomResponse {
		status: true,
		message: Some(format!("Ticket with id {} successfully deleted", ticket_id)),
		data: None
	};


	Ok(Json(response))
}