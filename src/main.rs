mod glob;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    limit: usize,
    pattern: String, // support regex as well as globs
}

fn main() {
    let args = Cli::parse();
    eprintln!("\x1b[34m{:?}\x1b[0m", args);

    match glob::generate_from_glob(args.pattern, args.limit) {
        Ok(s) => println!("{:?}", s),
        Err(e) => println!("\x1b[31m{}\x1b[0m", e),
    }
}

// todo: support regex, SQL LIKE operator
// todo: generate multiple strings
// todo: support optional wildcards in glob
// todo: transform glob to regex (refer wikipedia on glob)
