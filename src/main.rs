use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    length: usize, // todo: min & max lengths of generated string
    pattern: String, // support regex as well as globs
}

fn main() {
    let args = Cli::parse();
    println!("\x1b[34m{:?}\x1b[0m", args);

    match generate_from_glob(args.pattern, args.length) {
        Ok(s) => println!("{}", s),
        Err(e) => println!("\x1b[31m{}\x1b[0m", e),
    }
}

// todo: support regex, SQL LIKE operator
// todo: generate multiple strings
// todo: support optional wildcards in glob
// todo: transform glob to regex (refer wikipedia on glob)
fn generate_from_glob(regex: String, max_expansion_length: usize) -> Result<String, &'static str> {
    let mut gen_str: String = String::new();
    let mut last_char: Option<char> = None;
    let mut length_left = max_expansion_length;

    for ch in regex.chars() {
        match ch {
            'A'..='Z' | 'a'..='z' | '0'..='9' => {
                gen_str.push(ch);
                last_char = Some(ch);
                length_left = max_expansion_length;
            }
            '*' => {
                if last_char.is_none() {
                    return Err("Invalid pattern: '*' must be preceded by a character");
                }
                while length_left > 0 {
                    gen_str.push(last_char.unwrap());
                    length_left -= 1;
                }
                // last_char remains unchanged
            }
            _ => {
                todo!("encountered: {}", ch)
            }
        }
    }

    Ok(gen_str)
}
