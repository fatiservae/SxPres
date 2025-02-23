// See licence at the end.
//
// By Jefferson T.
// https://jeffersontorres.com.br

use sxpres::*;
use std::fmt::Error;
use clap::Parser;

fn main() -> Result<(), Error>{

    let args = Cli::parse();
    let input: Vec<String> = input(&args)?;

    // A primitive form of slides, that will be translate 
    // into structured `Element` and `Slide` data.
    let raw_slides: Vec<Vec<String>> = input
        .into_iter()
        .filter(|line| !line.is_empty() && !line.is_comment())
        .collect::<Vec<String>>()
        .split(|raw_slide| raw_slide.starts_with(SEPARATOR))
        .map(|slide| slide.to_vec())
        .collect::<Vec<Vec<String>>>();

    let mut slides: Vec<Slide> = vec![];
    let mut has_mermaid = false;
    let mut foot: Result<String, Error> = Ok(String::new());
    let mut logo_img : Result<String, Error> = Ok(String::new());
    
    for (slide_no, raw_slide) in raw_slides.into_iter().enumerate() {
        let mut elements: Vec<Element> = vec![];
        let mut is_draft = false;
        //let mut mermaid_script = (String::new(), false);
        
        for mut raw_element in raw_slide.split_on_tag() {
        raw_element = raw_element.clean_tag();
        // cleaning spaces on the tag line before processing 
        // is necessary, since the match bellow acts like a Turing 
        // machine over raw_element.
        //raw_element[0] = raw_element[0].replace(" ", "");
            let raw_result: Option<Element> = 
                // [1..] to skip tag marker
                match &raw_element[0][1..]{
                    tag if tag.starts_with(TAG_HEADING) => {
                        Some(heading(raw_element)?)
                    },
                    tag if tag.starts_with(TAG_SUBHEADING ) => {
                        Some(subheading(raw_element)?)
                    },
                    tag if tag.starts_with(TAG_ULIST) => {
                        Some(ulist(raw_element)?)
                    },
                    tag if tag.starts_with(TAG_ORDLIST) => { 
                        Some(ordlist(raw_element)?)
                    },
                    tag if tag.starts_with(TAG_TEXT) => {
                        Some(text(raw_element)?)
                    }
                    tag if tag.starts_with(TAG_VIDEO) => {
                        Some(video(raw_element)?)
                    },
                    tag if tag.starts_with(TAG_IMAGE) => {
                        Some(image(raw_element)?)
                    },
                    tag if tag.starts_with(TAG_TABLE) => {
                        Some(table(raw_element)?)
                    },
                    tag if tag.starts_with(TAG_MERMAID) => {
                        has_mermaid = true;
                        Some(mermaid(raw_element)?)
                    },
                    tag if tag.starts_with(TAG_MERMAIDSCRIPT) => {
                        None
                    },
                    tag if tag.starts_with(TAG_FOOTER) => {
                        foot = footer(raw_element);
                        None
                    },
                    tag if tag.starts_with(TAG_LOGO) => {
                        logo_img = logo(raw_element);
                        None
                    },
                    tag if tag.starts_with(TAG_DRAFT) => {
                        is_draft = true;
                        None
                    }
                    _ => {panic!(
                            "Unrecognised tag \"{}\".", 
                            &raw_element[0])
                    }
            };

            // Just ingore from the 5th element foward. 
            // See StultusVisio philosophy.
            match raw_result {
                Some(result) => {
                    if elements.len() < 4 {
                        elements.push(result);
                    } else {
                        eprintln!("The slide no. {} had to many elements. An element of nature {} was discarded.", slide_no, result.nature)
                    }
                    }
                None => ()
            }

            elements = elements.organize();
            // The `elements: Vec<Element>` should suffer ordering,
            // checking and other SxPres philosophy acts. 
            // e.g: if the user passes a .heading tag, it should always
            // be the first on the slide, to occupy the top. Some 
            // prohibitions are also desireble, like no more than two
            // tables per slide, a single video etc. In another words,
            // the main characteristic of SxPres is to free the 
            // user from formatting.
        };
        slides.push(
            Slide {
                //number: 1, 
                draft: is_draft, 
                content: Ok(elements)
            }
        )
    };

    let _ = output(render(logo_img, foot, has_mermaid, slides)?, args);
    
    println!("Done!\nIf some elements was discarded by the process, see SxPress phylosophy.\n");
    
    Ok(())
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
