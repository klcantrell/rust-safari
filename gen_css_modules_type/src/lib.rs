use notify::{EventFn, RecommendedWatcher, RecursiveMode, Result as NotifyResult, Watcher};
use regex::Regex;
use std::fs::File;
use std::io::prelude::Read;
use std::io::{self, BufRead, Error, Write};
use std::path::{Path, PathBuf};
use unicode_segmentation::UnicodeSegmentation;

pub const PATH_TO_WATCH: &str = ".";

pub fn create_watcher(path: &str, handler: Box<dyn EventFn>) -> NotifyResult<RecommendedWatcher> {
  let mut watcher: RecommendedWatcher = Watcher::new_immediate(handler)?;
  watcher.watch(path, RecursiveMode::Recursive)?;
  Ok(watcher)
}

pub fn read_line(line: &mut String) -> Result<(), Error> {
  line.clear();
  io::stdin().lock().read_line(line)?;
  Ok(())
}

pub fn handle_on_modify(paths: Vec<PathBuf>) -> () {
  if !paths.is_empty() {
    let path = &paths[0];
    match path.extension() {
      Some(os_ext) => {
        if let Some("css") = os_ext.to_str() {
          let type_defs_filename = create_type_defs_filename(path);
          let contents = extract_file_contents(path);
          let type_defs = handle_css_change(&contents);
          save_type_defs(type_defs, &type_defs_filename);
        }
      }
      _ => (),
    }
  } else {
    println!("Unknown path, did nothing");
  }
}

fn handle_css_change(content: &str) -> String {
  parse_rules(content)
    .into_iter()
    .map(remove_dot)
    .map(type_defs_of_rules)
    .fold(String::from(""), create_type_def_file_content)
}

fn parse_rules(css: &str) -> Vec<&str> {
  let rule_pattern = Regex::new(r"\.\w+").unwrap();
  rule_pattern
    .find_iter(&css)
    .map(|match_| match_.as_str())
    .collect()
}

fn remove_dot(rule: &str) -> &str {
  let mut graphemes_iter = rule.graphemes(true);
  graphemes_iter.next();
  graphemes_iter.as_str()
}

fn type_defs_of_rules(rule_name: &str) -> String {
  format!("export const {}: string;", rule_name)
}

fn create_type_def_file_content(content: String, type_def: String) -> String {
  format!("{}{}\n", content, type_def)
}

fn extract_file_contents(path: &PathBuf) -> String {
  let mut retries = 0;
  let mut content = String::new();
  while retries < 1_000 {
    let file = File::open(path);
    if let Ok(mut file_contents) = file {
      if let Ok(_) = file_contents.read_to_string(&mut content) {
        break;
      } else {
        retries = retries + 1;
      }
    } else {
      retries = retries + 1;
    }
  }
  content
}

fn create_type_defs_filename(path: &PathBuf) -> PathBuf {
  let parent = match path.parent() {
    Some(parent) => parent,
    None => Path::new("unknown"),
  };
  let name = format!("{}.d.ts", extract_filename(path));

  Path::new(parent).join(name)
}

fn save_type_defs(content: String, path: &PathBuf) -> () {
  let mut retries = 0;
  while retries < 1_000 {
    let file = File::create(path);
    if let Ok(mut opened_file) = file {
      if let Ok(_) = opened_file.write_all(content.as_bytes()) {
        println!("Saved type def to {}", extract_filename(path));
        return ();
      } else {
        retries = retries + 1;
      }
    } else {
      retries = retries + 1;
    }
  }
  println!("Failed to save type def to {}", extract_filename(path));
}

fn extract_filename(path: &PathBuf) -> String {
  let unknown_name = "unknown";
  let name = match path.file_name() {
    Some(name) => match name.to_str() {
      Some(name) => name,
      None => unknown_name,
    },
    None => unknown_name,
  };

  String::from(name)
}
