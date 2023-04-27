use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
enum Glob {
    Char(char),
    CharSet(HashSet<char>),
    // todo: use hashset
    Rep(char),
    CharSetRep(HashSet<char>),
    Opt(char),
    CharSetOpt(HashSet<char>), // merge with CharSet, making empty char a special case
}

impl Glob {
    fn parse(s: String) -> Result<Vec<Glob>, &'static str> {
        let mut g: Vec<Glob> = vec![];
        let mut in_cs = false;
        for ch in s.chars() {
            match ch {
                'A'..='Z' | 'a'..='z' | '0'..='9' => match g.last_mut() {
                    Some(Glob::CharSet(cs)) if in_cs => {
                        cs.insert(ch);
                    }
                    _ => {
                        g.push(Glob::Char(ch));
                    }
                },
                '*' => match g.pop() {
                    Some(Glob::Char(lch)) => {
                        g.push(Glob::Rep(lch));
                    }
                    Some(Glob::CharSet(cs)) => {
                        g.push(Glob::CharSetRep(cs));
                    }
                    None => {
                        return Err("No preceeding char or charset before '*'");
                    }
                    _ => {
                        return Err("You can only repeat a char or a charset");
                    }
                },
                '?' => match g.pop() {
                    Some(Glob::Char(lch)) => {
                        g.push(Glob::Opt(lch));
                    }
                    Some(Glob::CharSet(cs)) => {
                        g.push(Glob::CharSetOpt(cs));
                    }
                    None => {
                        return Err("No preceding char or charset before '?'");
                    }
                    _ => {
                        return Err("You can only make a char or charset optional");
                    }
                },
                '[' => {
                    in_cs = true;
                    g.push(Glob::CharSet(HashSet::new()));
                }
                ']' => {
                    in_cs = false;
                }
                _ => {
                    let e = format!("Unsupported char found: {}", ch);

                    println!("{}", e);

                    return Err("e.as_str()");
                }
            }
        }

        Ok(g)
    }
}

fn str_len(glob: &Vec<Glob>, limit: usize) -> usize {
    let mut s = 0;
    for t in glob {
        s += match t {
            Glob::Char(_) => 1,
            Glob::CharSet(_) => 1,
            Glob::Rep(_) => limit,
            Glob::CharSetRep(_) => limit,
            Glob::Opt(_) => 2,
            Glob::CharSetOpt(_) => 2,
        }
    }
    s
}

fn str_counts(glob: &Vec<Glob>, limit: usize) -> usize {
    let mut s = 1;
    for t in glob {
        s *= match t {
            Glob::Char(_) => 1,
            Glob::CharSet(cs) => cs.len(),
            Glob::Rep(_) => limit,
            Glob::CharSetRep(cs) => cs.len() * limit,
            Glob::Opt(_) => 2,
            Glob::CharSetOpt(_) => 2,
        }
    }
    s
}

pub fn generate_from_glob(pattern: String, limit: usize) -> Result<VecDeque<String>, &'static str> {
    let g = Glob::parse(pattern.to_string())?;

    let mut generated_strings: VecDeque<String> = VecDeque::new();
    for t in &g {
        match t {
            Glob::Char(c) => {
                if generated_strings.is_empty() {
                    generated_strings.push_back(c.to_string());
                } else {
                    for i in 0..generated_strings.len() {
                        let mut s = generated_strings.pop_front().unwrap();
                        s.push(*c);
                        generated_strings.push_back(s);
                    }
                }
            }
            Glob::CharSet(cs) => {
                for i in 0..generated_strings.len() {
                    let s = generated_strings.pop_front().unwrap();

                    cs.iter().for_each(|c| {
                        let mut s = s.clone();
                        s.push(*c);
                        generated_strings.push_back(s);
                    });
                }
            }
            Glob::Rep(_) => {}
            Glob::CharSetRep(_) => {}
            Glob::Opt(c) => {
                for i in 0..generated_strings.len() {
                    let mut s = generated_strings[i].clone();
                    s.push(*c);
                    generated_strings.push_back(s);
                }
            }
            Glob::CharSetOpt(cs) => {
                let n = generated_strings.len();
                for i in 0..n {
                    let mut s = generated_strings[i].clone();
                    generated_strings.push_back(s);
                }

                for i in 0..n {
                    let s = generated_strings.pop_front().unwrap();

                    cs.iter().for_each(|c| {
                        let mut s = s.clone();
                        s.push(*c);
                        generated_strings.push_back(s);
                    });
                }
            }
        }
    }

    // generated_strings.sort();
    Ok(generated_strings)
}


// a* b* c => limit = 2

// a  b  c
// a  bb c
// aa b  c
// aa bb c

#[cfg(test)]
mod tests {
    use crate::glob::{Glob, str_counts, str_len};

    fn p(pattern: &str) {
        let g = Glob::parse(pattern.to_string());
        if let Ok(ref v) = g {
            println!(
                "{: <12}| l={}, c={} => {:?}",
                pattern,
                str_len(v, 3),
                str_counts(v, 3),
                g
            );
        } else {
            println!("{: <12}| {:?}", pattern, g);
        }
    }

    #[test]
    fn it_works() {
        p("abc");

        p("a*b?c");

        p("a[bc]d");

        p("a[bc]*d");

        p("a[bc]d[ef]g");

        p("a**b?c");

        p("a*b??c");

        p("*b?c");

        p("?c");

        p("@?");

        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
