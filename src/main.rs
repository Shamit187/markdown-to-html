use regex::Regex;
use std::fs;
static INPUT: &str = "markdown.md";
static OUTPUT: &str = "index.html";
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Instant;

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
    Empty,
}

struct Comment {
    comment_token: String,
    replaceable: String,
    reference: String,
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
    } else if line.starts_with("![") {
        // need to modifiy this one
        LineToken::Image(
            "source".to_string(),
            "alt".to_string(),
            "height".to_string(),
            "width".to_string(),
        )
    }
    // else if line.starts_with("- ") {
    // process list
    // }
    // else if line.starts_with("```") {
    // process code block
    // }
    else if line.starts_with("!") {
        LineToken::Empty
    } else {
        LineToken::Paragraph(line.to_string())
    }
}

fn hash_string_to_integer(input: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish()
}

fn modifiy_text_with_design(text: String, comments: &mut Vec<String>) -> String {
    // substitute all escape characters
    /*
        \#
        \*
        \-
        \[
        \]
        \(
        \)
        \{
        \}
        \!
        \`
        \>
        \|
        \<
        \>
        \~
        \$
        \?
    */
    let text = text.replace(r"\#", "\\+hash");
    let text = text.replace(r"\*", "\\+star");
    let text = text.replace(r"\-", "\\+dash");
    let text = text.replace(r"\[", "\\+leftsquare");
    let text = text.replace(r"\]", "\\+rightsquare");
    let text = text.replace(r"\(", "\\+leftparen");
    let text = text.replace(r"\)", "\\+rightparen");
    let text = text.replace(r"\{", "\\+leftcurly");
    let text = text.replace(r"\}", "\\+rightcurly");
    let text = text.replace(r"\!", "\\+exclamation");
    let text = text.replace(r"\`", "\\+backtick");
    let text = text.replace(r"\>", "\\+greater");
    let text = text.replace(r"\|", "\\+pipe");
    let text = text.replace(r"\<", "\\+lesser");
    let text = text.replace(r"\>", "\\+greater");
    let text = text.replace(r"\~", "\\+tilde");
    let text = text.replace(r"\$", "\\+dollar");
    let text = text.replace(r"\?", "\\+question");

    // substitute ***text*** with <b><i> text </i></b>
    let bold_italic_regex = Regex::new(r"\*\*\*(.*?)\*\*\*").unwrap();
    let text = bold_italic_regex
        .replace_all(&text, "<b><i>$1</i></b>")
        .to_string();

    // substitute **text** with <b> text </b>
    let bold_regex = Regex::new(r"\*\*(.*?)\*\*").unwrap();
    let text = bold_regex.replace_all(&text, "<b>$1</b>").to_string();

    // substitute *text* with <i> text </i>
    let italic_regex = Regex::new(r"\*(.*?)\*").unwrap();
    let text = italic_regex.replace_all(&text, "<i>$1</i>").to_string();

    // substitute ~~text~~ with <s> text </s>
    let strikethrough_regex = Regex::new(r"~~(.*?)~~").unwrap();
    let text = strikethrough_regex
        .replace_all(&text, "<s>$1</s>")
        .to_string();

    // substitute ~text~ with <u> text </u>
    let underline_regex = Regex::new(r"~(.*?)~").unwrap();
    let text = underline_regex.replace_all(&text, "<u>$1</u>").to_string();

    // substitute `text` with <code> text </code>
    let code_regex = Regex::new(r"`(.*?)`").unwrap();
    let text = code_regex.replace_all(&text, "<code>$1</code>").to_string();

    // subsitute <color:red>text</color:red> with <span style="color: red"> text </span>
    let color_regex = Regex::new(r"<color:(.*?)>(.*?)</color>").unwrap();
    let text = color_regex
        .replace_all(&text, "<span style=\"color: $1\">$2</span>")
        .to_string();

    // substitute <!color:red>text</!color:red> with <span style="background-color: red"> text </span>
    let highlight_regex = Regex::new(r"<!color:(.*?)>(.*?)</!color>").unwrap();
    let text = highlight_regex
        .replace_all(&text, "<span style=\"background-color: $1\">$2</span>")
        .to_string();

    // substitute [text](link) with <a href="link"> text </a>
    let link_regex = Regex::new(r"\[(.*?)\]\((.*?)\)").unwrap();
    let text = link_regex
        .replace_all(&text, "<a href=\"$2\">$1</a>")
        .to_string();

    // substitute <spoiler>text</spoiler> with <span class="spoiler"> text </span>
    let spoiler_regex = Regex::new(r"<spoiler>(.*?)</spoiler>").unwrap();
    let text = spoiler_regex
        .replace_all(&text, "<span class=\"spoiler\">$1</span>")
        .to_string();

    // substitute (text)(This comment explains the text on side) with <span class="comment"> text </span>
    let comment_regex = Regex::new(r"\((.*?)\)\((.*?)\)").unwrap();
    let mut comments_found: Vec<Comment> = Vec::new();
    // capture all comments
    for comment in comment_regex.captures_iter(&text) {
        let comment_text = format!("({})({})", &comment[1], &comment[2]);
        let comment_id = hash_string_to_integer(&comment_text);
        comments_found.push(Comment {
            comment_token: comment_text,
            replaceable: format!(
                "<span class=\"comment\" target={}>{}</span>",
                comment_id, &comment[1]
            ),
            reference: format!("<span id={}>{}</span>", comment_id, &comment[2]),
        });
    }
    // replace all comments
    let mut replaced_text = text.clone();
    for comment in comments_found {
        replaced_text = replaced_text.replace(&comment.comment_token, &comment.replaceable);
        comments.push(comment.reference);
    }
    let text = replaced_text;

    // substitute back all escape characters
    let text = text.replace("\\+hash", "#");
    let text = text.replace("\\+star", "*");
    let text = text.replace("\\+dash", "-");
    let text = text.replace("\\+leftsquare", "[");
    let text = text.replace("\\+rightsquare", "]");
    let text = text.replace("\\+leftparen", "(");
    let text = text.replace("\\+rightparen", ")");
    let text = text.replace("\\+leftcurly", "{");
    let text = text.replace("\\+rightcurly", "}");
    let text = text.replace("\\+exclamation", "!");
    let text = text.replace("\\+backtick", "`");
    let text = text.replace("\\+greater", ">");
    let text = text.replace("\\+pipe", "|");
    let text = text.replace("\\+lesser", "&lt;");
    let text = text.replace("\\+greater", "&gt;");
    let text = text.replace("\\+tilde", "~");
    let text = text.replace("\\+dollar", "$");
    let text = text.replace("\\+question", "?");

    text
}

fn token_to_html(token: LineToken, comments: &mut Vec<String>) -> String {
    // static storage: String = String::new();
    match token {
        LineToken::Heading1(text) => {
            format!("<h1>{}</h1>", text)
        }
        LineToken::Heading2(text) => {
            format!("<h2>{}</h2>", text)
        }
        LineToken::Heading3(text) => {
            format!("<h3>{}</h3>", text)
        }
        LineToken::Heading4(text) => {
            format!("<h4>{}</h4>", text)
        }
        LineToken::Heading5(text) => {
            format!("<h5>{}</h5>", text)
        }
        LineToken::Heading6(text) => {
            format!("<h6>{}</h6>", text)
        }
        LineToken::Paragraph(text) => {
            let text = modifiy_text_with_design(text, comments);
            format!("<p>{}</p>", text)
        }
        LineToken::Quote(text) => {
            let text = modifiy_text_with_design(text, comments);
            format!("<blockquote>{}</blockquote>", text)
        }
        LineToken::HorizontalRule => "<hr>".to_string(),
        LineToken::Image(source, alt, height, width) => {
            format!(
                "<img src=\"{}\" alt=\"{}\" height=\"{}\" width=\"{}\">",
                source, alt, height, width
            )
        }
        LineToken::Empty => "".to_string(),
    }
}

fn markdown_to_html(markdown: String) -> String {
    let mut html = String::new();
    // static storage of vectors of strings for comments
    let mut comments: Vec<String> = Vec::new();

    for line in markdown.lines() {
        // tokenize line
        let line_token = line_tokenizer(line);

        // convert token to html
        let line_html = token_to_html(line_token, &mut comments);

        // add a newline for good luck
        // remove this line in final version
        let line_html = format!("{}\n", line_html);

        // append to html
        html.push_str(&line_html);
    }

    // add comments
    for comment in comments {
        html.push_str(&format!(
            "<span class=\"comment_explain\">{}</span>\n",
            comment
        ));
    }

    html
}

fn main() {
    let start_time = Instant::now();

    start();

    let end_time = Instant::now();
    let duration = end_time - start_time;

    println!("Elapsed time: {:?}", duration);
}
