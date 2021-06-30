#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use self::models::NewMemo;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = "sample.db";
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_memo(conn: &SqliteConnection, comment: &str) -> usize {
    use crate::schema::memos;

    let new_memo = NewMemo { comment };

    diesel::insert_into(memos::table)
        .values(&new_memo)
        //SQLiteはget_result()は対応していないため、execute()
        .execute(conn)
        .expect("Error saving new memo")
}
