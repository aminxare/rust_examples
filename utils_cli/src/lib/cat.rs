use std::{fs::read, path::Path};

/// Returns a String that is content of file
///
/// # Arguments
///
/// * `path` - file path
///
/// # Examples
///
/// ```
// /// use super::*;
// /// let content = cat("/foo.txt");
// /// println!("{}", content);
/// ```
pub fn cat(path: impl Into<String>) -> Result<String, Box<dyn std::error::Error>> {
    let path = path.into();
    let path = Path::new(&path);
    let contents = String::from_utf8(read(path)?)?;
    Ok(contents)
}

#[cfg(test)]
mod tests {
    use super::cat;
    use std::fs::{remove_file, File};
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn ex1() -> Result<(), Box<dyn std::error::Error>> {
        let text = "hello world".to_string();
        let temp_dir = tempdir()?;
        let path = temp_dir.path().join("foo.txt");

        let mut file = File::create(&path)?;
        let _ = write!(file, "{}", text);

        let content = cat(path.to_str().unwrap())?;

        assert_eq!(content, "hello world");

        remove_file(path)?;
        Ok(())
    }
}
