use owo_colors::{AnsiColors, Effect, OwoColorize, Style};
use pest::iterators::Pairs;
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
    tracing::trace!("{:#?}", &pair);

    for piece in pair.into_inner() {
        match piece.as_rule() {
            Rule::word | Rule::WHITESPACE => {
                output.push_str(piece.as_str());
            }
            Rule::literal => {
                let mut output_ = String::new();
                for literal in piece.into_inner() {
                    match literal.as_rule() {
                        Rule::word => {
                            output_.push_str(literal.as_str());
                        }
                        Rule::style => {
                            parse_style(literal.into_inner(), &mut output_);
                        }
                        Rule::WHITESPACE => (),
                        _ => {
                            tracing::debug!("unreachable `literal` {:?}", &literal);
                            unreachable!();
                        }
                    }
                }
                output.push_str(&output_);
            }
            Rule::variable => {
                let mut output_ = String::new();
                parse_variables(piece.into_inner(), &mut output_, &time)?;
                output.push_str(&output_);
            }
            Rule::EOI => (),
            _ => {
                tracing::debug!("unreachable `piece` {:?}", &piece);
                unreachable!();
            }
        }
    }
    tracing::debug!("{:#?}", &output);
    Ok(output)
}

fn parse_variables(
    variables: Pairs<'_, Rule>,
    output: &mut String,
    time: &Time,
) -> Result<(), crate::Error> {
    for variable in variables {
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
            Rule::style => {
                parse_style(variable.into_inner(), output);
            }
            Rule::WHITESPACE => (),
            _ => {
                tracing::debug!("unreachable `variable` {:?}", &variable);
                unreachable!();
            }
        }
    }

    Ok(())
}

fn parse_style(styles: Pairs<'_, Rule>, output: &mut String) {
    for style in styles {
        match style.as_rule() {
            Rule::color => {
                *output = format!("{}", output.color(AnsiColors::from(style.as_str())));
            }
            Rule::effect => {
                let style = Style::new().effect(Effect::Underline);
                *output = format!("{}", output.style(style));
            }
            Rule::WHITESPACE => (),
            _ => {
                tracing::debug!("unreachable literal's style: {:?}", &style);
                unreachable!();
            }
        }
    }
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
    fn built_in_variables() -> Result<(), crate::Error> {
        let moment = test_case("He was born on {{ start_date }}. His age is {{ duration }}");
        let result = parse(&moment);
        assert!(result.is_ok());
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
    #[test]
    fn newline() -> Result<(), crate::Error> {
        let moment = test_case("Faramir age.\nHe was born on {{ start_date }}.\nThanks god.");
        let result = parse(&moment);
        assert!(result.is_ok());
        Ok(())
    }
    #[test]
    fn literal() -> Result<(), crate::Error> {
        let moment = test_case("{{ 'Faramir' }} was born on {{ start_date }}");
        let result = parse(&moment);
        assert!(result.is_ok());
        Ok(())
    }
    #[test]
    fn style_in_literal() -> Result<(), crate::Error> {
        let moment = test_case("{{ 'Faramir' | blue }} was born on {{ start_date }}");
        let result = parse(&moment);
        assert!(result.is_ok());
        Ok(())
    }
    #[test]
    fn style_in_variable() -> Result<(), crate::Error> {
        let moment = test_case("{{ 'Faramir' | blue }} was born on {{ start_date | red }}");
        let result = parse(&moment);
        assert!(result.is_ok());
        Ok(())
    }
    #[test]
    fn effect_in_literal() -> Result<(), crate::Error> {
        let moment = test_case("{{ 'Faramir' | underline }} was born on {{ start_date }}");
        let result = parse(&moment);
        assert!(result.is_ok());
        Ok(())
    }
}
