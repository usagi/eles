use anyhow::Result;
use eles::{
 solve_with,
 ElesOption
};

fn your_parser(input: &str) -> Result<bool>
{
 match input
 {
  "🆗" | "🍣" | "🍵" => Ok(true),
  _ => Ok(false)
 }
}

fn main()
{
 let args = std::env::args().collect::<Vec<_>>();
 match args.len()
 {
  1 =>
  {
   println!("Usage: {} <your input>", args[0]);
   println!("\teg.: {} \"[ 🆗 or 🆖 ] ⚖️ [ 🍣 and 🍵 or 🙅‍♀️ ☕ ]\"", args[0]);
   println!("\n\tand more complexed inputs and implement with your syntax parser.😃");
  },
  _ =>
  {
   let your_user_input = &args[1..].join(" ");
   let your_customized_logical_operation_tokens = ElesOption::new("[", "]", "and", "or", "⚖️", "🙅‍♀️", " ");
   let r = solve_with(&your_user_input, your_parser, your_customized_logical_operation_tokens);
   println!("{:?}", r);
  }
 }
}
