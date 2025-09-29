use crate::problems::p_1::p1;
use crate::problems::p_2::p2;

mod problems;

fn main() {
    print!("p1 ans: {}", p1(3, 5, 1000));
    print!("p2 ans: {}", p2(4000000));
}
