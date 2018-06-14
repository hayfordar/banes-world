use gotham::router::Router;
use gotham::router::builder::*;

use routes::index;

pub fn router() -> Router {
    build_simple_router(|route| {
        route.get("/").to(index::index);
    })
}