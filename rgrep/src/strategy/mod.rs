use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;
use colored::*;
use regex::{Match, Regex};

pub trait Strategy {
    fn grep<R: Read, W: Write>(
        &self,
        input: BufReader<R>,
        pattern: &Regex,
        output: &mut W,
    ) -> Result<()>;
}

pub struct DefaultStrategy;

impl Strategy for DefaultStrategy {
    fn grep<R: Read, W: Write>(
        &self,
        input: BufReader<R>,
        pattern: &Regex,
        output: &mut W,
    ) -> Result<()> {
        for (no, line) in input.lines().enumerate() {
            let line = line?;
            if let Some(m) = pattern.find(&line) {
                output.write_all(decorate_line(&line, m, no).as_bytes())?;
            }
        }
        Ok(())
    }
}

fn decorate_line(line: &str, m: Match, lineno: usize) -> String {
    format!(
        "{0: >6}:{1: <3}  {2}{3}{4}\n",
        lineno.to_string().blue(),
        m.start().to_string().blue(),
        &line[..m.start()],
        m.as_str().red(),
        &line[m.end()..]
    )
}
