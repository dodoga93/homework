/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
    let numbers = 
    let mut primes = Vec::new();

    for i in 2..n {
        if !primes.contains(&i) {
            for j in i*i..n {
                if j % i != 0 {
                    
                }
            }
        }
    }
    // TODO
    unimplemented!();
}
