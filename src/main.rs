use clap::Parser;
use rand::Rng;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    /// Uppercase
    #[arg(short, default_value_t = false)]
    pub upper: bool,
    /// Lowercase
    #[arg(short, default_value_t = false)]
    pub lower: bool,
    /// Number
    #[arg(short, default_value_t = false)]
    pub number: bool,
    /// Special character
    #[arg(short, default_value_t = false)]
    pub special: bool,
    /// Extended special character
    #[arg(short, default_value_t = false)]
    pub extend: bool,
    /// Password length
    #[arg(long, default_value_t = 32, value_parser = clap::value_parser!(u32).range(1..))]
    pub length: u32,
    /// password count
    #[arg(long, default_value_t = 1, value_parser = clap::value_parser!(u32).range(1..))]
    pub count: u32,
}

const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBER: &[u8] = b"0123456789";
const SPECIAL: &[u8] = b"*&^%$#@!";
const EXTENDED: &[u8] = b"~`()_-+={[}]|\\:;\"'<,>.?/";

fn main() {
    generate_passwords(&mut Args::parse())
        .iter()
        .for_each(|password| {
            println!("{password}");
        });
}

fn generate_passwords(args: &mut Args) -> Vec<String> {
    if [
        args.lower,
        args.upper,
        args.number,
        args.special,
        args.extend,
    ]
    .iter()
    .all(|&x| !x)
    {
        (args.lower, args.upper, args.number) = (true, true, true);
    }

    let mut charset: Vec<u8> = Vec::new();
    [
        (args.lower, LOWER),
        (args.upper, UPPER),
        (args.number, NUMBER),
        (args.special, SPECIAL),
        (args.extend, EXTENDED),
    ]
    .into_iter()
    .filter(|(flag, _)| *flag)
    .for_each(|(_, set)| {
        charset.extend(set);
    });

    let mut rng = rand::thread_rng();
    let mut passwords: Vec<String> = vec![];
    for _ in 0..args.count {
        passwords.push(
            (0..args.length)
                .map(|_| charset[rng.gen_range(0..charset.len())] as char)
                .collect(),
        );
    }
    passwords
}
