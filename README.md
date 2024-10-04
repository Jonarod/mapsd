<div align="center">

# mapsd

Because sometimes `sed` needs an explicit key/value map.

[Get Started](#quickstart) - [Installation](#installation) - [Usage](#usage)

</div>

---

<div align="center">
Find & Replace text in multiple files using an explicit CSV of Before/After pairs.
</div>

---

<br/>
<br/>

# Quickstart

#### 1. Download [latest release](https://github.com/Jonarod/mapsd/releases) for your system
Or you can also build it from source using `cargo` (see [Installation](#installation)). Then move the binary somewhere like in `/usr/local/bin` *(just make sure it is some folder already in your `$PATH`)*


#### 2. Create a `.csv` file with 2 columns:

```csv
old_string1,new_string1
old_string2,new_string2
old_string3,new_string3
```

> ***WARNING: beware of the `spaces` and `separator` you put here !! Every character counts, even spaces***


#### 3. Replace all occurrences of the 1st column with the 2nd, over all `.html` files in the `./Documents` directory

```sh
mapsd "./Documents/**/*.html" -m ./my_map.csv
```

*(No worries, by default it applies the replacement in a copy of the files. When you feel you are ready you can add the `--DANGEROUSLY-REPLACE-INPLACE` flag to actually replace things in place.)*


<br/>
<br/>

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



<br/>
<br/>

# Installation

<details>
<summary>Download pre-compiled binaries</summary>
Go to the [releases](https://github.com/Jonarod/mapsd/releases), and download the lastest binary for your platform.
</details>


<details>
<summary>Build from source</summary>
[Install rust](https://www.rust-lang.org/tools/install), then:

```sh
git clone git@github.com:Jonarod/mapsd.git
cd mapsd
cargo build --release

# Move it somewhere, like this
sudo mv ./target/release/mapsd /usr/local/bin/
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


<br/>
<br/>

# Roadmap
- [ ] add a `--regex` flag to interpret each `key` in the `.csv` as a regex instead of a litteral string
- [ ] publish to crates.io automatically to be able to `cargo install mapsd`
