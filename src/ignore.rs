use std::fs;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct IgnoreRules {
    pub global_ignore: Vec<String>,
    pub local_ignore: Vec<String>,
    pub exceptions: Vec<String>,
}

pub fn parse_gitignore(file_path: &Path) -> io::Result<IgnoreRules> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut global_ignore = Vec::new();
    let mut local_ignore = Vec::new();
    let mut exceptions = Vec::new();

    for line in reader.lines() {
        let line = line?.trim().to_string();

        // Skip blank lines and comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Handle exceptions (negations)
        if line.starts_with('!') {
            exceptions.push(line[1..].to_string());
            continue;
        }

        // Patterns starting with `/` or `**` are global ignore patterns
        if line.starts_with('/') || line.starts_with("**/") {
            global_ignore.push(line.clone());
            local_ignore.push(line);
        } else {
            // All other patterns are local ignore patterns
            local_ignore.push(line);
        }
    }

    Ok(IgnoreRules {
        global_ignore,
        local_ignore,
        exceptions,
    })
}

pub fn is_ignored(path: &PathBuf, rules: &IgnoreRules) -> bool {
    let path_str = path.to_str().unwrap_or("");

    // Check exceptions first
    for pattern in &rules.exceptions {
        if let Ok(matcher) = glob::Pattern::new(pattern) {
            if matcher.matches(path_str) {
                return false; // Path is explicitly allowed by an exception
            }
        }
    }

    // Check against global ignore patterns
    for pattern in &rules.global_ignore {
        if let Ok(matcher) = glob::Pattern::new(pattern) {
            if matcher.matches(path_str) {
                return true; // Path matches a global ignore pattern
            }
        }
    }

    // Check against local ignore patterns
    for pattern in &rules.local_ignore {
        if let Ok(matcher) = glob::Pattern::new(pattern) {
            if matcher.matches(path_str) {
                return true; // Path matches a local ignore pattern
            }
        }
    }

    false // Path does not match any ignore pattern
}
