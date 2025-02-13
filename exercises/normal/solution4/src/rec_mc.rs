pub fn dp_rec_mc(amount: u32) -> u32 {
    // TODO: 这里写逻辑
    let mut res = 0;
    let mut cur = amount;
    let mut currencies = vec![1, 2, 5, 10, 20, 30, 50, 100];
    currencies.reverse();
    for kind in currencies.into_iter() {
        while cur >= kind {
            cur -= kind;
            res += 1;
        }
    }
    res
}
