use std::cmp;
use std::fmt;

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut deducted_hours = 0;
        let mut wrapped_hours;
        let mut wrapped_minutes;
        let mut extra_hours = 0;

        if minutes < 0 {
            wrapped_minutes = minutes % 60;
            wrapped_minutes = 60 - wrapped_minutes.abs();
            deducted_hours = (minutes / 60).abs() + 1;
        } else {
            wrapped_minutes = minutes % 60;
            extra_hours = minutes / 60;
        }

        if hours < 0 {
            wrapped_hours = hours % 24;
            wrapped_hours = 24 - wrapped_hours.abs();
        } else {
            wrapped_hours = hours % 24
        }
        
        wrapped_hours = wrapped_hours + extra_hours - deducted_hours.abs();

        if wrapped_hours < 0 {
            wrapped_hours = 24 - (wrapped_hours % 24).abs();
        }

        wrapped_hours = wrapped_hours % 24;

        Clock {
            hours: wrapped_hours,
            minutes: wrapped_minutes,
        }
    }

    pub fn add_minutes(mut self, minutes: i32) -> Self {
        let mut deduct_hours = 0;
        self.minutes += minutes;

        if self.minutes < 0 {
            self.minutes = self.minutes % 60;
            self.minutes = 60 - self.minutes.abs();
            deduct_hours += 1;
            if minutes.abs() / 60 > 24 {
                deduct_hours -= 1;
            }
            if minutes.abs() / 60 > 0 {
                deduct_hours += minutes.abs() / 60;
            }

            println!("minutes: {}, deduct hours: {}", minutes, deduct_hours);

            self.hours -= deduct_hours;
            self.hours = self.hours % 24;

            if self.hours < 0 {
                self.hours = 24 - self.hours.abs();
            }

            return self;
        }



        let roll_hours = self.minutes / 60;
        let extra_minutes = self.minutes % 60;
        self.hours += roll_hours;
        self.hours = self.hours % 24;
        self.minutes = extra_minutes;

        self
    }

    pub fn to_string(self) -> String {
        let mut string = String::new();
        let mut hour_string = self.hours.to_string();
        let mut minutes_string = self.minutes.to_string();
        
        if self.hours < 10 {
            hour_string.insert_str(0, "0")
        }
        string.push_str(&hour_string);
        string.push_str(":");

        if self.minutes < 10 {
            minutes_string.insert_str(0, "0")
        }
        string.push_str(&minutes_string);

        string
    }
}


impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.hours, self.minutes)
    }
}

impl cmp::PartialEq for Clock {
    fn eq(&self, clock: &Clock) -> bool {
        if self.minutes == clock.minutes && self.hours == clock.hours {
            return true
        } else {
            return false
        }
    }
}