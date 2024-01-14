use std::{fs};
use std::fs::Metadata;
use std::path::Path;
use crate::cached_metadata::CachedMetadata;
use crate::visitable::Visitable;

pub struct FileSystemTraversal;

impl FileSystemTraversal {

    pub (crate) fn traverse(&self, path: &Path, metadata: &mut CachedMetadata, visitors: &mut [&mut dyn Visitable]) {
        for visitor in &mut *visitors {
            visitor.visit(&path, metadata);
        }

        if metadata.is_dir() && !metadata.is_symlink() {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries {
                    let e = entry.unwrap();
                    let mut cached_metadata = CachedMetadata::new(&e.path());
                    self.traverse(&e.path(), &mut cached_metadata, visitors);
                }
            } else {
                // Todo do something different here
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use std::fs;
//     use std::path::Path;
//     use crate::visitable::Visitable;
//     use super::*;
//
//     // Mock Visitable implementation for testing
//     struct MockVisitor {
//         visited_paths: Vec<String>,
//     }
//
//     impl Visitable for MockVisitor {
//         fn visit(&mut self, path: &Path, _is_dir: bool) {
//             self.visited_paths.push(path.to_str().unwrap().to_string());
//         }
//
//         fn recap(&mut self) {
//         }
//     }
//
//     #[test]
//     fn test_traverse_single_file() {
//         let mut mock_visitor = MockVisitor { visited_paths: Vec::new() };
//
//         // Create a Vec<&mut dyn Foo> and add mutable references to the implementations
//         let mut visitors: Vec<&mut dyn Visitable> = Vec::new();
//         visitors.push(&mut mock_visitor);
//
//         let traversal = FileSystemTraversal {};
//         let test_file = Path::new("path_to_single_file");
//
//         // Create a single file for testing
//         fs::write(&test_file, "Test content").expect("Unable to create test file");
//         traversal.traverse(&test_file, false, &mut visitors);
//
//         assert_eq!(mock_visitor.visited_paths, vec![test_file.to_str().unwrap().to_string()]);
//
//         // Clean up: Delete the created file
//         fs::remove_file(&test_file).expect("Unable to delete test file");
//     }
//
//     #[test]
//     fn test_traverse_directory_structure() {
//
//         let mut mock_visitor = MockVisitor { visited_paths: Vec::new() };
//
//         // Create a Vec<&mut dyn Foo> and add mutable references to the implementations
//         let mut visitors: Vec<&mut dyn Visitable> = Vec::new();
//         visitors.push(&mut mock_visitor);
//
//         let traversal = FileSystemTraversal {};
//         let test_directory = Path::new("test_directory");
//
//         // Create a directory structure for testing
//         fs::create_dir_all(test_directory.join("subdir1/subdir2")).expect("Unable to create test directory structure");
//         fs::write(test_directory.join("file1.txt"), "Content").expect("Unable to create test file");
//         fs::write(test_directory.join("subdir1/file2.txt"), "Content").expect("Unable to create test file");
//         fs::write(test_directory.join("subdir1/subdir2/file3.txt"), "Content").expect("Unable to create test file");
//
//         traversal.traverse(&test_directory, true, &mut visitors);
//
//         let expected_paths = vec![
//             test_directory.to_str().unwrap().to_string(),
//             test_directory.join("file1.txt").to_str().unwrap().to_string(),
//             test_directory.join("subdir1").to_str().unwrap().to_string(),
//             test_directory.join("subdir1/file2.txt").to_str().unwrap().to_string(),
//             test_directory.join("subdir1/subdir2").to_str().unwrap().to_string(),
//             test_directory.join("subdir1/subdir2/file3.txt").to_str().unwrap().to_string(),
//         ];
//
//         assert_eq!(mock_visitor.visited_paths, expected_paths);
//
//         // Clean up: Delete the created directory structure
//         fs::remove_dir_all(&test_directory).expect("Unable to delete test directory structure");
//     }
// }