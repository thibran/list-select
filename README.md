# list-select

TUI to select a vertical list entry.

**This command needs a lot more testing, be careful where you use it!**

Tested on Linux using `fish` shell (should work on all Unix platforms).


## Usage
``` bash
USAGE:
    list-select <ROWS>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <ROWS>...    rows to display
```


### Examples
``` bash
$ ls -1 | list-select
>  build
   deps
   examples
   incremental
   list-select
   list-select.d

# multiple input strings are merged into one vertical list
$ list-select foo bar hello world
   foo
   bar
>  hello
   world
```


## Exit Codes

| Exit Code | Explanation                |
|-----------|----------------------------|
| 0         | Program run as intended    |
| 1         | Wrong input arguments      |
| 2         | No Match found or canceled |


# brainstorming

## options that could be added
``` bash
--skip-lines [num]            # skip first lines
--default-row [num or string] # which to hilight at start
--fallback [string]           # string to return on error/abort
--show-info                   # display info-bar
--row-number                  # number of rows to show
--split-col-at                # split input at [col] displaying > after col
--multi-select                # choose more than one item
```

## features that could be added
- use terminal theme colors
- display row-marker as long as max-row-len