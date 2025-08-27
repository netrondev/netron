use std::str::FromStr;

#[cfg(feature = "ssr")]
use surrealdb::Datetime;

#[cfg(not(feature = "ssr"))]
use crate::Datetime;

#[allow(unused)]
pub fn format_date(datetime: &Datetime) -> String {
    #[cfg(feature = "ssr")]
    {
        // For surrealdb::Datetime
        datetime
            .to_string()
            .split('T')
            .next()
            .unwrap_or("")
            .to_string()
    }
    #[cfg(not(feature = "ssr"))]
    {
        // For crate::Datetime
        datetime.format("%Y-%m-%d")
    }
}

#[allow(unused)]
pub fn format_date_custom(datetime: &Datetime, format: &str) -> String {
    #[cfg(feature = "ssr")]
    {
        // For surrealdb::Datetime
        let date_str = datetime.to_string();
        if let Some(date_part) = date_str.split('T').next() {
            if format == "%Y-%m-%d" {
                date_part.to_string()
            } else {
                // For other formats, we can only return the date part
                date_part.to_string()
            }
        } else {
            String::new()
        }
    }
    #[cfg(not(feature = "ssr"))]
    {
        // For crate::Datetime
        datetime.format(format)
    }
}

#[allow(unused)]
pub fn format_datetime(datetime: &Datetime) -> String {
    #[cfg(feature = "ssr")]
    {
        // For surrealdb::Datetime - returns full ISO string
        datetime.to_string()
    }
    #[cfg(not(feature = "ssr"))]
    {
        // For crate::Datetime - format with date and time
        datetime.format("%Y-%m-%d %H:%M")
    }
}

#[allow(unused)]
pub fn format_time(unix_time: u64) -> (String, String) {
    use chrono::{DateTime, Local, Utc};

    // Convert unix timestamp to DateTime
    let datetime =
        DateTime::<Utc>::from_timestamp(unix_time as i64, 0).unwrap_or_else(|| Utc::now());

    // Convert to local time
    let local_time: DateTime<Local> = datetime.into();
    let formatted_time = local_time.format("%Y-%m-%d %H:%M:%S").to_string();

    // Calculate relative time
    let now = Local::now();
    let duration = now.signed_duration_since(local_time);

    let relative = if duration.num_seconds() < 60 {
        format!("{} seconds ago", duration.num_seconds())
    } else if duration.num_minutes() < 60 {
        let mins = duration.num_minutes();
        if mins == 1 {
            "1 minute ago".to_string()
        } else {
            format!("{} minutes ago", mins)
        }
    } else if duration.num_hours() < 24 {
        let hours = duration.num_hours();
        if hours == 1 {
            "1 hour ago".to_string()
        } else {
            format!("{} hours ago", hours)
        }
    } else if duration.num_days() < 30 {
        let days = duration.num_days();
        if days == 1 {
            "1 day ago".to_string()
        } else {
            format!("{} days ago", days)
        }
    } else if duration.num_days() < 365 {
        let months = duration.num_days() / 30;
        if months == 1 {
            "1 month ago".to_string()
        } else {
            format!("{} months ago", months)
        }
    } else {
        let years = duration.num_days() / 365;
        if years == 1 {
            "1 year ago".to_string()
        } else {
            format!("{} years ago", years)
        }
    };

    (formatted_time, relative)
}

pub enum TimeFormatVariant {
    Ago,
    Format(String),
}

#[allow(unused)]
pub fn format_time_iso(timestamp_iso: String, variant: TimeFormatVariant) -> (String, String) {
    use chrono::{DateTime, Local, Utc};

    // Convert unix timestamp to DateTime
    let datetime = DateTime::<Utc>::from_str(&timestamp_iso).unwrap_or_else(|_| Utc::now());

    // Convert to local time
    let local_time: DateTime<Local> = datetime.into();

    let formatted_time = local_time.format("%Y-%m-%d %H:%M:%S").to_string();

    match variant {
        TimeFormatVariant::Ago => {
            // Calculate relative time
            let now = Local::now();
            let duration = now.signed_duration_since(local_time);

            let relative = if duration.num_seconds() < 60 {
                format!("{} seconds ago", duration.num_seconds())
            } else if duration.num_minutes() < 60 {
                let mins = duration.num_minutes();
                if mins == 1 {
                    "1 minute ago".to_string()
                } else {
                    format!("{} minutes ago", mins)
                }
            } else if duration.num_hours() < 24 {
                let hours = duration.num_hours();
                if hours == 1 {
                    "1 hour ago".to_string()
                } else {
                    format!("{} hours ago", hours)
                }
            } else if duration.num_days() < 30 {
                let days = duration.num_days();
                if days == 1 {
                    "1 day ago".to_string()
                } else {
                    format!("{} days ago", days)
                }
            } else if duration.num_days() < 365 {
                let months = duration.num_days() / 30;
                if months == 1 {
                    "1 month ago".to_string()
                } else {
                    format!("{} months ago", months)
                }
            } else {
                let years = duration.num_days() / 365;
                if years == 1 {
                    "1 year ago".to_string()
                } else {
                    format!("{} years ago", years)
                }
            };

            (formatted_time, relative)
        }
        TimeFormatVariant::Format(format) => {
            let custom = local_time.format(format.as_str()).to_string();

            (formatted_time, custom)
        }
    }
}
