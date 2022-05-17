use time::Duration;
use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.

const GIGA_SECOND: i64 = 1_000_000_000;
pub fn after(start: DateTime) -> DateTime {
    let duration = Duration::new(GIGA_SECOND, 0);
    start + duration
}
