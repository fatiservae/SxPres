// See licence at the end.
//
// By Jefferson T.
// https://jeffersontorres.com.br

//#![allow(unused)]
use {
    base64,
    clap::Parser,
    core::panic,
    std::{
        fmt, fs,
        fs::File,
        io::{self, BufRead, Write},
        path::{Path, PathBuf},
    },
};
pub const COMMENT_MARKER: &str = "#";
pub const STD_OUTPUT_FMT: &str = "html";
pub const TAG_MARKER: &str = ".";
pub const SEPARATOR: &str = "---";
pub const TAG_FOOTER: &str = "footer";
pub const TAG_LOGO: &str = "logo";
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
pub const TAG_DRAFT: &str = "draft";

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
pub fn output(content: HTML, args: Cli) -> io::Result<()> {
    let output_path = match args.output {
        Some(mut output) => {
            output.set_extension(STD_OUTPUT_FMT);
            output
        }
        None => {
            let mut output = args.input.clone();
            output.set_extension(STD_OUTPUT_FMT);
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
            Err(err) => panic!("Can't open file {}", err),
        };

        let reader = io::BufReader::new(file);
        let lines: Result<Vec<String>, io::Error> = reader.lines().collect();

        match lines {
            Ok(lines) => Ok(lines),
            Err(err) => {
                eprintln!("Error reading lines {}", err);
                Err(fmt::Error)
            }
        }
    } else {
        panic!("Error: no arguments passed.")
    }
}

/// Define the nature of the elements. This can help organize the way
/// multiple elements will be arranged. The order of declaration matters
/// since that organize() uses sort_by_key() method.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ElementNature {
    Heading,
    Subheading,
    Text,
    OrdList,
    List,
    Video,
    Image,
    Mermaid,
}
impl fmt::Display for ElementNature {
    #![allow(unused)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\"{}\"",
            match self {
                Self::Heading => "heading",
                Self::Text => "text",
                Self::List => "list",
                Self::OrdList => "ordered list",
                Self::Image => "image",
                Self::Mermaid => "mermaid",
                Self::Subheading => "subheading",
                _ => "unknown",
            }
        );
        Ok(())
    }
}

/// An slide is built from elements that are rendered to
/// a `<div class=element>` that respect the SxPres philosophy.
pub struct Element {
    /// Will be used to control Elements combinations.
    pub nature: ElementNature,
    pub content: String,
}

impl fmt::Display for Element {
    #![allow(unused)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let nature = match self.nature {
            ElementNature::Heading => "heading",
            _ => "unknown",
        };

        write!(
            f,
            "content: \"{}\"... of nature:\"{}\"",
            self.content[..20].to_string(),
            nature
        );
        Ok(())
    }
}

pub trait Organize {
    fn organize(self) -> Self;
}
impl Organize for Vec<Element> {
    /// For now, organize() just separates the `headings` and the
    /// `subheadings` in different `<div class=elementGroup>` so
    /// that in the `CSS` configuration it can be formatted separately.
    fn organize(mut self) -> Self {
        if self.len() < 1 {
            self
        } else {
            // Cloning here is ok since Element::Nature is a
            // unit type enum.
            self.sort_by_key(|e| e.nature.clone());
            self
        }
    }
}

/// Each fully presentable slide from the entire slideshow. It is
/// a `<div class=slide>` that will be formatted by `CSS` to fill
/// the screen and respect the `Javascript` controls.
pub struct Slide {
    //pub number: usize, // NEEDED??
    pub content: Result<Vec<Element>, fmt::Error>,
    pub draft: bool,
}

impl fmt::Display for Slide {
    #![allow(unused)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut before = String::new();

        match self.draft {
            false => before = "<div class=\"slide\">".to_string(),
            true => before = "<div class=\"slide, draft-slide\">".to_string(),
        };

        let mut fill = String::new();

        match &self.content {
            Ok(elements) => {
                elements
                    .iter()
                    .for_each(|element| fill.push_str(&format!("{}", element.content)));
            }
            Err(_) => (),
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
        if self.starts_with(COMMENT_MARKER) {
            true
        } else {
            false
        }
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

pub trait CleanTag {
    fn clean_tag(self) -> Self;
}
impl CleanTag for Vec<String> {
    fn clean_tag(mut self) -> Self {
        self[0] = self[0].replace(" ", "");
        self
    }
}

/// Convert external files into raw base64 data to be embedded into
/// the final `HTML`.
pub fn file_base64(file: String, tipo: &str) -> Result<String, fmt::Error> {
    let file_data = fs::read(&file).expect("Media file passed to file_base64() not found.");

    Ok(format!(
        "data:{}/{};base64,{}",
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
pub fn is_element_ok(raw_element: &Vec<String>, reference: &str) -> Result<(), fmt::Error> {
    if raw_element.len() < 2 {
        eprintln!("A tag {} was not followed by its argument.", raw_element[0]); // Improve error msg to point where?
        Err(fmt::Error)
    } else if !(*raw_element[0] == format!("{}{}", TAG_MARKER, reference)) {
        eprintln!("The tag \"{}\" is not valid.", raw_element[0]);
        Err(fmt::Error)
    } else {
        Ok(())
    }
}

/// The `<p>` rendering function.
pub fn text(raw_element: Vec<String>) -> Result<Element, fmt::Error> {
    is_element_ok(&raw_element, TAG_TEXT)?;
    let mut p = "<div class=\"element\"><p>".to_string();
    p = p + &raw_element[1].to_string();
    for raw_line in &raw_element[2..] {
        p = p + &format!("<br>{}", raw_line);
    }
    Ok(Element {
        nature: ElementNature::Text,
        content: p + "</p></div>",
    })
}

/// The `<h1>` rendering function.
pub fn heading(raw_element: Vec<String>) -> Result<Element, fmt::Error> {
    // Ignores info passed beyond raw_element[1].
    is_element_ok(&raw_element, TAG_HEADING)?;
    let heading = format!(
        "
        <div class=\"element\"><h1>{}</h1></div>",
        raw_element[1]
    );
    Ok(Element {
        nature: ElementNature::Heading,
        content: heading,
    })
}

/// The `<h2>` rendering function.
pub fn subheading(raw_element: Vec<String>) -> Result<Element, fmt::Error> {
    // Ignores info passed beyond raw_element[1].
    is_element_ok(&raw_element, TAG_SUBHEADING)?;
    let subheading = format!("<div class=\"element\"><h2>{}</h2></div>", raw_element[1]);
    Ok(Element {
        nature: ElementNature::Subheading,
        content: subheading,
    })
}

/// The `<video>` rendering function.
pub fn video(raw_element: Vec<String>) -> Result<Element, fmt::Error> {
    // Ignores info passed beyond raw_element[1].
    is_element_ok(&raw_element, TAG_VIDEO)?;
    let video_path: String = raw_element[1].clone();
    let video = file_base64(video_path, "video");
    let vid_content = format!("<video controls src=\"{}\"></video>", video?);
    Ok(Element {
        nature: ElementNature::Video,
        content: vid_content,
    })
}

/// The `<table>` rendering function.
pub fn table(raw_element: Vec<String>) -> Result<Element, fmt::Error> {
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
    }
    table += "</table></div>";
    Ok(Element {
        nature: ElementNature::Video,
        content: table,
    })
}

/// Render a `<footer>` element with a foot message on all slides.
pub fn footer(raw_element: Vec<String>) -> Result<String, fmt::Error> {
    is_element_ok(&raw_element, TAG_FOOTER)?;
    Ok(format!("<footer>{}</footer>", raw_element[1]))
}

/// Render a `<image class=logo>` that is treated in `CSS` as
/// a fixed right top logomark to the slides.
///
/// TODO: 1) Make the option to change the logo from a new definition
/// onwards; and 2) Make the option to choose the position.
pub fn logo(raw_element: Vec<String>) -> Result<String, fmt::Error> {
    is_element_ok(&raw_element, TAG_LOGO)?;
    let image = file_base64(raw_element[1].clone(), "image");
    Ok(format!("<img class=\"logo\" src=\"{}\"></img>", image?))
}

/// The `<image>` rendering function.
/// Captions of figures are buided from the third line fowards
/// on the raw_element.
pub fn image(raw_element: Vec<String>) -> Result<Element, fmt::Error> {
    is_element_ok(&raw_element, TAG_IMAGE)?;
    let image_path: String = raw_element[1].clone();
    let image = file_base64(image_path, "image");
    let mut _content = format!("<div class=\"element\"><img src=\"{}\">", image?);

    // To treat captions...
    if raw_element.len() > 2 {
        let mut captions = String::from("<figcaption>");
        captions = captions + &format!("{}", raw_element[2]);
        for line in &raw_element[3..] {
            captions = captions + &format!("<br>{}", line);
        }
        _content = _content + &captions + "</figcaption></img></div>";
    } else {
        _content = _content + "</img></div>";
    };

    Ok(Element {
        nature: ElementNature::Image,
        content: _content,
    })
}

/// The `<pre class=mermaid>` element rendering function. Resolves a mermaid
/// diagram passed line-by-line.
pub fn mermaid(raw_element: Vec<String>) -> Result<Element, fmt::Error> {
    is_element_ok(&raw_element, TAG_MERMAID)?;
    let mut content = String::new();
    for raw_line in &raw_element[1..] {
        content = content + &format!("{}", raw_line);
    }
    Ok(Element {
        nature: ElementNature::Mermaid,
        content: format!(
            "<div class=\"element\"><pre class=\"mermaid\">{}</pre></div>",
            &content
        ),
    })
}

/// The `<ul>` rendering function.
pub fn ulist(raw_element: Vec<String>) -> Result<Element, fmt::Error> {
    is_element_ok(&raw_element, TAG_ULIST)?;
    let mut out = String::from("<div class=\"element\"><ul>");
    for raw_line in &raw_element[1..] {
        out = out + &format!("<li>{}</li>", raw_line);
    }
    Ok(Element {
        nature: ElementNature::List,
        content: out + "</ul></div>",
    })
}

/// Generate an `<ol>` style listing.
pub fn ordlist(raw_element: Vec<String>) -> Result<Element, fmt::Error> {
    is_element_ok(&raw_element, TAG_ORDLIST)?;
    let mut out = "<div class=\"element\"><ol>".to_string();
    for raw_line in &raw_element[1..] {
        out = out + &format!("<li>{}</li>", raw_line);
    }
    Ok(Element {
        nature: ElementNature::OrdList,
        content: out + "</ol></div>",
    })
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
pub fn render(
    logo: Result<String, fmt::Error>,
    footer: Result<String, fmt::Error>,
    mermaid: bool,
    slides: Vec<Slide>,
) -> Result<HTML, fmt::Error> {
    let mut body: String = String::from("<body>");
    if slides.len() < 1 {
        panic!("Zero slides built.");
    } else {
        for slide in slides {
            body = body + &format!("{}", slide);
        }
    };

    body = body + "</body>";

    let mut mermaid_script = String::new();
    // TODO: Wrap mermaid_file in Option<String>.
    //
    // Mermaid is cumbersome to integrate, since the only way to inject
    // the script to a page is to import, because the script calls for
    // other ones in the mermaid remote server, thus turning the job too
    // error prone.
    if mermaid {
        mermaid_script = "<script type=\"module\">import mermaid from 'https://cdn.jsdelivr.net/npm/mermaid@11/dist/mermaid.esm.min.mjs';mermaid.initialize({ startOnLoad: true });</script>".to_string();
    };

    let foot = match footer {
        Ok(foot) => foot,
        _ => "".to_string(),
    };

    let logo_img = match logo {
        Ok(logo) => logo,
        _ => "".to_string(),
    };

    let script = "<script>".to_owned()
        + &String::from_utf8(include_bytes!("./script.js").to_vec())
            .expect("Can't include \'script.js\' during compilation.")
        + "</script>";

    let css = "<style>".to_owned()
        + &String::from_utf8(include_bytes!("./style.css").to_vec())
            .expect("Can't include \'style.css\' during compilation.")
        + "</style>";

    Ok(HTML(String::from(format!(
        "<!DOCTYPE html>\n
                    <html>\n
                    <head>\n
                    {}{}{}{}
                    <div id=\"marcador\"></div>
                    <div id=\"popup\">
                        <p><span id=\"conteudo-popup\"></span></p>
                    </div></head>\n
                    {}{}
                    </html>",
        &mermaid_script,
        &css,
        &foot,
        &logo_img,
        &body,
        // `&script` has to be inserted at the end, so that only with
        // the whole page built it can calls to document.ElementById's
        // methods in the /src/script.js.
        &script
    ))))
}

//    This file is part of StultusVisio.
//
//    StultusVisio is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, either version 3 of the License, or
//    any later version.
//
//    StultusVisio is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    You should have received a copy of the GNU General Public License
//    along with StultusVisio.  If not, see <https://www.gnu.org/licenses/>.
