mod lib;

fn main() {
    let linha1 = String::from("---\n.list\n1. Primeiro\n2. Segundo\n3.Terceiro\n");
    let linha2 = String::from("---\n.list\n1. Primeiro\n2. Segundo\n3.Terceiro\n");

    let teste = vec![linha1, linha2];

    let slide1 = lib::Slide::new(); 
    // input -> process(input) -> printa
    // let apresentacao: Presentation = teste.process();
    println!("{}", lib::Presentation::build(None, vec![slide1], None, None)); 
}
