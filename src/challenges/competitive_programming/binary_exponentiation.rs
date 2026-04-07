use fun_stuff::parse_input;

struct BinaryExponentiation;

enum CalculationType {
    Recursion,
    Iterative,
}

impl BinaryExponentiation {
    pub fn new(num: u128, exponent: u128, action: CalculationType) -> u128 {
        match action {
            CalculationType::Recursion => BinaryExponentiation::power_recursive(num, exponent),
            CalculationType::Iterative => BinaryExponentiation::power_iteratively(num, exponent),
            _ => panic!("Not implemented"),
        }
    }

    pub fn power_recursive(a: u128, b: u128) -> u128 {
        if b == 0 {
            return 1;
        }

        let res = BinaryExponentiation::power_recursive(a, b / 2);

        if b % 2 == 1 {
            res * res * a
        } else {
            res * res
        }
    }

    pub fn power_iteratively(mut a: u128, mut b: u128) -> u128 {
        let mut result: u128 = 1;
        while b > 0 {
            if b & 1 == 1 {
                result = result * a
            }
            a = a * a;
            b >>= 1;
        }

        result
    }
}



pub fn run() {
    println!("2**12 {}", BinaryExponentiation::new(2, 12, CalculationType::Iterative));
    println!("12**8 {}", BinaryExponentiation::new(12, 8, CalculationType::Iterative));
    println!("8**8 {}", BinaryExponentiation::new(8, 8, CalculationType::Iterative));
    println!("10**150 {}", BinaryExponentiation::new(10, 19, CalculationType::Iterative));
}
