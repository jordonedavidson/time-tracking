use rusqlite::{Connection, Error, Result};

#[derive(Debug)]
pub struct Timetype {
    pub id: i32,
    pub label: String,
    pub default_hours: i32,
}

impl Timetype {
    pub fn get(id: i32) -> Result<Timetype, Error> {
        // Need to better understand the ? at the end of a lot of these statements. I think it short circuts the Result type to the OK branch.

        //Opening connection to the db.
        let database = Connection::open("timetrack.db")?;

        // Creating the prepared statment for the db query. :id is a variable that will be inserted when the query is performed.
        let mut statement = database.prepare("select * from timetypes where id = :id;")?;

        // Perform the query by mapping over an array of tuples that provides the values for the prepared statement.
        let mut results = statement.query_map(&[(":id", &id.to_string())], |row| {
            // Load the result row into the TimeType struct. the Ok() syntax preserves the Result<> type we need.
            Ok(Timetype {
                id: row.get(0)?,
                label: row.get(1)?,
                default_hours: row.get(2)?,
            })
        })?;

        // Each member of the iterator is an Option type whose Some branch contains the row result, None means the query returned no rows.
        // This query can return at most one result so we take it.
        let result = results.next();

        // Either there are no rows or a Result. Return the QueryReturnedNoRows error or the Timetype result.
        match result {
            None => Err(Error::QueryReturnedNoRows),
            Some(x) => x,
        }
    }
}
