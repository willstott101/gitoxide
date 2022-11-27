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

/// E.g. `Thu Sep 04 2022 10:45:06`
const DEFAULT_NO_OFFSET: &[FormatItem<'_>] =
    format_description!("[weekday repr:short] [month repr:short] [day] [year] [hour]:[minute]:[second]");
/// E.g. `Thu 10:45:06 -0400`
const DEFAULT_NO_DATE: &[FormatItem<'_>] =
    format_description!("[weekday repr:short] [hour]:[minute]:[second] [offset_hour sign:mandatory][offset_minute]");
/// If day-of-week is enough to determine the date
const HUMAN_SHORT: &[FormatItem<'_>] = format_description!("[weekday repr:short] [hour]:[minute]:[second]");

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
    // TODO: "9 minutes ago" and similar
    // TODO: Investigate original git? It seems to show timezones inconsistently
        // Investigate `git log --format="human= %<(25)%ah human-local= %<(25)%ad iso=%ai" --date=human-local`
        // human= 4 hours ago               human-local= 4 hours ago               iso=2022-11-27 10:15:32 +0100
        // human= Wed 20:01 +0100           human-local= Wed 19:01                 iso=2022-11-23 20:01:23 +0100
        // human= Mon Nov 21 07:02          human-local= Mon Nov 21 06:02          iso=2022-11-21 07:02:10 +0100
        // human= Sun Nov 20 22:39          human-local= Mon Nov 21 03:39          iso=2022-11-20 22:39:26 -0500
    let show_offset = compare.offset() != format.offset();
    // TODO: skip the year if equal
    let show_date = format > compare || (compare - format) > (time::Duration::DAY * 6);

    format.format(match (show_offset, show_date) {
        (true, true) => &DEFAULT,
        (false, true) => &DEFAULT_NO_OFFSET,
        (true, false) => &DEFAULT_NO_DATE,
        (false, false) => &HUMAN_SHORT,
    })
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
            .replace_offset(time::UtcOffset::from_whole_seconds(self.offset_in_seconds).expect("valid offset"))
    }
}
