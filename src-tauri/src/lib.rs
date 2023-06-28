use rusqlite::{Connection, Error, Result};

#[derive(Debug)]
pub struct Timetype {
    pub id: i32,
    pub label: String,
    pub default_hours: i32,
}

impl Timetype {
    pub fn get(id: i32) -> Result<Timetype, Error> {
        let database = Connection::open("timetrack.db")?;

        let mut statement = database.prepare("select * from timetypes where id = :id;")?;

        let mut results = statement.query_map(&[(":id", &id.to_string())], |row| {
            println!("returned row from the query: {:?}", row);
            Ok(Timetype {
                id: row.get(0)?,
                label: row.get(1)?,
                default_hours: row.get(2)?,
            })
        })?;

        let result = results.next();

        match result {
            None => Err(Error::QueryReturnedNoRows),
            Some(x) => x,
        }
    }
}
