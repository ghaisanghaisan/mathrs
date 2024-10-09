pub mod primes {
    pub fn get_primes_upto(n: i32) -> Result<Vec<i32>, String> {
        if n <= 0 {
            return Err("n must be a positive integer".to_string());
        }
        let mut sieve = vec![false; n as usize];
        let mut primes = Vec::new();

        for i in 2..n {
            if sieve[i as usize] {
                continue;
            }

            primes.push(i as i32);

            let mut idx = 1;
            while i * idx < n {
                let multiple = idx * i;
                sieve[multiple as usize] = true;
                idx += 1;
            }
        }

        Ok(primes)
    }
}

pub mod factorization {
    use std::collections::HashSet;

    use super::primes::get_primes_upto;

    pub fn get_factors(n: i32) -> Result<Vec<i32>, String> {
        let primes = get_primes_upto(n)?;

        let mut fact_set: HashSet<i32> = HashSet::from([1, n]);

        let mut num = n.clone();

        for p in primes {
            if fact_set.contains(&p) {
                break;
            }
            let mut used = 1;
            while num % p == 0 {
                fact_set.insert(p.pow(used));
                num /= p;

                fact_set.insert(num);

                used += 1;
            }
        }

        let mut factors: Vec<i32> = fact_set.into_iter().collect();

        factors.sort();

        Ok(factors)
    }
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//}
