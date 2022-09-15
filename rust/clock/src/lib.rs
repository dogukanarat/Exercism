use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock
{
    time_in_minutes: i64,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hours = (self.time_in_minutes / 60) % 24;
        let minutes = self.time_in_minutes  % 60;
        write!(f, "{:02}:{:02}", hours, minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut time_in_minutes = 0;

        time_in_minutes += hours as i64 * 60;
        time_in_minutes += minutes as i64;

        if time_in_minutes > 0
        {
            time_in_minutes %= 1440;
        }
        else
        {
            time_in_minutes = 1440 + time_in_minutes % 1440;
        }

        time_in_minutes = time_in_minutes % (24 * 60);

        Clock { time_in_minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut time_in_minutes = self.time_in_minutes;

        time_in_minutes += minutes as i64;

        if time_in_minutes > 0
        {
            time_in_minutes %= 1440;
        }
        else
        {
            time_in_minutes = 1440 + time_in_minutes % 1440;
        }

        time_in_minutes = time_in_minutes % (24 * 60);

        Clock { time_in_minutes }
    }
}
