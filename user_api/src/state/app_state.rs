use crate::state::user_state::UserState;
use axum::extract::FromRef;
use sqlx::{MySql, Pool};

#[derive(Clone)]
pub struct AppState {
    user_state: UserState,
}

impl AppState {
    pub fn new(sql_pool: Pool<MySql>) -> Self {
        Self {
            user_state: UserState::init(sql_pool),
        }
    }

    #[inline]
    pub fn user_state(&self) -> UserState {
        self.user_state.clone()
    }
}
