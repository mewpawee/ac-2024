use std::{fs, process::Command, error::Error};
use itertools::Itertools;

fn main()-> Result<(), Box<dyn Error>> {
    let days = fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/src/bin/"))?
    .filter_map(|p| p.ok()?.path().file_stem()?.to_str().map(str::to_string))
    .sorted()
    .collect::<Vec<_>>();
    for day in &days {
        let cmd = Command::new("cargo").args(["run", "--release", "--bin", day]).output()?;
        let output = String::from_utf8(cmd.stdout)?;
        println!("Day {}:\n{}", day, output);
    }

    Ok(())
}
