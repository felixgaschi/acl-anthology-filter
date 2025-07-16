# ACL Anthology filter

The ACL anthology bib file is quite big, and more often than not, you need only a part of it. This Rust program takes a LaTeX project and creates a filtered ACL anthology that contains only the papers that are cited in the project.

**Note**: This was only intended as a small project to learn Rust.

## Install

This package is not published so you can install it using either with git or using a local clone:

With git:

```
cargo intall --git https://github.com/felixgaschi/acl-anthology-filter
```

Using a local clone:

```
git clone git@github.com:felixgaschi/acl-anthology-filter.git
cargo install --path acl-anthology-filter
```

## How to use

There are two available commands:

- filter: to create a reduced version of the anthology bib file keeping only papers that are cited in tex files in a given directory
- search: to search for a given entry in the anthology bib file

### Filter

```
Usage: anthology_filter filter [OPTIONS]

Options:
  -a, --anthology-path <ANTHOLOGY_PATH>  
  -d, --directory <DIRECTORY>            [default: .]
  -o, --output-file <OUTPUT_FILE>        
  -h, --help                             Print help
  -V, --version                          Print version
```

`anthology-path` is the path to the original anthology.bib file (you need to download it first)

`directory` is the location where the program will recursively search for .tex file in which to find \cite* commands, default is the current directory.

`output-file` is the path for the generated file

### Search

```
Usage: anthology_filter search [OPTIONS] --search-string <SEARCH_STRING>

Options:
  -a, --anthology-path <ANTHOLOGY_PATH>  
  -s, --search-string <SEARCH_STRING>    
  -n, --n <N>                            
  -h, --help                             Print help
  -V, --version                          Print version
```
