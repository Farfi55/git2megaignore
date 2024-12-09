// this executable reads the .gitignore file in the current directory
// and creates a new file called .megaignore that contains the same ignore rules
// but using the .megaignore format, trying to keep the same rules as much as possible

// from https://help.mega.io/installs-apps/desktop/megaignore
// How does it work?

// You can set .megaignore rules to be shared across devices when the synced folder is synced to multiple devices. This is achieved by syncing the .megaignore file itself, so that it is present in the sync on those other devices too, and changes to the rules will also be synced. By default .megaignore files are not synced.

// Rules in the local copy of a .megaignore file are always in effect. You can’t make the sync ignore a .megaignore file itself, though you can exclude its parent folder (provided it’s not in the sync root), and then it would not be part of the sync anymore. If there is no local .megaignore, and there is one in the cloud then that one will be downloaded before syncing proceeds in that folder and below.

// Most syncs would just contain one .megaignore file in the sync root with its rules deciding inclusion and exclusion for each folder and file in the whole sync. If a sync starts without one, a default one will be added. Additional rules for excluding or re-including various names can be added in other folders in the sync, and they will apply in that folder and below.

// Moving, deleting, or renaming .megaignore files can mean that the rules from those files are suddenly not present, causing previously excluded files and folders to be synced, so caution is advised there.

// If a .megaignore file contains errors, such as not following the syntax below, or due to one or more of the contained rules not being understandable when loaded into the sync, then that folder cannot be synced, and a sync stall will result. Other parts of the sync may be affected too, as it may not be possible to determine which folders to scan and whether missing items were moved or not. The user can check the sync stall issues to see where these problems occurred, and then fix them.

// Here are instructions on how to view and edit .megaignore files.
// What’s in a .megaignore file?

// .megaignore files can contain comments, white space characters, and filters.

// Comments are used for documentation purposes and are ignored when loading a .megaignore file. You start a comment by beginning a line with the # character. Note that the # character must not be preceded by any whitespace and that the comment ends at the end of the line.

// You can find more information about white space characters and how to use them here.

// Filters are specified line by line and come in two forms: Name filters and size filters.
// Name filters

// Name filters are matched against a file’s name or type. You can use these kinds of filters to exclude folders or files whose name matches a specific pattern.

// Name filters have the general format: <CLASS><TARGET><TYPE><STRATEGY>:<PATTERN>
// <CLASS>

// Filters must be one of the following two classes: exclude or include.

// -: Filters of the exclude class specify files or folders that should not be synchronized.
// This class is indicated by the – character.

// +: Filters of the include class specify files or folders that should be synchronized.
// This class is indicated by the + character.
// <TARGET>

// Filters may have one of the following three targets: all, folder, file or symlink.

// a : The all target specifies that this filter applies to files, folders and symbolic links and is indicated by the a character.

// d : The directory target specifies that this filter applies only to folders and is indicated by the d character.

// f : The file target specifies that this filter applies only to files and is indicated by the f character.

// s : The symlink target specifies that this filter applies only to symbolic links and is indicated by the s character.

// The a all target will be used if you do not explicitly specify a target.
// <TYPE>

// Filters may be one of the following three types: local name, subtree name or path.

// N : local name filters have an effect only in the the folder containing the .megaignore file.

// n : subtree name filters have an effect in all folders below the one containing the .megaignore file, as well as that folder.

// p : path filters are indicated by the p character and are matched against the file or folder’s path relative to the folder containing the .megaignore file. Note that the path separator is always /, even on Windows.

// The n subtree name type will be used if you do not specify a type explicitly. If not specified, default to p or n based on the presence of /, i.e. whether the user seems to be trying to use a path.
// <STRATEGY>

// Filters may use one of the following two match strategies: glob or regexp.

// Gg : glob filters are indicated by the g or G characters and match against a name or path using a wildcard pattern.

// Rr : regexp filters are indicated by the r or R characters and match against a name or path using a pattern expressed as a POSIX Extended Regular Expression.

// The glob match strategy will be used if you do not explicitly specify a match strategy.

// Uppercase G or R specifies that matching should be case-sensitive.
// <SYNC_THIS_FILE>

// Use this exact syntax on one line to turn on or turn off syncing of the .megaignore files itself, in order to share or not share the rules in the file with other syncs of the same folder. By default, the file is not synced.

// +sync:.megaignore

// -sync:.megaignore
// Examples

// # This is a comment.

// Everything after and including the # is ignored.

// -f:*.txt

// This is a subtree name exclusion matched using a wildcard pattern.
// It will exclude all *.txt files in and beneath the folder containing the .megaignore file.

// +fg:work*.txt

// This is a subtree name inclusion matched using a wildcard pattern. It will include all work*.txt files excluded by *.txt above.

// -fn:*.jpg

// This is also a subtree name exclusion matched using a wildcard pattern. It will exclude all *.jpg files in and beneath this folder.

// +fng:family*.jpg

// This is also a subtree name inclusion matched using a wildcard pattern. It will include all family*.jpg files previously excluded by the *.jpg exclusion above.

// -N:*.avi

// This is a local name exclusion matched using a wildcard pattern. It will exclude all *.avi files contained directly by this folder.

// -fp:video/*.avi

// This is a path exclusion matched using a wildcard pattern. It will exclude all *.avi files in the video subfolder.

// +fpg:video/family/*.avi

// This is a path inclusion matched using a wildcard pattern. It will include all *.avi files in the video/family subfolder.

// -nr:.*foo.*

// This is a subtree name exclusion matched using a regular expression. It will exclude all files whose name contains foo.

// -d:private

// This will exclude all directories with the name private.

// -:*

// Exclude everything in and below this folder.
// Size filters

// Size filters match against a file’s size and can be used to exclude files whose size is above, below or between a specified threshold.

// Size filters are only intended to avoid adding files to the sync. Once a file is synced, size filters have no further effect. Therefore, adjusting the size thresholds won’t stop already synced files from continuing to be synced, and neither will edits of files that result in a different size.

// Size filters have the general form: <THRESHOLD>:<VALUE><UNIT>
// <THRESHOLD>

// The threshold element specifies which bound you’re specifying a value for.

// There are two thresholds that you can specify:

// exclude-smaller : Specifies the lower-bound. Files smaller than the specified value will be excluded.
// exclude-larger : Specifies the upper bound. Files larger than the specified value will be excluded.

// If you specify both thresholds, files with a size outside of the specified range will be excluded.

// It is invalid to specify a lower bound greater than (or equal to) the upper bound.
// <VALUE>

// The value element states the bound for a specified threshold.

// Any non-negative integer value is valid.
// <UNIT>

// The optional unit element specifies which unit the value is stated in.

// K : The value is specified in kilobytes.
// M : The value is specified in megabytes.
// G : The value is specified in gigabytes.

// Units are not case-sensitive so k would also specify kilobytes.

// If no unit is specified, the value is interpreted as being in bytes.
// Examples

// exclude-smaller:4k

// Exclude files smaller than 4 kilobytes.

// exclude-larger:8M

// Exclude files larger than 8 megabytes.

// exclude-smaller:4K
// exclude-larger:8m

// Exclude files smaller than 4 kilobytes and larger than 8 megabytes.
// Caveats

// Note that when a .megaignore file is being downloaded, its rules do not come into effect until that download has completed and the .megaignore file has been successfully loaded. It’s very important to keep this in mind if you try to move .megaignore files around in the cloud.

// An example of how this behavior might surprise you, consider the case of moving a .megaignore file up a folder hierarchy. You might expect that the .megaignore file’s rules remain in effect at the source folder until the .megaignore file appears at its new location.

// This is not necessarily the case as the sync engine may interpret these events as the .megaignore file being removed at the source and a new .megaignore file being added at the target. In this case, the rules present at the source are purged and are only reapplied when the .megaignore file has been redownloaded.

// If you’re moving .megaignore files around and it’s important that the filters stay in effect at the source until the move has been completed, you will want to copy the .megaignore file to the new location first and then remove the original after the copy has been synchronized.

// from https://git-scm.com/docs/gitignore#_pattern_format
// .gitignore file format
// PATTERN FORMAT

// A blank line matches no files, so it can serve as a separator for readability.

// A line starting with # serves as a comment. Put a backslash ("\") in front of the first hash for patterns that begin with a hash.

// Trailing spaces are ignored unless they are quoted with backslash ("\").

// An optional prefix "!" which negates the pattern; any matching file excluded by a previous pattern will become included again. It is not possible to re-include a file if a parent directory of that file is excluded. Git doesn’t list excluded directories for performance reasons, so any patterns on contained files have no effect, no matter where they are defined. Put a backslash ("\") in front of the first "!" for patterns that begin with a literal "!", for example, "\!important!.txt".

// The slash "/" is used as the directory separator. Separators may occur at the beginning, middle or end of the .gitignore search pattern.

// If there is a separator at the beginning or middle (or both) of the pattern, then the pattern is relative to the directory level of the particular .gitignore file itself. Otherwise the pattern may also match at any level below the .gitignore level.

// If there is a separator at the end of the pattern then the pattern will only match directories, otherwise the pattern can match both files and directories.

// For example, a pattern doc/frotz/ matches doc/frotz directory, but not a/doc/frotz directory; however frotz/ matches frotz and a/frotz that is a directory (all paths are relative from the .gitignore file).

// An asterisk "*" matches anything except a slash. The character "?" matches any one character except "/". The range notation, e.g. [a-zA-Z], can be used to match one of the characters in a range. See fnmatch(3) and the FNM_PATHNAME flag for a more detailed description.

// Two consecutive asterisks ("**") in patterns matched against full pathname may have special meaning:

// A leading "**" followed by a slash means match in all directories. For example, "**/foo" matches file or directory "foo" anywhere, the same as pattern "foo". "**/foo/bar" matches file or directory "bar" anywhere that is directly under directory "foo".

// A trailing "/**" matches everything inside. For example, "abc/**" matches all files inside directory "abc", relative to the location of the .gitignore file, with infinite depth.

// A slash followed by two consecutive asterisks then a slash matches zero or more directories. For example, "a/**/b" matches "a/b", "a/x/b", "a/x/y/b" and so on.

// Other consecutive asterisks are considered regular asterisks and will match according to the previous rules.

use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct GitIgnoreRule {
    /// The raw line from the .gitignore file
    raw_line: String,

    /// Whether the rule is an exclude or include rule
    /// default: exclude
    is_exclude: bool,

    /// The pattern to match
    pattern: String,

    /// The pattern split into
    parts: Vec<String>,

    /// Whether the pattern is relative to the .gitignore file
    /// default: false
    is_relative: bool,

    /// The target of the rule
    /// default: All
    target: GitIgnoreRuleTarget,
}

#[derive(Debug)]
enum GitIgnoreRuleTarget {
    All,
    Folder,
    File,
}

fn skip_line(line: &str) -> bool {
    let line = line.trim();
    line.is_empty() || line.starts_with("#")
}

impl GitIgnoreRule {
    fn from_line(line: &str) -> Self {
        let line = line.trim();
        let mut is_exclude = true;
        let mut is_relative = false;
        let mut target = GitIgnoreRuleTarget::All;
        let mut pattern = String::new();
        let mut parts = Vec::new();

        if line.starts_with("!") {
            is_exclude = false;
            pattern = line[1..].to_string();
        } else {
            pattern = line.to_string();
        }

        parts = pattern.split("/").map(|s| s.to_string()).collect();
        parts.retain(|s| !s.is_empty());

        if pattern.starts_with("/") || parts.len() > 1 {
            is_relative = true;
        }

        if pattern.ends_with("/") {
            target = GitIgnoreRuleTarget::Folder;
            pattern.pop();
        }

        Self {
            raw_line: line.to_string(),
            is_exclude,
            pattern,
            is_relative,
            target,
            parts,
        }
    }

    fn to_megaignore(&self) -> String {
        let mut megaignore = String::new();
        let exclude = if self.is_exclude { "-" } else { "+" };
        let relative = if self.is_relative { "p" } else { "n" };
        let target = match self.target {
            GitIgnoreRuleTarget::All => "a",
            GitIgnoreRuleTarget::Folder => "d",
            GitIgnoreRuleTarget::File => "f",
        };

        megaignore.push_str(&format!(
            "# {}\n{}{}{}{}:{}\n",
            self.raw_line, exclude, target, relative, "G", self.pattern
        ));

        megaignore
    }
}

fn main() {
    let current_dir = std::env::current_dir().unwrap_or("Current directory not found".into());

    // Read the .gitignore file
    if let Ok(lines) = read_lines(".gitignore") {
        let mut megaignore = String::new();

        megaignore.push_str("# Start of generated .megaignore using git2megaignore\n");
        megaignore.push_str("+sync:.megaignore\n");

        for line in lines {
            if let Ok(line) = line {
                if skip_line(&line) {
                    if line.is_empty() {
                        megaignore.push_str("\n");
                    } else {
                        megaignore.push_str(&format!("# {}\n", line));
                    }
                } else {
                    let rule = GitIgnoreRule::from_line(&line);
                    megaignore.push_str(&rule.to_megaignore());
                }
            }
        }

        megaignore.push_str("# End of generated .megaignore\n");

        // println!("{}", megaignore);

        let mut file = File::create(".megaignore").expect("Unable to create file");
        file.write_all(megaignore.as_bytes())
            .expect("Unable to write data");

        // print success message and current directory
        println!(
            "Successfully created .megaignore file in {}",
            current_dir.display()
        );
    } else {
        eprintln!(
            "Unable to read .gitignore file in {}",
            current_dir.display()
        );
    }
}
