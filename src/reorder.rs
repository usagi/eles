use super::{
 Operator,
 Token,
 TokenStream
};
use anyhow::{
 Context,
 Result
};

pub fn reorder(tokens: TokenStream) -> Result<TokenStream>
{
 let mut r = Vec::<Token>::new();
 let mut stack = Vec::<Token>::new();

 for token in tokens.into_iter()
 {
  match token
  {
   Token::Operator(Operator::And) | Token::Operator(Operator::Or) | Token::Operator(Operator::Xor) | Token::Operator(Operator::Not) =>
   {
    while let Some(last) = stack.last()
    {
     match last.order()? >= token.order()?
     {
      true =>
      {
       let operator = stack.pop().unwrap();
       r.push(operator);
      },
      false => break
     }
    }
    stack.push(token);
   },
   Token::Operator(Operator::OpenParen) => stack.push(token),
   Token::Operator(Operator::CloseParen) =>
   {
    loop
    {
     match stack.is_empty()
     {
      true => break,
      false =>
      {
       let token = stack.pop().context("postfix: failed to pop stack.")?;
       match token
       {
        Token::Operator(Operator::OpenParen) => break,
        _ => r.push(token)
       }
      }
     }
    }
   },
   _ => r.push(token)
  }
 }

 while !stack.is_empty()
 {
  let token = stack.pop().unwrap();
  r.push(token);
 }

 Ok(r)
}
