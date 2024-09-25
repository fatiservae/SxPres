mod lib;

fn main() {
    let linha1 = String::from("---\n.list\n1. Primeiro\n2. Segundo\n3.Terceiro\n");
    let linha2 = String::from("---\n.list\n1. Primeiro\n2. Segundo\n3.Terceiro\n");

    let teste = vec![linha1, linha2];

    // input -> process(input) -> printa
    // let apresentacao: Presentation = teste.process();
    println!("{}", lib::Presentation::new()); 
}
