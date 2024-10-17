#![allow(unused)]
use std::fmt;

const DUMMY_LIST1 : &str = "This is a dummy list item to Simplex Presentation.";
const DUMMY_LIST2: &str = "This is another dummy list item to Simplex Presentation.";
const DUMMY_FOOT: &str = "This is a dummy foot to Simplex Presentation.";
const DUMMY_PANVIEW_HTML: &str = "This is a dummy of panview HTML to Simplex Presentation.";
const DUMMY_PANVIEW_CODE: &str = "This is a dummy of panview Code to Simplex Presentation.";
const SEPARATOR: &str = "---";
const TAG_MARKER: &str = ".";
const TAG_ULIST: &str = "list";
const TAG_ORDLIST: &str = "ordlist";
const TAG_TEXT: &str = "text";
const TAG_MERMAID: &str = "mermaid";
const TAG_VIDEO: &str = "video";
const TAG_IMAGE: &str = "image";
const COMMENT_MARKER: &str = "//";

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
// Consider just a title or numbering for a first implementation
// later, consider thumbnails
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
    Unknow,
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
    number: usize,
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
        match self.draft {
            false => write!(f, "<div class=slide>{}</div>", out),
            true => write!(f, "<!-- a draft slide -->\n<div class=slide></div>")
        }
    }
}

trait IsComment {
    fn is_comment(self) -> bool;
}

impl IsComment for String {
    fn is_comment(self) -> bool {
        if self.starts_with(COMMENT_MARKER) {true} else {false}
    }
}

impl IsComment for &&[String] {
    fn is_comment(self) -> bool {
        if self[0] == COMMENT_MARKER {
            true
        } else {false}
    }
}

/// The slideshow entity itself.
pub struct Presentation {
    head: String,
    slides: Vec<Slide>,
    panview: Panview,
    notas: Option<Vec<Notes>>,
    custom: Option<Custom>,
    foot: String,
}
impl Presentation {
    /// Make a Presentation by receiving all the ingredients.
    // Deactivated!
    pub fn build (panview: Panview, slides: Vec<Slide>, notas: Option<Vec<Notes>>, custom: Option<Custom>) -> Result<Presentation> {
        //// HEADER
        //let mut header: String = "<head></head>".to_string(); // aquirir LANG, UTF-8 etc
        //let mut code: String = "<script></script>".to_string();
        //let mut body = String::new();

        //// PANVIEW
        //let mut panview: String = format!("<body><div class=panview>{}</div><script>{}</script>", panview.html, panview.code);

        //// SLIDES
        ////for slide in slides {
        ////    body.push_str(&format!("<div class=slide>{}</div><div class=slidefoot>{}</div>", &slide.body, &slide.foot));
        ////}

        //// END BODY
        //body.push_str("</body>");

        //format!("{}{}{}", header, body, code);

        //// FOR A WHILE
        Ok(Presentation::new())

        // DEFINITIVE
    }

    /// Spawn a dummy Presentation.
    pub fn new() -> Presentation {
        Presentation {
            head: "<head></head>".to_string(),
            panview: Panview {
                html: format!("<div class=panview>{}</div>", DUMMY_PANVIEW_HTML),
                code: format!("<script>{}</script>", DUMMY_PANVIEW_CODE)
            },
            slides: vec![],
            notas: None,
            custom: None,
            foot: DUMMY_FOOT.to_string()
        }
    }

}
impl fmt::Display for Presentation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::from("<html>");
        let head = &self.head;
        //out = out + head;

        //// TODO: criar regex para output com quebras no HTML
        ////       make a regex for beaultiful output
        //for slide in &self.slides {
        //    let elements : Vec<&Element>= slide.body.iter().collect();
        //    for element in elements {
        //        out.push_str(&format!("{}", element.content));
        //    };
        //}
        //self.slides.iter().map(|slide| out.push_str(&format!("{}", slide.body.iter().map(|element| format!("{}", element)))));
        //out = out + "</html>";
        for slide in &self.slides {
            write!(f, "<html>{}{}</html>", head, slide)?
        }
        Ok(())
    }
}

pub type Result<T> = std::result::Result<T, SimplexError>;

/// The miriad of errors in Simplex Presentation.
enum ErrorNature {
    /// When a Slide is built but error occurs.
    BrokenSlide,
    FaultyLine,
    FaultyTag,
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
impl fmt::Display for SimplexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error_nature = match self.nature {
            ErrorNature::FaultyTag => "FaultyTag",
            _ => todo!()
        };
        write!(f, "Error nature: {}<br>{} na linha {}", error_nature, self.msg, self.loc)
    }
}

struct RawSlide {
    elements: Vec<String>,
    number: i32
}

/// This trait is born because split methods of primitive str 
/// doesn't work.
trait SplitOnTag {
    fn split_on_tag(self) -> Vec<Vec<String>>;
}

impl SplitOnTag for Vec<String> {
    fn split_on_tag(self) -> Vec<Vec<String>> {
        let mut result = Vec::new();
        let mut temp_group = Vec::new();
    
        for s in self {
            if s.starts_with(TAG_MARKER) {
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

fn text (raw_element: Vec<String>) -> Result<Element> {todo!()}
fn video (raw_element: Vec<String>) -> Result<Element> {todo!()}
fn image (raw_element: Vec<String>) -> Result<Element> {todo!()}
fn mermaid (raw_element: Vec<String>) -> Result<Element> {todo!()}

/// Generate an `<ul>` style listing.
fn ulist (raw_element: Vec<String>) -> Result<Element> {
    if !(raw_element[0] == format!("{}{}", TAG_MARKER, TAG_ULIST)) {
        return Err(SimplexError {
            loc: 0, // fix
            msg: "Tried to parse a .list tag, but ulist() receives a different one".to_string(),
            nature: ErrorNature::FaultyTag
        })
    };
    let mut content = "<ul>".to_string();
    for raw_line in &raw_element[1..] {
        content = content + &format!("<li>{}</li>", raw_line);
    };
    Ok(Element {nature: ElementNature::List, content: content + "</ul>", number: 0}) // fix number
}
/// Generate an `<ul>` style listing.
fn ordlist (raw_element: Vec<String>) -> Result<Element> {
    if !(raw_element[0] == format!("{}{}", TAG_MARKER, TAG_ORDLIST)) {
        return Err(SimplexError {
            loc: 0, // fix
            msg: "Tried to parse a .list tag, but ordlist() receives a different one".to_string(),
            nature: ErrorNature::FaultyTag
        })
    };
    let mut content = "<ol>".to_string();
    for raw_line in &raw_element[1..] {
        content = content + &format!("<li>{}</li>", raw_line);
    };
    Ok(Element {nature: ElementNature::List, content: content + "</ol>", number: 0}) // fix number
}


/// The main parser, reads a `Vec<String>` --- mainly from a file input or stdin --- and
/// outputs a Presentations is everything is fine. Also works as a wraper around
/// Presentaton::build.
pub fn simplex_parser(input: Vec<String>) -> Result<Presentation> {
    let tag_olist = format!("{}{}", TAG_MARKER, TAG_ORDLIST);
    let tag_ulist = format!("{}{}", TAG_MARKER, TAG_ULIST);
    let tag_video = format!("{}{}", TAG_MARKER, TAG_VIDEO);
    let tag_image = format!("{}{}", TAG_MARKER, TAG_IMAGE);
    let tag_text = format!("{}{}", TAG_MARKER, TAG_TEXT);
    let tag_mermaid = format!("{}{}", TAG_MARKER, TAG_MERMAID);
    let mut presentation: Presentation = Presentation::new();

    let mut raw_slides: Vec<Vec<String>> = input.split(|raw_slide| raw_slide.starts_with(SEPARATOR))
        .filter(|line| !line.is_empty())
        .filter(|line| line.is_comment())
        .map(|slide| slide.to_vec())
        .collect::<Vec<Vec<String>>>();
    //println!("{:?}\n\n", raw_slides);
    
    let mut slides: Vec<Slide> = vec![];
    for (raw_slide_no, raw_slide) in raw_slides.into_iter().enumerate() {
        //println!("\nSlide no. {}", i);   

        let mut elements: Vec<Element> = vec![];
        for (raw_element_no, raw_element) in raw_slide.split_on_tag().into_iter().enumerate() {
            //print!("Element no {}\n", j);
            //print!("{:?}\n", element);

            elements.push(match &raw_element[0] {
                tag if *tag == tag_ulist => {
                    ulist(raw_element)?
                },
                tag if *tag == tag_olist => {
                    ordlist(raw_element)?
                },
                tag if *tag == tag_text => {
                    text(raw_element)?
                }
                tag if *tag == tag_video => {
                    video(raw_element)?
                },
                tag if *tag == tag_image => {
                    image(raw_element)?
                },
                tag if *tag == tag_mermaid => {
                    mermaid(raw_element)?
                },
                _ => {
                    let error = SimplexError {
                        nature: ErrorNature::FaultyTag,
                        loc: 10,
                        msg: format!("Tag desconhecida")
                    };
                    Err(error)?
                }
            }); 
        }

        slides.push(Slide {body: elements, draft: false, number: raw_slide_no});
        //slides.push(raw_slide); 
    };
    presentation.slides = slides;
    Ok(presentation)
}
