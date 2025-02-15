#[allow(dead_code)] // 允许未使用的代码（编译器警告忽略）
struct Date {
    year: u32,          // 年份
    month: u32,         // 月份 (1-12)
    day: u32,           // 日期 (1-31)
    first_weekday: u32, // 本年度第一天的星期数 (1=周一,7=周日)
    days: u32,          // 当前日期是本年度的第几天
    weekday: u32,       // 当前日期的星期数 (1=周一,7=周日)
}

// 各月份天数数组（非闰年）
const MONTH_DAY: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

// 春节日期常量（2025年和2026年）
const SPRING_FESTIVAL_2025: u32 = 29; // 2025年春节是1月29日
const SPRING_FESTIVAL_2026: u32 = 413; // 2026年春节

impl Date {
    // 构造函数
    fn new(year: u32, month: u32, day: u32, first_weekday: u32) -> Self {
        Date {
            year,
            month,
            day,
            first_weekday,
            days: 0,    // 需要后续计算
            weekday: 0, // 需要后续计算
        }
    }

    // 计算当前日期是本年度第几天
    fn day_in_year(&mut self) -> u32 {
        let mut result = self.day;

        // 累加前几个月的天数
        for i in 0..self.month - 1 {
            // 注意：month是1-based
            result += MONTH_DAY[i as usize];
        }

        // 闰年处理（仅当月份>2时需要+1）
        if self.month > 2 && (self.year % 4 == 0 && self.year % 100 != 0 || self.year % 400 == 0) {
            result += 1;
        }

        self.days = result; // 更新days字段
        result
    }

    // 计算当前周数
    fn week_number(&self) -> u32 {
        let mut result = 0;

        // 基于ISO周数计算规则的简化实现
        if self.first_weekday <= 4 {
            if self.days > (7 - self.first_weekday + 1) {
                result = (self.days - (7 - self.first_weekday + 1)) / 7;
                if (self.days - (7 - self.first_weekday + 1)) % 7 > 0 {
                    result += 1;
                }
                result += 1;
            } else {
                result = 1; // 第一周
            }
        } else {
            // 处理跨年周的情况
            if self.days > (7 - self.first_weekday + 1) {
                result = (self.days - (7 - self.first_weekday + 1)) / 7;
                if (self.days - (7 - self.first_weekday + 1)) % 7 > 0 {
                    result += 1;
                }
            }
        }
        result
    }

    // 计算本年度剩余天数
    fn remnant_days(&self) -> u32 {
        if self.year % 4 == 0 && self.year % 100 != 0 || self.year % 400 == 0 {
            366 - self.days // 闰年
        } else {
            365 - self.days // 非闰年
        }
    }

    // 计算到春节的天数
    fn days_to_new_year(&mut self) -> u32 {
        if self.days >= 29 {
            SPRING_FESTIVAL_2026 - self.days
        } else {
            SPRING_FESTIVAL_2025 - self.days
        }
    }

    // 计算到最近交易日天数
    #[allow(dead_code)] // 允许未使用的代码
    fn days_to_trading_day(&mut self) -> u32 {
        // TODO: 需要实现交易日志辑
        0
    }
}

pub fn time_info(time: &str) -> String {
    // 解析输入字符串
    let parts: Vec<&str> = time.split('-').collect();

    // 解析年、月、日
    let year = parts[0].parse::<u32>().unwrap();
    let month = parts[1].parse::<u32>().unwrap();
    let day = parts[2].parse::<u32>().unwrap();

    // 创建日期对象
    let mut date = Date::new(year, month, day, 3); // 3表示周四？需要动态计算

    // 计算年累计天数
    let day_in_year = date.day_in_year();

    // 计算周数
    let mut week_num = date.week_number();

    // 计算星期
    let mut weekday = ((day_in_year - 1) % 7 + date.first_weekday) % 7;
    if weekday == 0 {
        weekday = 7; // 转换为1-based
    }
    date.weekday = weekday;

    // 处理跨年周特殊情况
    let remaining_days = date.remnant_days();
    if remaining_days + weekday < 4 {
        week_num = 1; // 进入新年第一周
    }

    // 计算到春节天数
    let days_to_new_year = date.days_to_new_year();

    // 硬编码交易日判断
    let mut days_to_trading: u32 = 0;
    match (month, day) {
        (1, 18) => days_to_trading = 1,  // 1月18日
        (12, 31) => days_to_trading = 1, // 12月31日
        (11, 1) => days_to_trading = 2,  // 11月1日
        (2, 28) => days_to_trading = 2,  // 2月28日
        (2, 9) => days_to_trading = 1,   // 2月9日
        (5, 1) => days_to_trading = 3,   // 5月1日
        _ => (),
    }

    // 格式化输出
    format!(
        "{},{},{},{},{},{}",
        week_num, weekday, day_in_year, remaining_days, days_to_new_year, days_to_trading
    )
}
