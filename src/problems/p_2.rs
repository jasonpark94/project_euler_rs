// https://projecteuler.net/problem=2

// Fibonacci sequence whose values do not exceed four million,
// find the sum of the even-valued terms.

// 1 2 3 5 8 ... < 4,000,000
// fib(i) = fib(i-1) + fib(i-2)

pub fn p2(max: u32) -> u32 {
    let mut prev: u32 = 1;
    let mut curr: u32 = 2;
    let mut sum: u32 = 0;

    while curr < max {
        if curr % 2 == 0 {
            sum += curr;
        }
        let tmp: u32 = prev;
        prev = curr;
        curr += tmp;
    }

    sum
}
