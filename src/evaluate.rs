use super::{
 Operator,
 Token,
 TokenStream
};
use anyhow::{
 anyhow,
 Context,
 Result
};

pub fn evaluate<F>(postfix: TokenStream, literal_to_bool: F) -> Result<bool>
where
 F: Fn(&str) -> Result<bool>
{
 let mut stack = vec![];

 let postfix = match postfix.is_empty()
 {
  true => vec![Token::Literal(String::new())],
  false => postfix
 };

 for token in postfix.into_iter()
 {
  match token
  {
   Token::Operator(Operator::And) =>
   {
    let rhs = stack.pop().context("eval: failed to pop rhs")?;
    let lhs = stack.pop().context("eval: failed to pop lhs")?;
    stack.push(lhs && rhs)
   },
   Token::Operator(Operator::Or) =>
   {
    let rhs = stack.pop().context("eval: failed to pop rhs")?;
    let lhs = stack.pop().context("eval: failed to pop lhs")?;
    stack.push(lhs || rhs)
   },
   Token::Operator(Operator::Xor) =>
   {
    let rhs = stack.pop().context("eval: failed to pop rhs")?;
    let lhs = stack.pop().context("eval: failed to pop lhs")?;
    stack.push(lhs ^ rhs)
   },
   Token::Operator(Operator::Not) =>
   {
    let rhs = stack.pop().context("eval: failed to pop rhs")?;
    stack.push(!rhs)
   },
   Token::Operator(operator) => Err(anyhow!("eval: failed to solve the oprator {:?}", operator))?,
   Token::Literal(literal) => stack.push(literal_to_bool(&literal)?),
   Token::Ether => ()
  }
 }

 Ok(stack.pop().context("eval: failed to pop the final result.")?)
}
