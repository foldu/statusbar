use pest::Parser;
use pest_derive::*;

use crate::Error;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct FormatParser;

#[inline]
pub fn parse(s: &str) -> Result<pest::iterators::Pairs<Rule>, Error> {
    FormatParser::parse(Rule::format_string, s)
        .map_err(|e| e.into())
        .map(|mut parsed| parsed.next().unwrap().into_inner())
}
