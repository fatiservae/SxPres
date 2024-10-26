use sxpres::*;

fn main() {
    // A dummy input.
    let input = vec!["---".to_string(), ".list".to_string(), "Primeiro da lista".to_string(), "Segundo da lista".to_string(),".list".to_string(), "Primeiro da lista".to_string(), "Segundo da lista".to_string(), "---".to_string(), ".list".to_string(), "Primeiro da lista".to_string(), "Segundo da lista".to_string()]; // idealy, a Slide Type

    let presentation = sx_parser(input);
    match presentation {
        Ok(press) => println!("{}", press),
        Err(err) => println!("{}", err)
    }
}
