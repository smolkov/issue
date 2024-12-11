use chrono::TimeDelta;






const MINUT_IN_SECOND: i64 = 60;
const HOUR_IN_SECOND: i64 = MINUT_IN_SECOND * 60;
const DAY_IN_SECOND: i64 = HOUR_IN_SECOND * 24;
const MONTH_IN_SECOND: i64 = DAY_IN_SECOND * 30;
const YEAR_IN_SECOND: i64 = DAY_IN_SECOND * 364;

pub fn print_age(timedelta: TimeDelta) -> String {
    match timedelta.num_seconds() {
        sec if sec > YEAR_IN_SECOND => format!("{}y", sec / YEAR_IN_SECOND),
        sec if sec > MONTH_IN_SECOND => format!("{}mo", sec / MONTH_IN_SECOND),
        sec if sec > DAY_IN_SECOND => format!("{}d", sec / DAY_IN_SECOND),
        sec if sec > HOUR_IN_SECOND => format!("{}h", sec / HOUR_IN_SECOND),
        sec if sec > MINUT_IN_SECOND => format!("{}m", sec / MINUT_IN_SECOND),
        sec => format!("{}s", sec),
    }
}
