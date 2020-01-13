// Class: CS 410P, Winter2020
// Instructor: Bart Massey
// Author: Trevor Johnson
// Last Updated: 1/12/20

fn main() {
    // let mut argv: [u64; 3];
    let mut argv: Vec<u64> = Vec::new();

    for arg in std::env::args().skip(1) {
        argv.push(arg.parse().unwrap_or_else(|_| error()));
    }

    if argv.len() < 3 {
        error();
    }
    println!("{}", modexp(argv[0], argv[1], argv[2]));
}

fn modexp(base: u64, exp: u64, m: u64) -> u64 {
    if base == 0 {
        // Base case 1
        return 0;
    } else if exp == 0 {
        // Base case 2
        return 1;
    }
    let mut result = modexp(base, exp / 2, m); // Recursive call with exp/2
    result = (result * result) % m;
    // if exp is odd
    if exp % 2 == 1 {
        result = result * base % m;
        return result;
    }
    result
}

fn error() -> ! {
    eprintln!(
        "\nError: Invalid args, Usage Example:: modexp <base(u64)> <exponent(u64)> <mod_by(u64)>\n"
    );
    std::process::exit(1);
}

// Begin Test Suite

#[test]
fn modexp_test1() {
    assert_eq!(modexp(2, 20, 17), 16);
    assert_eq!(modexp(3, 2, 3), 0);
    assert_eq!(modexp(0, 0, 0), 0);
    assert_eq!(modexp(9223372036854775807, 2, 2532411), 138460);
}
