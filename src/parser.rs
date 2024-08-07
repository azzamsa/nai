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
            Rule::raw | Rule::WHITESPACE => {
                output.push_str(piece.as_str());
            }
            Rule::expr => {
                let mut output_ = String::new();
                for expr in piece.into_inner() {
                    match expr.as_rule() {
                        Rule::variable => match expr.as_str() {
                            "start_date" => {
                                output_.push_str(&time.date()?.to_string());
                            }
                            "duration" => {
                                output_.push_str(&time.duration()?.to_string());
                            }
                            _ => {
                                return Err(crate::Error::InvalidBuiltInVariable {
                                    variable: expr.as_str().to_string(),
                                })
                            }
                        },
                        Rule::style => {
                            parse_style(expr.into_inner(), &mut output_);
                        }
                        Rule::string => {
                            for string in expr.into_inner() {
                                match string.as_rule() {
                                    Rule::content => {
                                        output_.push_str(string.as_str());
                                    }
                                    Rule::WHITESPACE => (),
                                    _ => {
                                        tracing::debug!("unreachable `string` {:?}", &string);
                                        unreachable!();
                                    }
                                }
                            }
                        }
                        Rule::WHITESPACE => (),
                        _ => {
                            tracing::debug!("unreachable `expr` {:?}", &expr);
                            unreachable!();
                        }
                    }
                }
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

fn parse_style(styles: Pairs<'_, Rule>, output: &mut String) {
    for style in styles {
        match style.as_rule() {
            Rule::color => {
                *output = format!("{}", output.color(color_from_str(style.as_str())));
            }
            Rule::effect => {
                let style = Style::new().effect(effect_from_str(style.as_str()));
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

/// Get `Color` from string
fn color_from_str(color: &str) -> AnsiColors {
    match color {
        "black" => AnsiColors::Black,
        "red" => AnsiColors::Red,
        "green" => AnsiColors::Green,
        "yellow" => AnsiColors::Yellow,
        "blue" => AnsiColors::Blue,
        "magenta" => AnsiColors::Magenta,
        "purple" => AnsiColors::Magenta,
        "cyan" => AnsiColors::Cyan,
        "white" => AnsiColors::White,
        "brightBlack" => AnsiColors::BrightBlack,
        "brightRed" => AnsiColors::BrightRed,
        "brightGreen" => AnsiColors::BrightGreen,
        "brightYellow" => AnsiColors::BrightYellow,
        "brightBlue" => AnsiColors::BrightBlue,
        "brightMagenta" => AnsiColors::BrightMagenta,
        "brightCyan" => AnsiColors::BrightCyan,
        "brightWhite" => AnsiColors::BrightWhite,
        _ => AnsiColors::White,
    }
}

/// Get `Effect` from string
fn effect_from_str(effect: &str) -> Effect {
    match effect {
        "bold" => Effect::Bold,
        "dimmed" => Effect::Dimmed,
        "italic" => Effect::Italic,
        "underline" => Effect::Underline,
        "blink" => Effect::Blink,
        "blinkfast" => Effect::BlinkFast,
        "reversed" => Effect::Reversed,
        "hidden" => Effect::Hidden,
        "strikethrough" => Effect::Strikethrough,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_case(format: &str) -> Moment {
        Moment {
            start_date: "1987-12-19".to_string(),
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
    #[test]
    fn emoji() -> Result<(), crate::Error> {
        let moment = test_case("👶 {{ 'Faramir' | underline }} was born");
        let result = parse(&moment);
        assert!(result.is_ok());
        Ok(())
    }
    #[test]
    fn multiple_style() -> Result<(), crate::Error> {
        let moment = test_case("👶 {{ 'Faramir' | magenta | bold }} was born");
        let result = parse(&moment);
        assert!(result.is_ok());
        Ok(())
    }
    #[test]
    fn bright_color() -> Result<(), crate::Error> {
        let moment = test_case("👶 {{ 'Faramir' | brightMagenta | bold }} was born");
        let result = parse(&moment);
        assert!(result.is_ok());
        Ok(())
    }
    #[test]
    fn all() -> Result<(), crate::Error> {
        let moment = test_case("👶 {{ 'Faramir' | blue }} was born on {{ start_date | red }}. His age is {{ duration }}. {{ 'Bye' | underline }}!");
        let result = parse(&moment);
        assert!(result.is_ok());
        Ok(())
    }
}
