//
//    Copyright (C) 2015  Rory McCann
//
//    This program is free software: you can redistribute it and/or modify
//    it under the terms of the GNU Affero General Public License as published by
//    the Free Software Foundation, either version 3 of the License, or
//    (at your option) any later version.
//
//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU Affero General Public License for more details.
//
//    You should have received a copy of the GNU Affero General Public License
//    along with this program.  If not, see <http://www.gnu.org/licenses/>.

extern crate num;
extern crate regex;

use std::env;
use std::fmt;
use num::integer::{div_rem, Integer};
use regex::Regex;

struct Duration<T> {
    weeks: T,
    days: T,
    hours: T,
    minutes: T,
    seconds: T,
}

impl<T:Integer> Duration<T> {
    fn new() -> Duration<T> {
        Duration{ weeks: T::zero(), days: T::zero(), hours: T::zero(), minutes: T::zero(), seconds: T::zero() }
    }

    fn set_weeks<'a>(&'a mut self, weeks: T) -> &'a mut Duration<T> {
        self.weeks = weeks;
        self
    }
    fn set_days<'a>(&'a mut self, days: T) -> &'a mut Duration<T> {
        self.days = days;
        self
    }
    fn set_hours<'a>(&'a mut self, hours: T) -> &'a mut Duration<T> {
        self.hours = hours;
        self
    }
    fn set_minutes<'a>(&'a mut self, minutes: T) -> &'a mut Duration<T> {
        self.minutes = minutes;
        self
    }
    fn set_seconds<'a>(&'a mut self, seconds: T) -> &'a mut Duration<T> {
        self.seconds = seconds;
        self
    }

}

impl fmt::Display for Duration<u64> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut writing = false;
        if self.weeks != 0 {
            try!(write!(f, "{}wk", self.weeks));
            writing = true;
        }
        if self.days != 0 || writing {
            try!(write!(f, "{}dy", self.days));
            writing = true;
        }
        if self.hours != 0 || writing {
            try!(write!(f, "{}hr", self.hours));
            writing = true;
        }
        if self.minutes != 0 || writing {
            if writing {
                try!(write!(f, "{:02}m", self.minutes));
            } else {
                try!(write!(f, "{}m", self.minutes));
            }
            writing = true;
        }
        if self.seconds != 0 || writing {
            if writing {
                try!(write!(f, "{:02}s", self.seconds));
            } else {
                try!(write!(f, "{}s", self.seconds));
            }
            //writing = true;
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


// Converts a textual input to 
fn input2sec(input:String) -> Result<u64, String> {
    // TODO replace with regex! macro so the is guaranteed to be OK

    if let Some(cap) = Regex::new(r"^\s*(?P<sec>[0-9]+)\s*(s|sec|seconds?)?\s*$").unwrap().captures(&input) {
        let sec: u64 = try!(try!(cap.name("sec").ok_or("0")).parse().or(Err("Invalid seconds")));
        Ok(sec)
    } else if let Some(cap) = Regex::new(r"^\s*(?P<min>[0-9]+)\s*(m|min|minutes?)\s*((?P<sec>[0-9]+)\s*(s|sec|seconds?)?)?\s*$").unwrap().captures(&input) {
        let sec: u64 = try!(cap.name("sec").unwrap_or(&"0").parse::<u64>().or(Err("Invalid seconds")));
        let min: u64 = try!(cap.name("min").unwrap_or(&"0").parse::<u64>().or(Err("Invalid seconds")));
        Ok(min*60 + sec)
    } else {
        Err("Invalid input".to_string())
    }
    
}


fn prettytime(input: Vec<String>) -> Result<String, String> {
    if input.len() < 1 { return Err("No input".to_string()); }

    let input = input.join(" ");
    let seconds: u64 = try!(input2sec(input));

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

#[test]
fn test_print_duration() {
    assert_eq!("", format!("{}", Duration::new()));
    assert_eq!("1s", format!("{}", Duration::new().set_seconds(1)));
    assert_eq!("1m00s", format!("{}", Duration::new().set_minutes(1)));
    assert_eq!("1m03s", format!("{}", Duration::new().set_minutes(1).set_seconds(3)));
    assert_eq!("1hr00m00s", format!("{}", Duration::new().set_hours(1)));
    assert_eq!("1dy0hr00m00s", format!("{}", Duration::new().set_days(1)));
    assert_eq!("1wk0dy0hr00m00s", format!("{}", Duration::new().set_weeks(1)));
}

#[test]
fn test_parse_input() {
    // simple seconds
    assert_eq!(input2sec("10".to_string()), Ok(10));
    assert_eq!(input2sec("10s".to_string()), Ok(10));
    assert_eq!(input2sec("10 s".to_string()), Ok(10));
    assert_eq!(input2sec("10 sec".to_string()), Ok(10));
    assert_eq!(input2sec("10 foos".to_string()), Err("Invalid input".to_string()));

    // minutes
    assert_eq!(input2sec("2m".to_string()), Ok(120));
    assert_eq!(input2sec("2m 20".to_string()), Ok(140));
    assert_eq!(input2sec("2m 20s".to_string()), Ok(140));
    assert_eq!(input2sec("2m20s".to_string()), Ok(140));
    assert_eq!(input2sec("   2 m   20  s   ".to_string()), Ok(140));
}
