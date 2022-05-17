use std::fmt::*;

#[derive(PartialEq, Eq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    fn correct_clock(&self) -> Self {
        let mut hours = self.hours;
        let mut minutes = self.minutes;
        hours = hours + self.minutes / 60;
        minutes = minutes % 60;
        if minutes < 0 {
            hours = hours - 1;
        }
        minutes = (minutes + 60) % 60;

        hours = (hours % 24 + 24) % 24;
        Clock { hours, minutes }
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        Self { hours, minutes }.correct_clock()
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            hours: self.hours,
            minutes: self.minutes + minutes,
        }
        .correct_clock()
    }
}
