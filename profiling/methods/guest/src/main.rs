
use nalgebra::Matrix2;
use risc0_zkvm::guest::env;

fn main() {
    let iterations: u32 = env::read();
    let answer_1 = fibonacci_1(iterations);
    let answer_2 = fibonacci_2(iterations);
    let answer_3 = fibonacci_3(iterations);
    assert_eq!(answer_1, answer_2);
    assert_eq!(answer_1, answer_3);
    env::commit(&answer_1);
}



#[inline(never)]
pub fn fibonacci_1(n: u32) -> u64 {
    let (mut a, mut b) = (0, 1);
    if n <= 1 {
        return n as u64;
    }
    let mut i = 2;
    while i <= n {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
    }
    b
}

#[inline(never)]
pub fn fibonacci_2(n: u32) -> u64 {
    let (mut a, mut b) = (0, 1);
    if n <= 1 {
        return n as u64;
    }
    let mut i = 2;
    while i <= n {
        if i + 5 <= n {
            let c = a + b;
            let d = b + c;
            let e = c + d;
            let f = d + e;
            let g = e + f;
            a = f;
            b = g;
            i += 5;
        } else {
            let c = a + b;
            a = b;
            b = c;
            i += 1;
        }
    }
    b
}

#[inline(never)]
pub fn fibonacci_3(n: u32) -> u64 {
    Matrix2::new(1, 1, 1, 0).pow(n - 1)[(0, 0)]
}