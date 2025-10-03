// p3. get largest prime factor
// idea: keep a vec of prime numbers, keep finding larger prime numbers and divide to get rid of smaller prime factors

pub fn p3(n: u64) -> u64 {
    let mut primes: Vec<u64> = vec![2];
    let mut target: u64 = n;

    while *primes.last().unwrap() < target {
        match target % *primes.last().unwrap() {
            0 => {
                while target % *primes.last().unwrap() == 0 {
                    target /= *primes.last().unwrap();
                }
            }
            _ => {}
        }

        let mut a = primes.last().unwrap() + 1;
        primes.push(loop {
            let prime: bool = primes.iter().all(|&p| a % p != 0);
            if prime {
                break a;
            }
            a += 1;
        });
    }
    *primes.last().unwrap()
}
