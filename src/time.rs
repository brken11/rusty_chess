pub mod time_format {
    use std::time::{SystemTime,UNIX_EPOCH,Instant,SystemTimeError};
    use std::fmt;

    pub type Millis = u16;
    pub type Second = u8;
    pub type Minute = u8;
    pub type Hour = u8;
    pub type Day = u8;
    #[repr(u8)]
    #[derive(Clone, Copy,Debug)]
    pub enum Month{
        January = 1,
        February = 2,
        March = 3,
        April = 4,
        May = 5,
        June = 6,
        July = 7,
        August = 8,
        September = 9,
        October = 10,
        November = 11,
        December = 12,
    }
    impl fmt::Display for Month {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", match &self {
                Month::January => "January",
                Month::February => "February",
                Month::March => "March",
                Month::April => "April",
                Month::May => "May",
                Month::June => "June",
                Month::July => "July",
                Month::August => "August",
                Month::September => "September",
                Month::October => "October",
                Month::November => "November",
                Month::December => "December",
            })
        }
    }
    pub type Year = u32;
    pub fn is_leap_year(year : Year) -> bool {
        (year % 4 == 0) && ((year % 100 != 0) || (year % 400 == 0))
    }

    pub struct DateMillis{
        start: Instant,
        display_mode: DisplayMode,
        milliseconds: Millis,
        seconds: Second,
        minutes: Minute,
        hours: Hour,
        day: Day,
        month: Month,
        year: Year,
    }

    #[derive(Clone,Copy)]
    pub enum DisplayMode {
        ISO8601,
        Simple,
        Debug,
    }

    impl DateMillis{

        const MAX_MILLIS: u128 = 1000;

        const SECONDS: u128 = 1000;
        const MAX_SECONDS: u128 = 60;
        const MINUTES: u128 = DateMillis::MAX_SECONDS * DateMillis::SECONDS;
        const MAX_MINUTES: u128 = 60;
        const HOURS: u128 = DateMillis::MAX_MINUTES * DateMillis::MINUTES;
        const MAX_HOURS: u128 = 24;
        const DAY: u128 = DateMillis::MAX_HOURS * DateMillis::HOURS;

        const DAYS_IN_REGULAR_YEAR: u128 = 365;
        const REGULAR_YEAR: u128 = DateMillis::DAYS_IN_REGULAR_YEAR * DateMillis::DAY;
        const DAYS_IN_LEAP_YEAR: u128 = 366;
        const LEAP_YEAR: u128 = DateMillis::DAYS_IN_LEAP_YEAR * DateMillis::DAY;
        const QUAD_YEAR: u128 = 3 * DateMillis::REGULAR_YEAR + DateMillis::LEAP_YEAR;

        const CENTURY: u128 = 76 * DateMillis::REGULAR_YEAR + 24 * DateMillis::LEAP_YEAR;
        const QUAD_CENTURY: u128 = 4 * DateMillis::CENTURY + DateMillis::DAY;

        /// Year 2000 is 23 regular years plus 7 years after UNIX_EPOCH
        const Y2K: u128 = 23 * DateMillis::REGULAR_YEAR + 7 * DateMillis::LEAP_YEAR;

        fn days_in_month(month: Month, is_leap_year: bool)-> Day{
            match month {
                Month::January => 31,
                Month::February => if is_leap_year {29} else {28},
                Month::March => 31,
                Month::April => 30,
                Month::May => 31,
                Month::June => 30,
                Month::July => 31,
                Month::August => 31,
                Month::September => 30,
                Month::October => 31,
                Month::November => 30,
                Month::December => 31,
            }
        }
        fn day_to_day_before_month_pair(day: u16, is_leap_year: bool) -> (Month, u16) {
            if is_leap_year { match day {
                0..31 => (Month::January, 0),
                31..60 => (Month::February, 31),
                60..91 => (Month::March, 60),
                91..121 => (Month::April, 91),
                121..152 => (Month::May, 121),
                152..182 => (Month::June, 152),
                182..213 => (Month::July, 182),
                213..244 => (Month::August, 213),
                244..274 => (Month::September, 244),
                274..305 => (Month::October, 274),
                305..335 => (Month::November, 305),
                335..366 => (Month::December, 335),
                _ => (Month::December, 335)
            } } else { match day {
                0..31 => (Month::January, 0),
                31..59 => (Month::February, 31),
                59..90 => (Month::March, 59),
                90..120 => (Month::April, 90),
                120..151 => (Month::May, 120),
                151..181 => (Month::June, 151),
                181..212 => (Month::July, 181),
                212..243 => (Month::August, 212),
                243..273 => (Month::September, 243),
                273..304 => (Month::October, 273),
                304..334 => (Month::November, 304),
                334..365 => (Month::December, 334),
                _ => (Month::December, 334)
            } }
        }

        pub fn new() -> Result<DateMillis, SystemTimeError> {        
            //println!("DateMillis::new()");
            let now = Instant::now();

            let mut time_millis: u128 = match SystemTime::now().duration_since(UNIX_EPOCH) {
                Err(sys_time_err) => return Err(sys_time_err),
                Ok(millis) => millis.as_millis(),
            };
            //println!("DateMillis\n\ttime_millis={}", time_millis);
            
            let mut date = DateMillis{
                start: now,
                display_mode: DisplayMode::ISO8601,
                milliseconds : 0,
                seconds : 0,
                minutes : 0,
                hours : 0,
                day : 1,
                month : Month::January,
                year : 1970
            };
            //println!("DateMillis:{{\n\t{}\n}}", date);

            if time_millis >= DateMillis::Y2K {
                date.year = 2000;
                time_millis -= DateMillis::Y2K;
            } else {
                date.year = 1970;
            }
            //println!("DateMillis\n\ttime_millis={}", time_millis);
            //println!("DateMillis:{{\n\t{}\n}}", date);

            //println!("DateMillis::new QUAD_CENTURY");
            if time_millis >= DateMillis::QUAD_CENTURY {
                let quad_century_count: Year = (time_millis / DateMillis::QUAD_CENTURY) as Year;
                date.year += quad_century_count * 400;
                time_millis -= (quad_century_count as u128 * DateMillis::QUAD_CENTURY) as u128;
            }
            //println!("DateMillis\n\ttime_millis={}", time_millis);
            //println!("DateMillis:{{\n\t{}\n}}", date);

            //println!("DateMillis::new CENTURY");
            if time_millis >= DateMillis::CENTURY {
                let century_count: Year = (time_millis / DateMillis::CENTURY) as Year;
                date.year += century_count * 100;
                time_millis -= (century_count as u128 * DateMillis::CENTURY) as u128;
            }
            //println!("DateMillis\n\ttime_millis={}", time_millis);
            //println!("DateMillis:{{\n\t{}\n}}", date);

            //println!("DateMillis::new QUAD_YEAR");
            if time_millis >= DateMillis::QUAD_YEAR{
                let quad_year_count: Year = (time_millis / DateMillis::QUAD_YEAR) as Year;
                date.year += quad_year_count * 4;
                time_millis -= (quad_year_count as u128 * DateMillis::QUAD_YEAR) as u128;
            }
            //println!("DateMillis\n\ttime_millis={}", time_millis);
            //println!("DateMillis:{{\n\t{}\n}}", date);

            //println!("DateMillis::new Year Calc");
            let is_leap_year: bool;
            if date.year < 2000 {
                if time_millis >= 2 * DateMillis::REGULAR_YEAR + DateMillis::LEAP_YEAR {
                    is_leap_year = false;
                    date.year += 3;
                } else if time_millis >= 2 * DateMillis::REGULAR_YEAR {
                    is_leap_year = true;
                    date.year += 2;
                } else {
                    is_leap_year = false;
                    let year_count: Year = (time_millis / (2*DateMillis::REGULAR_YEAR)) as Year;
                    date.year += year_count;
                    time_millis -= year_count as u128 * DateMillis::REGULAR_YEAR;
                }
            } else {
                if time_millis >= 3 * DateMillis::REGULAR_YEAR {
                    is_leap_year = true;
                    date.year += 3;
                    time_millis -= 3 * DateMillis::REGULAR_YEAR;
                } else {
                    is_leap_year = false;
                    let year_count: Year = (time_millis / DateMillis::REGULAR_YEAR) as Year;
                    date.year += year_count;
                    time_millis -= year_count as u128 * DateMillis::REGULAR_YEAR;
                }
            }
            //println!("DateMillis\n\ttime_millis={}", time_millis);
            //println!("DateMillis:{{\n\t{}\n}}", date);

            //println!("DateMillis::new day_count calc");
            let day_count: u32 = (time_millis / DateMillis::DAY) as u32;
            time_millis -= day_count as u128 * DateMillis::DAY;
            //println!("DateMillis::new\n\tday_count={}", day_count);
            //println!("DateMillis\n\ttime_millis={}", time_millis);
            //println!("DateMillis:{{\n\t{}\n}}", date);

            //println!("DateMillis::new DAY calc");
            let month: Month;
            let days_before_month: u16;
            (month, days_before_month) = DateMillis::day_to_day_before_month_pair(day_count as u16, is_leap_year);
            date.month = month;
            //println!("DateMillis::new\n\tmonth={}\n\tdays_before_month={}", month, days_before_month);
            date.day = (day_count as u16 - days_before_month) as u8;
            //println!("DateMillis\n\ttime_millis={}", time_millis);
            //println!("DateMillis:{{\n\t{}\n}}", date);
            
            //println!("DateMillis::new time_millis post month adjustment");
            //println!("DateMillis\n\ttime_millis={}", time_millis);
            //println!("DateMillis:{{\n\t{}\n}}", date);

            //println!("DateMillis::new HOURS calc");
            if time_millis >= DateMillis::HOURS {
                let hour_count: u8 = (time_millis / DateMillis::HOURS) as u8;
                date.hours = hour_count;
                time_millis -= hour_count as u128 * DateMillis::HOURS;
            }
            //println!("DateMillis\n\ttime_millis={}", time_millis);
            //println!("DateMillis:{{\n\t{}\n}}", date);

            //println!("DateMillis::new MINUTES calc");
            if time_millis >= DateMillis::MINUTES {
                let minute_count: u8 = (time_millis / DateMillis::MINUTES) as u8;
                date.minutes = minute_count;
                time_millis -= minute_count as u128 * DateMillis::MINUTES;
            }
            //println!("DateMillis\n\ttime_millis={}", time_millis);
            //println!("DateMillis:{{\n\t{}\n}}", date);
            
            //println!("DateMillis::new SECONDS calc");
            if time_millis >= DateMillis::SECONDS {
                let second_count: u8 = (time_millis / DateMillis::SECONDS) as u8;
                date.seconds = second_count;
                time_millis -= second_count as u128 * DateMillis::SECONDS;
            }
            //println!("DateMillis\n\ttime_millis={}", time_millis);
            //println!("DateMillis:{{\n\t{}\n}}", date);

            date.milliseconds = time_millis as u16;

            Ok(date)
        }
        pub fn update(&mut self) {
            let mut elapsed = self.start.elapsed().as_millis();
            if elapsed == 0 {return}
            self.start = Instant::now();
            
            // Calc millis
            let millis_elapsed = elapsed % DateMillis::MAX_MILLIS;
            elapsed -=  millis_elapsed;
            self.milliseconds += millis_elapsed as u16;

            // Calc seconds
            let mut seconds_elapsed: u128 = elapsed / DateMillis::SECONDS;
            elapsed -= seconds_elapsed * DateMillis::SECONDS;
            if self.milliseconds as u128 >= DateMillis::MAX_MILLIS {
                let carried_millis = self.milliseconds as u128 / DateMillis::MAX_MILLIS;
                self.milliseconds -= (carried_millis * DateMillis::MAX_MILLIS) as u16;
                seconds_elapsed += carried_millis;
            }
            self.seconds += seconds_elapsed as u8;

            // Calc minutes
            let mut minutes_elapsed: u128 = elapsed / DateMillis::MINUTES;
            elapsed -= minutes_elapsed * DateMillis::MINUTES;
            if self.seconds as u128 >= DateMillis::MAX_SECONDS {
                let carried_seconds: u128 = self.seconds as u128 / DateMillis::MAX_SECONDS;
                self.seconds %= DateMillis::MAX_SECONDS as u8;
                minutes_elapsed += carried_seconds;
            }
            self.minutes += minutes_elapsed as u8;

            // Calc hours
            let mut hours_elapsed: u128 = elapsed / DateMillis::HOURS;
            elapsed -= hours_elapsed * DateMillis::HOURS;
            if self.minutes as u128 >= DateMillis::MAX_MINUTES {
                let carried_minutes: u128 = self.minutes as u128 / DateMillis::MAX_MINUTES;
                self.minutes %= DateMillis::MAX_MINUTES as u8;
                hours_elapsed += carried_minutes;
            }
            self.hours += hours_elapsed as u8;
            
            // Extracts days remaining
            let mut days_elapsed = elapsed / DateMillis::DAY;
            let mut leap_year = is_leap_year(self.year);

            // Consule days remaining until end of year or until days run out
            loop {
                let days_in_month: u128 = DateMillis::days_in_month(self.month, leap_year) as u128;
                
                // Early finish
                if self.day as u128 + days_elapsed < days_in_month {
                    self.day += days_elapsed as Day;
                    return;
                }

                days_elapsed -= days_in_month - self.day as u128 + 1;
                self.day = 1;
                self.month = match self.month {
                    Month::January => Month::February,
                    Month::February => Month::March,
                    Month::March => Month::April,
                    Month::April => Month::May,
                    Month::May => Month::June,
                    Month::June => Month::July,
                    Month::July => Month::August,
                    Month::August => Month::September,
                    Month::September => Month::October,
                    Month::October => Month::November,
                    Month::November => Month::December,

                    // Exit on year tick
                    Month::December => {
                        self.year += 1;
                        self.month = Month::January;
                        break;
                    }
                }
            }

            // Process years, convert days back to millis and jump forward
            elapsed = days_elapsed * DateMillis::DAY;

            if elapsed > DateMillis::QUAD_CENTURY {
                let quad_century_count: Year = (elapsed / DateMillis::QUAD_CENTURY) as Year;
                self.year += quad_century_count * 400;
                elapsed -= (quad_century_count as u128 * DateMillis::QUAD_CENTURY) as u128;
            }
            if elapsed > DateMillis::CENTURY {
                let century_count: Year = (elapsed / DateMillis::CENTURY) as Year;
                self.year += century_count * 100;
                elapsed -= (century_count as u128 * DateMillis::CENTURY) as u128;
            }
            if elapsed > DateMillis::QUAD_YEAR{
                let quad_year_count: Year = (elapsed / DateMillis::QUAD_YEAR) as Year;
                self.year += quad_year_count * 4;
                elapsed -= (quad_year_count as u128 * DateMillis::QUAD_YEAR) as u128;
            }

            // Finish years
            days_elapsed = elapsed / DateMillis::DAY;
            leap_year = is_leap_year(self.year);
            loop {
                let days_in_year = match leap_year{
                    true => DateMillis::DAYS_IN_LEAP_YEAR,
                    false => DateMillis::DAYS_IN_REGULAR_YEAR,
                };

                // Less than a year remaining
                if days_elapsed < days_in_year { break; }

                self.year += 1;
                days_elapsed -= days_in_year;
                leap_year = is_leap_year(self.year);
            }
            
            // Traverse final year and set correct month and day
            loop {
                let days_in_month: u128 = DateMillis::days_in_month(self.month, leap_year) as u128;
                
                // Loop exit
                if self.day as u128 + days_elapsed < days_in_month {
                    self.day += days_elapsed as Day;
                    return;
                }

                days_elapsed -= days_in_month - self.day as u128 + 1;
                self.day = 1;
                self.month = match self.month {
                    Month::January => Month::February,
                    Month::February => Month::March,
                    Month::March => Month::April,
                    Month::April => Month::May,
                    Month::May => Month::June,
                    Month::June => Month::July,
                    Month::July => Month::August,
                    Month::August => Month::September,
                    Month::September => Month::October,
                    Month::October => Month::November,
                    Month::November => Month::December,
                    Month::December => { // Should be unreachable
                        self.year += 1;
                        leap_year = is_leap_year(self.year);

                        Month::January
                    }
                }
            }
        }
        pub fn set_display_mode(&mut self, display_mode: DisplayMode) {
            self.display_mode = display_mode;
        }

    }
    impl fmt::Display for DateMillis {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self.display_mode {
                DisplayMode::ISO8601 => {
                    write!(f,
                        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",

                        self.year, self.month as u8, self.day,

                        self.hours, self.minutes, self.seconds,
                        self.milliseconds
                    )
                }
                DisplayMode::Simple => {
                    write!(f,
                        "{:02}-{:02} {:02}:{:02}:{:02}",
                        self.month as u8, self.day,

                        self.hours, self.minutes, self.seconds,
                    )
                }
                DisplayMode::Debug => {
                    write!(f,
                        "{}/{}/{} {}:{}:{}.{}",
                        self.year, self.month, self.day,

                        self.hours, self.minutes, self.seconds,
                        self.milliseconds

                    )
                }

            }
        }
    }
}

