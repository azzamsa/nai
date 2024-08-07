use jiff::civil;

pub struct Time {
    date: civil::Date,
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
        let today = jiff::Zoned::now().date();
        let duration = today.since(self.date)?;
        Self::format_duration(duration)
    }

    /// Parse string time into `plain Date`
    fn parse(date: &str) -> Result<civil::Date, crate::Error> {
        let date: civil::Date = date.parse()?;
        Ok(date)
    }

    /// Turn duration into human readable format
    fn format_duration(duration: jiff::Span) -> Result<humantime::FormattedDuration, crate::Error> {
        let duration = humantime::parse_duration(&format!("{}hours", duration.get_hours()))?;
        let duration = humantime::format_duration(duration);
        Ok(duration)
    }

    /// Formats the given date into a custom string format.
    fn format_date(date: civil::Date) -> Result<String, crate::Error> {
        Ok(date.strftime("%a, %d %b %Y").to_string())
    }
}
