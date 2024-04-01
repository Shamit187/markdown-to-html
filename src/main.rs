use std::fs;
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
