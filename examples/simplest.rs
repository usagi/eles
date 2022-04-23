fn main()
{
 let input = "true && (( false ^^ true && !! false )) || false";
 let literal_parser = |literal: &str| Ok(literal.parse::<bool>().unwrap_or_default());
 let r = eles::solve(&input, literal_parser);
 println!("{:?}", r);
}
