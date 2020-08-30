use rusqlite::Connection;
use std::fs;
use std::io;

fn create_database() -> rusqlite::Result<Connection> {
    let ribbit_path = "./.ribbit";
    match fs::metadata(&ribbit_path) {
        Ok(_) => (),
        Err(error) => match error.kind() {
            io::ErrorKind::AlreadyExists => fs::create_dir(&ribbit_path).unwrap(),
            other_error => panic!(other_error),
        },
    };
    let db = Connection::open("./.ribbit/repo.db")?;
    db.execute(
        "create table if not exists changes (
            id text primary key,
            date_created integer not null,
            revision_id text not null,
            file_path text not null
        )",
        rusqlite::NO_PARAMS,
    )?;
    return Ok(db);
}

fn main() -> rusqlite::Result<()> {
    let _db = create_database()?;

    Ok(())
}
