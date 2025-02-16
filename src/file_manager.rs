use std::fs::{create_dir_all, File, OpenOptions};
use std::io::Write;
use std::io::{Read, Result};

pub struct FileManager {
    db_path: String,
}

impl FileManager {
    pub fn new(db_path: &str) -> Self {
        create_dir_all(db_path).expect("failed to create database directory.");
        Self {
            db_path: db_path.to_string(),
        }
    }

    pub fn create_file(&self, file_name: &str) -> Result<File> {
        let path = format!("{}/{}", self.db_path, file_name);
        OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(&path)
    }

    pub fn write_to_file(&self, file_name: &str, content: &str) -> Result<()> {
        let mut file = self.create_file(file_name)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    pub fn read_from_file(&self, file_name: &str) -> Result<String> {
        let path = format!("{}/{}", self.db_path, file_name);
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn setup_temp_dir() -> PathBuf {
        let temp_dir = std::env::current_dir().unwrap().join("test_output");
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).expect("Failed to clean up test dir.");
        }

        create_dir_all(&temp_dir).expect("Failed to create test dir.");
        temp_dir
    }

    fn setup_test() -> (PathBuf, FileManager) {
        let temp_dir = setup_temp_dir();
        let file_manager = FileManager::new(temp_dir.to_str().unwrap());

        return (temp_dir, file_manager);
    }

    #[test]
    fn test_create_file() {
        let temp_dir = setup_temp_dir();
        let file_manager = FileManager::new(temp_dir.to_str().unwrap());

        let file_name = "test_file.txt";
        let result = file_manager.create_file(file_name);

        assert!(result.is_ok(), "File creation should succeed");

        let file_path = temp_dir.join(file_name);
        assert!(file_path.exists(), "File should exist after creation")
    }

    #[test]
    fn test_write_to_file() {
        let temp_dir = setup_temp_dir();
        let file_manager = FileManager::new(temp_dir.to_str().unwrap());

        let file_name = "test_write.txt";
        let content = "Hello, me!";

        let result = file_manager.write_to_file(file_name, content);
        assert!(result.is_ok(), "Writing to file should succeed.");

        let file_path = temp_dir.join(file_name);
        let file_content = fs::read_to_string(file_path).expect("failed to test file");
        assert_eq!(
            file_content, content,
            "File content should match expected content."
        );
    }

    #[test]
    fn test_read_from_file() {
        let (_temp_dir, file_manager) = setup_test();
        let file_name = "test_read.txt";
        let expected_content = "This is a test.";

        file_manager
            .write_to_file(file_name, expected_content)
            .expect("Failed to write to test file");

        let result = file_manager.read_from_file(file_name);
        assert!(result.is_ok(), "Reading from file should succeed.");
        assert_eq!(
            result.unwrap(),
            expected_content,
            "Read content should match expected content"
        )
    }
}
