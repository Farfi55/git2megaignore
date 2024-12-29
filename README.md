# Gitignore to Megaignore converter

<p align="center">
  <a href="https://crates.io/crates/git2megaignore">
    <img src="https://img.shields.io/crates/v/git2megaignore" alt="crates.io version">
  </a>
  <a href="https://crates.io/crates/git2megaignore">
    <img src="https://img.shields.io/crates/d/git2megaignore" alt="crates.io downloads">
  <a href="https://github.com/Farfi55/git2megaignore/releases">
    <img src="https://img.shields.io/github/v/release/Farfi55/git2megaignore" alt="github release">
  </a>
  </a>
  <a href="https://github.com/Farfi55/git2megaignore/blob/main/LICENSE">
    <img src="https://img.shields.io/github/license/Farfi55/git2megaignore" alt="github license">
  </a>
  <a href="https://github.com/Farfi55/git2megaignore/stargazers">
    <img src="https://img.shields.io/github/stars/Farfi55/git2megaignore?style=flat" alt="github stars">
  </a>
</p>

This is a simple tool to convert a `.gitignore` file to a `.megaignore` file. The `.megaignore` file is used by the [Mega](https://mega.nz) cloud storage service to exclude files from being uploaded to the cloud much like the `.gitignore` file is used to exclude files from being tracked by git.

You can read more about the `.megaignore` file format [here](https://help.mega.io/installs-apps/desktop/megaignore).

The **changelog** is available [here](CHANGELOG.md).

## Installation

### Cargo

Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed.

You can then install git2megaignore using cargo:

```sh
cargo install git2megaignore
```

### Precompiled binaries

If you don't have Rust installed, you can download precompiled binaries from the [releases page](https://github.com/Farfi55/git2megaignore/releases).

## Usage example

```sh<!--  -->
# go to a directory with a .gitignore file
$ cd example

# or create a new .gitignore file
$ echo "*.txt
foo/
/[Bb]uild/
src/**/index.js
\!src/index.js" > .gitignore

# convert the .gitignore file to a .megaignore file
$ git2megaignore -em
```

**resulting `.megaignore` file:**

```txt filename="example/.megaignore"
+sync:.megaignore
-nG:*.txt
-dnG:foo
-dpR:^[Bb]uild$
-pR:^src/(.*/)?index\.js$
+pG:src/index.js
```

- `sync:.megaignore` is a special rule that tells Mega to sync the `.megaignore` file itself, this was added with the `-m` option
- `+` prefix means that the rule is an **include rule**
- `-` prefix means that the rule is an **exclude rule**
- `n` option means that the rule should be applied to the **file name**, not the whole path
- `p` option means that the rule should be applied to the **path** of the file, not the file name
- `d` option means that the rule targets **directories only**, otherwise it targets everything
- `G` option means that the rule is a case-sensitive **glob** pattern
- `R` option means that the rule is a case-sensitive **regex** pattern
- for more information about the rule format, see the [Mega help page](https://help.mega.io/installs-apps/desktop/megaignore)

### Other usage examples

```sh
# take input from stdin and write to stdout
# -e, --no-extras  do not add extra header and footer lines
# -p, --print      print to stdout
# -v, --verbose <VERBOSE> set verbosity level (0-4)
$ echo "foo/**/bar" | git2megaignore -ep -v=0
-pR:^foo/(.*/)?bar$

# copy .gitignore lines when converting to .megaignore
# -c --copy copy .gitignore lines to the .megaignore file
$ echo "/[Ll]ibrary/" | git2megaignore -c -ep -v=0
# from /[Ll]ibrary/
-dpR:^[Ll]ibrary$

# run from other directory
# this will read the .gitignore file from the specified directory 
# and write the .megaignore file to the same directory
git2megaignore ~/path/to/repo

# specify input and output files explicitly
git2megaignore -i ../.gitignore -o /tmp/.megaignore
```

## Options

```txt
Convert a .gitignore file to a .megaignore file

Usage: git2megaignore [OPTIONS] [DIRECTORY]

Arguments:
  [DIRECTORY]  directory to start searching for .gitignore files [default: ]

Options:
  -m, --sync-megaignore    sync the .megaignore file with the +sync:.megaignore rule
  -c, --copy               If `true`, every .gitignore line will be copied to the .megaignore file as **a comment**
  -e, --no-extras          the header and footer will not be added to the .megaignore file
  -x, --force-regexp       Force the use of the Regex <STRATEGY> for all rules, even if the glob pattern does not require it
  -I, --ignore-case        .megaignore rules will be case insensitive
  -i, --input <FILE>       Input file to read .gitignore rules from, defaults to .gitignore in the current directory
  -o, --output <FILE>      Output file to write .megaignore rules to, defaults to .megaignore in the current directory
  -p, --print              print the generated .megaignore file to STDOUT
  -v, --verbose <VERBOSE>  Set the level of verbosity from 0 (off) to 4 (debug) [default: 2]
  -h, --help               Print help
  -V, --version            Print version
```

## Mass conversion

you can run the following bash script to convert all `.gitignore` files in the subdirectories of the current directory to `.megaignore` files:

```sh
find . -name .gitignore -execdir git2megaignore \;
```

for windows, you can use the following powershell script:

```powershell
Get-ChildItem -Recurse -Filter .gitignore | ForEach-Object { git2megaignore $_.DirectoryName }
```
