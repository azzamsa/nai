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
        .map_err(|e| crate::Error::InvalidSyntax {
            message: e.to_string(),
        })?
        .next()
        .unwrap();
    tracing::debug!("{:#?}", &pair);

    for piece in pair.into_inner() {
        match piece.as_rule() {
            Rule::variable => {
                for variable in piece.into_inner() {
                    match variable.as_rule() {
                        Rule::variable_name => match variable.as_str() {
                            "start_date" => {
                                output.push_str(&time.date()?.to_string());
                            }
                            "duration" => {
                                output.push_str(&time.duration()?.to_string());
                            }
                            _ => {
                                return Err(crate::Error::InvalidBuiltInVariable {
                                    variable: variable.as_str().to_string(),
                                })
                            }
                        },
                        Rule::WHITESPACE => (),
                        _ => {
                            // tracing::debug!("unreachable {:?}", &variable);
                            unreachable!();
                        }
                    }
                }
            }
            Rule::word | Rule::WHITESPACE => {
                output.push_str(piece.as_str());
            }
            Rule::EOI => (),
            _ => {
                // tracing::debug!("unreachable {:?}", &piece);
                unreachable!();
            }
        }
    }
    tracing::debug!("{:#?}", &output);
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_case(format: &str) -> Moment {
        Moment {
            start_date: "1987-Dec-19".to_string(),
            format: format.to_string(),
        }
    }

    #[test]
    fn simple() -> Result<(), crate::Error> {
        let moment = test_case("He was born on {{ start_date }}.");
        let result = parse(&moment)?;
        let expected = "He was born on Sat, 19 Dec 1987.";
        assert_eq!(result, expected);
        Ok(())
    }
    #[test]
    fn leading_and_trailing_whitespace() -> Result<(), crate::Error> {
        let moment = test_case(" He was born on {{ start_date }}. ");
        let result = parse(&moment)?;
        let expected = " He was born on Sat, 19 Dec 1987. ";
        assert_eq!(result, expected);
        Ok(())
    }
    #[test]
    fn no_whitespace_variable() -> Result<(), crate::Error> {
        let moment = test_case("He was born on {{start_date}}.");
        let result = parse(&moment)?;
        let expected = "He was born on Sat, 19 Dec 1987.";
        assert_eq!(result, expected);
        Ok(())
    }
    #[test]
    fn punctuation_and_special_characters() -> Result<(), crate::Error> {
        let moment = test_case(
            "Faramir's child was born on {{ start_date }}, his favorite quote is: 'Not - all those who wander are lost!?.'"
        );
        let result = parse(&moment)?;
        let expected = "Faramir's child was born on Sat, 19 Dec 1987, his favorite quote is: 'Not - all those who wander are lost!?.'";
        assert_eq!(result, expected);
        Ok(())
    }
}
