use super::Operator;
use anyhow::{
 anyhow,
 Result
};

#[derive(Debug)]
pub enum Token
{
 Operator(Operator),
 Literal(String),
 Ether
}

impl Token
{
 pub fn order(&self) -> Result<u8>
 {
  match self
  {
   Token::Operator(operator) => Ok(operator.order()),
   _ => Err(anyhow!("Token::order: failed to order."))
  }
 }
}
