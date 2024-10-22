use anyhow::Result;
use rand::{seq::SliceRandom, thread_rng};
use zxcvbn::zxcvbn;

pub fn generate_password(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> Result<String> {
    const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
    const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
    const NUMBER: &[u8] = b"123456789";
    const SYMBOL: &[u8] = b"!@#$%&*,.<>?";

    let mut password = Vec::new();
    let mut chars = Vec::new();
    let mut rng = thread_rng();

    // make sure at least one character is included of each type
    if upper {
        chars.extend_from_slice(UPPER);
        let c = UPPER.choose(&mut rng).expect("Chars won't be empty");
        password.push(*c);
    }

    if lower {
        chars.extend_from_slice(LOWER);
        let c = LOWER.choose(&mut rng).expect("Chars won't be empty");
        password.push(*c);
    }

    if number {
        chars.extend_from_slice(NUMBER);
        let c = NUMBER.choose(&mut rng).expect("Chars won't be empty");
        password.push(*c);
    }

    if symbol {
        chars.extend_from_slice(SYMBOL);
        let c = SYMBOL.choose(&mut rng).expect("Chars won't be empty");
        password.push(*c);
    }

    for _ in 0..(length - password.len() as u8) {
        let c = chars.choose(&mut rng).expect("Chars won't be empty");
        password.push(*c);
    }

    password.shuffle(&mut rng);
    let password = String::from_utf8(password)?;
    println!("Password: {:?}", password);
    let estimate = zxcvbn(&password, &[]);
    eprintln!("{}", estimate.score());

    Ok(password)
}
