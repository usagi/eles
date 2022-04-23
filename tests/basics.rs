use anyhow::{
 anyhow,
 Result
};
use eles::solve;

fn basic_logical_output_parser(input: &str) -> Result<bool>
{
 match input
 {
  "true" | "do your already parser -> true" => Ok(true),
  "false" | "do(your, already, parser) -> false" => Ok(false),
  sql if sql.to_lowercase().starts_with("select") => Err(anyhow!("I'm not a SQL.")),
  _ => Err(anyhow!("oops! your syntax {} is something wrong.", input))
 }
}

#[test]
fn just_true()
{
 assert_eq!(solve("true", basic_logical_output_parser).unwrap(), true);
 assert_eq!(solve("do your already parser -> true", basic_logical_output_parser).unwrap(), true);
}

#[test]
fn just_false()
{
 assert_eq!(solve("false", basic_logical_output_parser).unwrap(), false);
 assert_eq!(
  solve("do(your, already, parser) -> false", basic_logical_output_parser).unwrap(),
  false
 );
}

#[test]
fn err()
{
 assert!(solve("something wrong syntax", basic_logical_output_parser).is_err());
 let r = solve("SELECT * FROM my_nyanko_pictures_folder;", basic_logical_output_parser);
 assert_eq!(r.err().unwrap().to_string(), "I'm not a SQL.");
}

#[test]
fn x_and_x()
{
 assert_eq!(solve("true && true", basic_logical_output_parser).unwrap(), true);
 assert_eq!(solve("true && false", basic_logical_output_parser).unwrap(), false);
 assert_eq!(solve("false && true", basic_logical_output_parser).unwrap(), false);
 assert_eq!(solve("false && false", basic_logical_output_parser).unwrap(), false);
}

#[test]
fn x_or_x()
{
 assert_eq!(solve("true || true", basic_logical_output_parser).unwrap(), true);
 assert_eq!(solve("true || false", basic_logical_output_parser).unwrap(), true);
 assert_eq!(solve("false || true", basic_logical_output_parser).unwrap(), true);
 assert_eq!(solve("false || false", basic_logical_output_parser).unwrap(), false);
}

#[test]
fn x_xor_x()
{
 assert_eq!(solve("true ^^ true", basic_logical_output_parser).unwrap(), false);
 assert_eq!(solve("true ^^ false", basic_logical_output_parser).unwrap(), true);
 assert_eq!(solve("false ^^ true", basic_logical_output_parser).unwrap(), true);
 assert_eq!(solve("false ^^ false", basic_logical_output_parser).unwrap(), false);
}

#[test]
fn not()
{
 assert_eq!(solve("!! true", basic_logical_output_parser).unwrap(), false);
 assert_eq!(solve("!! false", basic_logical_output_parser).unwrap(), true);
 assert_eq!(solve("true || !! true", basic_logical_output_parser).unwrap(), true);
 assert_eq!(solve("true || !! false", basic_logical_output_parser).unwrap(), true);
 assert_eq!(solve("true && !! true", basic_logical_output_parser).unwrap(), false);
 assert_eq!(solve("true && !! false", basic_logical_output_parser).unwrap(), true);
 assert_eq!(solve("(( true )) && !! true", basic_logical_output_parser).unwrap(), false);
 assert_eq!(solve("(( true )) && !! false", basic_logical_output_parser).unwrap(), true);
 assert_eq!(solve("true && (( !! true  ))", basic_logical_output_parser).unwrap(), false);
 assert_eq!(solve("true && (( !! false ))", basic_logical_output_parser).unwrap(), true);
}

#[test]
fn parenthes()
{
 assert_eq!(
  solve("(( true || true  )) && (( true  || true  ))", basic_logical_output_parser).unwrap(),
  true
 );
 assert_eq!(
  solve("(( true || false )) && (( true  || false ))", basic_logical_output_parser).unwrap(),
  true
 );
 assert_eq!(
  solve("(( true || false )) && (( false || false ))", basic_logical_output_parser).unwrap(),
  false
 );

 assert_eq!(
  solve("(( true && true  )) || (( true  && true  ))", basic_logical_output_parser).unwrap(),
  true
 );
 assert_eq!(
  solve("(( true && false )) || (( true  && false ))", basic_logical_output_parser).unwrap(),
  false
 );
 assert_eq!(
  solve("(( true && false )) || (( false && false ))", basic_logical_output_parser).unwrap(),
  false
 );
 assert_eq!(
  solve(
   "(( (( true || true )) && (( false || false )) )) ^^ (( true && (( false || true )) ))",
   basic_logical_output_parser
  )
  .unwrap(),
  true
 );
}
