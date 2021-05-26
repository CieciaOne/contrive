# Contrive
Contrive is a simple utility to automate creation of directory and file structures.
It has basic functionalities like creation of directories and files with content in them.
You can use static names or create dynamic ones with {{your_variable_name}} syntax.

## Installation
cargo install contrive

## Example usage
contrive help
contrive add lecture lecture.json\
contrive lecture topic:trigonometry date:3_14_21\
contrive remove lecture

## Example template
In example_template directory is json file used in above example that shows how your config should be formatted

