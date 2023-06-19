pub struct Time {
    date: time::Date,
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
        let duration: time::Duration = today()? - self.date;
        Self::format_duration(duration)
    }
    /// Parse string time into `Date` struct
    fn parse(date: &str) -> Result<time::Date, crate::Error> {
        let format = time::format_description::parse("[year]-[month repr:short]-[day]")?;
        Ok(time::Date::parse(date, &format)?)
    }
    /// Turn duration into human readable format
    fn format_duration(
        duration: time::Duration,
    ) -> Result<humantime::FormattedDuration, crate::Error> {
        let duration = humantime::parse_duration(&duration.to_string())?;
        Ok(humantime::format_duration(duration))
    }
    /// Formats the given date into a custom string format.
    fn format_date(date: time::Date) -> Result<String, crate::Error> {
        let format = time::format_description::parse(
            "[weekday repr:short], [day] [month repr:short] [year]",
        )?;
        Ok(date.format(&format)?)
    }
}

/// Today's date
fn today() -> Result<time::Date, crate::Error> {
    Ok(time::OffsetDateTime::now_local()?.date())
}
