use dotenv_codegen::dotenv;

use crate::db_tools::db_connect;
use crate::user::get_user;
use crate::user::structs::User;

pub fn query_users() -> Vec<User> {
    let database_url = dotenv!("DATABASE_URL");
    let conn = db_connect(database_url);
    get_user(conn)
}
