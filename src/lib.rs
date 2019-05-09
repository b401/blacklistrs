#[allow(unused_imports)]
use rusqlite::types::ToSql;
use rusqlite::{params, Connection, Result, NO_PARAMS};
use std::path::Path;
use time::Timespec;

pub struct DBC {
    connection: Connection,
}

impl DBC {
    pub fn new(path: &str) -> DBC {
        let db_path = Path::new(path);
        let connection = DBC::connect(db_path);
        connection
            .execute(
                "
            create table if not exists blacklist (
                id              INTEGER PRIMARY KEY,
                name            TEXT NOT NULL,
                time_created    TEXT NOT NULL,
                created_by      TEXT NOT NULL
            )",
                params![],
            )
            .unwrap();

        DBC { connection }
    }

    fn connect(path: &Path) -> Connection {
        match Connection::open(path) {
            Ok(connection) => connection,
            Err(e) => panic!("Error: {}", e),
        }
    }

    pub fn insert_entry(&self, param: &str) -> Result<()> {
        let mut prepared_statement = self.connection.prepare("
                                                INSERT INTO blacklist (name, time_created, created_by)
                                                VALUES (?, ?, ?)")?;
        prepared_statement.execute(params![param, time::get_time(), "192.168.1.1".to_string()])?;
        Ok(())
    }

    pub fn get_entry(&self) -> Result<Vec<String>> {
        let mut prepared_statement = self.connection.prepare("SELECT name from blacklist")?;
        let rows = prepared_statement.query_map(NO_PARAMS, |row| row.get(0))?;

        let mut domains = Vec::new();
        for domain in rows {
            domains.push(domain?);
        }

        Ok(domains)
    }

    pub fn remove_entry(&self, param: &str) -> Result<()> {
        let mut prepared_statement = self
            .connection
            .prepare("DELETE FROM blacklist where name = ?")?;
        prepared_statement.execute(params![param])?;
        Ok(())
    }
}
