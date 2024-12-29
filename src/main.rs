/// this executable reads the .gitignore file in the current directory
/// and creates a new file called .megaignore that contains the same ignore rules
/// but using the .megaignore format, trying to keep the .megaignore rules as similar as possible to the .gitignore rules
///
/// ## Further reading
///
/// [Megaignore format](from https://help.mega.io/installs-apps/desktop/megaignore)
/// [Gitignore format](https://git-scm.com/docs/gitignore#_pattern_format)
///
/// ## Usage examples
///
/// ```console
/// $ cd example
///
/// $ cat .gitignore
/// *.txt
/// foo/
/// /[Bb]uild/
/// src/**/index.js
///
/// $ git2megaignore -e
///
/// $ cat .megaignore
/// -nG:*.txt
/// -dnG:foo
/// -dpR:^[Bb]uild$
/// ```
use clap::Parser;
use log::{error, info, warn, LevelFilter};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::{self, Path};

/// Configuration options for translating .gitignore rules to .megaignore rules
#[derive(Parser, Debug, Default)]
#[command(author, about, version)]
struct Config {
    /// directory to start searching for .gitignore files
    #[clap(default_value = "")]
    directory: String,

    /// sync the .megaignore file with the +sync:.megaignore rule
    #[clap(short = 'm', long = "sync-megaignore")]
    sync_megaignore: bool,

    /// If `true`, every .gitignore line will be copied to the .megaignore file as **a comment**
    #[clap(short = 'c', long = "copy")]
    copy_gitignore_lines: bool,

    /// the header and footer will not be added to the .megaignore file
    #[clap(short = 'e', long)]
    no_extras: bool,

    /// Force the use of the RegEx STRATEGY for all rules, even if the glob pattern does not require it
    #[clap(short = 'x', long)]
    force_regexp: bool,

    /// .megaignore rules will be case insensitive
    #[clap(short = 'I', long)]
    ignore_case: bool,

    /// Input file to read .gitignore rules from, defaults to .gitignore in the current directory
    #[clap(short = 'i', long = "input", value_name = "FILE")]
    input: Option<String>,

    /// Output file to write .megaignore rules to, defaults to .megaignore in the current directory
    #[clap(short = 'o', long = "output", value_name = "FILE")]
    output: Option<String>,

    /// print the generated .megaignore file to STDOUT
    #[clap(short = 'p', long, conflicts_with = "output")]
    print: bool,

    /// Set the level of verbosity from 0 (off) to 4 (debug)
    #[clap(short='v', long, default_value = "2", value_parser = clap::value_parser!(u8).range(0..=4))]
    verbose: u8,
}
#[derive(Debug)]
struct IgnoreRule {
    /// The raw line from the .gitignore file
    git_rule: String,

    /// The rule in the .megaignore format
    mega_rule: String,

    /// Whether the rule is an exclude or include rule
    ///
    /// default: exclude
    is_exclude: bool,

    /// The glob pattern to match
    glob_pattern: String,

    /// Whether the pattern is relative to the .gitignore file
    /// default: false
    ///
    /// If there is a separator at the beginning or middle (or both) of the pattern,
    /// then the pattern is relative to the directory level of the particular .gitignore file itself.
    ///
    /// Otherwise the pattern may also match at any level below the .gitignore level.
    is_relative: bool,

    /// Should the rule only apply to folders
    ///
    /// default: false
    folders_only: bool,

    /// The strategy to use for the rule, either Glob or Regexp
    ///
    /// default: Glob
    strategy: MegaStrategy,
}

#[derive(Debug)]
enum MegaStrategy {
    Glob,
    Regexp,
}

impl IgnoreRule {
    /// Create a new IgnoreRule from a line in a .gitignore file
    /// # Examples
    /// ```
    /// let rule = IgnoreRule::from_line("*.txt", &config);
    /// assert_eq!(rule.git_rule, "*.txt");
    /// assert_eq!(rule.mega_rule, "-nG:*.txt\n");
    /// assert_eq!(rule.is_exclude, true);
    /// assert_eq!(rule.glob_pattern, "*.txt");
    /// assert_eq!(rule.is_relative, false);
    /// assert_eq!(rule.folders_only, false);
    /// assert_eq!(rule.strategy, MegaStrategy::Glob);
    /// ```
    fn from_line(line: &str, config: &Config) -> Self {
        let git_rule = line.trim();
        let mut is_exclude = true;
        let mut is_relative = false;
        let mut folders_only = false;
        let mut glob_pattern: String;

        // if the line starts with a ! then it is an include rule
        if git_rule.starts_with("!") {
            is_exclude = false;
            glob_pattern = git_rule.strip_prefix("!").unwrap().trim().to_string();
        } else {
            glob_pattern = git_rule.trim().to_string();
        }

        let starts_at_root = glob_pattern.starts_with("/");
        let has_multiple_parts = glob_pattern
            .split('/')
            .filter(|part| !part.is_empty())
            .count()
            > 1;

        // if the pattern starts at the root or if it is a path i.e. foo/bar
        // then it is relative to the .megaignore file
        if starts_at_root || has_multiple_parts {
            is_relative = true;
            if starts_at_root {
                // remove the leading slash
                glob_pattern.remove(0);
            }
        }

        // if the pattern ends with a slash, then it is a folder
        if glob_pattern.ends_with("/") {
            folders_only = true;
            // remove the trailing slash
            glob_pattern.pop();
        }

        let strategy = if config.force_regexp || pattern_needs_regexp(&glob_pattern) {
            MegaStrategy::Regexp
        } else {
            MegaStrategy::Glob
        };

        let mut rule = Self {
            git_rule: git_rule.to_string(),
            mega_rule: String::new(),
            is_exclude,
            glob_pattern,
            is_relative,
            folders_only,
            strategy,
        };

        rule.mega_rule = rule.to_megaignore(config);
        rule
    }

    /// Convert the rule to the .megaignore format
    /// # Examples
    /// ```
    /// let rule = IgnoreRule {
    ///    git_rule: "*.txt".to_string(),
    ///    mega_rule: "".to_string(),
    ///    is_exclude: true,
    ///    glob_pattern: "*.txt".to_string(),
    ///    is_relative: false,
    ///    folders_only: false,
    ///    strategy: MegaStrategy::Glob,
    /// };
    ///
    /// let megaignore = rule.to_megaignore(&config);
    /// assert_eq!(megaignore, "-nG:*.txt\n");
    /// ```
    fn to_megaignore(&self, config: &Config) -> String {
        let exclude = if self.is_exclude { '-' } else { '+' };
        let relative = if self.is_relative { 'p' } else { 'n' };

        // 'a' (default) is used to indicate that the rule applies to all files
        // 'd' is used to indicate that the rule only applies to folders (directories)
        let target = if self.folders_only { "d" } else { "" };

        let pattern;
        let strategy;

        match self.strategy {
            MegaStrategy::Glob => {
                // since the default strategy is case-insensitive glob, we don't need to specify it
                pattern = self.glob_pattern.clone();
                strategy = if config.ignore_case { "" } else { "G" };
            }
            MegaStrategy::Regexp => {
                pattern = match glob_to_regex(&self.glob_pattern) {
                    Ok(pattern) => pattern.clone(),
                    Err(e) => {
                        warn!("Error converting glob to regex: {}", e);
                        return format!(
                            "# ERROR: rule could not be converted: {}\n",
                            self.git_rule
                        );
                    }
                };
                strategy = if config.ignore_case { "r" } else { "R" };
            }
        }

        let mut out = String::new();
        if config.copy_gitignore_lines {
            out = format!("# from {}\n", self.git_rule);
        }
        out += &format!("{exclude}{target}{relative}{strategy}:{pattern}\n");
        out
    }
}

/// check if a line is not a .gitignore rule
/// i.e. if it is empty or is a comment
fn is_not_gitignore_rule(line: &str) -> bool {
    let line = line.trim();
    line.is_empty() || line.starts_with("#")
}

/// Determines if a glob pattern needs to be converted to a regex pattern
///
/// # Examples
///
/// ```
/// let needs_regexp = pattern_needs_regexp("foo");
/// assert_eq!(needs_regexp, false);
///
/// let needs_regexp = pattern_needs_regexp("*.txt");
/// assert_eq!(needs_regexp, false);
///
/// let needs_regexp = pattern_needs_regexp("foo/**/bar");
/// assert_eq!(needs_regexp, true);
///
/// let needs_regexp = pattern_needs_regexp("[Ff]oo");
/// assert_eq!(needs_regexp, true);
/// ```
fn pattern_needs_regexp(pattern: &str) -> bool {
    let mut chars = pattern.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '[' | ']' => return true, // Character sets
            '*' => {
                if chars.peek() == Some(&'*') {
                    chars.next(); // Consume the second '*'
                    if chars.peek() == Some(&'/') {
                        return true; // '**/' detected
                    }
                }
            }
            '\\' => {
                return true; // Escaped character
            }
            _ => continue,
        }
    }
    false
}

/// Converts a glob pattern to a regex pattern
///
/// # Examples
///
/// ```
/// let regex = glob_to_regex("foo").unwrap();
/// assert_eq!(regex, r"^foo$");
///
/// let regex = glob_to_regex("*.txt").unwrap();
/// assert_eq!(regex, r"^[^/]*\.txt$");
///
/// let regex = glob_to_regex("foo/**/bar").unwrap();
/// assert_eq!(regex, r"^foo/.*?/bar$");
/// ```
fn glob_to_regex(glob: &str) -> Result<String, Box<dyn Error>> {
    let mut regex_pattern = String::new();
    let mut chars = glob.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                // Escape the next character literally
                if let Some(next) = chars.next() {
                    regex_pattern.push('\\');
                    regex_pattern.push(next);
                } else {
                    return Err("Trailing backslash in glob pattern".into());
                }
            }
            '*' => {
                if chars.peek() == Some(&'*') {
                    chars.next(); // Consume the second '*'
                    if chars.peek() == Some(&'/') {
                        // pattern: **/
                        chars.next(); // Consume the '/'
                        regex_pattern.push_str("(.*/)?"); // Match any number of directories
                    } else {
                        // pattern: **
                        regex_pattern.push_str(".*");
                    }
                } else {
                    // pattern: *
                    regex_pattern.push_str("[^/]*"); // Match anything within the directory
                }
            }
            '?' => regex_pattern.push_str("[^/]"), // Match any single character but not a directory separator
            '[' => {
                // put [...] into the regex pattern as is
                regex_pattern.push('[');
                for inner in chars.by_ref().take_while(|c| *c != ']') {
                    print!("{}", inner);
                    regex_pattern.push(inner);
                }
                regex_pattern.push(']');
            }
            '!' => {
                // Gitignore negation should be handled externally
                return Err("Negation patterns ('!') are not directly supported".into());
            }
            '.' | '+' | '(' | ')' | '^' | '$' | '|' | '{' | '}' => {
                // Escape regex metacharacters
                regex_pattern.push('\\');
                regex_pattern.push(c);
            }
            '/' => regex_pattern.push('/'), // Directory separator
            _ => regex_pattern.push(c),
        }
    }
    Ok(format!("^{}$", regex_pattern)) // Anchor regex to match the whole path
}

/// Read the lines from a file and return an iterator over them
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Read the input lines from either STDIN or a file
///
/// If STDIN is detected, read from it
/// Otherwise, read from the file specified in the config or the default .gitignore file in the current directory
fn get_input_lines(config: &Config) -> Vec<String> {
    if atty::isnt(atty::Stream::Stdin) {
        // If STDIN is detected, read from it
        info!("Reading .gitignore rules from STDIN");
        let stdin = std::io::stdin();
        stdin
            .lock()
            .lines()
            .map_while(Result::ok)
            .collect::<Vec<String>>()
    } else {
        let input_file_name = config.input.as_deref().unwrap_or(".gitignore");
        let path = path::Path::new(&config.directory).join(input_file_name);
        info!("Reading .gitignore rules from file: {}", path.display());
        match read_lines(&path) {
            Ok(lines) => lines.map_while(Result::ok).collect(),
            Err(e) => {
                error!("Error reading file \"{}\": {}", path.display(), e);
                std::process::exit(1);
            }
        }
    }
}

/// create the header for the .megaignore file with the version and repository URL
///
/// # Example
///
/// ```text
/// # Start of generated .megaignore using git2megaignore v0.3.0
/// # For more info visit: https://github.com/Farfi55/git2megaignore.git
/// ```
fn get_header() -> String {
    let version: &str = env!("CARGO_PKG_VERSION");
    let pkg_name: &str = env!("CARGO_PKG_NAME");
    let repo_url: Option<&str> = option_env!("CARGO_PKG_REPOSITORY");

    let mut header = format!("# Start of generated .megaignore using {pkg_name} v{version}\n",);
    if let Some(repo_url) = repo_url {
        header.push_str(format!("# For more info visit: {repo_url}\n").as_str());
    }
    header.push('\n');
    header
}

/// Setup the logger with the specified verbosity level
fn setup_logger(level: u8) {
    let log_level = match level {
        0 => LevelFilter::Off,
        1 => LevelFilter::Error,
        2 => LevelFilter::Warn,
        3 => LevelFilter::Info,
        _ => LevelFilter::Debug,
    };
    env_logger::Builder::from_default_env()
        .filter_level(log_level)
        .format(|buf, record| writeln!(buf, "[{}] {}", record.level(), record.args()))
        .init();
}

fn main() {
    let config: Config = Config::parse();

    setup_logger(config.verbose);

    // Read the .gitignore file
    let lines: Vec<String> = get_input_lines(&config);

    let mut megaignore = String::new();

    if !config.no_extras {
        megaignore.push_str(&get_header());
    }

    if config.sync_megaignore {
        megaignore.push_str("+sync:.megaignore\n");
    }

    for line in lines {
        if is_not_gitignore_rule(&line) {
            if !config.copy_gitignore_lines {
                continue;
            }

            if line.is_empty() {
                megaignore.push('\n');
            } else {
                megaignore.push_str(&format!("# {}\n", line));
            }
        } else {
            let rule = IgnoreRule::from_line(&line, &config);
            megaignore.push_str(&rule.mega_rule);
        }
    }

    if !config.no_extras {
        megaignore.push_str("# End of generated .megaignore\n");
    }

    if config.print || atty::isnt(atty::Stream::Stdout) {
        // If STDOUT or requested or detected, print to it
        info!("Redirecting output to STDOUT");
        println!("{}", megaignore);
    } else {
        // Write the .megaignore file
        let file_name = config.output.as_deref().unwrap_or(".megaignore");
        let file_path = path::Path::new(&config.directory).join(file_name);

        let mut file = match File::create(&file_path) {
            Ok(file) => file,
            Err(e) => {
                error!("Error creating file \"{}\": {}", file_path.display(), e);
                std::process::exit(1);
            }
        };

        file.write_all(megaignore.as_bytes())
            .expect("Unable to write data");

        info!("Wrote .megaignore rules to file: {}", file_path.display());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_needs_regexp() {
        let needs_regexp = pattern_needs_regexp("foo");
        assert_eq!(needs_regexp, false);

        let needs_regexp = pattern_needs_regexp("*.txt");
        assert_eq!(needs_regexp, false);

        let needs_regexp = pattern_needs_regexp("foo/**/bar");
        assert_eq!(needs_regexp, true);

        let needs_regexp = pattern_needs_regexp("[Ff]oo");
        assert_eq!(needs_regexp, true);
    }

    #[test]
    fn test_glob_to_regex() {
        let regex = glob_to_regex("foo").unwrap();
        assert_eq!(regex, r"^foo$");

        let regex = glob_to_regex("*.txt").unwrap();
        assert_eq!(regex, r"^[^/]*\.txt$");

        let regex = glob_to_regex("foo/**/bar").unwrap();
        assert_eq!(regex, r"^foo/(.*/)?bar$");
    }
}
