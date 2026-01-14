use core::fmt;
use vec::Vec;
use std::vec;
use prime_factorization::Factorization;
const MAX_STEPS : u128 = 1000_u128;

#[derive(Debug)]
struct Fraction (u128, u128);

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.0, self.1)
    }
}


fn print_program(p : &Vec<Fraction>) {
    print!("[{}]\n", p.iter().fold(String::new(), |acc, arg| acc + &arg.0.to_string() + "/" + &arg.1.to_string() + ", "));
}

enum OptionalFraction {
    Frac(Fraction),
    Nothing
}



fn main() {
    
    println!("Fractran Interpreter");
    let i : u128 = 2_u128.pow(5)*3_u128.pow(6);
    println!("{:?}", Factorization::run(i).factors);
    println!("Input: {}", i);
    let p : &Vec<Fraction> = &vec![
        Fraction(455, 33),
        Fraction(11, 13),
        Fraction(1, 11),
        Fraction(3, 7),
        Fraction(11, 2),
        Fraction(1, 3),
        ];
    print_program(p);
    let result : Fraction = run_program(i, p);

    let num_repr : Factorization<u128> = Factorization::run(result.0);
    println!("Powers of PF of {} = {:?}", result.0, num_repr.factors);
}

fn run_program (input : u128, program : &Vec<Fraction>) -> Fraction {
    let mut steps = 0;
    let mut successes: u128 = 1; // Start loop
    let mut n : Fraction = Fraction(input, 1_u128);

    while successes != 0 {
        successes = 0;
        for part in program {
            if steps == MAX_STEPS {
                // End Program
                println!("Program Terminated due to Step Count");
                return n;
            }
            steps += 1;
            
            let res = int_multiply(&n, part);
            match res {
                OptionalFraction::Frac(t) => {
                    println!("{} * {} = {}", n, part, t);
                    successes += 1;
                    n = t;
                    break;
                },
                OptionalFraction::Nothing => {
                    println!("{} * {} not in N", n, part)
                    // Do nothing
                }
            }
        }
        println!("Pass");
    }
    println!("Program Terminated");
    return n;
}

fn int_multiply (a : &Fraction, b : &Fraction) -> OptionalFraction {
    let mut n : u128 = a.0 * b.0;
    let mut d : u128 = a.1 * b.1;
    let (big, little) = janky_max_min(n, d);
    let x = gcd(big, little);
    n /= x;
    if d == x {
        // Result is an integer
        OptionalFraction::Frac(Fraction(n, 1))
    } else {
        // Result is not an integer
        OptionalFraction::Nothing
    }
}

// Euclidean Algorithm
fn gcd(a : u128, b: u128) -> u128 {
    if a == 0 { 
        b
    }
    else {
        gcd(b % a, a)
    }
}

fn janky_max_min(a : u128, b : u128) -> (u128, u128) {
    if a > b {
        (a, b)
    }
    else {
        (b, a)
    }
}