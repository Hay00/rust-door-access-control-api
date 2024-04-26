use axum::routing::{delete, get, post, put};
use axum::{middleware, Router};

use crate::controllers::user_accesses;
use crate::controllers::users;
use crate::{controllers, middlewares, AppState};

pub fn builder(state: AppState) -> Router {
    Router::new()
        .merge(closed_routes(state.clone()))
        .route_layer(middleware::from_fn(middlewares::auth::intercept_request))
        .merge(open_routes(state.clone()))
}

fn closed_routes(state: AppState) -> Router {
    Router::new()
        .route("/user", post(users::create_user))
        .route("/user/:id", get(users::find_user))
        .route("/user", get(users::list_all))
        .route("/user/:id", put(users::update_user))
        .route("/user/:id", delete(users::delete_user))
        .route(
            "/user/:user_id/user-access",
            post(user_accesses::create_access),
        )
        .route(
            "/user/:user_id/user-access",
            get(user_accesses::find_by_user),
        )
        .route(
            "/user/:user_id/user-access/:day_id",
            delete(user_accesses::delete_access),
        )
        .route(
            "/user/:user_id/user-access/:day_id",
            put(user_accesses::update_access),
        )
        .with_state(state)
}

fn open_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(controllers::service_alive::alive_route))
        .route("/validate-password", post(controllers::door::unlock))
        .route("/login", post(controllers::auth::login))
        .with_state(state)
}
