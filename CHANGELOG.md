# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.4.0 - 2025-07-31

Minor update with a new command line argument to copy `.gitignore` lines as comments in the `.megaignore` file.
And a small fix to the way comments are added to the `.megaignore` file.

### Added

Added `-C` or `--comments` command line argument to copy `.gitignore` lines as comments in the `.megaignore` file.
Much like the `-c` or `--copy` argument, but it does not copy the rules themselves, only the comments and empty lines.s

### Changed

When copying `.gitignore` lines as comments, the `#` character is no longer added to the beginning of the line, as that was adding a unnecessary comment character to the `.megaignore` file.


## v0.3.1 - 2024-12-28

Minor update with a few bug fixes, refactoring and documentation improvements.

### Added

- created this changelog file
- added MIT license to the project

## v0.3.0 - 2024-12-24

Major update transitioning from a simple rust script to a CLI tool with configuration options trough command line arguments.

### Added

- `Config` struct to hold command line arguments as a configuration
- Used [clap](https://crates.io/crates/clap) to parse command line arguments and generate help messages
- Command line arguments:
  - `DIRECTORY` *(positional)* to specify the directory where the `.gitignore` file is located, defaults to the current directory
  - `--input` or `-i` to specify the input `.gitignore` file
  - `--output` or `-o` to specify the output `.megaignore` file
  - `-m` or `--sync-megaignore` to specify when the `.megaignore` file should be synced with megasync
  - `-c` or `--copy` to copy `.gitignore` lines to `.megaignore` as comments
  - `-e` or `--no-extras` to not add the header and footer to the `.megaignore` file
  - `-x` or `--force-regexp` to force the use of the RegEx STRATEGY for all rules
  - `-I` or `--ignore-case` to make `.megaignore` rules case insensitive
  - `-p` or `--print` to print the generated `.megaignore` file to STDOUT
  - `-v` or `--verbose` to set the level of verbosity from 0 (off) to 4 (debug)
  - `-h` or `--help` to print the help message
  - `-V` or `--version` to print the version of the tool
- support for getting the `.gitignore` rules from STDIN when detecting a pipe (e.g. `cat .gitignore | gitignore-to-megaignore`)
- support for printing the generated `.megaignore` file to STDOUT when the `-p` or `--print` argument is used or when piping the output (e.g. `gitignore-to-megaignore > ../.megaignore`)

### Changed

- converted rules do not include `g` option as it is the default megaignore STRATEGY

### Fixed

- Glob to Regex conversion for `?` and `**/` patterns

## v0.2.0 - 2024-12-13

### Added

- Supported for rules that can't be represented as globs (used by megasync) using Regular Expressions
- Automatic conversion of glob patterns to regular expressions when needed
- Header comment contains version of git2megaignore as well as link to the repository

### Changed

- Simplified the `IgnoreRule` struct

## v0.1.0 - 2024-12-09

### Added

- basic conversion of gitignore rules to glob .megaignore rules
- copy of gitignore lines to .megaignore as comments
- support for folder only rules
- suppert for exlcude/include rules
- support for comments
- header and footer comment inside .megaignore with some information about the conversion
