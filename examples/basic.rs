use anyhow::{
 anyhow,
 Result
};
use eles::solve;

fn just_basic_your_parser(input: &str) -> Result<bool>
{
 match input
 {
  "true" => Ok(true),
  "false" => Ok(false),
  x if x.to_lowercase() == "true" => Err(anyhow!(r#"Syntax error: {:?}; "true" maybe?😆"#, x)),
  x if x.to_lowercase() == "false" => Err(anyhow!(r#"Syntax error: {:?}; "false" maybe?😆"#, x)),
  _ => Err(anyhow!("Syntax error: {:?}.😭", input))
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
   println!("\teg.: {} true", args[0]);
   println!("\teg.: {} false", args[0]);
   println!("\teg.: {} \"true && false\"", args[0]);
   println!("\teg.: {} truE", args[0]);
   println!("\teg.: {} FALSe", args[0]);
   println!("\n\tand more complexed inputs and implement with your syntax parser.😃");
  },
  _ =>
  {
   let your_user_input = &args[1..].join(" ");
   let r = solve(&your_user_input, just_basic_your_parser);
   println!("{:?}", r);
  }
 }
}
