use pest::Parser;
use pest_derive::Parser;

use crate::{config::Moment, time::Time};

#[derive(Parser)]
#[grammar = "template.pest"]
pub struct TemplateParser;

/// Parse `format` string
pub fn parse(moment: &Moment) -> Result<String, crate::Error> {
    let mut output = String::new();
    let time = Time::new(&moment.start_date)?;

    let pair = TemplateParser::parse(Rule::spec, &moment.format)
        .unwrap()
        .next()
        .unwrap();
    tracing::debug!("{:#?}", &pair);

    for piece in pair.into_inner() {
        match piece.as_rule() {
            Rule::start_date => {
                output.push_str(&time.date()?.to_string());
            }
            Rule::duration => {
                output.push_str(&time.duration()?.to_string());
            }
            Rule::word | Rule::WHITESPACE => {
                output.push_str(piece.as_str());
            }
            _ => unreachable!(),
        }
    }
    tracing::debug!("{:#?}", &output);
    Ok(output)
}
