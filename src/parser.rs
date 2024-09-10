

#[cfg(test)]
use pest_test_gen::pest_tests;

pub mod rivescript {
    #[derive(pest_derive::Parser)]
    #[grammar = "rivescript.pest"]
    pub struct OurParser;
}


#[pest_tests(
  super::rivescript::OurParser,
  super::rivescript::Rule,
  "file",
  ext = "test",
  recursive = true,
  lazy_static = true,
)]
#[cfg(test)]
mod tests {}