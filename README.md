---
<div class="center">

# mapsd

Because sometimes `sed` needs an explicit key/value map.

[Get Started](#quickstart) - [Installation](#installation) - [Usage](#usage)

</div>
---

Find & Replace text in multiple files using an explicit CSV of Before/After pairs.

---

# Quickstart

1. Install

Download [latest release](https://github.com/Jonarod/mapsd/releases) binary for your system and move somewhere matching your `$PATH`, like in `/usr/local/bin`.

Or you can also build it from source using `cargo` (see [Installation](#installation))

2. Create a `.csv` file named `map.csv` with 2 columns:

```csv
old_string1,new_string1
old_string2,new_string2
old_string3,new_string3
```
**WARNING: beware of the `spaces` and `separator` you put here !! Every character counts, even spaces**


3. For all files ending with `.html` in the directory `~/Documents/` and its subdirectories: replace all occurrences of the old `key` with its corresponding new `value` found in `./map.csv`, and create a new resulting file prefixed with `replaced.` (e.g: `replaced.my_file.html`) without touching the original one. (if you need to replace the same file in-place, use the `--DANGEROUSLY-REPLACE-INPLACE` flag)

```sh
mapsd "./Documents/**/*.html" -m ./map.csv
```


4. Check (Usage)[#usage] to have a list of all things you can customize to fit your needs, or directly read the manpage:

```sh
mapsd --help
```


# Usage

```sh
mapsd 0.0.1
Find & Replace text in multiple files using an explicit CSV of Before/After pairs.

USAGE:
    mapsd [FLAGS] [OPTIONS] <FILES>

FLAGS:
        --has-headers                    CSV has headers
    -h, --help                           Prints help information
        --DANGEROUSLY-REPLACE-INPLACE    Replace files in-place (USE WITH CAUTION)
        --silent                         Suppress output
    -V, --version                        Prints version information

OPTIONS:
    -d, --delimiter <delimiter>    CSV delimiter [default: ,]
    -m, --map <map>                Path to the CSV file containing key/value pairs [default: map.csv]
    -p, --prefix <prefix>          Prefix to use for the resulting copied file [default: replaced.]

ARGS:
    <FILES>    Files to process (glob pattern)
```


The quickstart is quite explicit, but here are some tips & tricks.

- **Beware of spaces in the `.csv` file:** `old_string1,new_string1` is not the same thing as `old_string1, new_string1` nor `old_string1,new_string1 `. All characters count and WILL be matched/replaced as-is.


- **Pick the right delimiter:** the default delimiter is `,` but of course that means both your old `key` AND new `value` cannot contain the same character `,`. In some cases this cannot work, so you will need to get creative and maybe invent some new delimiters to be more explicit and avoid conflicts. In such case, just provide it with the `--delimiter` or `-d` flag, for example this could your delimiter if you wanted to `--delimiter "==="`, but if so, your `.csv` file should look like this:

    ```csv
    old_string1===new_string1
    old_string2=== new_string2
    old_string3===new_string3
    ```
    *(NOTE: in the above example, all occurences of the string `old_string2` will be replaced with the string ` new_string2` (with a space at the beginning)*

- **How to delete?** simply leaving the new `value` part empty, like this:
    ```csv
    old_string1===new_string1
    old_string2=== new_string2
    old_string3===
    ```
    in this example, all occurences of the string `old_string3` will be removed from all matching files.


- **My csv has headers:** add the `--has-headers` flag, and `mapsd` will skip the first line of your csv.

- **Change files directly without copy:** add the `--DANGEROUSLY-REPLACE-INPLACE` flag which, as its name suggests... comes with great responsibilities ;)

- **What happens under the hood?**: From a high perspective `mapsd` will parse your provided csv `./map.csv`, then load each `key/value` pairs into memory. Then it will look for each files matching your glob pattern. Once it has both `key/value` pairs and all matching documents paths into memory, it will spawn several workers/threads (using `rayon`) that will open each file to search each `key` and replace them with the corresponding `value`. For each file matched, it will create a copy of it with the prefix `replaced.`, with the relevant replacements. Each original filepath will be printed to console as they are treated (unless the `--silent` flag is provided) so that one can simply trash the files if needed. Once you are confortable with your changes, just repeat the process adding the `--DANGEROUSLY-REPLACE-INPLACE` flag which, as its name suggests... comes with great responsibilities ;) Now under the hood it involves just a bit more efficiency efforts to stream inputs/outputs and optimize search/replace,... check out the code.


# Installation

<details>
<summary>Download pre-compiled binaries</summary>
Go to the [releases](https://github.com/Jonarod/mapsd/releases), and download the lastest binary for your platform.
</details>


<details>
<summary>Build from source</summary>
Just `git clone` this repo and `cd` into it.
Then [install rust](https://www.rust-lang.org/tools/install), then do:
```sh
cargo build --release
```

This will build the binary and place it into `./target/release/mapsd`. You can just move it somewhere like in `/usr/local/bin/`:
```sh
mv ./target/release/mapsd /usr/local/bin/
```

Check everything is fine:
```sh
mapsd --version
```
</details>

<details>
<summary>Install using `cargo`</summary>
Coming soon...
</details>


# Roadmap
- [ ] add a `--regex` flag to interpret each `key` in the `.csv` as a regex instead of a litteral string
- [ ] publish to crates.io automatically to be able to `cargo install mapsd`
