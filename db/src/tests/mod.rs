#![cfg(test)]

use rusqlite::{Connection, OpenFlags};

use super::*;

#[test]
fn read_all_albums() -> Result<(), Error> {
    let conn = Connection::open_with_flags("tests/test.db", OpenFlags::SQLITE_OPEN_READ_ONLY)?;
    Album::read_all(&conn)?;
    Ok(())
}

#[test]
fn read_all_tracks() -> Result<(), Error> {
    let conn = Connection::open_with_flags("tests/test.db", OpenFlags::SQLITE_OPEN_READ_ONLY)?;
    Item::read_all(&conn)?;
    Ok(())
}
