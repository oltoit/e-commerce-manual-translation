use diesel::prelude::*;
use diesel::{PgConnection, QueryResult};
use crate::shared::entity::user::User;
use crate::schema::app_user::dsl::app_user;
use crate::schema::app_user::username;

pub fn get_by_username(connection: &mut PgConnection, user_name: &str) -> QueryResult<User> {
    app_user.filter(username.eq(user_name)).get_result(connection)
}

pub fn get_by_id(connection: &mut PgConnection, user_id: i64) -> QueryResult<User> {
    app_user.find(user_id).get_result(connection)
}