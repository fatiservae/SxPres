#![allow(unused)]
use std::{fmt};

const DUMMY_LIST1 : &str = "This is a dummy list item to Simplex Presentation.";
const DUMMY_LIST2: &str = "This is another dummy list item to Simplex Presentation.";
//const DUMMY_FOOT: &str = "This is a dummy foot to Simplex Presentation.";
//const DUMMY_PANVIEW_HTML: &str = "This is a dummy of panview HTML to Simplex Presentation.";
//const DUMMY_PANVIEW_CODE: &str = "This is a dummy of panview Code to Simplex Presentation.";
const SEPARATOR: &str = "---";
const TAG_MARKER: &str = ".";
const TAG_ULIST: &str = "list";
const TAG_ORDLIST: &str = "ordlist";
const TAG_TEXT: &str = "text";
const TAG_MERMAID: &str = "mermaid";
const TAG_VIDEO: &str = "video";
const TAG_IMAGE: &str = "image";
const COMMENT_MARKER: &str = "//";

enum ElementNature {
    //Unknow,
    Text,
    List,
    OrdList,
    Mermaid,
    Video,
    Image
}

/// An slide is built from elements that are rendered to 
/// a `<div class=element>` that respect the SxPres philosophy.
struct Element {
    nature: ElementNature,
    content: String,
    number: usize 
}

/// Each fully presentable slide from the entire slideshow. It is 
/// a `<div class=slide>` that will be formatted by `css` to fill
/// the screen and respect the `Javascript` controls.
struct Slide {
    number: usize, // NEEDED??
    content: Result<Vec<Element>>,
    foot: bool, // if carries a foot. Can be disable to remove footing on videos, e.g.
    draft: bool
}
//impl Slide {
//    /// Turns an entire Slide into a printable string.
//    fn flat(self) -> String {
//    let mut content = String::new();
//        for (i, element) in self.content.into_iter().enumerate() {
//            content.push_str(&format!("{}", element[i].content));
//        }
//    content
//    }
//
//    /// Create a new dummy slide.
//    pub fn new() -> Slide { 
//        Slide {
//            number: 1,
//            content: Ok(vec![Element {
//                nature: ElementNature::Text,
//                content: format!("<ul><li>{}</li><li>{}</li></ul>", DUMMY_LIST1, DUMMY_LIST2),
//                number: 0
//            }]),
//            foot: true,
//            draft: false
//        }
//    }
//}
impl fmt::Display for Slide {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        match &self.content {
            Ok(elements) => {
                elements.iter().for_each(|element| out.push_str(&format!("{}", element.content)));
            },
            Err(_) => () 
        };
        //for elements in &self.content {
        //    for element in elements {
        //        out.push_str(&format!("{}", element.content));
        //    }
        //};
        let _ = match self.draft {
            false => write!(f, "<div class=slide>{}</div>", out),
            true => write!(f, "<!-- a draft slide -->\n<div class=slide></div>")
        };
    Ok(())
    }
}

/// Recognise a comment line.
trait IsComment {
    fn is_comment(self) -> bool;
}
impl IsComment for String {
    fn is_comment(self) -> bool {
        if self.starts_with(COMMENT_MARKER) && self.len() > 0
        {true} else {false}
    }
}
impl IsComment for &&[String] {
    fn is_comment(self) -> bool {
        if (*(*self)).starts_with(&[COMMENT_MARKER.to_string()]) {true} else {false}
    }
}

/// Redefine `std::result::Result<T, Err>` to `SimplexError`.
pub type Result<T> = std::result::Result<T, SimplexError>;

/// The miriad of errors in Simplex Presentation.
enum ErrorNature {
    /// When a Slide is built but error occurs.
    FaultyTag,
    /// When a Slide is left empty and for some reason tried to be rendered.
    EmptySlide,
    FailedFunction,
    //BrokenSlide,
    //FaultyLine,
    //BrokenFoot,
    //BrokenBody,
    //NoCode,
    //EmptyTag
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

/// Split a `Vec<String>` right on a tag, grouping them into a new `Vec<String>`. 
/// The result of this process is a `Vec<Vec<String>>`, a primite form of slide.
///
/// This trait is born because split methods of primitive str doesn't work.
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

/// The `<p>` rendering function.
fn text (raw_element: Vec<String>) -> Result<Element> {todo!()}

/// The `<video>` rendering function.
fn video (raw_element: Vec<String>) -> Result<Element> {todo!()}

/// The `<image>` rendering function.
fn image (raw_element: Vec<String>) -> Result<Element> {todo!()}

/// The `<div class=mermaid>` rendering function.
fn mermaid (raw_element: Vec<String>) -> Result<Element> {todo!()}

/// The `<ul>` rendering function.
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
/// Generate an `<ol>` style listing.
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

/// The final HTML.
pub struct HTML(pub String);
impl fmt::Display for HTML {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

/// The main parser, walks into a `Vec<String>` --- mainly from a file input or stdin --- and
/// outputs a complete presentation if everything is fine. 
fn touring_machine(input: Vec<String>) -> Result<Vec<Slide>> {
    // has a ordering system to make a side by side if two 
    // elements, a pyramid if three or a grid if four
    // To group and priorytize titles and subtitles
    let raw_slides: Vec<Vec<String>> = input
        .split(|raw_slide| raw_slide.starts_with(SEPARATOR))
        .filter(|line| !line.is_empty() || !line.is_comment())
        .map(|slide| slide.to_vec())
        .collect::<Vec<Vec<String>>>();

    println!("{}", raw_slides.len());
    Ok(build(raw_slides))?
}

/// Finally condense back a `Vec<Slide>` into `HTML` that is capable of being
/// printed or outputed.
fn render(slides: Vec<Slide>) -> Result<HTML> {
    let mut html: String = String::new();
    if slides.len() < 1 {
        return Err(SimplexError {
            msg: "Empty Vec<Slide> passed to render()".to_string(),
            loc: 0,
            nature: ErrorNature::EmptySlide
        })
    }else{
        for slide in slides {
            html = html + &format!("{}", slide);
        }
    };
    Ok(HTML(html))
    //} else {
    //    return Err(SimplexError {
    //        msg: "The HTML generated was possible empty of String, so render() fails.".to_string(),
    //        loc: 0,
    //        nature: ErrorNature::FailedFunction
    //    })
    //}
}

/// Translate raw strings information into structured `Element` and `Slide` data.
fn build(raw_slides: Vec<Vec<String>>) -> Result<Vec<Slide>> {
    // Comparation tags.
    let tag_olist = format!("{}{}", TAG_MARKER, TAG_ORDLIST);
    let tag_ulist = format!("{}{}", TAG_MARKER, TAG_ULIST);
    let tag_video = format!("{}{}", TAG_MARKER, TAG_VIDEO);
    let tag_image = format!("{}{}", TAG_MARKER, TAG_IMAGE);
    let tag_text = format!("{}{}", TAG_MARKER, TAG_TEXT);
    let tag_mermaid = format!("{}{}", TAG_MARKER, TAG_MERMAID);

    let mut slides: Vec<Slide> = vec![];
    
    for raw_slide in raw_slides {
        let mut elements: Vec<Element> = vec![];

        // a fuction has to be called here to return 
        // elements with nature
        //for (_, raw_element) in raw_slide.split_on_tag().into_iter().enumerate() {
        for raw_element in raw_slide.split_on_tag() {
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
        };
        slides.push(Slide {draft: false, number: 1, content: Ok(elements), foot: true})
    };
    Ok(slides)
}

// The public exposed API that receives a raw `Vec<String>` and returns an `HTML`.
pub fn sx_parser(input: Vec<String>) -> Result<HTML> {
    Ok(render(touring_machine(input)?))?
}
