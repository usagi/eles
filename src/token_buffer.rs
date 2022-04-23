use super::{
 Operator,
 Token,
 TokenStream
};

#[derive(Debug)]
pub struct TokenBuffer<'a>
{
 token_buffer: Vec<Token>,
 literal_buffer: Vec<&'a str>
}

impl<'a> TokenBuffer<'a>
{
 pub fn new() -> Self
 {
  Self {
   token_buffer: vec![],
   literal_buffer: vec![]
  }
 }

 pub fn push_literal_part(&mut self, part: &'a str)
 {
  self.literal_buffer.push(part);
 }

 fn flush_literal_buffer(&mut self)
 {
  match self.literal_buffer.is_empty()
  {
   true => (),
   false =>
   {
    self
     .token_buffer
     .push(Token::Literal(self.literal_buffer.join(" ").trim().to_string()));
    self.literal_buffer.clear();
   }
  }
 }

 pub fn push_operator_token(&mut self, operator: Operator)
 {
  self.flush_literal_buffer();
  if operator == Operator::Not
  {
   self.token_buffer.push(Token::Ether);
  }
  self.token_buffer.push(Token::Operator(operator));
 }

 pub fn flush_tokens(mut self) -> TokenStream
 {
  self.flush_literal_buffer();
  self.token_buffer
 }
}
