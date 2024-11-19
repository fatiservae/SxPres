//#![allow(unused)]
use {
    core::panic,
    std::{fs::File, 
        fmt,
        fs,
        path::{Path, PathBuf}, 
        io::{self, BufRead, Write}},
    base64,
    clap::Parser
};
pub const COMMENT_MARKER: &str = "#";
pub const DEFAULT_OUTPUT: &str = "html";
pub const TAG_MARKER: &str = ".";
pub const SEPARATOR: &str = "---";
pub const TAG_FOOTER: &str = "footer";
pub const TAG_HEADING: &str = "heading";
pub const TAG_SUBHEADING: &str = "subheading";
pub const TAG_TABLE: &str = "table";
pub const TAG_ULIST: &str = "list";
pub const TAG_ORDLIST: &str = "ordlist";
pub const TAG_TEXT: &str = "text";
pub const TAG_MERMAID: &str = "mermaid";
pub const TAG_MERMAIDSCRIPT: &str = "mermaidscript";
pub const TAG_VIDEO: &str = "video";
pub const TAG_IMAGE: &str = "image";
pub const DRAFT: &str = "draft";

/// `Cli` from `Clap`.
#[derive(Parser)]
pub struct Cli {
    /// Points to a file as input.
    #[arg(short, long)]
    input: PathBuf,

    /// Defines the output file, no extension needed.
    #[arg(short, long)]
    output: Option<PathBuf>,

    //#[arg(short, long, action = clap::ArgAction::SetTrue)]
    //verbose: bool,
}

/// Uses `Clap` to handle the output.
pub fn output(content: HTML, args: Cli) -> io::Result<()>{
    let output_path = match args.output {
        Some(mut output) => {
            output.set_extension(DEFAULT_OUTPUT);
            output
        },
        None => {
            let mut output = args.input.clone();
            output.set_extension(DEFAULT_OUTPUT); 
            output
        }
    };
    let mut file = File::create(output_path)?;
    writeln!(file, "{}", content)?;

    Ok(())
}

/// Treats all the forms of input using `Clap`.
pub fn input(args: &Cli) -> Result<Vec<String>, fmt::Error> {
    if args.input.exists() && args.input.is_file() {
        let file = match File::open(&args.input) {
            Ok(file) => file,
            Err(err) => panic!("Erro ao abrir o arquivo: {}", err),
        };

        let reader = io::BufReader::new(file);
        let lines: Result<Vec<String>, io::Error> = reader.lines()
            .collect();

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

/// Define the nature of the elements. This can help organize the way 
/// multiple elements will be arranged.
enum ElementNature {
    Heading,
    Subheading,
    Text,
    List,
    OrdList,
    Mermaid,
    Video,
    Image
}

/// An slide is built from elements that are rendered to 
/// a `<div class=element>` that respect the SxPres philosophy.
pub struct Element {
    /// Will be used to control Elements combinations.
    nature: ElementNature,
    content: String,
}

pub trait Organize {
    fn organize(self) -> Self;
}
impl Organize for Vec<Element>{
    fn organize(self) -> Self {
        //if self.contains(
        //let title: Option<Element>;
        //for element in self {
        //    match element.nature {
        //        ElementNature::Text => 
        //    }
        //}
    self
    }
}

/// Each fully presentable slide from the entire slideshow. It is 
/// a `<div class=slide>` that will be formatted by `css` to fill
/// the screen and respect the `Javascript` controls.
pub struct Slide {
    //pub number: usize, // NEEDED??
    pub content: Result<Vec<Element>, fmt::Error>,
    pub draft: bool
}

impl fmt::Display for Slide {
    #![allow(unused)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut before = String::new();

        match self.draft {
            false => before = "<div class=\"slide\">".to_string(),
            true => {
                before = "<div class=\"slide, draft-slide\">"
                        .to_string()
            }
        };

        let mut fill = String::new();

        match &self.content {
            Ok(elements) => {
                elements.iter().for_each(|element| 
                    fill.push_str(&format!("{}", element.content))
                );
            },
            Err(_) => () 
        };

        write!(f, "{}{}</div>", before, fill);
        Ok(())
    }
}

/// Recognise a comment line.
pub trait IsComment {
    fn is_comment(&self) -> bool;
}
impl IsComment for String {
    fn is_comment(&self) -> bool {
        if self.starts_with(COMMENT_MARKER) 
        {true} else {false}
    }
}

/// Split a `Vec<String>` right on a tag, grouping them into a new 
/// `Vec<String>`. The result of this process is a `Vec<Vec<String>>`, 
/// a primite form of slide.
///
/// This trait is born because split methods of primitive str 
/// doesn't work.
pub trait SplitOnTag {
    fn split_on_tag(self) -> Vec<Vec<String>>;
}
pub trait CleanTag {
    fn clean_tag(self) -> Self;
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
impl CleanTag for Vec<String> {
    fn clean_tag(mut self) -> Self {
        self[0] = self[0].replace(" ", "");
        self
    }
}

/// Convert external files into raw base64 data to be embedded into 
/// the final `HTML`.
pub fn file_base64(file: String, tipo: &str) -> 
Result<String, fmt::Error> {
    let file_data = fs::read(&file)
        .expect("Media file passed to file_base64() not found.");

    Ok(format!("data:{}/{};base64,{}", 
            tipo, 
            Path::new(&file)
                .extension()
                .expect(&format!(
                    "Error trying to set the filetype of {}", 
                    &file
                ))
                .to_str()
                .ok_or(&format!(
                    "Error converting the path {} to string.", 
                    &file
                ))
                .expect(&format!(
                    "Error trying validate {} as a file path.", 
                    &file
                )),
            base64::encode(&file_data)
    ))
}

/// Do all the checks necessary to validate a `raw_element` as `Element`.
pub fn is_element_ok(raw_element: &Vec<String>, reference: &str) -> 
Result<(), fmt::Error> {
    if raw_element.len() < 2 {
        eprintln!(
            "A tag {} was not followed by its argument.", 
            raw_element[0]
        ); // Improve error msg to point where? 
        Err(fmt::Error)
    } else if !(*raw_element[0] == format!(
            "{}{}", 
            TAG_MARKER, 
            reference
        )) {
        eprintln!("The tag \"{}\" is not valid.", raw_element[0]);
        Err(fmt::Error)
    } else { 
        Ok(())
    }   
}

/// The `<p>` rendering function.
pub fn text (raw_element: Vec<String>) -> 
Result<Element, fmt::Error> {
    is_element_ok(&raw_element, TAG_TEXT)?;
    let mut p = "<div class=\"element\"><p>".to_string();
    p = p + &raw_element[1].to_string();
    for raw_line in &raw_element[2..] {
        p = p + &format!("<br>{}", raw_line);
    };
    Ok(Element {
        nature: ElementNature::Text, 
        content: p + "</p></div>"
        }
    )
}

/// The `<h1>` rendering function.
pub fn heading (raw_element: Vec<String>) -> 
Result<Element, fmt::Error> {
    // Ignores info passed beyond raw_element[1].
    is_element_ok(&raw_element, TAG_HEADING)?;
    let heading = format!("
        <div class=\"element\"><h1>{}</h1></div>", 
        raw_element[1]
    );
    Ok(Element {
        nature: ElementNature::Heading, 
        content: heading
    })
}

/// The `<h2>` rendering function.
pub fn subheading (raw_element: Vec<String>) -> 
Result<Element, fmt::Error> {
    // Ignores info passed beyond raw_element[1].
    is_element_ok(&raw_element, TAG_SUBHEADING)?;
    let subheading = format!("<div class=\"element\"><h2>{}</h2></div>", raw_element[1]);
    Ok(Element {
        nature: ElementNature::Subheading, 
        content: subheading
    })
}

/// The `<video>` rendering function.
pub fn video (raw_element: Vec<String>) -> 
Result<Element, fmt::Error> {
    // Ignores info passed beyond raw_element[1].
    is_element_ok(&raw_element, TAG_VIDEO)?;
    let video_path: String = raw_element[1].clone(); 
    let video = file_base64(video_path, "video");
    let vid_content = format!(
        "<video controls src=\"{}\"></video>", 
        video?
    );
    Ok(Element {nature: ElementNature::Video, content: vid_content}) 
}

/// The `<table>` rendering function.
pub fn table (raw_element: Vec<String>) -> 
Result<Element, fmt::Error> {
    is_element_ok(&raw_element, TAG_TABLE)?;
    let mut table = String::from("<div class=\"element\"><table>");
    table += &format!(
        "<thead><tr><th>{}</th></tr></thead>", 
        raw_element[1].replace("|", "</th><th>")
    );
    for line in &raw_element[2..] {
        table += &format!(
            "<tbody><tr><td>{}</td></tr></tbody>", 
            line.replace("|", "</td><td>")
        );
    };
    table += "</table></div>";
    Ok(Element {nature: ElementNature::Video, content: table}) 
}

/// Defines a foot message.
pub fn footer(raw_element: Vec<String>) -> 
Result<String, fmt::Error> {
    is_element_ok(&raw_element, TAG_FOOTER)?;
    Ok(format!("<footer>{}</footer>", raw_element[1]))
}

/// The `<image>` rendering function.
pub fn image (raw_element: Vec<String>) -> 
Result<Element, fmt::Error> {
    is_element_ok(&raw_element, TAG_IMAGE)?;
    let image_path: String = raw_element[1].clone(); 
    let image = file_base64(image_path, "image");
    let content = format!("<img src=\"{}\">", image?);
    let captions = format!(
        "<figcaption>{}</figcaption>", 
        raw_element[2..].join("<br>")
    );
    Ok(Element {
        nature: ElementNature::Image, 
        content: content + &captions + "</img>"
        }
    )
}

/// The `mermaid` element rendering function.
pub fn mermaid (raw_element: Vec<String>) -> 
Result<Element, fmt::Error> {
    is_element_ok(&raw_element, TAG_MERMAID)?;
    let mut content = String::new();
    for raw_line in &raw_element[1..] {
        content = content + &format!("{}", raw_line);
    };
    Ok(Element {
        nature: ElementNature::Mermaid, 
        content: 
            "<div class=\"element\"><pre class=\"mermaid\">".to_owned()
            + &content + "</pre></div>"
        }
    )
}

/// The `<ul>` rendering function.
pub fn ulist (raw_element: Vec<String>) -> 
Result<Element, fmt::Error> {
    is_element_ok(&raw_element, TAG_ULIST)?;
    let mut content = "<div class=\"element\"><ul>".to_string();
    for raw_line in &raw_element[1..] {
        content = content + &format!("<li>{}</li>", raw_line);
    };
    Ok(Element {
        nature: ElementNature::List, 
        content: content + "</ul></div>", 
        }
    )
}

/// Generate an `<ol>` style listing.
pub fn ordlist (raw_element: Vec<String>) -> 
Result<Element, fmt::Error> {
    is_element_ok(&raw_element, TAG_ORDLIST)?;
    let mut content = "<div class=\"element\"><ol>".to_string();
    for raw_line in &raw_element[1..] {
        content = content + &format!("<li>{}</li>", raw_line);
    };
    Ok(Element {
        nature: ElementNature::OrdList, 
        content: content + "</ol></div>", 
        }
    )
}

/// The final HTML.
pub struct HTML(pub String);
impl fmt::Display for HTML {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

/// Finally condense back a `Vec<Slide>` into `HTML` that can be 
/// printed or outputed.
pub fn render (
footer: Result<String, fmt::Error>, mermaid: bool, slides: Vec<Slide>) -> 
Result<HTML, fmt::Error> {
    let mut html: String = String::new();
    if slides.len() < 1 {
        panic!("Zero slides built.");
    }else{
        for slide in slides {
            html = html + &format!("{}", slide);
        }
    };

    let mut mermaid_script = String::new();
    // TODO: Wrap mermaid_file in Option<String>.
    // Mermaid is cumbersome to integrate, since the only way to inject
    // the script to a page is to import, because the script calls for 
    // other ones.
    if mermaid {
        mermaid_script = "<script type=\"module\">import mermaid from 'https://cdn.jsdelivr.net/npm/mermaid@11/dist/mermaid.esm.min.mjs';mermaid.initialize({ startOnLoad: true });</script>".to_string();
    };

    let foot = match footer {
        Ok(foot) =>  foot,
        _ => "".to_string()
    };

    Ok(HTML(
        "<!DOCTYPE html>\n<html>\n<head>\n".to_owned() 
        + &mermaid_script
        + &foot
        + "<div id=\"marcador\"></div>"
        + "<div id=\"popup\"><p><span id=\"conteudo-popup\"></span></p></div>" 
        + "</head>\n<body>\n"
        + &html 
        + "\n</body>\n<script src=\"./script.js\"></script>\n<link rel=\"stylesheet\" href=\"./style.css\">\n</html>"
    )) 
}

//    This file is part of StultusVisio.
//
//    StultusVisio is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, either version 3 of the License, or
//    (at your option) any later version.
//
//    StultusVisio is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    You should have received a copy of the GNU General Public License
//    along with StultusVisio.  If not, see <https://www.gnu.org/licenses/>.
