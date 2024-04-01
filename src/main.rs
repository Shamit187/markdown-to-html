use std::fs;
use regex::Regex;
static INPUT: &str = "markdown.md";
static OUTPUT: &str = "index.html";

enum LineToken {
    Heading1(String),
    Heading2(String),
    Heading3(String),
    Heading4(String),
    Heading5(String),
    Heading6(String),
    Paragraph(String),
    // List(String),
    /* 
        Idea for list (and CodeBlock): 
        1. line_tokenizer returns list token
        2. token_to_html does not return anything, it should have a static storage, where it will store the list item
        3. if the next token is not a list token, it will return the list along with the other token that was given
        4. I still don't know how to implement nested list
    */
    // CodeBlock(String),
    /*
        later use prism.js to highlight code
    */
    Quote(String),
    HorizontalRule,
    Image(String, String, String, String),
    // Table(Vec<Vec<String>>),
    /* 
        I have no idea how to add table
    */
    Empty
}

// struct Color {
//     r: u8,
//     g: u8,
//     b: u8,
// }

// enum InlineToken {
//     // line tokens where inline token should be used
//     // Paragraph, List, Quote
//     Plain(String),
//     Bold(String),
//     Italic(String),
//     BoldItalic(String),
//     Strikethrough(String),
//     Underline(String),
//     Emphasis(String),
//     Link(String, String),
//     Code(String),
//     Spoiler(String),
//     Comment(String, String),
//     // comment, citation, footnote reference needs a static storage, where everything will accumulate and then added after everything
//     Color(String, Color),
//     Highlight(String, Color),
//     // InlineEquation(String),
//     // BlockEquation(String),
//     // Superscript(String, String),
//     // Subscript(String, String),
//     // Citation(String),
//     // FootnoteReference(String),
// }


fn start() {
    let markdown = fs::read_to_string(INPUT).expect("Error reading file");
    let html = markdown_to_html(markdown);
    fs::write(OUTPUT, html).expect("Error writing file");
}

fn line_tokenizer(line: &str) -> LineToken {
    if line.is_empty() {
        LineToken::Empty
    } else if line.starts_with("# ") {
        LineToken::Heading1(line[2..].to_string())
    } else if line.starts_with("## ") {
        LineToken::Heading2(line[3..].to_string())
    } else if line.starts_with("### ") {
        LineToken::Heading3(line[4..].to_string())
    } else if line.starts_with("#### ") {
        LineToken::Heading4(line[5..].to_string())
    } else if line.starts_with("##### ") {
        LineToken::Heading5(line[6..].to_string())
    } else if line.starts_with("###### ") {
        LineToken::Heading6(line[7..].to_string())
    } else if line.starts_with("> ") {
        LineToken::Quote(line[2..].to_string())
    } else if line.starts_with("---") {
        LineToken::HorizontalRule
    } else if line.starts_with("![](") {
        // need to modifiy this one
        LineToken::Image("source".to_string(), "alt".to_string(), "height".to_string(), "width".to_string())
    } 
    // else if line.starts_with("- ") {
        // process list
    // }
    // else if line.starts_with("```") {
        // process code block
    // }
    else {
        LineToken::Paragraph(line.to_string())
    }
}

fn modifiy_text_with_design(text: String) -> String {
    // need to implement this
    
    // substitute *text* with <i> text </i>
    let italic_regex = Regex::new(r"\*(.*?)\*").unwrap();
    let text = italic_regex.replace_all(&text, "<i>$1</i>").to_string();

    // substitute **text** with <b> text </b>
    let bold_regex = Regex::new(r"\*\*(.*?)\*\*").unwrap();
    let text = bold_regex.replace_all(&text, "<b>$1</b>").to_string();

    // substitute ***text*** with <b><i> text </i></b>
    let bold_italic_regex = Regex::new(r"\*\*\*(.*?)\*\*\*").unwrap();
    let text = bold_italic_regex.replace_all(&text, "<b><i>$1</i></b>").to_string();

    // substitute ~~text~~ with <s> text </s>
    let strikethrough_regex = Regex::new(r"~~(.*?)~~").unwrap();
    let text = strikethrough_regex.replace_all(&text, "<s>$1</s>").to_string();

    // substitute ~text~ with <u> text </u>
    let underline_regex = Regex::new(r"~(.*?)~").unwrap();
    let text = underline_regex.replace_all(&text, "<u>$1</u>").to_string();

    // substitute `text` with <code> text </code>
    let code_regex = Regex::new(r"`(.*?)`").unwrap();
    let text = code_regex.replace_all(&text, "<code>$1</code>").to_string();

    // subsitute <color:red>text</color:red> with <span style="color: red"> text </span>
    let color_regex = Regex::new(r"<color:(.*?)>(.*?)</color:(.*?)>").unwrap();
    let text = color_regex.replace_all(&text, "<span style=\"color: $1\">$2</span>").to_string();

    // substitute <!color:red>text</!color:red> with <span style="background-color: red"> text </span>
    let highlight_regex = Regex::new(r"<!color:(.*?)>(.*?)</!color:(.*?)>").unwrap();
    let text = highlight_regex.replace_all(&text, "<span style=\"background-color: $1\">$2</span>").to_string();

    text
}

fn token_to_html(token: LineToken) -> String {
    // static storage: String = String::new();
    match token {
        LineToken::Heading1(text) => {
            format!("<h1>{}</h1>", text)
        },
        LineToken::Heading2(text) => {
            format!("<h2>{}</h2>", text)
        },
        LineToken::Heading3(text) => {
            format!("<h3>{}</h3>", text)
        },
        LineToken::Heading4(text) => {
            format!("<h4>{}</h4>", text)
        },
        LineToken::Heading5(text) => {
            format!("<h5>{}</h5>", text)
        },
        LineToken::Heading6(text) => {
            format!("<h6>{}</h6>", text)
        },
        LineToken::Paragraph(text) => {
            let text = modifiy_text_with_design(text);
            format!("<p>{}</p>", text)
        },
        LineToken::Quote(text) => {
            let text = modifiy_text_with_design(text);
            format!("<blockquote>{}</blockquote>", text)
        },
        LineToken::HorizontalRule => {
            "<hr>".to_string()
        }
        LineToken::Image(source, alt, height, width) => {
            format!("<img src=\"{}\" alt=\"{}\" height=\"{}\" width=\"{}\">", source, alt, height, width)
        },
        LineToken::Empty => "".to_string(),
    }
}


fn markdown_to_html(markdown: String) -> String {
    let mut html = String::new();
    for line in markdown.lines() {
        // tokenize line
        let line_token = line_tokenizer(line);

        // convert token to html
        let line_html = token_to_html(line_token);

        // add a newline for good luck
        // remove this line in final version
        let line_html = format!("{}\n", line_html);
        
        // append to html
        html.push_str(&line_html);
    }
    html
}

fn main() {
    start();
}
