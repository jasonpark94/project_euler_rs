// https://projecteuler.net/problem=1

// P1: sum of all the multiples of 3 or 5 below 1000

// observations and idea

// 3 * 1 + 3 * 2 ... 3 * n < 1000
// +
// 5 * 1 + 5 * 2 ... 5 * m < 1000
// -
// 15 * 1 ... 15 * l < 1000

// fn f -> i * ( 1 + 2 ... (max - 1) / i)
// ans: f(3, max) + f(5, max) - f(15, max)

pub fn p1(a: u32, b: u32, max: u32) -> u32 {
    f(a, max) + f(b, max) - f(a * b, max)
}

fn f(a: u32, max: u32) -> u32 {
    a * sum((max - 1) / a)
}

fn sum(a: u32) -> u32 {
    if a % 2 == 0 {
        (a / 2) * (a + 1)
    } else {
        a * ((a + 1) / 2)
    }
}
