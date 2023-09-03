#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::*;
use pulldown_cmark::{html, Options, Parser};
use std::fs::File;
use std::io::Read;

#[napi]
pub fn parse(markdown: String) -> String {
  let mut options = Options::empty();
  options.insert(Options::ENABLE_STRIKETHROUGH);
  let parser = Parser::new_ext(markdown.as_str(), options);
  let mut html_output = String::new();

  html::push_html(&mut html_output, parser);
  html_output
}

#[napi]
pub fn parse_markdown_file(markdown_path: String) -> Result<String, String> {
  let mut file = match File::open(markdown_path) {
    Ok(file) => file,
    Err(e) => {
      println!("{:?}", e);
      return Err(napi::Error::new(
        "200".to_string(),
        format!("{:?}", e).to_string(),
      ));
    } // Return None if there's an error opening the file
  };

  let mut content = String::new();
  if let Err(e) = file.read_to_string(&mut content) {
    println!("{:?}", e);
    return Err(napi::Error::new(
      "200".to_string(),
      format!("{:?}", e).to_string(),
    ));
  }
  println!("content -> {}", content);
  Ok(parse(content))
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::{fs::remove_dir, io::Write};

  #[test]
  fn test_parse() {
    let markdown = "Hello world, this is a ~~complicated~~ *very simple* example.";
    let expected_html =
      "<p>Hello world, this is a <del>complicated</del> <em>very simple</em> example.</p>\n";

    let actual_html = parse(markdown.into());

    assert_eq!(actual_html, expected_html);
  }

  #[test]
  fn test_parse_markdown_file() {
    let tmp_dir = tempfile::tempdir().expect("cannot create temp-dir");
    let path = tmp_dir.path().join("test.md").to_str().unwrap().to_string();
    let mut file = File::create(&path).expect("cannot create temp-file");

    let markdown = "Hello world, this is a ~~complicated~~ *very simple* example.";
    let expected_html =
      "<p>Hello world, this is a <del>complicated</del> <em>very simple</em> example.</p>\n"
        .to_string();

    file
      .write_all(format!("{}", markdown).as_bytes())
      .expect("fail to write in temp-file");

    let actual_html = parse_markdown_file(path).unwrap();
    assert_eq!(actual_html, expected_html);

    // it should return None on error
    let path = tmp_dir.path().join("a.txt").to_str().unwrap().to_string();
    let actual_html = parse_markdown_file(path.clone());
    assert!(actual_html.is_err());    
    
    let _ = remove_dir(tmp_dir);
  }
}
