use rusqlite::{Connection, Error, Result};
use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub display_name: String,
    pub roles: String,
    pub totals: String,
}

impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("User", 5)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("username", &self.username)?;
        state.serialize_field("display_name", &self.display_name)?;
        state.serialize_field("roles", &self.roles)?;
        state.serialize_field("totals", &self.totals)?;
        state.end()
    }
}

impl User {
    pub fn get_by_username(username: String) -> Result<User, Error> {
        // Open connection to the db.
        let database = Connection::open("timetrack.db")?;

        // Prepared statement for Query. :username variable is set on execution.
        let mut statement = database.prepare("select * from users where username = :username;")?;

        // Perform the query.
        let mut results = statement.query_map(&[(":username", &username)], |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                display_name: row.get(2)?,
                roles: row.get(3)?,
                totals: row.get(4)?,
            })
        })?;

        // Each member of the iterator is an Option type whose Some branch contains the row result, None means the query returned no rows.
        // This query can return at most one result so we take it.
        let result = results.next();

        // Either there are no rows or a Result. Return the QueryReturnedNoRows error or the timetype result.
        match result {
            None => Err(Error::QueryReturnedNoRows),
            Some(x) => x,
        }
    }
}
