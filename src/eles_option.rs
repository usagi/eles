#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElesOption
{
 pub open_parenthesis: String,
 pub close_parenthesis: String,
 pub and_operator: String,
 pub or_operator: String,
 pub xor_operator: String,
 pub not_operator: String,
 pub separator: String
}

pub const DEFAULT_OPEN_PARENTHESIS: &str = "((";
pub const DEFAULT_CLOSE_PARENTHESIS: &str = "))";
pub const DEFAULT_AND_OPERATOR: &str = "&&";
pub const DEFAULT_OR_OPERATOR: &str = "||";
pub const DEFAULT_XOR_OPERATOR: &str = "^^";
pub const DEFAULT_NOT_OPERATOR: &str = "!!";
pub const DEFAULT_SEPARATOR: &str = " ";

impl ElesOption
{
 pub fn default() -> Self
 {
  Self {
   open_parenthesis: DEFAULT_OPEN_PARENTHESIS.to_string(),
   close_parenthesis: DEFAULT_CLOSE_PARENTHESIS.to_string(),
   and_operator: DEFAULT_AND_OPERATOR.to_string(),
   or_operator: DEFAULT_OR_OPERATOR.to_string(),
   xor_operator: DEFAULT_XOR_OPERATOR.to_string(),
   not_operator: DEFAULT_NOT_OPERATOR.to_string(),
   separator: DEFAULT_SEPARATOR.to_string()
  }
 }

 pub fn new(
  open_parenthesis: &str,
  close_parenthesis: &str,
  and_operator: &str,
  or_operator: &str,
  xor_operator: &str,
  not_operator: &str,
  separator: &str
 ) -> Self
 {
  Self {
   open_parenthesis: open_parenthesis.to_string(),
   close_parenthesis: close_parenthesis.to_string(),
   and_operator: and_operator.to_string(),
   or_operator: or_operator.to_string(),
   xor_operator: xor_operator.to_string(),
   not_operator: not_operator.to_string(),
   separator: separator.to_string()
  }
 }
}
