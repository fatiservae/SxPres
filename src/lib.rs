use std::fmt;

pub struct Custom {}

pub struct Notas {}

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

pub struct Panview {
    view: String,
    code: String 
}

impl Panview {
    pub fn default_panview() -> Panview {
        Panview {
            view: "HTML PANVIEW".to_string(), 
            code: "PANVIEW JS CODE".to_string()
        }
    }
}

pub struct Presentation {
    slides: Vec<Slide>,
    panview: Panview,
    notas: Option<Vec<Notas>>,
    custom: Option<Custom>
}

impl fmt::Display for Presentation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut saida = String::new();

        // não consegui idiomático
        for slide in &self.slides {
            saida.push_str(&slide.head);
            saida.push_str(&slide.body);
            saida.push_str(&slide.foot);
        };
        write!(f, "{}", saida) 
    }
}

pub struct Slide {
    number: i32,
    head: &'static str,
    body: String,
    foot: String
}

impl Slide {
    fn flat(self) -> String {
        format!("{}{}{}", self.head, self.body, self.foot)
    }

    pub fn new() -> Slide {
        Slide {
            number: 1,
            head: "<div class=slide></div>",
            body: String::from("Body"),
            foot: String::from("Foot")
        }
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
    pub fn build (pan: Option<Panview>, slides: Vec<Slide>, notas: Option<Vec<Notas>>, custom: Option<Custom>) -> String {
        //let mut presentation = Presentation::new() //por enquanto
        //
        //
        let mut header: String = "<head></head>".to_string(); // aquiri LANG, UTF-8 etc
        let mut code: String = "<script></script>".to_string();
        
        let mut body: String = String::new();
        let mut panview_html: String = String::new();
        let mut panview_code: String = String::new();

        body.push_str("<body><div class=panview>");
        match pan {
            Some(pan) => {
                panview_html = pan.view;
                panview_code = pan.code;
            },
            None => {
                let def_panview = Panview::default_panview();
                panview_html = def_panview.view;
                panview_code = def_panview.code;
            }
        };

        body.push_str(&panview_html);
        body.push_str("</div>"); // fecha para Panview
        body.push_str(&format!("<script>{}</script>", &panview_code));


        //
        for slide in slides {
            body.push_str("<div class=slide>");
            body.push_str(&slide.body);
            body.push_str("</slide>");
        }

        body.push_str("</body>");

        format!("{}{}{}", header, body, code)
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
