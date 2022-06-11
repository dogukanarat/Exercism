use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock
{
    time_in_seconds: i64,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hours = (self.time_in_seconds / 3600) % 24;
        let minutes = ((self.time_in_seconds % 3600) / 60) % 60;
        write!(f, "{:02}:{:02}", hours, minutes)
    }
}

impl Clock {
    pub fn new(mut hours: i32, mut minutes: i32) -> Self {
        let mut time_in_seconds = 0;

        minutes = if minutes < 0 { hours = hours - 1; minutes % 60 + 60 } else { minutes };
        hours = if hours < 0 { hours % 24 + 24 } else { hours };


        time_in_seconds += hours as i64 * 60 * 60;
        time_in_seconds += minutes as i64 * 60;
        time_in_seconds = time_in_seconds % (24 * 60 * 60);
        Clock { time_in_seconds }
    }

    pub fn add_minutes(&self, mut minutes: i32) -> Self {
        let mut time_in_seconds = self.time_in_seconds;
        let mut hours = (time_in_seconds / 3600) % 24;

        minutes = if minutes < 0 { hours = hours - 1; minutes % 60 + 60 } else { minutes };

        time_in_seconds += minutes as i64 * 60;
        time_in_seconds = time_in_seconds % (24 * 60 * 60);
        Clock { time_in_seconds }
    }
}
