pub fn find_max_prime_factor(mut n: u128) -> u128 {
    // 如果输入为0，直接返回0
    if n == 0 {
        return 0;
    }

    // 初始化最大质因数
    let mut max_prime = 1;

    // 处理所有2的因子
    if n % 2 == 0 {
        max_prime = 2; // 更新最大质因数为2
        while n % 2 == 0 {
            n /= 2; // 不断除以2，去除所有2的因子
        }
        // 如果n变为1，说明2是最大质因数
        if n == 1 {
            return max_prime;
        }
        // 如果剩下的n是质数，则返回max_prime和n中的较大者
        if is_prime(n) {
            return max_prime.max(n);
        }
    }

    // 从3开始，检查奇数因子
    let mut i = 3;
    while i <= n / i {
        // 如果i是n的因子
        if n % i == 0 {
            max_prime = i; // 更新最大质因数
            while n % i == 0 {
                n /= i; // 不断除以i，去除所有i的因子
            }
            // 如果n变为1，说明i是最大质因数
            if n == 1 {
                return max_prime;
            }
            // 如果剩下的n是质数，则返回max_prime和n中的较大者
            if is_prime(n) {
                return max_prime.max(n);
            }
        }
        i += 2; // 只检查奇数因子
    }

    // 如果n仍然大于1，说明n本身是质数
    if n > 1 {
        max_prime = max_prime.max(n);
    }

    // 返回最大质因数
    max_prime
}

fn is_prime(n: u128) -> bool {
    // 处理特殊情况
    if n <= 1 {
        return false; // 小于等于1的数不是质数
    } else if n <= 3 {
        return true; // 2和3是质数
    } else if n % 2 == 0 {
        return false; // 偶数不是质数
    }

    // 将n-1分解为d * 2^s
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }

    // 测试的基底，针对u128选择足够多的基底以确保准确性
    let bases = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43];
    for &a in &bases {
        if a >= n {
            continue; // 跳过大于等于n的基底
        }
        let mut x = mod_pow(a, d, n); // 计算a^d mod n
        if x == 1 || x == n - 1 {
            continue; // 如果x为1或n-1，继续下一个基底
        }
        let mut found = false;
        for _ in 0..s - 1 {
            x = mod_pow(x, 2, n); // 计算x^2 mod n
            if x == n - 1 {
                found = true; // 如果x为n-1，标记为找到
                break;
            }
        }
        if !found {
            return false; // 如果未找到，n不是质数
        }
    }
    true // 通过所有基底测试，n是质数
}

fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    let mut result = 1;
    base %= modulus; // 先对base取模
    while exp > 0 {
        if exp % 2 == 1 {
            result = mul_mod(result, base, modulus); // 如果exp是奇数，累乘结果
        }
        base = mul_mod(base, base, modulus); // 平方base
        exp >>= 1; // 右移一位，相当于exp /= 2
    }
    result // 返回结果
}

fn mul_mod(a: u128, b: u128, m: u128) -> u128 {
    let mut a = a % m; // 先对a取模
    let mut b = b % m; // 先对b取模
    let mut res = 0;
    while b > 0 {
        if b % 2 == 1 {
            res = (res + a) % m; // 如果b是奇数，累加a到结果
        }
        a = (a * 2) % m; // 将a乘以2
        b /= 2; // 右移一位，相当于b /= 2
    }
    res // 返回结果
}
