use super::{
 ElesOption,
 Operator,
 TokenBuffer,
 TokenStream
};
use anyhow::Result;

pub fn tokenize(input: &str, o: ElesOption) -> Result<TokenStream>
{
 let mut b = TokenBuffer::new();

 for token in input.split(o.separator.as_str())
 {
  let token = token.trim();
  match token
  {
   t if t == o.and_operator => b.push_operator_token(Operator::And),
   t if t == o.or_operator => b.push_operator_token(Operator::Or),
   t if t == o.xor_operator => b.push_operator_token(Operator::Xor),
   t if t == o.not_operator => b.push_operator_token(Operator::Not),
   "((" => b.push_operator_token(Operator::OpenParen),
   "))" => b.push_operator_token(Operator::CloseParen),
   _ => b.push_literal_part(token)
  }
 }
 let r = b.flush_tokens();
 Ok(r)
}
