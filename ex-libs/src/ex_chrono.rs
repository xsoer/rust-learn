use chrono::{DateTime, Datelike, Local, TimeZone, Timelike, Utc};


pub fn c_chrono() {
    // 获取utc时间
    let utc_datetime = Utc::now();
    println!("utc_datetime: {}", utc_datetime); // utc_datetime: 2021-11-01 10:05:22.349150 UTC

    // 获取local时间
    let local_datetime = Local::now();
    println!("local_datetime: {}", local_datetime); //local_datetime: 2021-11-01 18:05:22.349230 +08:00

    // 获取日期的属性
    println!("local_datetime 年: {}", local_datetime.year());
    // local_datetime 年: 2021
    println!(
        "local_datetime 月: 12月制：{}， 11个月制：{}",
        local_datetime.month(),
        local_datetime.month0()
    );
    // local_datetime 月: 12月制：11， 11个月制：10
    println!("local_datetime 日: {}", local_datetime.day());
    // local_datetime day: 1
    println!(
        "local_datetime 小时: 24小时制:{}, 12小时制: {:?}",
        local_datetime.hour(),
        local_datetime.hour12()
    );
    // local_datetime 小时: 24小时制:19, 12小时制: (true, 7)
    println!("local_datetime 分钟: {}", local_datetime.minute());
    // local_datetime local_datetime 分钟: 36
    println!("local_datetime 秒: {}", local_datetime.second());
    // local_datetime 秒: 31
    println!("local_datetime 周几: {}", local_datetime.weekday());
    // local_datetime 周几: Mon
    println!(
        "local_datetime 周几(数字): {}",
        local_datetime.weekday().number_from_monday()
    );
    // local_datetime 周几(数字): 1
    println!("local_dateimte 一年的第几天: {}", local_datetime.ordinal());
    // local_dateimte 一年的第几天: 305

    // 日期时间转时间戳。Utc和Local转换后的时间戳是一致的
    println!("utc timestamp: {}", utc_datetime.timestamp()); // utc timestamp: 1635761246
    println!("local timestamp: {}", local_datetime.timestamp()); // local timestamp: 1635761246

    // 把时间转换成字符串
    let local_str = local_datetime.to_string();
    println!("lcoal_str: {}", local_str); //lcoal_str: 2021-11-01 19:24:38.939970 +08:00

    // 格式化字符串
    let local_str1 = local_datetime.format("%Y-%m-%dT%H:%M:%S").to_string();
    println!("local_str1: {}", local_str1); //local_str1: 2021-11-01T19:24:38

    // 只展示日期
    let local_str2 = local_datetime.format("%Y-%m-%d").to_string();
    println!("local_str2: {}", local_str2); //local_str2: 2021-11-01

    // 只展示时间
    let local_str3 = local_datetime.format("%H:%M:%S").to_string();
    println!("local_str3: {}", local_str3); //local_str3: 19:24:38

    // 获取日期格式
    let local_date = local_datetime.date();
    println!("local_date: {}", local_date); //local_date: 2021-11-01+08:00
    assert_eq!(Utc::today(), Utc::now().date());
    assert_eq!(Local::today(), Local::now().date());

    // 手动创建日期时间
    let local_handle_datetime1 = Local.ymd(2021, 10, 15).and_hms(10, 12, 30);
    println!("local_handle_datetime1: {}", local_handle_datetime1); //local_handle_datetime1: 2021-10-15 10:12:30 +08:00

    // 手动创建带有毫秒级的时间
    let local_handle_datetime2 = Local.ymd(2021, 10, 27).and_hms_milli(18, 18, 18, 889);
    println!("local_handle_datetime2: {}", local_handle_datetime2); // local_handle_datetime2: 2021-10-20 18:18:18.889 +08:00

    // 两个日期时间的差值
    let sub_datetime = local_handle_datetime2 - local_handle_datetime1;
    println!(
        "相差：{}天, {}小时, {}分钟, {}秒, {}周. 格式化：{}",
        sub_datetime.num_days(),
        sub_datetime.num_hours(),
        sub_datetime.num_minutes(),
        sub_datetime.num_seconds(),
        sub_datetime.num_weeks(),
        sub_datetime.to_string()
    );
    // 相差：12天, 296小时, 17765分钟, 1065948秒, 1周. 格式化：P12DT29148.889S

    // 获取前3天的日期
    let three_days_before = local_datetime - chrono::Duration::days(3);
    println!("three_days_before: {}", three_days_before); //three_days_before: 2021-10-29 19:55:41.982785 +08:00

    // 获取下周的今天
    let next_week_day = local_datetime + chrono::Duration::weeks(1);
    println!("next_week_day: {}", next_week_day); // next_week_day: 2021-11-08 19:55:41.982785 +08:00

    // 字符串转换时间
    let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
    let fixed_dt = dt.with_timezone(&chrono::FixedOffset::east(8 * 3600)); // 设置时区
                                                                           // 用parse来转换
    assert_eq!(
        "2014-11-28T12:00:09Z".parse::<chrono::DateTime<Utc>>(),
        Ok(dt.clone())
    );
    assert_eq!(
        "2014-11-28T20:00:09+08:00".parse::<chrono::DateTime<Utc>>(),
        Ok(dt.clone())
    );
    assert_eq!(
        "2014-11-28T20:00:09+08:00".parse::<chrono::DateTime<chrono::FixedOffset>>(),
        Ok(fixed_dt.clone())
    );
    // 用DateTime结构来转换
    assert_eq!(
        DateTime::parse_from_str("2014-11-28 21:00:09 +09:00", "%Y-%m-%d %H:%M:%S %z"),
        Ok(fixed_dt.clone())
    );
    assert_eq!(
        DateTime::parse_from_rfc2822("Fri, 28 Nov 2014 21:00:09 +0900"),
        Ok(fixed_dt.clone())
    );
    assert_eq!(
        DateTime::parse_from_rfc3339("2014-11-28T21:00:09+09:00"),
        Ok(fixed_dt.clone())
    );

    // 通过Utc的datetime_from_str转换
    assert_eq!(
        Utc.datetime_from_str("2014-11-28 12:00:09", "%Y-%m-%d %H:%M:%S"),
        Ok(dt.clone())
    );
    assert_eq!(
        Utc.datetime_from_str("Fri Nov 28 12:00:09 2014", "%a %b %e %T %Y"),
        Ok(dt.clone())
    );

    // 时间戳转换成时间
    let dt = Utc.timestamp(1_500_000_000, 0);
    assert_eq!(dt.to_rfc2822(), "Fri, 14 Jul 2017 02:40:00 +0000");

    // Utc转换成Local
    let utc_now = Utc::now();
    let local_now = utc_now
        .with_timezone(&chrono::FixedOffset::east(8 * 3600))
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();
    println!("local_now {}", local_now);
}

pub mod date_format {
    use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date
            .with_timezone(&FixedOffset::east(8 * 3600))
            .format(FORMAT)
            .to_string();
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let r = Local
            .datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)?;
        Ok(DateTime::from(r))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn t_c_chrono() {
        c_chrono();
    }
}
