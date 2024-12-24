# Gitignore to Megaignore converter

This is a simple tool to convert a `.gitignore` file to a `.megaignore` file. The `.megaignore` file is used by the [Mega](https://mega.nz) cloud storage service to exclude files from being uploaded to the cloud much like the `.gitignore` file is used to exclude files from being tracked by git.

You can read more about the `.megaignore` file format [here](https://help.mega.io/installs-apps/desktop/megaignore).

## Installation

```sh
cargo install git2megaignore
```

## Usage example

```sh
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

```txt:example/.megaignore
+sync:.megaignore
-nG:*.txt
-dnG:foo
-dpR:^[Bb]uild$
-pR:^src/(.*/)?index\.js$
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

## Mass conversion

you can run the following bash script to convert all `.gitignore` files in the subdirectories of the current directory to `.megaignore` files:

```sh
find . -name .gitignore -execdir git2megaignore \;
```

for windows, you can use the following powershell script:

```powershell
Get-ChildItem -Recurse -Filter .gitignore | ForEach-Object { git2megaignore $_.DirectoryName }
```
