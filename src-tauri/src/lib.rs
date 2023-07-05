use rusqlite::{Connection, Error, Result};
use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Debug)]
pub struct Timetype {
    pub id: i32,
    pub label: String,
    pub default_hours: i32,
}

impl Serialize for Timetype {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Timetype", 3)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("label", &self.label)?;
        state.serialize_field("default_hours", &self.default_hours)?;
        state.end()
    }
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

        println!("get {} returned {:?}", &id, &result);
        // Either there are no rows or a Result. Return the QueryReturnedNoRows error or the Timetype result.
        match result {
            None => Err(Error::QueryReturnedNoRows),
            Some(x) => x,
        }
    }

    pub fn get_all() -> Result<Vec<Timetype>, Error> {
        let database = Connection::open("timetrack.db")?;

        let mut statement = database.prepare("select * from timetypes order by label ASC")?;

        let results = statement.query_map((), |row| {
            Ok(Timetype {
                id: row.get(0)?,
                label: row.get(1)?,
                default_hours: row.get(2)?,
            })
        });

        let mut timetypes = Vec::new();

        match results {
            Ok(x) => {
                for result in x {
                    match result {
                        Ok(y) => timetypes.push(y),
                        Err(e) => return Err(e),
                    }
                }
            }
            Err(e) => println!("Something went wrong {:?}", e),
        };

        Ok(timetypes)
    }
}
