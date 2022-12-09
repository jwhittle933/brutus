use clap::Parser;
use lazy_static::lazy_static;

lazy_static! {
    static ref COMMON_PASSWORDS:  Vec<&'static str> = vec![
        "qwerty", "123456", "abcdef123", "a123456", "abc123", 
        "password", "asdf", "hello", "welcome", "zxcvbn", 
        "Qazwsx", "654321", "123321", "000000", "111111", 
        "987654321", "1q2w3e", "123qwe", "qwertyuiop", "gfhjkm",
    ];
}



#[derive(Parser, Debug)]
#[command(author, version, about)]
/// Brutus, the brute force password cracking educational tool
struct Args {
    /// Input to crack.
    ///
    /// Brutus will attempt to crack the value of --guess
    /// by brute force, printing statuses along the way. 
    /// When passed with --estimate, brutus will not 
    /// attempt to crack the password, but will estimate 
    /// time to completion based on the content of --guess.
    #[arg(short, long)]
    guess: Option<String>,

    /// Length of input
    ///
    /// Max input length for text to crack.
    #[arg(short, long, default_value_t = 0)]
    length: u64,

    #[arg(short, long, default_value_t = false)]
    alpha: bool,

    #[arg(short, long, default_value_t = false)]
    estimate: bool
}



fn main() {
    let args = Args::parse();

    println!("{:?}", args.guess);
}
