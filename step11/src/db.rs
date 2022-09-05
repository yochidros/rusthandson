use diesel::Connection;
use diesel::SqliteConnection;

pub fn make_db_conn(database_url: &str) -> SqliteConnection {
    SqliteConnection::establish(database_url)
        .expect(&format!("Error connection to {}", database_url))
}
