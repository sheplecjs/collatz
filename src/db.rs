use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;
use num_bigint::BigInt;

pub fn create_db() -> Result<()> {
    let conn = Connection::open("collatz.db")?;

    conn.execute(
        "create table if not exists collatz (
             id integer primary key,
             start text not null unique,
             steps integer not null
         )",
        NO_PARAMS,
    )?;

    conn.execute(
        "create table if not exists highest (
            start text not null unique,
            steps integer not null
        )",
        NO_PARAMS,
    )?;

    Ok(())
}

pub fn add_entry(start: BigInt, steps: u32) -> Result<()> {
    let conn = Connection::open("collatz.db")?;

    conn.execute(
        "insert into collatz (start, steps) values (?1, ?2)",
        &[&start.to_string(), &steps],)?;

    Ok(())
}

pub fn check_for_entry(start: BigInt) -> String {

    let conn = Connection::open("collatz.db")?;

    conn.execute(
        "select steps from collatz where start = ?1",
        &start.to_string()).unwrap();

}