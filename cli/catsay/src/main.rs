extern crate colored;
extern crate structopt;

use colored::*;
use exitfailure::ExitFailure;
use failure::ResultExt;
use std::io::{self, Read, Write};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    #[structopt(short = "i", long = "stdin")]
    /// Read the message from STDIN instead of the argument
    stdin: bool,

    #[structopt(default_value = "Meow!")]
    /// What does the cat say?
    message: String,

    #[structopt(short = "f", long = "file", parse(from_os_str))]
    /// Load the cat picture from the specified file
    catfile: Option<std::path::PathBuf>,

    #[structopt(short = "d", long = "dead")]
    /// Make the cat appears dead
    dead: bool,
}

fn main() -> Result<(), ExitFailure> {
    let options = Options::from_args();
    let mut message = String::new();
    if options.stdin {
        io::stdin().read_to_string(&mut message)?;
    } else {
        message = options.message;
    };

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog.")
    }

    let eye = if options.dead { "x" } else { "o" };

    let cat_template = match &options.catfile {
        Some(path) => std::fs::read_to_string(path)
            .with_context(|_| format!("could not read file {:?}", path))?,
        None => String::from(
            "
 \\
  \\
    /\\_/\\
   ( {eye} {eye} )
   =( I )=",
        ),
    };

    let cat_picture = cat_template.replace("{eye}", eye);
    writeln!(
        handle,
        "{}",
        message.bright_yellow().underline().on_purple()
    )?;
    writeln!(handle, "{}", &cat_picture)?;
    Ok(())
}
