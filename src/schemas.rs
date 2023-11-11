use axum::{response::IntoResponse, http::StatusCode, Json};
use serde::{Serialize, Deserialize};
use sqlx::{Pool, Postgres, FromRow};


// region: --Ticket

#[derive(Serialize, Deserialize, Debug, FromRow)] // To enable extraction of Ticket from requests to the server and sending of Tickets
pub struct Ticket {
	pub id: i64,
	pub title: String
}

// Struct for ticket creation
#[derive(Deserialize, Debug, Serialize)] // To enable extraction of Ticket from requests to the server and sending of Tickets
pub struct TicketForCreate {
	pub title: String
}

#[derive(Deserialize, Debug, Serialize)] // To enable extraction of Ticket from requests to the server and sending of Tickets
pub struct TicketForEdit {
	pub title: String
}

// endregion: --Ticket

// region: -- Custom Server response
#[derive(Serialize)]
pub struct CustomResponse<T> {
	pub status: bool,
	pub message: Option<String>,
	pub data: Option<CustomResponseData<T>>
}

#[derive(Serialize)] // TO serialize the response into a JSON string
#[serde(untagged)] // TO prevent "CustomReponseData" showing in the JSON response
pub enum CustomResponseData<T> {
	Item(T), // For singular items
	Collection(Vec<T>) // For a collection of items
}
// endregion: -- Custom server response

pub enum CustomError {
	TicketCreateError,
	TicketGetOneError,
	TicketGetManyError,
	TicketUpdateError,
	TicketDeleteError
}

// region: -- Custom error responses
impl IntoResponse for CustomError {
	fn into_response(self) -> axum::response::Response {
			match self {
				CustomError::TicketCreateError => {
					let response: CustomResponse<()> = CustomResponse {
						status: false,
						message: Some(String::from("Failed to create ticket")),
						data: None
					};

					(StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
				},
				CustomError::TicketGetOneError => {
					let response: CustomResponse<()> = CustomResponse {
						status: false,
						message: Some(String::from("Failed to get ticket")),
						data: None
					};

					(StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
				},
				CustomError::TicketGetManyError => {
					let response: CustomResponse<()> = CustomResponse {
						status: false,
						message: Some(String::from("Failed to get tickets")),
						data: None
					};

					(StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
				},
				CustomError::TicketUpdateError => {
					let response: CustomResponse<()> = CustomResponse {
						status: false,
						message: Some(String::from("Failed to update ticket")),
						data: None
					};

					(StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
				},
				CustomError::TicketDeleteError => {
					let response: CustomResponse<()> = CustomResponse {
						status: false,
						message: Some(String::from("Failed to delete ticket")),
						data: None
					};

					(StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
				}
			}
	}
}

// endregion: -- Custom error responses

// region: -- AppState
#[derive(Clone)]
pub struct AppState {
	pub pool: Pool<Postgres>
}

pub type AppStateResult<T> = Result<T, sqlx::Error>;

impl AppState {
	pub async fn create_ticket(self, ticket_information: TicketForCreate) -> AppStateResult<Ticket> {
		let q = r#"
			INSERT INTO tickets ( title )
			VALUES ($1)
			RETURNING *
		"#;

		let result = sqlx::query_as::<_, Ticket>(q)
			.bind(ticket_information.title)
			.fetch_one(&self.pool)
			.await.map_err(|e| {
				eprintln!("{:?}", e); // Print the SQLX error to standard error
				e
			})?;

		Ok(result)
	}
	
	pub async fn get_tickets(self) -> AppStateResult<Vec<Ticket>> {
		let q = r#"
			SELECT * FROM tickets
		"#;

		let result = sqlx::query_as::<_, Ticket>(q)
			.fetch_all(&self.pool)
			.await.map_err(|e| {
				eprintln!("{:?}", e); // Print the SQLX error to standard error
				e
			})?;

		Ok(result)
	}

	pub async fn get_single_ticket(self, id: i64) -> AppStateResult<Ticket> {
		let q = r#"
			SELECT * FROM tickets
			WHERE id = $1
		"#;

		let result = sqlx::query_as::<_, Ticket>(q)
			.bind(id)
			.fetch_one(&self.pool)
			.await.map_err(|e| {
				eprintln!("{:?}", e); // Print the SQLX error to standard error
				e
			})?;

		Ok(result)
	}

	pub async fn update_ticket(self, id: i64, ticket_information: TicketForEdit) -> AppStateResult<Ticket> {
		let q = r#"
			UPDATE tickets
			SET title = COALESCE(
				NULLIF($1, ''),
				title
			)
			WHERE id = $2
			RETURNING *
		"#;

		let result = sqlx::query_as::<_, Ticket>(q)
			.bind(ticket_information.title)
			.bind(id)
			.fetch_one(&self.pool)
			.await.map_err(|e| {
				eprintln!("{:?}", e); // Print the SQLX error to standard error
				e
			})?;

		Ok(result)
	}

	pub async fn delete_ticket(self, id: i64) -> AppStateResult<()> {
		let q = r#"
			DELETE FROM tickets
			WHERE id = $1
		"#;

		let _ = sqlx::query(q)
			.bind(id)
			.execute(&self.pool)
			.await.map_err(|e| {
				eprintln!("{:?}", e); // Print the SQLX error to standard error
				e
			})?;

		Ok(())
	}
}
// endregion: -- AppState