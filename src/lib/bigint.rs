struct BigInt {
    /// true for positive, false for negative
    sign: bool,
    /// Digits
    digits: Vec<u8>
}

impl BigInt {
    fn new(start: i64) {
        let sign = true;
        if start < 0 {
            sign = false;
        }
        BigInt {
            sign,
            digits: start
        }
    }
    fn add(a: i64, b: i64 ) {}
    fn subtraction(a: i64, b: i64 ) {}
    fn multiplication(a: i64, b: i64 ) {}
    fn division(a: i64, b: i64 ) {
    }
}