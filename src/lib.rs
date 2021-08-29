use pest_derive::Parser;
use pest::Parser;

#[derive(Parser)]
#[grammar = "../grammar/test.pest"] // relative to src
struct CalculationParser;



mod test{
    use super::*;
    #[test]
    fn simple_addition(){
        let result = CalculationParser::parse(Rule::calculation, "1+2").unwrap().next().unwrap();
        print!("{:#?}",result);
    }
}