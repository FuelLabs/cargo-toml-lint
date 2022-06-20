use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use clap::{ArgEnum, Parser};

/// A `Cargo.toml` linter
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Skip cargo-based verification
    #[clap(long)]
    no_cargo_verify: bool,

    /// Require sorted dependency list
    #[clap(short = 'D', long, arg_enum, default_value = "strict")]
    sort_dependencies: DependencySorting,

    /// Require `[[test]]` entries to be sorted by name field
    #[clap(short = 'T', long, arg_enum, default_value = "enabled")]
    sort_tests: Toggle,

    /// Require arrays of objects (`[[foo]]`) to placed contiguously
    #[clap(short = 'A', long, arg_enum, default_value = "enabled")]
    contiguous_object_arrays: Toggle,

    /// Require exactly one end-of-line at end of file
    #[clap(short = 'N', long, arg_enum, default_value = "enabled")]
    single_end_of_line: Toggle,

    /// File to lint
    target: PathBuf,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum Toggle {
    #[clap(alias = "y", alias = "e")]
    Enabled,
    #[clap(alias = "n", alias = "d")]
    Disabled,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum DependencySorting {
    #[clap(alias = "n", alias = "d")]
    None,
    Section,
    Strict,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let contents = fs::read(&args.target)
        .map_err(|err| format!("Could not read {:?}: {:?}", &args.target, err))?;

    let ff: toml::Value = toml::from_slice(&contents)
        .map_err(|err| format!("Could not parse {:?}: {:?}", &args.target, err))?;

    if !args.no_cargo_verify {
        cargo_verify_project(&args.target)?;
    }

    match args.sort_dependencies {
        DependencySorting::None => {}
        DependencySorting::Section => {
            verify_section_sorted(&contents, "[dependencies]")
                .map_err(|err| format!("[dependencies] {}", err))?;
            verify_section_sorted(&contents, "[dev-dependencies]")
                .map_err(|err| format!("[dev-dependencies] {}", err))?;
        }
        DependencySorting::Strict => {
            if let Some(d) = ff.get("dependencies") {
                let d = d
                    .as_table()
                    .ok_or_else(|| "[dependencies] should be a table".to_owned())?;
                verify_deps_sorted_strict(d).map_err(|err| format!("[dependencies] {}", err))?;
            }

            if let Some(d) = ff.get("dev-dependencies") {
                let d = d
                    .as_table()
                    .ok_or_else(|| "[dev-dependencies] should be a table".to_owned())?;
                verify_deps_sorted_strict(d)
                    .map_err(|err| format!("[dev-dependencies] {}", err))?;
            }
        }
    }

    if args.sort_tests == Toggle::Enabled {
        if let Some(d) = ff.get("test") {
            let t = d
                .as_array()
                .ok_or_else(|| "[[test]] should be an array".to_owned())?;
            verify_list_of_objects_is_sorted_by_str(t, "name")
                .map_err(|err| format!("[[test]] {}", err))?;
        }
    }

    if args.contiguous_object_arrays == Toggle::Enabled {
        verify_contiguous_object_arrays(&contents)?;
    }

    if args.single_end_of_line == Toggle::Enabled {
        verify_single_end_of_line(&contents)?;
    }

    Ok(())
}

fn cargo_verify_project<P: AsRef<Path>>(path: P) -> Result<(), String> {
    let p = Command::new("cargo")
        .arg("verify-project")
        .arg("--manifest-path")
        .arg(path.as_ref())
        .output()
        .expect("failed to execute process");

    if p.status.success() {
        Ok(())
    } else {
        Err(format!(
            "cargo verify-project failed: {}",
            String::from_utf8_lossy(&p.stdout)
        ))
    }
}

fn verify_deps_sorted_strict(
    deps: &toml::map::Map<std::string::String, toml::Value>,
) -> Result<(), String> {
    let mut prev = None;
    for (name, _) in deps {
        if let Some(p) = prev {
            if p > name {
                return Err(format!(
                    "not sorted correctly (strict): {} is specified after {}",
                    name, p
                ));
            }
        }

        prev = Some(name);
    }

    Ok(())
}

/// Verify that a section is sorted if it exists
fn verify_section_sorted(toml_data: &[u8], header: &str) -> Result<(), String> {
    let mut in_section = false;
    let mut prev = None;
    for line in toml_data.split(|c| *c == b'\n') {
        let line: Vec<u8> = line
            .iter()
            .filter(|c| !c.is_ascii_whitespace())
            .copied()
            .collect();

        if line.is_empty() {
            continue;
        }

        if in_section {
            if line.starts_with(&[b'[']) {
                break;
            }

            if line.starts_with(&[b'#']) {
                continue;
            }

            let mut it = line.split(|c| *c == b'=');
            if let Some(dep_name) = it.next() {
                let dep_name = String::from_utf8_lossy(dep_name).into_owned();
                if let Some(p) = prev {
                    if p > dep_name {
                        return Err(format!(
                            "not sorted correctly: {} is specified after {}",
                            dep_name, p
                        ));
                    }
                }
                prev = Some(dep_name);
            }
        } else {
            in_section = line.starts_with(header.as_bytes());
        }
    }

    Ok(())
}

fn verify_list_of_objects_is_sorted_by_str(items: &[toml::Value], key: &str) -> Result<(), String> {
    let mut prev = None;
    for (i, item) in items.iter().enumerate() {
        if let Some(t) = item.as_table() {
            if let Some(v) = t.get(key) {
                if let Some(v) = v.as_str() {
                    if let Some(p) = prev {
                        if p > v {
                            return Err(format!(
                                "not sorted correctly: item at index {} with {}={} is specified after {}={}",
                                i, key, v, key, p
                            ));
                        }
                    }
                    prev = Some(v);
                } else {
                    return Err(format!("item at {}: key {} has a non-string value", i, key));
                }
            } else {
                return Err(format!("item at {} is missing key {}", i, key));
            }
        } else {
            return Err(format!("item at {} is not a table", i));
        }
    }

    Ok(())
}

fn verify_contiguous_object_arrays(toml_data: &[u8]) -> Result<(), String> {
    let mut seen_headers: Vec<String> = Vec::new();

    for line in toml_data.split(|c| *c == b'\n') {
        let line: Vec<u8> = line
            .iter()
            .filter(|c| !c.is_ascii_whitespace())
            .copied()
            .collect();

        if line.is_empty() {
            continue;
        }

        let is_header = line.starts_with(&[b'[']) && line.ends_with(&[b']']);
        let is_array_header = line.starts_with(&[b'[', b'[']) && line.ends_with(&[b']', b']']);

        if is_header {
            let line = String::from_utf8_lossy(&line).to_string();

            if is_array_header {
                if let Some(i) = seen_headers.iter().position(|s| s == &line) {
                    if i + 1 != seen_headers.len() {
                        return Err(format!(
                            "Items of {} are separated by other headers, for instance {}",
                            line,
                            seen_headers.last().unwrap()
                        ));
                    } else {
                        // Don't push duplicates
                        continue;
                    }
                }
            }

            seen_headers.push(line);
        }
    }

    Ok(())
}

fn verify_single_end_of_line(contents: &[u8]) -> Result<(), String> {
    if !contents.ends_with(b"\n") {
        return Err("File does not end with a new line".to_string());
    }

    if contents.ends_with(b"\n\n") || contents.ends_with(b"\r\n\r\n") {
        return Err("File ends with multiple new lines".to_string());
    }

    Ok(())
}
