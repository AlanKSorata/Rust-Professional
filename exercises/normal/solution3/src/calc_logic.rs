pub fn new_birthday_probability(n: u32) -> f64 {
    // TODO: 这里写逻辑
    let res = 1.0;
    let mut tmp = 1.0;
    for i in 0..n {
        tmp *= (365 - i) as f64 / 365.0;
    }
    res - tmp
}
