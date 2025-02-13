pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    // TODO: 这里写逻辑
    let mut fib_vec = Vec::new();
    let mut i = 0u32;
    loop {
        if fibonacci(i) >= threshold {
            break;
        }
        fib_vec.push(fibonacci(i));
        i += 1;
    }
    let fib_vec = fib_vec
        .into_iter()
        .filter(|&x| x % 2 != 0)
        .collect::<Vec<u32>>();
    // println!("{:?}", fib_vec);
    fib_vec.iter().sum()
}

pub fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 || n == 2 {
        return 1;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}
