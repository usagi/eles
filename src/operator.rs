#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator
{
 And,
 Or,
 Xor,

 Not,

 OpenParen,
 CloseParen
}

impl Operator
{
 pub fn order(&self) -> u8
 {
  use Operator::*;
  match self
  {
   Not => 100,
   Or => 10,
   Xor => 9,
   And => 8,
   OpenParen | CloseParen => 1
  }
 }
}
