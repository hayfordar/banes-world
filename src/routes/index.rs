use hyper::{Response, StatusCode};

use gotham::http::response::create_response;
use gotham::state::State;
use mime::TEXT_PLAIN;

pub fn index(state : State) -> (State, Response) {
    let response = create_response(
        &state,
        StatusCode::Ok,
        Some((String::from("Hello Router!").into_bytes(), TEXT_PLAIN)),
    );

    (state, response)
}