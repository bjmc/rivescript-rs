use std::env;
use std::fs;
use pest::Parser;
use pest_derive::Parser;
use pest_test_gen::pest_tests;


mod rivescript {
    #[derive(pest_derive::Parser)]
    #[grammar = "rivescript.pest"]
    pub struct Parser;
}

#[pest_tests(
  super::rivescript::Parser,
  super::rivescript::Rule,
  "file",
  ext = "test",
  recursive = true,
  lazy_static = true,
)]
#[cfg(test)]
mod tests {}