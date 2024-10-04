use glob::glob;
use log::{error, info};
use memchr::memmem;
use rayon::prelude::*;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufReader, BufWriter, Read, Write},
    path::Path,
    sync::Arc,
};

use crate::arguments::Opt;

pub mod arguments;

pub fn run(opt: &Opt) -> io::Result<()> {
    // Step 1: Read the CSV file and store the key/value pairs in a HashMap
    let key_value_map = read_csv(&opt.map, &opt.delimiter, opt.has_headers)?;
    let key_value_map = Arc::new(key_value_map);

    // Step 2: Use glob pattern to match files
    let paths: Vec<_> = glob(&opt.files)
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
        .filter(|path| path.is_file())
        .collect();

    // Step 3: Process files in parallel
    paths.par_iter().for_each(|path| {
        if let Err(e) = replace_in_file(path, Arc::clone(&key_value_map), opt.inplace, &opt.prefix)
        {
            error!("Failed to process file {}: {}", path.display(), e);
        }
    });

    Ok(())
}

pub fn read_csv(
    file_path: &str,
    delimiter: &str,
    has_headers: bool,
) -> io::Result<HashMap<String, String>> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut map = HashMap::new();
    let mut lines = content.lines();

    if has_headers {
        lines.next(); // Skip the header line
    }

    for line in lines {
        let parts: Vec<&str> = line.split(delimiter).collect();
        if parts.len() >= 2 {
            let key = parts[0].to_string();
            let value = parts[1..].join(delimiter);
            map.insert(key, value);
        }
    }

    Ok(map)
}

pub fn replace_in_file(
    path: &Path,
    key_value_map: Arc<HashMap<String, String>>,
    inplace: bool,
    prefix: &str,
) -> io::Result<()> {
    // Step 4: Read the contents of the file
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut content = Vec::new();
    reader.read_to_end(&mut content)?;

    // Step 5: Replace occurrences of each key with the corresponding value
    let mut modified = false;
    for (key, value) in key_value_map.iter() {
        let key_bytes = key.as_bytes();
        let value_bytes = value.as_bytes();

        let mut new_content = Vec::new();
        let mut last_match = 0;
        for match_index in memmem::find_iter(&content, key_bytes) {
            modified = true;
            new_content.extend_from_slice(&content[last_match..match_index]);
            new_content.extend_from_slice(value_bytes);
            last_match = match_index + key_bytes.len();
        }

        if modified {
            new_content.extend_from_slice(&content[last_match..]);
            content = new_content;
        }
    }

    // Step 6: Write the modified contents back to the file or create a new file
    if modified {
        let output_path = if inplace {
            path.to_path_buf()
        } else {
            let parent = path.parent().unwrap_or(Path::new(""));
            let filename = path.file_name().unwrap();
            parent.join(format!("{prefix}{}", filename.to_str().unwrap()))
        };

        let file = File::create(&output_path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(&content)?;
        writer.flush()?;

        info!("{}", output_path.display());
    }

    Ok(())
}
