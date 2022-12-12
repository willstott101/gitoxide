use time::{format_description::FormatItem, macros::format_description};

use crate::{time::Format, Time};

/// E.g. `2018-12-24`
pub const SHORT: &[FormatItem<'_>] = format_description!("[year]-[month]-[day]");

/// E.g. `Thu, 18 Aug 2022 12:45:06 +0800`
pub const RFC2822: &[FormatItem<'_>] = format_description!(
    "[weekday repr:short], [day] [month repr:short] [year] [hour]:[minute]:[second] [offset_hour sign:mandatory][offset_minute]"
);

/// E.g. `2022-08-17 22:04:58 +0200`
pub const ISO8601: &[FormatItem<'_>] =
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour sign:mandatory][offset_minute]");

/// E.g. `2022-08-17T21:43:13+08:00`
pub const ISO8601_STRICT: &[FormatItem<'_>] =
    format_description!("[year]-[month]-[day]T[hour]:[minute]:[second][offset_hour sign:mandatory]:[offset_minute]");

/// E.g. `123456789`
pub const UNIX: Format<'static> = Format::Unix;

/// E.g. `1660874655 +0800`
pub const RAW: Format<'static> = Format::Raw;

/// E.g. `Thu Sep 04 2022 10:45:06 -0400`
pub const DEFAULT: &[FormatItem<'_>] = format_description!(
    "[weekday repr:short] [month repr:short] [day] [year] [hour]:[minute]:[second] [offset_hour sign:mandatory][offset_minute]"
);

/// E.g. `Dec 31 2021`
const HUMAN_OTHER_YEAR: &[FormatItem<'_>] = format_description!("[month repr:short] [day] [year]");
/// E.g. `Thu Sep 04 10:45`
const HUMAN_SAME_YEAR: &[FormatItem<'_>] =
    format_description!("[weekday repr:short] [month repr:short] [day] [hour]:[minute]");
/// If day-of-week is enough to determine the date, but the timezones don't match
const HUMAN_SAME_WEEK: &[FormatItem<'_>] =
    format_description!("[weekday repr:short] [hour]:[minute] [offset_hour sign:mandatory][offset_minute]");
/// If day-of-week is enough to determine the date
const HUMAN_SAME_WEEK_NO_OFFSET: &[FormatItem<'_>] = format_description!("[weekday repr:short] [hour]:[minute]");

mod format_impls {
    use time::format_description::FormatItem;

    use crate::time::Format;

    impl<'a> From<&'a [FormatItem<'a>]> for Format<'a> {
        fn from(f: &'a [FormatItem<'a>]) -> Self {
            Format::Custom(f)
        }
    }
}

/// Formats the given time in relation to the `compare` time to produce a more human time representation. Excludes the timezone offset if it matches the local offset. Excludes the date if the day was in the past week (only including the day of the week).
pub fn human_format_comparing_to(
    format: time::OffsetDateTime,
    compare: time::OffsetDateTime,
) -> Result<String, time::error::Format> {
    // `git log --format="human= %<(25)%ah human-local= %<(25)%ad iso=%ai" --date=human-local`
    // human= 4 hours ago               human-local= 4 hours ago               iso=2022-11-27 10:15:32 +0100
    // human= Fri 11:31                 human-local= Fri 11:31                 iso=2022-11-25 11:31:34 +0000
    // human= Wed 20:01 +0100           human-local= Wed 19:01                 iso=2022-11-23 20:01:23 +0100
    // human= Sun Nov 20 22:39          human-local= Mon Nov 21 03:39          iso=2022-11-20 22:39:26 -0500
    // human= Dec 31 2021               human-local= Dec 31 2021               iso=2021-12-31 20:41:47 +0800

    let show_offset = compare.offset() != format.offset();

    let difference = compare - format;

    // TODO: Translation
    if format > compare {
        Ok("in the future".to_string())
    } else if format.year() != compare.year() {
        format.format(HUMAN_OTHER_YEAR)
    } else if difference > (time::Duration::DAY * 4) {
        format.format(HUMAN_SAME_YEAR)
    } else if format.day() == compare.day() {
        let secs = difference.whole_seconds();
        if secs == 1 {
            // All other breakpoints round-up to >= 2 so can be plural
            Ok("1 second ago".to_string())
        } else if secs < 90 {
            Ok(format!("{} seconds ago", secs))
        } else if secs < 5400 {
            let mins = ((secs as f32) / 60f32).round();
            Ok(format!("{} minutes ago", mins))
        } else {
            let hours = ((secs as f32) / 3600f32).round();
            Ok(format!("{} hours ago", hours))
        }
    } else if show_offset {
        format.format(HUMAN_SAME_WEEK)
    } else {
        format.format(HUMAN_SAME_WEEK_NO_OFFSET)
    }
}

/// Formatting
impl Time {
    /// Format this instance according to the given `format`.
    ///
    /// Use the [`format_description`](https://time-rs.github.io/book/api/format-description.html) macro to create and
    /// validate formats at compile time, courtesy of the [`time`] crate.
    pub fn format<'a>(&self, format: impl Into<Format<'a>>) -> String {
        match format.into() {
            Format::Custom(format) => self
                .to_time()
                .format(&format)
                .expect("well-known format into memory never fails"),
            Format::Unix => self.seconds_since_unix_epoch.to_string(),
            Format::Raw => self.to_bstring().to_string(),
            Format::Human => if let Ok(compare) = time::OffsetDateTime::now_local() {
                human_format_comparing_to(self.to_time(), compare)
            } else {
                self.to_time().format(&DEFAULT)
            }
            .expect("well-known format into memory never fails"),
        }
    }
}

impl Time {
    fn to_time(self) -> time::OffsetDateTime {
        time::OffsetDateTime::from_unix_timestamp(self.seconds_since_unix_epoch as i64)
            .expect("always valid unix time")
            .to_offset(time::UtcOffset::from_whole_seconds(self.offset_in_seconds).expect("valid offset"))
    }
}
