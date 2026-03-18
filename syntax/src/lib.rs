use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "trx.pest"]
pub struct TrxParser;

pub use pest::Parser as _;
pub use pest::iterators::{Pair, Pairs};
pub type TrxPair<'a> = Pair<'a, Rule>;
pub type TrxPairs<'a> = Pairs<'a, Rule>;