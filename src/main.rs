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
    Paragraph(String),
    List(ListItem),
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

struct ListItem {
    text: String,
    level: usize,
}

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
        LineToken::Heading3(line[5..].to_string())
    } else if line.starts_with("##### ") {
        LineToken::Heading3(line[6..].to_string())
    } else if line.starts_with("###### ") {
        LineToken::Heading3(line[7..].to_string())
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
    } else if line.starts_with("- ") {
        let mut indentation_count = 0;
        for c in line.chars() {
            if c == '-' {
                indentation_count += 1;
            }
        }
        indentation_count -= 1;
        LineToken::List(ListItem {
            text: line.trim().to_string().replace("- ", ""),
            level: indentation_count,
        })
    }
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
        \#\*\-\[\]\(\)\{\}\!\`\>\|\<\>\~\$\?
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

    // subsitute <red>text</red> with <span class="red-text"> text </span>
    // subsitute <teal>text</teal> with <span class="teal-text"> text </span>
    // subsitute <green>text</green> with <span class="green-text"> text </span>
    // subsitute <orange>text</orange> with <span class="orange-text"> text </span>
    // allowed color red, teal, green, orange
    let color_regex =
        Regex::new(r"<(red|teal|green|orange)>(.*?)</(red|teal|green|orange)>").unwrap();
    let text = color_regex
        .replace_all(&text, "<span class=\"$1-text\">$2</span>")
        .to_string();

    // allowed highlight colors: red, green, yellow, pink
    // subsitute <!red>text</!red> with <span class="red-highlight"> text </span>
    // subsitute <!green>text</!green> with <span class="green-highlight"> text </span>
    // subsitute <!yellow>text</!yellow> with <span class="yellow-highlight"> text </span>
    // subsitute <!pink>text</!pink> with <span class="pink-highlight"> text </span>
    let highlight_regex =
        Regex::new(r"<\!(red|green|yellow|pink)>(.*?)</\!(red|green|yellow|pink)>").unwrap();
    let text = highlight_regex
        .replace_all(&text, "<span class=\"$1-highlight\">$2</span>")
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

    // substitute $$text$$ with \[ text \]
    let block_equation_regex = Regex::new(r"\$\$(.*?)\$\$").unwrap();
    let text = block_equation_regex
        .replace_all(&text, "\\[$1\\]")
        .to_string();

    // substitue $text$ with \( text \)
    let inline_equation_regex = Regex::new(r"\$(.*?)\$").unwrap();
    let text = inline_equation_regex
        .replace_all(&text, "\\($1\\)")
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
                "<span class=\"comment\" data-target={}>{}</span>",
                comment_id, &comment[1]
            ),
            reference: format!(
                "<div id={} class=\"comment_explain hidden\">{}</div>",
                comment_id, &comment[2]
            ),
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
        LineToken::Paragraph(text) => {
            let text = modifiy_text_with_design(text, comments);
            format!("<p>{}</p>", text)
        }
        LineToken::Quote(text) => {
            let text = modifiy_text_with_design(text, comments);
            format!("<div class=\"block_quote\"> 💬 {}</div>", text)
        }
        LineToken::HorizontalRule => "<hr>".to_string(),
        LineToken::Image(source, alt, height, width) => {
            format!(
                "<img src=\"{}\" alt=\"{}\" height=\"{}\" width=\"{}\">",
                source, alt, height, width
            )
        }
        LineToken::Empty => "".to_string(),
        LineToken::List(list_item) => {
            let symbol = match list_item.level % 3 {
                0 => "•",
                1 => "◦",
                2 => "▪",
                _ => "",
            };
            format!(
                "<div class=\"list_item pl-{} indent-{}\">{} {}</div>",
                list_item.level * 4,
                list_item.level,
                symbol,
                list_item.text
            )
        }
    }
}

static HTML_HEAD: &str = r#"
<!doctype html>
<html>

<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <script src="https://cdn.tailwindcss.com"></script>
    <style type="text/tailwindcss">
        @layer components {
            @media (prefers-color-scheme: dark) {
                body.dark {
                    @apply bg-gray-900 text-gray-200;
                }
            }
        }
        @layer utilities {
            body {
                @apply bg-gray-100 text-gray-900 dark:bg-gray-900 dark:text-gray-200;
            }
            .spoiler {
                @apply bg-gray-300 dark:bg-gray-700 text-gray-300 dark:text-gray-700;
            }
            h1 {
                @apply text-4xl font-bold text-center py-4;
            }
            h2 {
                @apply text-3xl font-bold py-1;
            }
            h3 {
                @apply text-2xl font-bold py-1;
            }
            .red-text {
                @apply text-red-500 dark:text-red-300;
            }
            .green-text {
                @apply text-green-500 dark:text-green-300;
            }
            .teal-text {
                @apply text-teal-500 dark:text-teal-300;
            }
            .orange-text {
                @apply text-orange-500 dark:text-orange-300;
            }
            .red-highlight {
                @apply bg-red-500 bg-opacity-75 rounded;
            }
            .green-highlight {
                @apply bg-green-500 bg-opacity-75 rounded;
            }
            .yellow-highlight {
                @apply bg-yellow-500 bg-opacity-75 rounded;
            }
            .pink-highlight {
                @apply bg-pink-500 bg-opacity-75 rounded;
            }
            .hidden {
                display: none;
            }
            a {
                @apply text-blue-500 dark:text-blue-300 hover:underline; 
            }
            .comment_explain {
                @apply fixed top-1/2 left-1/2 transform -translate-x-1/2;
                @apply bg-gray-100 dark:bg-gray-800 p-2 m-2 rounded shadow;
            }
            .block_quote {
                @apply bg-gray-200 dark:bg-gray-700 p-2 rounded shadow;
            }
            .spoiler {
                @apply bg-gray-300 dark:bg-gray-700 text-gray-300 dark:text-gray-700 p-2 rounded shadow transition duration-500 ease-in-out;
            }
            .spoiler.revealed {
                @apply bg-transparent text-gray-900 dark:text-gray-200; /* Change background and text color */
            }
            .spoiler:hover {
                @apply cursor-pointer;
            }
        }   
    </style>
    <script src="https://polyfill.io/v3/polyfill.min.js?features=es6"></script>
    <script id="MathJax-script" async src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>
    <link rel="stylesheet" href="styles.css">
    <script>
        document.addEventListener('DOMContentLoaded', function() {
            const spoilers = document.querySelectorAll('.spoiler');

            spoilers.forEach(spoiler => {
                spoiler.addEventListener('click', function() {
                    // Toggle the 'revealed' class when clicked
                    this.classList.toggle('revealed');
                });
            });
        });
        document.addEventListener('DOMContentLoaded', function () {
            const comments = document.querySelectorAll('.comment');
            const commentExplain = document.querySelector('.comment_explain');

            comments.forEach(comment => {
                comment.addEventListener('mouseover', function (event) {
                    const x = event.clientX;
                    const y = event.clientY;

                    // Subtract width and height of tooltip to start from cursor corner
                    const commentExplainWidth = commentExplain.offsetWidth;
                    const commentExplainHeight = commentExplain.offsetHeight;
                    const adjustedX = x - commentExplainWidth;
                    const adjustedY = y - commentExplainHeight;

                    // Adjust position if too close to the right edge
                    const windowWidth = window.innerWidth;
                    const maxRight = windowWidth - commentExplainWidth - 10; // 10px buffer
                    const finalX = adjustedX > maxRight ? maxRight : adjustedX;

                    // Adjust position if too close to the bottom edge
                    const windowHeight = window.innerHeight;
                    const maxBottom = windowHeight - commentExplainHeight - 10; // 10px buffer
                    const finalY = adjustedY > maxBottom ? maxBottom : adjustedY;

                    commentExplain.style.left = `${finalX}px`;
                    commentExplain.style.top = `${finalY}px`;

                    const targetId = this.getAttribute('data-target');
                    const targetElement = document.getElementById(targetId);
                    targetElement.classList.remove('hidden');
                });

                comment.addEventListener('mouseout', function () {
                    const targetId = this.getAttribute('data-target');
                    const targetElement = document.getElementById(targetId);
                    targetElement.classList.add('hidden');
                });
            });
        });
    </script>
</head>

<body>
    <div class="py-10 px-20  flex-col space-y-2 w-100%">
"#;

static HTML_TAIL: &str = r#"
</div>
</body>

</html>
"#;

#[derive(PartialEq, Eq)]
enum MultiLineState {
    List,
    // CodeBlock,
    // Table,
    None,
}

fn markdown_to_html(markdown: String) -> String {
    let mut html = String::new();

    html.push_str(HTML_HEAD);

    // static storage of vectors of strings for comments
    let mut comments: Vec<String> = Vec::new();

    // switch states
    let mut state = MultiLineState::None;

    for line in markdown.lines() {
        // tokenize line
        let line_token = line_tokenizer(line);

        let new_state = match line_token {
            LineToken::List(_) => MultiLineState::List,
            // LineToken::CodeBlock(_) => MultiLineState::CodeBlock,
            // LineToken::Table(_) => MultiLineState::Table,
            _ => MultiLineState::None,
        };

        // if state swithces, add a newline
        if state != new_state {
            /*
                New State Fix
            */

            // if new state is List, add a <div class="list"> tag
            if new_state == MultiLineState::List {
                html.push_str("<div class=\"list\">\n");
            }

            /*
                Previous State Fix
            */

            // if prev state is List, add a </div> tag
            if state == MultiLineState::List {
                html.push_str("</div>\n");
            }
        }
        state = new_state;

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
        html.push_str(&format!("{}\n", comment));
    }

    html.push_str(HTML_TAIL);

    html
}

fn main() {
    let start_time = Instant::now();

    start();

    let end_time = Instant::now();
    let duration = end_time - start_time;

    println!("Elapsed time: {:?}", duration);
}
