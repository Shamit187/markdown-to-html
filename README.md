# Markdown to HTML Parser

## Syntax

* Title: #
* Subtitle: !#
* Chapter: ##
* Subchapter: ###
* Italic: *
* Bold: **
* Bold and Italic: ***
* Strikethrough: ~~
* Underline: ~
* Monospace: `
* Emphasis `<em>`
* Link: [word](link\)
* Color: `<red> </red>`
  * red
  * green
  * teal
  * orange
* Highlight: <!red> </!red>
  * red
  * green
  * yellow
  * pink
* Linkebreak `<br>`
* Comment: (word)(description)
* Comment: !
* Horizontal rule: ---
* Block quote: >
* Spoiler: `<spoiler>`
* List: -
* SubList: - -
* Image: ![alt][height](link\)
* Table: | item1 | item2 | item3 |
* Code: ```language
* Inline equation: \$equation\$
* Block equation: \$\$equation\$\$
* Escape Characters: #*-[](){}!`|<>~$?

## Usage

```rust
mod markdown_to_html;
use markdown_to_html::start;

fn main() {
	start(input_file, output_file);
}
```
