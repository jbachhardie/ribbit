use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()> {
    let f = File::open("Cargo.toml")?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}
