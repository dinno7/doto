use chrono::{DateTime, Utc};
use console::style;

const DAY: i64 = 86_400;
const HOUR: i64 = 3_600;
const MINUTE: i64 = 60;

pub fn time_ago(dt: DateTime<Utc>) -> String {
    let now = Utc::now();
    let delta = now - dt; // chrono Duration (can be negative if future)
    let secs = delta.num_seconds().abs();

    // pick units
    let days = secs / DAY;
    let hours = (secs % DAY) / HOUR;
    let mins = (secs % HOUR) / MINUTE;
    let remain_secs = secs % MINUTE;

    // format time
    let mut date_str = String::new();
    if days > 0 {
        date_str.push_str(&format!("{days}{}", style("d").bold().red()));
    }
    if hours > 0 {
        date_str.push_str(&format!("{hours}{}", style("h").bold().yellow()));
    }
    if mins > 0 {
        date_str.push_str(&format!("{mins}{}", style("m").bold().green()));
    } else {
        date_str.push_str(&format!("{remain_secs}{}", style("s").bold().green()));
    }

    // add past or future string
    if delta.num_seconds() >= 0 {
        date_str.push_str(" ago");
    } else {
        date_str = format!("in {date_str}");
    }
    date_str
}
