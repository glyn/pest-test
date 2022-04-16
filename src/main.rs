extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "test.pest"]
pub struct TestParser;

fn main() {
    TestParser::parse(Rule::selector, " $")
        .map_err(|e| format!("{}", e))
        .map_err(|e| {
            println!("{}", e);
            e
        })
        .expect("parse failure as expected");

    println!("Test failed - should have errored already");
}
