mod handlers;

use actix_web::{
    web::{get, post, scope},
    Scope,
};

pub fn service(root_path: &str) -> Scope {
    scope(root_path)
        .route("/graphql", post().to_async(handlers::graphql))
        .route("/graphiql", get().to(handlers::graphiql))
}
