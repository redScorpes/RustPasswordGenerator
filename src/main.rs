use clap::{Parser, Arg};
use rand::prelude::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    #[arg(short, long, help = "Length of the password")]
    length: u8,

    #[arg(short, long, help = "Include all possible characters")]
    all: bool,

    #[arg(short = 'c', long = "lowercase letters", help = "Lowercase letters include: abcdefghijklmnopqrstuvwxyz")]
    lowercase: bool,

    #[arg(short, long = "uppercase letters", help = "Uppercase letters include: ABCDEFGHIJKLMNOPQRSTUVWXYZ")]
    uppercase: bool,

    #[arg(short, long = "numbers", help = "Numbers include: 0123456789")]
    number: bool,

    #[arg(short, long = "special characters", help = "Special characters include: !\"#$%&'()*+,-./")]
    special: bool,

    #[arg(short = 'z', long, default_value = "-", help = "Separator between segments")]
    separator: char,

    #[arg(short = 'p', long, default_value_t = 0, help = "Length of each segment. Set it to 0 to disable")]
    segment_length: usize,
}

fn password_generator(length: u8, special: bool, uppercase: bool, number: bool, separator: char, segment_length: usize) -> String {
    let args = Args::parse();

    let mut password = String::new();
    let mut rng = thread_rng();

    let lowercase_range = 97..123; // a-z
    let uppercase_range = 65..91;  // A-Z
    let number_range = 48..58;     // 0-9
    let special_range = "!\"#$%&'()*+,-./"; // Special characters

    let mut char_types = Vec::new();
    if args.all || args.lowercase {
        char_types.push(0);
    }
    if args.all || args.uppercase {
        char_types.push(1);
    }
    if args.all || args.number {
        char_types.push(2);
    }
    if args.all || special {
        char_types.push(3);
    }

    if char_types.is_empty() {
        char_types.push(0);
    }

    for i in 0..length {
        if char_types.is_empty() {
            break;
        }
        let char = match char_types[rng.gen_range(0..char_types.len())] {
            0 => rng.gen_range(lowercase_range.clone()),
            1 => rng.gen_range(uppercase_range.clone()),
            2 => rng.gen_range(number_range.clone()),
            3 => *special_range.as_bytes().choose(&mut rng).unwrap(),
            _ => unreachable!(),
        } as u8 as char;
        password.push(char);
        if segment_length > 0 && (i + 1) % segment_length as u8 == 0 && i < length - 1 {
            password.push(separator);
        }
    }
    password
}

fn main() {
    let args = Args::parse();

    let password = password_generator(args.length, args.special, args.uppercase, args.number, args.separator, args.segment_length);

    println!("{}", password);
}