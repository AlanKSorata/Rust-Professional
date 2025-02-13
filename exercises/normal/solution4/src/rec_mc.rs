pub fn dp_rec_mc(amount: u32) -> u32 {
    // TODO: 这里写逻辑
    let mut res = 0;
    let mut cur = amount.clone();
    let mut currencies = vec![1, 2, 5, 10, 20, 30, 50, 100];
    currencies.reverse();
    for kind in currencies.into_iter() {
        if cur <= 0 {
            break;
        }
        if cur >= kind {
            res += cur / kind;
            cur -= (cur / kind) * kind;
        }
    }
    res
}
