use gotham::state::State;
use responses::index::IndexResponse;

pub fn index(state : State) -> (State, IndexResponse) {
    (state, IndexResponse {
        status : String::from("ok"),
        message : String::from("Successfully initialized service.")
    })
}