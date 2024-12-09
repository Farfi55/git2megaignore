# Gitignore to Megaignore converter

This is a simple tool to convert a `.gitignore` file to a `.megaignore` file. The `.megaignore` file is used by the [Mega](https://mega.nz) cloud storage service to exclude files from being uploaded to the cloud.

You can read more about the `.megaignore` file format [here](https://help.mega.io/installs-apps/desktop/megaignore).

## Installation

```sh
cargo install git2megaignore
```

## Usage

```sh
cd /path/to/your/project
git2megaignore
```

This will create a `.megaignore` file in the current directory.

## Mass conversion

you can run the following script to convert all `.gitignore` files in the subdirectories of the current directory to `.megaignore` files:

```sh
find . -name .gitignore -execdir git2megaignore \;
```
