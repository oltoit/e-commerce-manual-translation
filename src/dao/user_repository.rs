use diesel::prelude::*;
use diesel::{PgConnection, QueryResult};
use crate::entity::user::User;
use crate::schema::app_user::dsl::app_user;
use crate::schema::app_user::username;

pub fn get_by_username(connection: &mut PgConnection, user_name: &str) -> QueryResult<User> {
    app_user.filter(username.eq(user_name)).get_result(connection)
}