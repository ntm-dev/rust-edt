use time::macros::datetime;
use time_format::{TimestampFormat, format_date, format_date_medium, format_local_timestamp};

#[test]
fn vietnamese_timestamps() {
    i18n::set_language(i18n::Language::Vietnamese);

    let reference = datetime!(1990-04-12 23:00 UTC);
    let relative =
        |timestamp| format_local_timestamp(timestamp, reference, TimestampFormat::Relative);
    assert_eq!(relative(reference), "Vừa xong");
    assert_eq!(relative(datetime!(1990-04-12 22:59 UTC)), "1 phút trước");
    assert_eq!(relative(datetime!(1990-04-12 22:15 UTC)), "45 phút trước");
    assert_eq!(relative(datetime!(1990-04-12 22:00 UTC)), "1 giờ trước");
    assert_eq!(relative(datetime!(1990-04-12 03:00 UTC)), "20 giờ trước");
    assert_eq!(relative(datetime!(1990-04-11 12:00 UTC)), "Hôm qua");
    assert_eq!(relative(datetime!(1990-04-09 12:00 UTC)), "3 ngày trước");
    assert_eq!(relative(datetime!(1990-04-05 12:00 UTC)), "1 tuần trước");
    assert_eq!(relative(datetime!(1990-02-12 12:00 UTC)), "2 tháng trước");
    assert_eq!(relative(datetime!(1989-04-12 12:00 UTC)), "1 năm trước");
    assert_eq!(
        relative(datetime!(1989-03-12 12:00 UTC)),
        "1 năm 1 tháng trước"
    );
    assert_eq!(relative(datetime!(1980-04-12 12:00 UTC)), "10 năm trước");

    let earlier_today = datetime!(1990-04-12 12:00 UTC);
    let yesterday = datetime!(1990-04-11 12:00 UTC);
    assert_eq!(format_date(earlier_today, reference, true), "Hôm nay");
    assert_eq!(format_date(yesterday, reference, true), "Hôm qua");
    assert_eq!(format_date_medium(yesterday, reference, true), "Hôm qua");

    #[cfg(target_os = "macos")]
    {
        let absolute = |timestamp, format| format_local_timestamp(timestamp, reference, format);
        let enhanced_today = absolute(earlier_today, TimestampFormat::EnhancedAbsolute);
        assert!(enhanced_today.starts_with("Hôm nay "), "{enhanced_today}");
        let enhanced_yesterday = absolute(yesterday, TimestampFormat::EnhancedAbsolute);
        assert!(
            enhanced_yesterday.starts_with("Hôm qua "),
            "{enhanced_yesterday}"
        );
    }

    i18n::set_language(i18n::Language::English);
}
