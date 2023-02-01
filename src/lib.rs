use pest::{
    error::{Error, ErrorVariant},
    iterators::Pair,
    Parser, Position,
};
use pest_derive;

#[derive(pest_derive::Parser)]
#[grammar = "wdl.pest"]
pub struct PestParser;

pub fn parse_document<'a>(text: &'a str) -> Result<Pair<'a, Rule>, Error<Rule>> {
    let mut root_pairs = PestParser::parse(Rule::document, text)?;
    root_pairs.next().ok_or_else(|| {
        Error::new_from_pos(
            ErrorVariant::CustomError {
                message: String::from("Empty WDL document"),
            },
            Position::from_start(text),
        )
    })
}
