/*
    This program converts between number bases
    It takes three arguments:
        <inbase:u8> <outbase:u8> <value:String>
    It returns the converted value with specified base
*/

use std::env;

fn get_args() -> Vec<String> {
    let args: Vec<String> = env::args().skip(1).collect();
    check_args(&args);
    args
}

fn check_args(args: &Vec<String>) {
    // check arg count and provide usage
    if args.len() != 3 {
        eprint!("Usage: convert <from_base:u8> <to_base:u8> <value:String>...\n");
        std::process::exit(1);
    }

    // ensure first two args are usize ints between 2 and 36
    for s in &args[0..2] {
        match s.parse::<usize>() {
            Ok(v) => match v {
                2..=36 => (),
                _ => {
                    eprintln!("{} is not a valid base; bases: 2-36 allowed", v);
                    std::process::exit(1);
                }
            },
            Err(e) => {
                eprintln!("{} can not be parsed into a usize; invalid base\n{}", s, e);
                std::process::exit(1);
            }
        }
    }

    //ensure third argument is valid first argument base.
    //eg. if base1 = 11 then all chars <= 'a' etc
    let max_char: char = char_map(args[0].parse::<u8>().unwrap()).unwrap();
    for c in args[2].chars() {
        if c >= max_char {
            eprintln!("`{}` exceeds the exclusive max value `{}` of base {}", c, max_char, args[0]);
            std::process::exit(1);
        }
    }
}

fn char_map(i: u8) -> Option<char> {
    // assumes no base greater than 36
    match i {
        0..=9 => Some((i + 48) as char),
        10..=36 => Some((i + 87) as char),
        _ => None,
    }
}

fn map_char(c: char) -> Option<u8> {
    let c_low: char = c.to_ascii_lowercase();
    match c_low {
        '0'..='9' => Some((c_low as u8) - 48u8),
        'a'..='z' => Some((c_low as u8) - 87u8),
        _ => None,
    }
}

fn base_to_dec(in_val: &str, base: usize) -> usize {
    //handle single char case
    if in_val.len() == 1 {
        let first_char: char = in_val.chars().next().unwrap();
        return map_char(first_char).unwrap().try_into().unwrap();
    }

    //handle leading 0x chars if present
    let in_str: String = match &in_val[0..2] {
        "0x" | "0b" | "0o" => in_val[2..].to_string(),
        _ => in_val.to_string(),
    };

    let mut out_val: usize = 0;
    for (i, c) in in_str.chars().rev().enumerate() {
        let char_val: usize = map_char(c).unwrap().try_into().unwrap();
        let pval: usize = usize::pow(base, i.try_into().unwrap());
        out_val += char_val * pval;
    }
    out_val
}

fn dec_to_base(in_dec: usize, base: usize) -> String {
    if in_dec == 0 {
        return "0".to_string();
    }

    let mut output = String::new();
    let mut cur_val = in_dec;
    while cur_val != 0 {
        let res = (cur_val / base, cur_val % base);
        cur_val = res.0;
        output.insert(0, char_map(res.1.try_into().unwrap()).expect("whoops"))
    }
    output
}

fn convert_value(bases: (usize, usize), val: &str) -> String {
    let mut is_neg: bool = false;
    let mut use_val = val;

    // handle negative values as absolute values
    if val.chars().next().unwrap() == '-' {
        is_neg = true;
        use_val = &val[1..];
    }

    let conv_val = match bases {
        // dec to base
        (10, _) => {
            let dec_val: usize = use_val.parse().unwrap();
            dec_to_base(dec_val, bases.1)
        }
        // base to dec
        (_, 10) => format!("{}", base_to_dec(use_val, bases.0)),
        // base to base
        _ => {
            let b2d: usize = base_to_dec(use_val, bases.0);
            dec_to_base(b2d, bases.1)
        }
    };

    match is_neg {
        true => String::from("-") + &conv_val,
        false => conv_val,
    }
}

fn main() {
    let args = get_args();
    let bases: (usize, usize) = (args[0].parse().unwrap(), args[1].parse().unwrap());
    println!("{}", convert_value(bases, &args[2]));
}

#[cfg(test)]
mod tests {
    use super::*; // brings main scope into test scope

    #[test]
    fn dec_to_base_test() {
        let vals: Vec<(usize, &str)> = vec![
            (0, "0"),
            (10, "a"),
            (15, "f"),
            (256, "100"),
            (4660, "1234"),
            (65535, "ffff"),
        ];

        for v in vals {
            assert_eq!(dec_to_base(v.0, 16), v.1);
        }
    }

    #[test]
    fn base_to_dec_test() {
        let vals: Vec<(&str, usize)> = vec![
            ("0", 0),
            ("a", 10),
            ("f", 15),
            ("100", 256),
            ("1234", 4660),
            ("ffff", 65535),
        ];

        for v in vals {
            assert_eq!(base_to_dec(v.0, 16), v.1);
        }
    }

    #[test]
    fn char_map_test() {
        let vals: Vec<(u8, Option<char>)> = vec![
            (0, Some('0')),
            (5, Some('5')),
            (10, Some('a')),
            (15, Some('f')),
            (100, None),
        ];

        for v in vals {
            assert_eq!(char_map(v.0), v.1);
        }
    }

    #[test]
    fn map_char_test() {
        let vals: Vec<(char, Option<u8>)> = vec![
            ('a', Some(10)),
            ('0', Some(0)),
            ('f', Some(15)),
            ('5', Some(5)),
            ('z', Some(35)),
            ('Z', Some(35)),
        ];

        for v in vals {
            assert_eq!(map_char(v.0), v.1);
        }
    }

    #[test]
    fn convert_value_test() {
        let vals: Vec<((usize, usize), &str, &str)> = vec![
            ((10, 10), "100", "100"),
            ((10, 16), "10", "a"),
            ((10, 16), "4660", "1234"),
            ((10, 8), "668", "1234"),
            ((8, 10), "1234", "668"),
            ((8, 16), "100", "40"),
            ((16, 10), "0xffff", "65535"),
            ((10, 16), "-10", "-a"),
            ((10, 30), "1000", "13a"),
            ((30, 10), "13a", "1000"),
        ];

        for v in vals {
            assert_eq!(convert_value(v.0, v.1), v.2);
        }
    }
}

/*
    TODO:
    - better error handling
    - check all valid chars
*/
