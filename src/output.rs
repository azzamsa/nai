use std::io::{self, Write};

use tera::Tera;

pub struct Printer {}

impl Default for Printer {
    fn default() -> Self {
        Self::new()
    }
}

impl Printer {
    pub fn new() -> Self {
        Self {}
    }
    pub fn print(
        &self,
        start_date: &str,
        duration: &str,
        format: &str,
    ) -> Result<(), crate::Error> {
        let mut tera = Tera::default();
        let json = serde_json::json!({"start_date": start_date, "duration": duration});
        let context = tera::Context::from_serialize(json)?;
        let content = tera.render_str(format, &context)?;

        println!("{content}");
        Ok(())
    }
}

pub fn stderr(input: &str) {
    writeln!(io::stderr(), "{}", input).ok();
}
