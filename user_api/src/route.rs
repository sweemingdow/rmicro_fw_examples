pub mod user;

use crate::state::user_state::UserState;
use axum::Router;

pub fn configure_user_model(router: Router, user_state: UserState) -> Router {
    router.nest("/user", user::router().with_state(user_state))
}
