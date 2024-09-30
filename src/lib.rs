#![allow(unused)]
use std::fmt;

const DUMMY_LIST1 : &str = "This is a dummy list item to Simplex Presentation.";
const DUMMY_LIST2: &str = "This is another dummy list item to Simplex Presentation.";
const DUMMY_FOOT: &str = "This is a dummy foot to Simplex Presentation.";
const DUMMY_PANVIEW_HTML: &str = "This is a dummy of panview HTML to Simplex Presentation.";
const DUMMY_PANVIEW_CODE: &str = "This is a dummy of panview Code to Simplex Presentation.";
const SEPARATOR: &str = "---";
const TAG_MARKER: &str = ".";
const TAG_LIST: &str = "list";

/// Organized options for customizations.
pub struct Custom {}
impl fmt::Display for Custom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Custom")
    }
}

/// Notes or drafts to be shown to host.
pub struct Notes {}
impl fmt::Display for Notes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Notes")
    }
}

/// Build a fully browseable view of the presentation.
pub struct Panview {
    html: String,
    code: String 
}
impl Panview {
    pub fn default_panview() -> Panview {
        Panview {
            html: "HTML PANVIEW".to_string(), 
            code: "PANVIEW JS CODE".to_string()
        }
    }
}

enum ElementNature {
    Text,
    List,
    OrdList,
    Mermaid,
    Video,
    Image
}

pub struct Element {
    nature: ElementNature,
    content: String,
    number: usize 
}

/// Each fully presentable slide from the entire slideshow.
pub struct Slide {
    number: i32,
    body: Vec<Element>,
    draft: bool
}
impl Slide {
    fn flat(self) -> String {
    let mut content = String::new();
        for element in self.body {
            content.push_str(&format!("{}", element.content));
        }
    content
    }

    pub fn new() -> Slide { 
        Slide {
            number: 1,
            body: vec![Element {
                nature: ElementNature::Text,
                content: format!("<ul><li>{}</li><li>{}</li></ul>", DUMMY_LIST1, DUMMY_LIST2),
                number: 0
            }],
            draft: true
        }
    }
}
impl fmt::Display for Slide {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        for element in &self.body {
            out.push_str(&element.content);
        }
        write!(f, "{}", out)
    }
}

/// The slideshow entity.
pub struct Presentation {
    slides: Vec<Slide>,
    panview: Panview,
    notas: Option<Vec<Notes>>,
    custom: Option<Custom>,
    foot: String,
}
impl Presentation {
    /// Dilute into simplex_parser()
    /// Make a Presentation by receiving all the ingredients.
    pub fn build (panview: Panview, slides: Vec<Slide>, notas: Option<Vec<Notes>>, custom: Option<Custom>) -> Result<Presentation> {
        // HEADER
        let mut header: String = "<head></head>".to_string(); // aquirir LANG, UTF-8 etc
        let mut code: String = "<script></script>".to_string();
        let mut body = String::new();

        // PANVIEW
        let mut panview: String = format!("<body><div class=panview>{}</div><script>{}</script>", panview.html, panview.code);

        // SLIDES
        //for slide in slides {
        //    body.push_str(&format!("<div class=slide>{}</div><div class=slidefoot>{}</div>", &slide.body, &slide.foot));
        //}

        // END BODY
        body.push_str("</body>");

        format!("{}{}{}", header, body, code);

        // FOR A WHILE
        Ok(Presentation::new())

        // DEFINITIVE
    }

    /// Spawn a dummy Presentation.
    pub fn new() -> Presentation {
        Presentation {
            panview: Panview {
                html: format!("<div class=panview>{}</div>", DUMMY_PANVIEW_HTML),
                code: format!("<script>{}</script>", DUMMY_PANVIEW_CODE)
            },
            slides: vec![Slide::new()],
            notas: None,
            custom: None,
            foot: DUMMY_FOOT.to_string()
        }
    }
}
impl fmt::Display for Presentation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();

        // TODO: criar regex para output com quebras no HTML
        //       make a regex for beaultiful output
        for slide in &self.slides {
            let elements : Vec<&Element>= slide.body.iter().collect();
            for element in elements {
                out.push_str(&format!("{}", element.content));
            };
        }
        //self.slides.iter().map(|slide| out.push_str(&format!("{}", slide.body.iter().map(|element| format!("{}", element)))));

        write!(f, "{}", out) 
    }
}

pub type Result<T> = std::result::Result<T, SimplexError>;

/// The miriad of errors in Simplex Presentation.
enum ErrorNature {
    /// When a Slide is built but error occurs.
    BrokenSlide,
    FaultyLine,
    BrokenFoot,
    BrokenBody,
    NoCode,
    EmptyTag
}
/// The default error behavior of Simplex Presentation needs a nature of error, a message associated
/// and a location for debugging.
pub struct SimplexError {
    nature: ErrorNature,
    msg: String,
    loc: i32  // number line for now
}

struct RawSlide {
    elements: Vec<String>,
    number: i32
}

/// This trait is born because split methods of primitive str 
/// doesn't works
trait SplitOnDot {
    fn split_on_dot(self) -> Vec<Vec<String>>;
}

impl SplitOnDot for Vec<String> {
    fn split_on_dot(self) -> Vec<Vec<String>> {
        let mut result = Vec::new();
        let mut temp_group = Vec::new();
    
        for s in self {
            if s.starts_with(".") {
                if !temp_group.is_empty() {
                    result.push(temp_group);
                }
                temp_group = Vec::new();
            }
            temp_group.push(s);
        }
    
        // Adiciona o último grupo
        if !temp_group.is_empty() {
            result.push(temp_group);
        }
    
    result
    }
}

fn list (element: &String, i: usize) -> Element {
    Element {
        nature: ElementNature::List,
        content: format!("<div>{}</div>", element),
        number: i
    }
}

/// The main parser, reads a `Vec<String>` --- mainly from a file input or stdin --- and
/// outputs a Presentations is everything is fine. Also works as a wraper around
/// Presentaton::build.
pub fn simplex_parser(input: Vec<String>) -> Result<Presentation> {
    let tag_list = format!("{}{}", TAG_MARKER, TAG_LIST);
    let mut presentation = Presentation::new();
    let another_slide: Slide = Slide::new(); 

    presentation.slides.push(another_slide); // just so to presentation to have 2 slides

    let mut raw_slides: Vec<Vec<String>> = input.split(|raw_slide| raw_slide.starts_with(SEPARATOR))
        .filter(|line| !line.is_empty())
        .map(|slide| slide.to_vec())
        .collect::<Vec<Vec<String>>>();
    println!("{:?}\n\n", raw_slides);
    
    for (i, slide) in raw_slides.into_iter().enumerate() {
        let elements = slide.split_on_dot();
        println!("\nSlide no. {}", i);

        for (j, element) in elements.into_iter().enumerate() {
            print!("Element no {}\n", j);
            print!("{:?}\n", element);

            match &element[j] {
                // j sync is an ok procedure?
                tag_list => presentation.slides[i].body.push(list(&element[j], j)),
                _ => todo!()
            }
        }
    };

    Ok(presentation)
}
