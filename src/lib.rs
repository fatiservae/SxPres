//#![allow(unused)]
use {
    core::panic,
    std::{fs::File, 
        fmt,
        fs,
        path::{Path, PathBuf}, 
        io::{self, BufRead}},
    base64,
    clap::Parser
};

const SEPARATOR: &str = "---";
const TAG_MARKER: &str = ".";
const TAG_FOOTER: &str = "footer";
const TAG_ULIST: &str = "list";
const TAG_ORDLIST: &str = "ordlist";
const TAG_TEXT: &str = "text";
const TAG_MERMAID: &str = "mermaid";
const TAG_MERMAIDSCRIPT: &str = "mermaidscript";
const TAG_VIDEO: &str = "video";
const TAG_IMAGE: &str = "image";
const COMMENT_MARKER: &str = "//";
const DRAFT: &str = "draft";

#[derive(Parser)]
pub struct Cli {
    #[arg(short, long)]
    input: PathBuf,

    //#[arg(short, long, action = clap::ArgAction::SetTrue)]
    //verbose: bool,
}

pub fn input() -> Result<Vec<String>, fmt::Error> {
    let args = Cli::parse();

    //if args.verbose {
    //    println!("Modo verbose ativado!");
    //}

    if args.input.exists() && args.input.is_file() {
        let file = match File::open(&args.input) {
            Ok(file) => file,
            Err(err) => panic!("Erro ao abrir o arquivo: {}", err),
        };

        let reader = io::BufReader::new(file);
        let lines: Result<Vec<String>, io::Error> = reader.lines().collect();

        match lines {
            Ok(lines) => Ok(lines),
            Err(err) => {
                eprintln!("Erro ao ler linhas: {}", err);
                Err(fmt::Error) 
            }
        }
    } else {
        panic!("error")
    }
}

enum ElementNature {
    Empty,
    Text,
    List,
    OrdList,
    Mermaid,
    Video,
    //Image
}

/// An slide is built from elements that are rendered to 
/// a `<div class=element>` that respect the SxPres philosophy.
struct Element {
    nature: ElementNature,
    content: String,
    number: usize 
}

impl Element {
    fn empty() -> Element {
        Element {
           content: "".to_string(),
           nature: ElementNature::Empty,
           number: 0 // fix
        }
    }
}

/// Each fully presentable slide from the entire slideshow. It is 
/// a `<div class=slide>` that will be formatted by `css` to fill
/// the screen and respect the `Javascript` controls.
struct Slide {
    number: usize, // NEEDED??
    content: Result<Vec<Element>, fmt::Error>,
    footer: String, // if carries a foot. Can be disable to remove footing on videos, e.g.
    draft: bool,
    mermaid: bool
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
    #![allow(unused)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // To handle foot disabling when playing videos, is a best approach
        // to use JS.
        //match self.footer {
        //;
        
        let mut before = String::new();

        match self.draft {
            false => before = "<div class=\"slide\">".to_string(),
            true => before = "<div class=\"slide, draft-slide\">".to_string(),
        };

        let mut fill = String::new();

        match &self.content {
            Ok(elements) => {
                elements.iter().for_each(|element| fill.push_str(&format!("{}", element.content)));
            },
            Err(_) => () 
        };

        write!(f, "{}{}</div>", before, fill);
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
        // Add last group
        if !temp_group.is_empty() {
            result.push(temp_group);
        }
    result
    }
}

pub fn file_base64(file: String, tipo: &str) -> Result<String, fmt::Error> {
    let file_data = fs::read(&file)
        .expect("Media file passed to file_base64() not found.");

    Ok(format!("data:{}/{};base64,{}", 
            tipo, 
            Path::new(&file)
                .extension()
                .expect(&format!("Error trying to set the filetype of {}", &file))
                .to_str()
                .ok_or(&format!("Error converting the path {} to string.", &file))
                .expect(&format!("Error trying validate {} as a file path.", &file)),
            base64::encode(&file_data)
    ))
}

/// Do all the checks necessary to validate a `raw_element` as `Element`.
fn is_element_ok(raw_element: &Vec<String>, reference: &str) -> Result<(), fmt::Error> {
    if raw_element.len() < 2 {
        eprintln!("A tag {} was not followed by its argument.", raw_element[0]); // Improve error msg
        Err(fmt::Error)
    } else if !(*raw_element[0] == format!("{}{}", TAG_MARKER, reference)) {
        eprintln!("The tag \"{}\" is not valid.", raw_element[0]);
        Err(fmt::Error)
    } else { 
        Ok(())
    }   
}

// Deprecated?
///// Make sure that the tag passed to any tag function corresponds to expectations.
//fn check_tag (tag: &String, reference: &str) -> Result<(), fmt::Error> {
//    if !(*tag == format!("{}{}", TAG_MARKER, reference)) {
//        Err(fmt::Error)
//    } else { 
//        Ok(())
//    }
//}

/// The `<p>` rendering function.
fn text (raw_element: Vec<String>) -> Result<Element, fmt::Error> {
    is_element_ok(&raw_element, TAG_TEXT)?;
    let mut content = "<p>".to_string();
    for raw_line in &raw_element[1..] {
        content = content + &format!("<br>{}", raw_line);
    };
    Ok(Element {nature: ElementNature::Text, content: content + "</p>", number: 0}) // fix number
}

/// The `<video>` rendering function.
fn video (raw_element: Vec<String>) -> Result<Element, fmt::Error> {
    // Ignores info passed beyond raw_element[1].
    is_element_ok(&raw_element, TAG_VIDEO)?;
    let video_path: String = raw_element[1].clone(); // it's ok to clone here, too little
    //println!("{}", video_path);
    let video = file_base64(video_path, "video");
    let vid_content = format!("<video controls src=\"{}\"></video>", video?);
    Ok(Element {nature: ElementNature::Video, content: vid_content, number: 0}) // fix number
}

fn footer(raw_element: Vec<String>) -> Result<String, fmt::Error> {
    is_element_ok(&raw_element, TAG_FOOTER)?;
    Ok(raw_element[1].clone()) //ok to clone, just a text
}

/// The `<image>` rendering function.
fn image (raw_element: Vec<String>) -> Result<Element, fmt::Error> {
    is_element_ok(&raw_element, TAG_IMAGE)?;
    let image_path: String = raw_element[1].clone(); // it's ok to clone here, too little
    let image = file_base64(image_path, "image");
    let content = format!("<img src=\"{}\">", image?);
    let captions = format!("<figcaption>{}</figcaption>", raw_element[2..].join("<br>"));
    Ok(Element {nature: ElementNature::Video, content: content + &captions + "</img>", number: 0}) // fix number
}

/// The `<div class=mermaid>` rendering function.
fn mermaid (raw_element: Vec<String>) -> Result<Element, fmt::Error> {
    is_element_ok(&raw_element, TAG_MERMAID)?;
    let mut content = "<div class=\"center\">".to_string();
    for raw_line in &raw_element[1..] {
        content = content + &format!("{}", raw_line);
    };
    Ok(Element {nature: ElementNature::Mermaid, content: content + "<pre class=\"mermaid\">", number: 0}) // fix number
}

/// The `<ul>` rendering function.
fn ulist (raw_element: Vec<String>) -> Result<Element, fmt::Error> {
    is_element_ok(&raw_element, TAG_ULIST)?;
    let mut content = "<ul>".to_string();
    for raw_line in &raw_element[1..] {
        content = content + &format!("<li>{}</li>", raw_line);
    };
    Ok(Element {nature: ElementNature::List, content: content + "</ul>", number: 0}) // fix number
}

/// Generate an `<ol>` style listing.
fn ordlist (raw_element: Vec<String>) -> Result<Element, fmt::Error> {
    is_element_ok(&raw_element, TAG_ORDLIST)?;
    let mut content = "<ol>".to_string();
    for raw_line in &raw_element[1..] {
        content = content + &format!("<li>{}</li>", raw_line);
    };
    Ok(Element {nature: ElementNature::OrdList, content: content + "</ol>", number: 0}) // fix number
}

/// The final HTML.
pub struct HTML(pub String);
impl fmt::Display for HTML {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

/// The main parser, walks into a `Vec<String>` --- mainly from a file input 
/// or stdin --- and outputs a complete presentation if everything is fine. 
fn turing_machine (input: Vec<String>) -> Result<Vec<Slide>, fmt::Error> {
    // has a ordering system to make a side by side if two 
    // elements, a pyramid if three or a grid if four
    // To group and priorytize titles and subtitles
    let raw_slides: Vec<Vec<String>> = input
        .split(|raw_slide| raw_slide.starts_with(SEPARATOR))
        .filter(|line| !line.is_empty() || !line.is_comment())
        .map(|slide| slide.to_vec())
        .collect::<Vec<Vec<String>>>();

    // Split applied before creates a first Vec<String> empty.
    //raw_slides.remove(0);
    
    //raw_slides.remove_invalids();

    Ok(build(raw_slides))?
}

/// Finally condense back a `Vec<Slide>` into `HTML` that is capable of being
/// printed or outputed.
fn render (slides: Vec<Slide>) -> Result<HTML, fmt::Error> {
    let mut html: String = String::new();
    if slides.len() < 1 {
        panic!("Zero slides built.");
    }else{
        for slide in slides {
            html = html + &format!("{}", slide);
        }
    };

    // A global variable is needed to avoid generate duplicates of mermaid script
    let mermaid_file = String::from_utf8(include_bytes!("../example/mermaid-00886c59.js").to_vec()).expect("Não foi possível integrar o script mermaid");

    Ok(HTML("<!DOCTYPE html><html><script type=\"module\">".to_string() + &mermaid_file + ";Tt.initialize({{ startOnLoad: true }});</script><body><div id=\"marcador\"></div> <div id=\"popup\"><p><span id=\"conteudo-popup\"></span></p></div>" + &html + "</body><script src=\"./script.js\"></script><link rel=\"stylesheet\" href=\"./style.css\"></html>")) 
}

/// Translate raw strings information into structured `Element` and `Slide` data.
fn build (raw_slides: Vec<Vec<String>>) -> Result<Vec<Slide>, fmt::Error> {
    let mut slides: Vec<Slide> = vec![];
    
    for raw_slide in raw_slides {
        let mut elements: Vec<Element> = vec![];
        let mut draft_sld = false;
        let mut foot: String = String::new();
        let mut mermaid_script = (String::new(), false);
        let mut has_mermaid = false;

        // a fuction has to be called here to return 
        // elements with nature
        //for (_, raw_element) in raw_slide.split_on_tag().into_iter().enumerate() {
        for raw_element in raw_slide.split_on_tag() {
            elements.push(match &raw_element[0][1..] { // [1..] to skip tag marker
                tag if tag == TAG_ULIST => {
                    ulist(raw_element)?
                },
                tag if tag == TAG_ORDLIST => {
                    ordlist(raw_element)?
                },
                tag if tag == TAG_TEXT => {
                    text(raw_element)?
                }
                tag if tag == TAG_VIDEO => {
                    video(raw_element)?
                },
                tag if tag == TAG_IMAGE => {
                    image(raw_element)?
                },
                tag if tag == TAG_MERMAID => {
                    has_mermaid = true;
                    mermaid(raw_element)?
                },
                tag if tag == TAG_MERMAIDSCRIPT => {
                    Element::empty()                
                },
                tag if tag == TAG_FOOTER => {
                    foot = match footer(raw_element) {
                        Ok(footer) => format!("<footer>{}</footer>", footer),
                        Err(err) => err.to_string() 
                    };
                    Element::empty()                
                },
                tag if tag == DRAFT => {
                    draft_sld = true;
                    Element::empty()                
                }
                _ => {panic!("Unrecognised tag \"{}\".", &raw_element[0])}
            }); 
        };
        slides.push(Slide {draft: draft_sld, number: 1, content: Ok(elements), footer: foot, mermaid: has_mermaid})
    };
    Ok(slides)
}

/// The public exposed API that receives a raw `Vec<String>` and returns an `HTML`.
pub fn sx_parser (input: Vec<String>) -> Result<HTML, fmt::Error> {
    Ok(render(turing_machine(input)?))?
}
