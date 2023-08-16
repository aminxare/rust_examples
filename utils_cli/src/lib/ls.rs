use std::{
    error::Error,
    fs::read_dir,
    os::unix::prelude::{MetadataExt, PermissionsExt},
    path::Path,
    time::SystemTime,
};

use chrono::{DateTime, Local};

/// lists directories
pub fn ls(path: &str, full: bool) -> Result<Vec<String>, Box<dyn Error>> {
    let path = Path::new(path);
    let mut paths = Vec::new();
    for entry in read_dir(path)? {
        let entry = entry?;
        let mut file_name = entry
            .file_name()
            .into_string()
            .or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
        let metadata = entry.metadata()?;

        if full == true {
            file_name = append_to_filename(file_name, parse_systemtime(metadata.created()?));
            file_name = append_to_filename(file_name, parse_systemtime(metadata.modified()?));
            file_name = append_to_filename(file_name, metadata.len().to_string());
            file_name = append_to_filename(file_name, metadata.gid().to_string());
            file_name = append_to_filename(file_name, metadata.uid().to_string());
            file_name =
                append_to_filename(file_name, parse_permission(metadata.permissions().mode()));
        }

        paths.push(file_name);
    }
    Ok(paths)
}

fn parse_permission(permission: u32) -> String {
    unix_mode::to_string(permission)
}

fn parse_systemtime(st: SystemTime) -> String {
    let time: DateTime<Local> = DateTime::from(st);
    time.format("%d_%b_%H:%M").to_string()
}

fn append_to_filename(filename: String, txt: impl Into<String>) -> String {
    let txt = txt.into();
    format!("{:>10} {filename}", txt)
}

#[cfg(test)]
mod test {
    use super::ls; // Assuming your code is in a module named "super"
    use std::fs::File;
    use tempfile::TempDir;

    #[test]
    fn test_ls_with_full() {
        // Create a temporary directory and files for testing
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let file1_path = temp_dir.path().join("file1.txt");
        let file2_path = temp_dir.path().join("file2.txt");
        let file3_path = temp_dir.path().join("file3.txt");

        File::create(&file1_path).expect("Failed to create file1");
        File::create(&file2_path).expect("Failed to create file2");
        File::create(&file3_path).expect("Failed to create file3");

        // Call the ls function with full details
        let result = ls(temp_dir.path().to_str().unwrap(), true);

        // Assert that the ls function returns successfully
        assert!(result.is_ok());

        // Extract the output paths from the result
        let paths = result.unwrap();

        // Assert that the number of paths matches the number of files in the directory
        assert_eq!(paths.len(), 3);
    }

    #[test]
    fn test_ls_without_full() {
        // Create a temporary directory and files for testing
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let file1_path = temp_dir.path().join("file1.txt");
        let file2_path = temp_dir.path().join("file2.txt");
        let file3_path = temp_dir.path().join("file3.txt");

        File::create(&file1_path).expect("Failed to create file1");
        File::create(&file2_path).expect("Failed to create file2");
        File::create(&file3_path).expect("Failed to create file3");

        // Call the ls function without full details
        let result = ls(temp_dir.path().to_str().unwrap(), false);

        // Assert that the ls function returns successfully
        assert!(result.is_ok());

        // Extract the output paths from the result
        let paths = result.unwrap();

        // Assert that the number of paths matches the number of files in the directory
        assert_eq!(paths.len(), 3);

        // Assuming you're using a simple format without full details
        // Modify this assertion based on the actual format your function uses
        assert!(
            !paths[0].contains("gid:") && !paths[0].contains("uid:") && !paths[0].contains("mode:")
        );
    }
}
