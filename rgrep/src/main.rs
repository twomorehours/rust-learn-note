use anyhow::Result;
use clap::{App, Arg, SubCommand};
use regex::Regex;
use std::{
    fs,
    io::{BufReader, Cursor},
};

mod strategy;
pub use strategy::*;

fn main() -> Result<()> {
    let matches = App::new("rgrep")
        .arg(
            Arg::with_name("rg")
                .help("the regex")
                .takes_value(true)
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("input")
                .help("the input")
                .takes_value(true)
                .required(true)
                .index(2),
        )
        .get_matches();

    let rg = matches.value_of("rg").unwrap();
    let input = matches.value_of("input").unwrap();

    let regex = Regex::new(rg)?;

    let paths = glob::glob(input)?;

    for path in paths {
        let path = path?;
        if path.is_dir() {
            continue;
        }
        let data = fs::read(&path)?;
        println!("{}", path.display());
        DefaultStrategy.grep(
            BufReader::new(Cursor::new(&data)),
            &regex,
            &mut std::io::stdout(),
        )?;
    }

    Ok(())
}
