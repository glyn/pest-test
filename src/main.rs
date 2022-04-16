extern crate pest;
#[macro_use]
extern crate pest_derive;

mod ast;
use ast::*;
use pest::Parser;
use std::num::ParseIntError;

#[derive(Parser)]
#[grammar = "csv.pest"]
pub struct CSVParser;

fn main() {
    let selector_rule = CSVParser::parse(Rule::selector, "$")
        .map_err(|e| format!("{}", e))
        .expect("invalid parse result")
        .nth(1)
        .unwrap();

    let result = selector_rule
        .into_inner()
        .fold(Ok(Path::Root), |prev: Result<_, String>, r| {
            match r.as_rule() {
                Rule::matcher => Ok(Path::Sel(
                    Box::new(prev?),
                    parse_selector(r).map_err(|e| format!("{}", e))?,
                )),
                _ => panic!("invalid parse tree {:?}", r),
            }
        });

    println!("result={:?}", result);
}

fn parse_selector(matcher_rule: pest::iterators::Pair<Rule>) -> Result<Selector, ParseIntError> {
    let r = matcher_rule.into_inner().next().unwrap();

    Ok(match r.as_rule() {
        Rule::matcher => Selector::Matcher,
        _ => panic!("invalid parse tree {:?}", r),
    })
}
