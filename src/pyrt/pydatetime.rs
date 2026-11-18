//! Minimal Python datetime: date, datetime, timedelta with proleptic Gregorian math.
use std::cmp::Ordering;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::pyrt::exc::PyExc;
use crate::pyrt::value::*;

#[derive(Clone, Debug)]
pub struct PyTimeDelta {
    pub days: i64,
    pub seconds: i64, // 0..86399
}
impl PyTimeDelta {
    pub fn new(days: i64, seconds: i64) -> PyTimeDelta {
        let extra = seconds.div_euclid(86400);
        PyTimeDelta { days: days + extra, seconds: seconds.rem_euclid(86400) }
    }
    pub fn total_seconds(&self) -> f64 { (self.days * 86400 + self.seconds) as f64 }
    pub fn add(&self, o: &PyTimeDelta) -> PyTimeDelta { PyTimeDelta::new(self.days + o.days, self.seconds + o.seconds) }
    pub fn cmp_ord(&self, o: &PyTimeDelta) -> Ordering { (self.days, self.seconds).cmp(&(o.days, o.seconds)) }
    pub fn repr(&self) -> String { format!("datetime.timedelta(days={}, seconds={})", self.days, self.seconds) }
}
impl PartialEq for PyTimeDelta { fn eq(&self, o: &Self) -> bool { self.days == o.days && self.seconds == o.seconds } }

#[derive(Clone, Debug)]
pub struct PyDate {
    pub year: i64,
    pub month: i64,
    pub day: i64,
}

pub fn is_leap(y: i64) -> bool { (y % 4 == 0 && y % 100 != 0) || y % 400 == 0 }
pub fn days_in_month(y: i64, m: i64) -> i64 {
    match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if is_leap(y) { 29 } else { 28 },
        _ => 30,
    }
}

/// Days since 0001-01-01 (ordinal - 1).
pub fn date_to_ord(y: i64, m: i64, d: i64) -> i64 {
    let mut days = 0i64;
    let yy = y - 1;
    days += yy * 365 + yy / 4 - yy / 100 + yy / 400;
    for mm in 1..m { days += days_in_month(y, mm); }
    days + d
}

pub fn ord_to_date(mut ord: i64) -> (i64, i64, i64) {
    // ord counts days since 0001-01-01 with 0001-01-01 == 0
    let mut y = 1i64;
    loop {
        let diy = if is_leap(y) { 366 } else { 365 };
        if ord >= diy { ord -= diy; y += 1; } else { break; }
    }
    let mut m = 1i64;
    loop {
        let dim = days_in_month(y, m);
        if ord >= dim { ord -= dim; m += 1; } else { break; }
    }
    (y, m, ord + 1)
}

impl PyDate {
    pub fn days(&self) -> i64 { date_to_ord(self.year, self.month, self.day) }
    pub fn from_ord(ord: i64) -> PyDate {
        let (year, month, day) = ord_to_date(ord);
        PyDate { year, month, day }
    }
    pub fn add_days(&self, days: i64) -> PyDate { PyDate::from_ord(self.days() + days) }
    /// Monday=0 .. Sunday=6 (Python weekday()): 0001-01-01 was a Monday.
    pub fn weekday(&self) -> i64 { self.days().rem_euclid(7) }
    pub fn isoformat(&self) -> String { format!("{:04}-{:02}-{:02}", self.year, self.month, self.day) }
    pub fn repr(&self) -> String { format!("datetime.date({}, {}, {})", self.year, self.month, self.day) }
    pub fn cmp_ord(&self, o: &PyDate) -> Ordering { self.days().cmp(&o.days()) }
    pub fn strftime(&self, fmt: &str) -> String { strftime_date(self, fmt) }
}
impl PartialEq for PyDate { fn eq(&self, o: &Self) -> bool { self.days() == o.days() } }

#[derive(Clone, Debug)]
pub struct PyDateTime {
    pub date: PyDate,
    pub hour: i64,
    pub minute: i64,
    pub second: i64,
}
impl PyDateTime {
    pub fn add(&self, t: &PyTimeDelta) -> PyDateTime {
        let total = PyTimeDelta::new(t.days, t.seconds + self.hour * 3600 + self.minute * 60 + self.second);
        let date = self.date.add_days(total.days);
        let secs = total.seconds;
        PyDateTime { date, hour: secs / 3600, minute: (secs % 3600) / 60, second: secs % 60 }
    }
    pub fn sub(&self, o: &PyDateTime) -> PyTimeDelta {
        PyTimeDelta::new(
            self.date.days() - o.date.days(),
            (self.hour - o.hour) * 3600 + (self.minute - o.minute) * 60 + (self.second - o.second),
        )
    }
    pub fn ord_key(&self) -> (i64, i64) { (self.date.days(), self.hour * 3600 + self.minute * 60 + self.second) }
    pub fn cmp_ord(&self, o: &PyDateTime) -> Ordering { self.ord_key().cmp(&o.ord_key()) }
    pub fn repr(&self) -> String {
        format!("datetime.datetime({}, {}, {}, {}, {}, {})", self.date.year, self.date.month, self.date.day, self.hour, self.minute, self.second)
    }
}
impl PartialEq for PyDateTime { fn eq(&self, o: &Self) -> bool { self.ord_key() == o.ord_key() } }

const MONTH_NAMES: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
const MONTH_ABBR: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
const DAY_NAMES: [&str; 7] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
const DAY_ABBR: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

pub fn strftime_date(d: &PyDate, fmt: &str) -> String {
    let dt = PyDateTime { date: d.clone(), hour: 0, minute: 0, second: 0 };
    strftime_datetime(&dt, fmt)
}

pub fn strftime_datetime(d: &PyDateTime, fmt: &str) -> String {
    let mut out = String::new();
    let chars: Vec<char> = fmt.chars().collect();
    let mut i = 0;
    let weekday = d.date.weekday();
    while i < chars.len() {
        if chars[i] == '%' && i + 1 < chars.len() {
            let c = chars[i + 1];
            let s = match c {
                'Y' => format!("{}", d.date.year),
                'y' => format!("{:02}", d.date.year % 100),
                'm' => format!("{:02}", d.date.month),
                'd' => format!("{:02}", d.date.day),
                'H' => format!("{:02}", d.hour),
                'I' => format!("{:02}", if d.hour % 12 == 0 { 12 } else { d.hour % 12 }),
                'M' => format!("{:02}", d.minute),
                'S' => format!("{:02}", d.second),
                'p' => if d.hour < 12 { "AM".into() } else { "PM".into() },
                'A' => DAY_NAMES[weekday as usize].into(),
                'a' => DAY_ABBR[weekday as usize].into(),
                'B' => MONTH_NAMES[(d.date.month - 1) as usize].into(),
                'b' => MONTH_ABBR[(d.date.month - 1) as usize].into(),
                'j' => {
                    let mut n = d.date.day;
                    for mm in 1..d.date.month { n += days_in_month(d.date.year, mm); }
                    format!("{:03}", n)
                }
                'w' => ((weekday + 1) % 7).to_string(),
                'u' => (weekday + 1).to_string(),
                'x' => format!("{:02}/{:02}/{:02}", d.date.month, d.date.day, d.date.year % 100),
                'X' => format!("{:02}:{:02}:{:02}", d.hour, d.minute, d.second),
                'c' => format!("{} {} {} {:02}:{:02}:{:02} {}", DAY_ABBR[weekday as usize], MONTH_ABBR[(d.date.month - 1) as usize], d.date.day, d.hour, d.minute, d.second, d.date.year),
                '%' => "%".into(),
                'G' => format!("{}", d.date.year),
                'V' => {
                    // ISO week number (approximate)
                    let mut n = d.date.day;
                    for mm in 1..d.date.month { n += days_in_month(d.date.year, mm); }
                    format!("{:02}", (n - 1) / 7 + 1)
                }
                'e' => format!("{:2}", d.date.day),
                other => { let mut s = String::from("%"); s.push(other); s }
            };
            out.push_str(&s);
            i += 2;
        } else {
            out.push(chars[i]);
            i += 1;
        }
    }
    out
}

fn local_now() -> (i64, i64) {
    // Returns (days since 0001-01-01, seconds within day) in local time.
    // Local offset: read TZ-less system local offset via `date +%z` is not available
    // from std; use a fixed offset from the TZ env or default to +08 (user tz).
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0) as i64;
    let offset = tz_offset_seconds();
    let local = secs + offset;
    let days = local.div_euclid(86400);
    let sod = local.rem_euclid(86400);
    // unix epoch 1970-01-01 ordinal
    let epoch_ord = date_to_ord(1970, 1, 1);
    (epoch_ord + days, sod)
}

fn tz_offset_seconds() -> i64 {
    if let Ok(tz) = std::env::var("PYBOT_TZ_OFFSET") {
        if let Ok(v) = tz.parse::<i64>() { return v; }
    }
    8 * 3600 // Asia/Singapore
}

pub fn date_today() -> Py {
    let (ord, _) = local_now();
    Py::Date(Rc::new(PyDate::from_ord(ord)))
}

pub fn datetime_now() -> Py {
    let (ord, sod) = local_now();
    Py::DateTime(Rc::new(PyDateTime { date: PyDate::from_ord(ord), hour: sod / 3600, minute: (sod % 3600) / 60, second: sod % 60 }))
}

pub fn make_date(y: i64, m: i64, d: i64) -> Py { Py::Date(Rc::new(PyDate { year: y, month: m, day: d })) }
pub fn make_datetime(y: i64, mo: i64, d: i64, h: i64, mi: i64, s: i64) -> Py {
    Py::DateTime(Rc::new(PyDateTime { date: PyDate { year: y, month: mo, day: d }, hour: h, minute: mi, second: s }))
}
pub fn make_timedelta(args: &[Py]) -> Py {
    // (days, seconds, microseconds, milliseconds, minutes, hours, weeks)
    let g = |i: usize| args.get(i).and_then(|p| p.as_f64()).unwrap_or(0.0);
    let days = g(0) + g(6) * 7.0;
    let seconds = g(1) + g(3) / 1000.0 + g(4) * 60.0 + g(5) * 3600.0;
    let total_secs = days * 86400.0 + seconds + g(2) / 1_000_000.0;
    let d = (total_secs / 86400.0).floor() as i64;
    let s = (total_secs - d as f64 * 86400.0).round() as i64;
    Py::TimeDelta(Rc::new(PyTimeDelta::new(d, s)))
}

/// datetime module call dispatcher.
pub fn datetime_call(path: &[&str], args: &[Py]) -> Result<Py, PyExc> {
    match path {
        ["date", "today"] => Ok(date_today()),
        ["datetime", "now"] => Ok(datetime_now()),
        ["date", "fromisoformat"] => {
            let s = args.first().and_then(|p| p.as_str()).unwrap_or("");
            let parts: Vec<&str> = s.split('-').collect();
            if parts.len() == 3 {
                Ok(make_date(parts[0].parse().unwrap_or(1), parts[1].parse().unwrap_or(1), parts[2].parse().unwrap_or(1)))
            } else { Err(val_err("invalid isoformat string")) }
        }
        ["date"] => {
            let g = |i: usize| args.get(i).and_then(|p| p.as_i64()).unwrap_or(1);
            Ok(make_date(g(0), g(1), g(2)))
        }
        ["datetime"] => {
            let g = |i: usize| args.get(i).and_then(|p| p.as_i64()).unwrap_or(0);
            Ok(make_datetime(g(0), g(1).max(1), g(2).max(1), g(3), g(4), g(5)))
        }
        ["timedelta"] => Ok(make_timedelta(args)),
        _ => Err(attr_err(path.last().unwrap_or(&"?"))),
    }
}

/// attribute access on date/datetime/timedelta values
pub fn date_attr(v: &Py, name: &str) -> Option<Py> {
    match v {
        Py::Date(d) => match name {
            "year" => Some(Py::Int(d.year)),
            "month" => Some(Py::Int(d.month)),
            "day" => Some(Py::Int(d.day)),
            _ => None,
        },
        Py::DateTime(d) => match name {
            "year" => Some(Py::Int(d.date.year)),
            "month" => Some(Py::Int(d.date.month)),
            "day" => Some(Py::Int(d.date.day)),
            "hour" => Some(Py::Int(d.hour)),
            "minute" => Some(Py::Int(d.minute)),
            "second" => Some(Py::Int(d.second)),
            _ => None,
        },
        Py::TimeDelta(t) => match name {
            "days" => Some(Py::Int(t.days)),
            "seconds" => Some(Py::Int(t.seconds)),
            _ => None,
        },
        _ => None,
    }
}

pub fn date_method(v: &Py, name: &str, args: &[Py]) -> Option<Py> {
    match (v, name) {
        (Py::Date(d), "weekday") => Some(Py::Int(d.weekday_impl())),
        (Py::Date(d), "isoweekday") => Some(Py::Int(d.weekday_impl() + 1)),
        (Py::Date(d), "isoformat") => Some(py_string(d.isoformat())),
        (Py::Date(d), "strftime") => Some(py_string(strftime_date(d, args.first().and_then(|p| p.as_str()).unwrap_or("")))),
        (Py::Date(d), "replace") => {
            let g = |i: usize, cur: i64| args.get(i).and_then(|p| p.as_i64()).unwrap_or(cur);
            Some(make_date(g(0, d.year), g(1, d.month), g(2, d.day)))
        }
        (Py::Date(d), "timetuple") => Some(py_tuple(vec![
            Py::Int(d.year), Py::Int(d.month), Py::Int(d.day),
            Py::Int(0), Py::Int(0), Py::Int(0), Py::Int(d.weekday_impl()), Py::Int(1), Py::Int(-1)])),
        (Py::DateTime(d), "weekday") => Some(Py::Int(d.date.weekday_impl())),
        (Py::DateTime(d), "isoweekday") => Some(Py::Int(d.date.weekday_impl() + 1)),
        (Py::DateTime(d), "date") => Some(Py::Date(Rc::new(d.date.clone()))),
        (Py::DateTime(d), "isoformat") => Some(py_string(format!("{}T{:02}:{:02}:{:02}", d.date.isoformat(), d.hour, d.minute, d.second))),
        (Py::DateTime(d), "strftime") => Some(py_string(strftime_datetime(d, args.first().and_then(|p| p.as_str()).unwrap_or("")))),
        (Py::DateTime(d), "replace") => {
            let g = |i: usize, cur: i64| args.get(i).and_then(|p| p.as_i64()).unwrap_or(cur);
            Some(make_datetime(g(0, d.date.year), g(1, d.date.month), g(2, d.date.day), g(3, d.hour), g(4, d.minute), g(5, d.second)))
        }
        (Py::DateTime(d), "timetuple") => Some(py_tuple(vec![
            Py::Int(d.date.year), Py::Int(d.date.month), Py::Int(d.date.day),
            Py::Int(d.hour), Py::Int(d.minute), Py::Int(d.second), Py::Int(d.date.weekday_impl()), Py::Int(1), Py::Int(-1)])),
        (Py::TimeDelta(t), "total_seconds") => Some(Py::Float(t.total_seconds())),
        _ => None,
    }
}

impl PyDate {
    fn weekday_impl(&self) -> i64 { self.weekday() }
}
