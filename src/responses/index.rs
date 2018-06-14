use gotham::handler::IntoResponse;
use gotham::http::response::create_response;
use gotham::state::State;
use hyper::{Response, StatusCode};
use mime::APPLICATION_JSON;
use serde_json;

#[derive(Serialize)]
pub struct IndexResponse {
    pub status : String,
    pub message : String
}

impl IntoResponse for IndexResponse {
    fn into_response(self, state: &State) -> Response {
        create_response(
            state,
            StatusCode::Ok,
            Some(
                (serde_json::to_string(&self)
                    .expect("Serialized response object")
                    .into_bytes()
                , APPLICATION_JSON)
            )
        )
    }
}
