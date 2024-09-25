use std::fmt;

struct Custom {}

struct Notas {}

impl fmt::Display for Custom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "Custom")
    }
}

impl fmt::Display for Notas {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "Notas")
    }
}

struct Panview {
    view: String,
    code: String 
}

pub struct Presentation {
    slides: Vec<Slide>,
    panview: Panview,
    notas: Option<Vec<Notas>>,
    custom: Option<Custom>
}

impl fmt::Display for Presentation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let conteudo = for slide in &self.slides {
            write!(f, "{}", slide)?
        };

        Ok(())
    }
}

struct Slide {
    number: i32,
    head: &'static str,
    body: String,
    foot: String
}

impl Slide {
    fn new() -> Slide {
        let slide: Slide = Slide {
            number: 1,
            head: "<div class=slide></div>",
            body: String::from("Body"),
            foot: String::from("Foot")
        };

    slide
    }
}

impl fmt::Display for Slide {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}{}{}", self.head, self.body, self.foot)
    }
}

impl Presentation {
    fn build (slides: Vec<Slide>, notas: Option<Vec<Notas>>, custom: Option<Custom>) -> Presentation {
        Presentation::new() //por enquanto
    }

    pub fn new() -> Presentation {
        Presentation {
            panview: Panview {view: String::from(""), code: String::from("<script>JS CODE</script>") }, 
            slides: vec![Slide::new()],
            notas: None,
            custom: None 
        }
    }
}

enum TipoErro {
    tipo1,
    tipo2
}

struct Erro {
    tipo: TipoErro,
    msg: String,
    loc: i32 
}

pub fn process(input: Vec<String>) -> Result<Presentation, Erro>{
    Ok(Presentation::new())
}
