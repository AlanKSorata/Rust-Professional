pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // TODO

    // 解析输入字符串，提取数值部分和基数部分
    let (num_part, from_base) = parse_input(num_str);

    // 将输入数值从其原始基数转换为十进制
    let decimal = to_decimal(&num_part, from_base);

    // 将十进制数转换为目标基数的字符串表示
    from_decimal(decimal, to_base)
}

// 解析输入字符串，提取数值部分和基数部分
fn parse_input(num_str: &str) -> (String, u32) {
    let parts: Vec<&str> = num_str.split('(').collect();
    let num_part = parts[0].to_owned();
    let base_part = parts[1].trim_end_matches(')');
    let from_base = base_part.parse::<u32>().unwrap();
    (num_part, from_base)
}

// 将任意基数的数值转换为十进制
fn to_decimal(num_str: &str, from_base: u32) -> u32 {
    let mut decimal = 0;
    for (i, c) in num_str.chars().rev().enumerate() {
        let digit = char_to_digit(c);
        decimal += digit * from_base.pow(i as u32);
    }
    decimal
}

// 将十进制数转换为目标基数的字符串表示
fn from_decimal(mut decimal: u32, to_base: u32) -> String {
    if decimal == 0 {
        return "0".to_owned();
    }
    let mut result = String::new();
    while decimal > 0 {
        let remainder = decimal % to_base;
        result.push(digit_to_char(remainder));
        decimal /= to_base;
    }
    result.chars().rev().collect()
}

// 将字符转换为对应的数字值
fn char_to_digit(c: char) -> u32 {
    match c {
        '0'..='9' => c.to_digit(10).unwrap(),
        'a'..='f' => c.to_digit(16).unwrap(),
        'A'..='F' => c.to_digit(16).unwrap(),
        _ => panic!("Invalid character in input"),
    }
}

// 将数字值转换为对应的字符
fn digit_to_char(digit: u32) -> char {
    match digit {
        0..=9 => (digit as u8 + b'0') as char,
        10..=15 => (digit as u8 - 10 + b'a') as char,
        _ => panic!("Invalid digit for base conversion"),
    }
}
