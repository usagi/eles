use super::{
 evaluate,
 reorder,
 tokenize,
 ElesOption
};
use anyhow::Result;

pub fn solve_with<F>(input: &str, literal_to_bool: F, eles_option: ElesOption) -> Result<bool>
where
 F: Fn(&str) -> Result<bool>
{
 let tokens = tokenize(input, eles_option)?;
 let tokens = reorder(tokens)?;
 let r = evaluate(tokens, literal_to_bool)?;
 Ok(r)
}
