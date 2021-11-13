# Contrive
Contrive is a utility that automates creation of directory and file structures written in Rust.
It has functionalities like creation of directories and files with content in them.
You can use static names or create dynamic ones with {{your_variable_name}} syntax.

## Installation
cargo install contrive\
If you don't have rust installed: https://www.rust-lang.org/tools/install

## Example usage
contrive help\
contrive add lecture lecture.json\
contrive lecture topic:trigonometry date:3_14_21\
contrive remove lecture

## Example template
In example_template directory is json file used in above example that shows how your config should be formatted

