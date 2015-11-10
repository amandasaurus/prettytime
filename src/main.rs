extern crate num;

use std::env;
use std::fmt;
use num::integer::div_rem;

struct Duration<T> {
    weeks: T,
    days: T,
    hours: T,
    minutes: T,
    seconds: T,
}

impl fmt::Display for Duration<u64> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.weeks != 0 {
            try!(write!(f, "{}wk", self.weeks));
        }
        if self.days != 0 {
            try!(write!(f, "{}dy", self.days));
        }
        if self.hours != 0 {
            try!(write!(f, "{}h", self.hours));
        }
        if self.minutes != 0 {
            try!(write!(f, "{}m", self.minutes));
        }
        if self.seconds != 0 {
            try!(write!(f, "{}s", self.seconds));
        }
        Ok(())
    }
}
    

fn sec2duration(seconds: u64) -> Duration<u64> {
    let (minutes, seconds) = div_rem(seconds, 60);
    let (hours, minutes) = div_rem(minutes, 60);
    let (days, hours) = div_rem(hours, 24);
    let (weeks, days) = div_rem(days, 7);

    Duration{ weeks: weeks, days: days, hours: hours, minutes: minutes, seconds: seconds }
}


fn prettytime(input: Vec<String>) -> Result<String, String> {
    if input.len() < 1 { return Err("No input".to_string()); }

    let ref args1: String = input[0];
    let seconds: u64 = try!(args1.parse().or(Err("Not an integer".to_string())));

    let duration = sec2duration(seconds);

    Ok(format!("{}", duration))
}
    
fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    match prettytime(args) {
        Ok(s) => { println!("{}", s); },
        Err(s) => { println!("{}", s); }
    }
}
