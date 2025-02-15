pub fn goldbach_conjecture() -> String {
    let mut primes = vec![2];
    let mut results = vec![];
    let mut candidate = 3;

    while results.len() < 2 {
        if is_prime(candidate) {
            primes.push(candidate);
        } else {
            let mut found = false;

            // 仅遍历小于候选数的质数
            for &p in primes.iter().take_while(|&&p| p < candidate) {
                let remainder = candidate - p;
                if remainder % 2 != 0 {
                    continue;
                }

                // 平方根验证
                let k_squared = remainder / 2;
                let k = (k_squared as f64).sqrt() as u64;
                if k * k == k_squared {
                    found = true;
                    break;
                }
            }

            if !found {
                results.push(candidate);
            }
        }

        candidate += 2;
    }

    format!("{},{}", results[0], results[1])
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    } else if n <= 3 {
        return true;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let sqrt_n = (n as f64).sqrt() as u64;
    let mut i = 5;
    let mut w = 2;

    while i <= sqrt_n {
        if n % i == 0 {
            return false;
        }
        i += w;
        w = 6 - w; // 跳过3的倍数（步长交替为2和4）
    }
    true
}
