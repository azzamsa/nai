use chrono::{Local, NaiveDate};

pub struct Time {
    date: NaiveDate,
}

impl Time {
    pub fn new(date: &str) -> Result<Self, crate::Error> {
        let date = Self::parse(date)?;
        Ok(Self { date })
    }

    /// Get the date
    pub fn date(&self) -> Result<String, crate::Error> {
        Self::format_date(self.date)
    }

    /// Calculate elapsed time
    pub fn duration(&self) -> Result<humantime::FormattedDuration, crate::Error> {
        let today = today()?;
        let duration = today.signed_duration_since(self.date);
        Self::format_duration(duration)
    }

    /// Parse string time into `NaiveDate` struct
    fn parse(date: &str) -> Result<NaiveDate, crate::Error> {
        let format = "%Y-%b-%d";
        Ok(NaiveDate::parse_from_str(date, format)?)
    }

    /// Turn duration into human readable format
    fn format_duration(
        duration: chrono::Duration,
    ) -> Result<humantime::FormattedDuration, crate::Error> {
        Ok(humantime::format_duration(duration.to_std()?))
    }

    /// Formats the given date into a custom string format.
    fn format_date(date: NaiveDate) -> Result<String, crate::Error> {
        let format = "%a, %d %b %Y";
        Ok(date.format(format).to_string())
    }
}

/// Today's date
fn today() -> Result<NaiveDate, crate::Error> {
    Ok(Local::now().date_naive())
}
