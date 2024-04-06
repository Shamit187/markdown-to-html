use std::time::Instant;
mod markdown_to_html;
use markdown_to_html::start;
static INPUT: &str = "demo/markdown.md";
static OUTPUT: &str = "demo/index.html";

fn main() {
    let start_time = Instant::now();

    start(INPUT, OUTPUT);

    let end_time = Instant::now();
    let duration = end_time - start_time;

    println!("Elapsed time: {:?}", duration);
}
