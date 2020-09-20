pub type Year = i16;
pub type Month = i8;
pub type Day = i8;
pub type Hour = i8;
pub type Minute = i8;
pub type Second = i8;
pub type Millisec = i16;
pub type Microsec = i16;

use serde::Deserialize;

#[derive(Deserialize, PartialEq, Debug)]
pub struct Timestamp(pub i64);

impl Timestamp {
    #[cfg(feature = "chrono")]
    pub fn to_naive_date_time(&self) -> chrono::NaiveDateTime {
        chrono::NaiveDateTime::from_timestamp(self.0, 0)
    }
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct YearMonth(pub Year, pub Month);

impl YearMonth {
    #[cfg(feature = "chrono")]
    pub fn to_naive_date(&self) -> chrono::NaiveDate {
        chrono::NaiveDate::from_ymd(self.0 as i32, self.1 as u32, 1)
    }
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Date(pub Year, pub Month, pub Day);

impl Date {
    #[cfg(feature = "chrono")]
    pub fn to_naive_date(&self) -> chrono::NaiveDate {
        chrono::NaiveDate::from_ymd(self.0 as i32, self.1 as u32, self.2 as u32)
    }
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct DateTime(
    pub Year,
    pub Month,
    pub Day,
    pub Hour,
    pub Minute,
    pub Second,
    pub Millisec,
    pub Microsec,
);

impl DateTime {
    #[cfg(feature = "chrono")]
    pub fn to_naive_date_time(&self) -> chrono::NaiveDateTime {
        let d = chrono::NaiveDate::from_ymd(self.0 as i32, self.1 as u32, self.2 as u32);
        let t = chrono::NaiveTime::from_hms_milli(
            self.3 as u32,
            self.4 as u32,
            self.5 as u32,
            self.6 as u32,
        );
        chrono::NaiveDateTime::new(d, t)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "chrono")]
    use super::*;

    use std::io;

    #[cfg(feature = "chrono")]
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

    #[test]
    fn chrono_for_timestamp() -> io::Result<()> {
        #[cfg(feature = "chrono")]
        assert_eq!(
            Timestamp(1577836800).to_naive_date_time(),
            NaiveDateTime::new(
                NaiveDate::from_ymd(2020, 1, 1),
                NaiveTime::from_hms(0, 0, 0)
            )
        );

        Ok(())
    }

    #[test]
    fn chrono_for_year_month() -> io::Result<()> {
        #[cfg(feature = "chrono")]
        assert_eq!(
            YearMonth(2020, 1).to_naive_date(),
            NaiveDate::from_ymd(2020, 1, 1)
        );

        Ok(())
    }

    #[test]
    fn chrono_for_date() -> io::Result<()> {
        #[cfg(feature = "chrono")]
        assert_eq!(
            Date(2020, 1, 2).to_naive_date(),
            NaiveDate::from_ymd(2020, 1, 2)
        );

        Ok(())
    }

    #[test]
    fn chrono_for_datetime() -> io::Result<()> {
        #[cfg(feature = "chrono")]
        assert_eq!(
            DateTime(2020, 1, 2, 3, 4, 5, 6, 7).to_naive_date_time(),
            NaiveDateTime::new(
                NaiveDate::from_ymd(2020, 1, 2),
                NaiveTime::from_hms_milli(3, 4, 5, 6)
            )
        );

        Ok(())
    }
}
