use anyhow::{
 anyhow,
 bail,
 Context,
 Result
};
use eles::solve;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

enum DatabaseValue
{
 S(String),
 N(f64)
}

type Database = HashMap<String, DatabaseValue>;

static DATABASE: Lazy<Database> = Lazy::<Database>::new(|| {
 let mut db = Database::new();
 db.insert("abc".to_string(), DatabaseValue::S("abc".to_string()));
 db.insert("abc.p".to_string(), DatabaseValue::N(std::f64::consts::PI));
 db.insert("def".to_string(), DatabaseValue::S("def".to_string()));
 db.insert("def.q".to_string(), DatabaseValue::N(std::f64::consts::SQRT_2));
 db
});

fn data_tester(input: &str) -> Result<bool>
{
 const GTLT_PATTERN: &str = r#"^(?P<lhs>.+)(?P<op>[><][=]?)(?P<rhs>.+)$"#;
 static GTLT: Lazy<Regex> = Lazy::<Regex>::new(|| Regex::new(GTLT_PATTERN).unwrap());
 if input.starts_with("is_exists(")
 {
  let key = input["is_exists(".len()..input.len() - 1].trim();
  let r = DATABASE.contains_key(key);
  println!("[debug] is_exists(key) -> {:?}; key={:?}", r, key);
  Ok(r)
 }
 else if let Some(captures) = GTLT.captures(input)
 {
  let lhs = captures.name("lhs").context("lhs error 1")?.as_str().trim();
  let lhs = DATABASE.get(lhs).context("lhs error 2")?;
  let lhs = match lhs
  {
   DatabaseValue::N(lhs) => *lhs,
   _ => bail!("lhs error 3")
  };

  let op = captures
   .name("op")
   .context("op error 1")?
   .as_str()
   .chars()
   .next()
   .context("op error 2")?;

  let rhs = captures.name("rhs");
  let rhs = rhs.with_context(|| format!("rhs error 1; rhs={:?}", rhs))?.as_str().trim();
  let rhs = rhs.parse::<f64>().map_err(|_| anyhow!("rhs error 2; rhs={:?}", rhs))?;

  let r = match op
  {
   '>' =>
   {
    println!("[debug] lhs > rhs -> {:?}; lhs={:?} rhs={:?}", lhs > rhs, lhs, rhs);
    lhs > rhs
   },
   '<' =>
   {
    println!("[debug] lhs < rhs -> {:?}; lhs={:?} rhs={:?}", lhs < rhs, lhs, rhs);
    lhs < rhs
   },
   _ => bail!("opration error 1")
  };
  Ok(r)
 }
 else
 {
  Err(anyhow!("syntax error 1; input={:?}", input))
 }
}

fn main()
{
 let query = "(( is_exists(abc) && abc.p > 3.0 && abc.p < 4.0 )) || (( is_exists(def) && def.q > 1.23e-6 && def.q < 1.66e-6 ))";
 let r = solve(&query, data_tester);
 println!("{:?}", r);
}
