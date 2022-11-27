use git_date::{
    time::{format, Format, Sign},
    Time,
};
use time::{macros::format_description, UtcOffset};

#[test]
fn short() {
    assert_eq!(time().format(format::SHORT), "1973-11-29");
}

#[test]
fn unix() {
    let expected = "123456789";
    assert_eq!(time().format(Format::Unix), expected);
    assert_eq!(time().format(format::UNIX), expected);
}

#[test]
fn raw() {
    let expected = "123456789 +0230";
    assert_eq!(time().format(Format::Raw), expected);
    assert_eq!(time().format(format::RAW), expected);
}

#[test]
fn iso8601() {
    assert_eq!(time().format(format::ISO8601), "1973-11-29 21:33:09 +0230");
}

#[test]
fn iso8601_strict() {
    assert_eq!(time().format(format::ISO8601_STRICT), "1973-11-29T21:33:09+02:30");
}

#[test]
fn rfc2822() {
    assert_eq!(time().format(format::RFC2822), "Thu, 29 Nov 1973 21:33:09 +0230");
}

#[test]
fn default() {
    assert_eq!(
        time().format(git_date::time::format::DEFAULT),
        "Thu Nov 29 1973 21:33:09 +0230"
    );
}

#[test]
fn human() {
    let expected = "in the future";
    assert_eq!(
        format::human_format_comparing_to(raw_time(), raw_time() - time::Duration::DAY).unwrap(),
        expected
    );

    let expected = "0 seconds ago";
    assert_eq!(
        format::human_format_comparing_to(raw_time(), raw_time()).unwrap(),
        expected
    );

    let expected = "1 second ago";
    assert_eq!(
        format::human_format_comparing_to(raw_time(), raw_time() + time::Duration::SECOND).unwrap(),
        expected
    );

    let expected = "89 seconds ago";
    assert_eq!(
        format::human_format_comparing_to(raw_time(), raw_time() + time::Duration::SECOND * 89).unwrap(),
        expected
    );

    let expected = "89 minutes ago";
    assert_eq!(
        format::human_format_comparing_to(raw_time(), raw_time() + time::Duration::MINUTE * 89).unwrap(),
        expected
    );

    let expected = "2 hours ago";
    assert_eq!(
        format::human_format_comparing_to(raw_time(), raw_time() + time::Duration::MINUTE * 90).unwrap(),
        expected
    );

    // TODO: Verify why this is showing as 3 hours ago......???? There's a timezone misunderstanding here somewhere
    let expected = "Thu 21:33 +0230";
    assert_eq!(
        format::human_format_comparing_to(raw_time(), raw_time().replace_offset(UtcOffset::UTC)).unwrap(),
        expected
    );

    // Timezone matches, but was more than a week ago
    let expected = "Thu Nov 29 21:33";
    assert_eq!(
        format::human_format_comparing_to(raw_time(), raw_time() + time::Duration::WEEK).unwrap(),
        expected
    );

    // Timezone does not match, more than a week ago
    let expected = "Thu Nov 29 21:33";
    assert_eq!(
        format::human_format_comparing_to(
            raw_time(),
            raw_time().replace_offset(UtcOffset::UTC) + time::Duration::WEEK
        )
        .unwrap(),
        expected
    );

    // Time was previous year
    let expected = "Nov 29 1973";
    assert_eq!(
        format::human_format_comparing_to(raw_time(), raw_time() + time::Duration::DAY * 365).unwrap(),
        expected
    );
}

#[test]
fn custom_compile_time() {
    assert_eq!(
        time().format(format_description!("[year]-[month]-[day] [hour]:[minute]:[second]")),
        "1973-11-29 21:33:09",
    );
}

fn raw_time() -> time::OffsetDateTime {
    let t = time();
    time::OffsetDateTime::from_unix_timestamp(t.seconds_since_unix_epoch as i64)
        .unwrap()
        .replace_offset(time::UtcOffset::from_whole_seconds(t.offset_in_seconds).expect("valid offset"))
}

fn time() -> Time {
    Time {
        seconds_since_unix_epoch: 123456789,
        offset_in_seconds: 9000,
        sign: Sign::Plus,
    }
}
