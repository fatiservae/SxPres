> ⚠️ Warning: everything in a very premature state.

# StultusVisio
**Stupidless minimalist slide presentations software!**
## About
Just a software to lazy people make quick presentations.
## Philosophy
A slide presentation should be clean and simple.

StultusVisio do not accept more than four elements per slide. An element can be an image, a video, a text, a listing etc.

If you put more than four elements in a slide, the fifth onwards will be ignored.

The principle is to produce a minimalist, unique, plataform independent and complete slide presentation.

## How to use
It's self explanatory:

```
.text
This is a text in a slide.
.list 
And this is a first item of a list.
This is the second.
Etc.
---
# This is a comment and will be ingored.
# Those three dashes (---) starts a new slide.
.ordlist 
Lists can be ordered, producing numbered items.
In this case, two!
---
.video
./example.mp4
# The .video tag will insert the example.mp4 video on the same directory.
---
.image
./example.jpg
.draft
# This slide contains an image but will not be rendered, since it's marked as draft.
---
# Mermaid diagrams can be inserted also!
.mermaid
graph TD
A --> B
B -- Yep! --> C
---
.heading 
A title for the slide
.subheading 
A subtitle can be used
.table 
1st column | 2nd | 3rd 
2nd line | middle collumn | last
---
.footer 
A foot note to be shown in all slides.
# Footer text can be inserted at any slide.
``` 

Then, in `example` dir, compile the presentation:

```
$ cargo build --release

$ ../target/release/sxpres -i example.stv
```

A `example.html` file will be produced. Just open with some browser.

In the presentation, use the controls like vim mode:

```
t   :   Close and open the help.
j   :   Next slide.
k   :   Back slide.
p   :   Print mode. 
gg  :   Go to 1st slide.
G   :   To the last slide.
m   :   A circular marker.
x   :   Marker size.

On cellphones, finger swipe to next/back slides.
```

## Licence

This file is part of StultusVisio.

StultusVisio is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

StultusVisio is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with StultusVisio.  If not, see <https://www.gnu.org/licenses/>.
