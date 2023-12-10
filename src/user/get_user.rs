use crate::user::structs::User;
use mysql::prelude::Queryable;
use mysql::*;

pub fn get_user(mut conn: PooledConn) -> Vec<User> {
    let selected_users: Vec<User> = conn
        .query_map(
            "SELECT
                 id,
                 name,
                 email,
                 emailVerified,
                 image
                 FROM User",
            |(id, name, email, email_verified, image)| User {
                id,
                name,
                email,
                email_verified,
                image,
            },
        )
        .unwrap();
    selected_users
}
