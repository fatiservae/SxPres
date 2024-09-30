use sxpres::*;

fn main() {
    let input = vec!["---".to_string(), ".list".to_string(), "Primeiro da lista".to_string(), "Segundo da lista".to_string(),".list".to_string(), "Primeiro da lista".to_string(), "Segundo da lista".to_string(), "---".to_string(), ".list".to_string(), "Primeiro da lista".to_string(), "Segundo da lista".to_string()]; // idealy, a Slide Type

    //let pres: Result<Presentation, SimplexError> = simplex_parser(slide);

    //match pres{
    //    Ok(presentation) => println!("{}", presentation),
    //    _ => todo!()
    //}

    let presentation = simplex_parser(input);

    match presentation {
        Ok(press) => println!("{}", press),
        _ => todo!()
    }
}
