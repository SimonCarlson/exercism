extern crate chrono;
use chrono::{DateTime, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let duration = chrono::Duration::seconds(1000000000);
    start.checked_add_signed(duration).unwrap()
    
}
