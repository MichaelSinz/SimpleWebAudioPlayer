//! Unit tests for the CLI types module.

use std::str::FromStr;
use crate::cli::types::{Width, Height, AudioPath, FileExtension, FileExtensions};
use std::path::Path;
use tempfile::NamedTempFile;

// Test Width from_str implementation
#[cfg(test)]
mod width_tests {
    use super::*;

    #[test]
    fn test_valid_width() {
        // Minimum allowed width
        let width = Width::from_str("16").unwrap();
        assert_eq!(width.value(), 16);

        // Larger width
        let width = Width::from_str("100").unwrap();
        assert_eq!(width.value(), 100);

        // Very large width
        let width = Width::from_str("10000").unwrap();
        assert_eq!(width.value(), 10000);
    }

    #[test]
    fn test_invalid_width_format() {
        // Non-numeric input
        let result = Width::from_str("abc");
        assert!(result.is_err(), "Should reject non-numeric input");
        let err = result.unwrap_err();
        assert_eq!(err.to_string(), "Invalid argument: Width must be a positive integer", 
                  "Should provide clear error message for non-numeric input");

        // Negative number
        let result = Width::from_str("-10");
        assert!(result.is_err(), "Should reject negative numbers");
        let err = result.unwrap_err();
        assert_eq!(err.to_string(), "Invalid argument: Width must be a positive integer", 
                  "Should provide clear error message for negative numbers");

        // Decimal number
        let result = Width::from_str("16.5");
        assert!(result.is_err(), "Should reject decimal numbers");
        let err = result.unwrap_err();
        assert_eq!(err.to_string(), "Invalid argument: Width must be a positive integer", 
                  "Should provide clear error message for decimal numbers");
    }

    #[test]
    fn test_below_minimum_width() {
        // Below minimum width
        let result = Width::from_str("15");
        assert!(result.is_err(), "Should reject width below minimum");
        let err = result.unwrap_err();
        assert_eq!(err.to_string(), "Invalid argument: Width must be at least 16 pixels", 
                  "Should provide clear error message about minimum width");

        // Zero width
        let result = Width::from_str("0");
        assert!(result.is_err(), "Should reject zero width");
        let err = result.unwrap_err();
        assert_eq!(err.to_string(), "Invalid argument: Width must be at least 16 pixels", 
                  "Should provide clear error message about minimum width");
    }
}

// Test Height from_str implementation
#[cfg(test)]
mod height_tests {
    use super::*;

    #[test]
    fn test_valid_height() {
        // Minimum allowed height
        let height = Height::from_str("6").unwrap();
        assert_eq!(height.value(), 6);
        assert_eq!(height.center(), 3);

        // Larger height
        let height = Height::from_str("100").unwrap();
        assert_eq!(height.value(), 100);
        assert_eq!(height.center(), 50);

        // Very large height
        let height = Height::from_str("10000").unwrap();
        assert_eq!(height.value(), 10000);
        assert_eq!(height.center(), 5000);
    }

    #[test]
    fn test_invalid_height_format() {
        // Non-numeric input
        let result = Height::from_str("abc");
        assert!(result.is_err(), "Should reject non-numeric input");
        let err = result.unwrap_err();
        assert_eq!(err.to_string(), "Invalid argument: Height must be a positive integer", 
                  "Should provide clear error message for non-numeric input");

        // Negative number
        let result = Height::from_str("-10");
        assert!(result.is_err(), "Should reject negative numbers");
        let err = result.unwrap_err();
        assert_eq!(err.to_string(), "Invalid argument: Height must be a positive integer", 
                  "Should provide clear error message for negative numbers");

        // Decimal number
        let result = Height::from_str("6.5");
        assert!(result.is_err(), "Should reject decimal numbers");
        let err = result.unwrap_err();
        assert_eq!(err.to_string(), "Invalid argument: Height must be a positive integer", 
                  "Should provide clear error message for decimal numbers");
    }

    #[test]
    fn test_below_minimum_height() {
        // Below minimum height
        let result = Height::from_str("4");
        assert!(result.is_err(), "Should reject height below minimum");
        let err = result.unwrap_err();
        assert_eq!(err.to_string(), "Invalid argument: Height must be at least 6 pixels", 
                  "Should provide clear error message about minimum height");

        // Zero height
        let result = Height::from_str("0");
        assert!(result.is_err(), "Should reject zero height");
        let err = result.unwrap_err();
        assert_eq!(err.to_string(), "Invalid argument: Height must be at least 6 pixels", 
                  "Should provide clear error message about minimum height");
    }

    #[test]
    fn test_odd_height() {
        // Odd height values
        let result = Height::from_str("7");
        assert!(result.is_err(), "Should reject odd height values");
        let err = result.unwrap_err();
        assert_eq!(err.to_string(), "Invalid argument: Height must be an even number", 
                  "Should provide clear error message about even height requirement");

        // Larger odd height value
        let result = Height::from_str("101");
        assert!(result.is_err(), "Should reject odd height values");
        let err = result.unwrap_err();
        assert_eq!(err.to_string(), "Invalid argument: Height must be an even number", 
                  "Should provide clear error message about even height requirement");
    }
}

// Test AudioPath from_str implementation
// Note: These tests require creating temporary files to properly test the path existence check
#[cfg(test)]
mod audio_path_tests {
    use super::*;

    #[test]
    fn test_existing_audio_path() {
        // Create a temporary file that definitely exists
        let temp_file = NamedTempFile::new().unwrap();
        let path_str = temp_file.path().to_string_lossy().to_string();
        
        let audio_path = AudioPath::from_str(&path_str).unwrap();
        assert_eq!(audio_path.path(), Path::new(&path_str));
        assert!(!audio_path.is_dir());
    }

    #[test]
    fn test_nonexistent_audio_path() {
        // Path that definitely doesn't exist
        let path = "/path/that/does/not/exist/audio.mp3";
        let result = AudioPath::from_str(path);
        assert!(result.is_err(), "Should reject non-existent paths");
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Path does not exist"), 
                "Error message should indicate the path doesn't exist");
        assert!(err.to_string().contains(path), 
                "Error message should include the specific path that doesn't exist");
    }
}

// Test FileExtension from_str implementation
#[cfg(test)]
mod file_extension_tests {
    use super::*;

    #[test]
    fn test_valid_extension() {
        // Simple extension
        let ext = FileExtension::from_str("mp3").unwrap();
        assert_eq!(ext.as_str(), "mp3");
    }

    #[test]
    fn test_extension_case_insensitivity() {
        // Test case insensitivity
        let ext = FileExtension::from_str("MP3").unwrap();
        assert_eq!(ext.as_str(), "mp3");

        let ext = FileExtension::from_str("WaV").unwrap();
        assert_eq!(ext.as_str(), "wav");
    }

    #[test]
    fn test_extension_trim() {
        // Test trimming
        let ext = FileExtension::from_str(" mp3 ").unwrap();
        assert_eq!(ext.as_str(), "mp3");
    }

    #[test]
    fn test_empty_extension() {
        // Empty extension
        let result = FileExtension::from_str("");
        assert!(result.is_err(), "Should reject empty extensions");
        let err = result.unwrap_err();
        assert_eq!(err.to_string(), "Invalid argument: File extension cannot be empty", 
                  "Should provide clear error message for empty extension");

        // Only whitespace
        let result = FileExtension::from_str("   ");
        assert!(result.is_err(), "Should reject whitespace-only extensions");
        let err = result.unwrap_err();
        assert_eq!(err.to_string(), "Invalid argument: File extension cannot be empty", 
                  "Should provide clear error message for whitespace-only extension");
    }
}

// Test FileExtensions from_str implementation
#[cfg(test)]
mod file_extensions_tests {
    use super::*;

    #[test]
    fn test_valid_extensions_list() {
        // Simple list
        let exts = FileExtensions::from_str("mp3,wav,flac").unwrap();
        let strings = exts.as_strings();
        assert_eq!(strings, vec!["mp3", "wav", "flac"]);
    }

    #[test]
    fn test_extensions_with_whitespace() {
        // List with whitespace
        let exts = FileExtensions::from_str("mp3, wav, flac").unwrap();
        let strings = exts.as_strings();
        assert_eq!(strings, vec!["mp3", "wav", "flac"]);
    }

    #[test]
    fn test_extensions_mixed_case() {
        // List with mixed case
        let exts = FileExtensions::from_str("MP3,Wav,FLAC").unwrap();
        let strings = exts.as_strings();
        assert_eq!(strings, vec!["mp3", "wav", "flac"]);
    }

    #[test]
    fn test_single_extension() {
        // Single extension
        let exts = FileExtensions::from_str("mp3").unwrap();
        let strings = exts.as_strings();
        assert_eq!(strings, vec!["mp3"]);
    }

    #[test]
    fn test_empty_extensions_list() {
        // Empty list
        let result = FileExtensions::from_str("");
        assert!(result.is_err(), "Should reject empty extension lists");
        let err = result.unwrap_err();
        assert_eq!(err.to_string(), "Invalid argument: No file extensions specified", 
                  "Should provide clear error message for empty extension list");

        // Only commas
        let result = FileExtensions::from_str(",,,");
        assert!(result.is_err(), "Should reject lists with only commas");
        let err = result.unwrap_err();
        assert_eq!(err.to_string(), "Invalid argument: No file extensions specified", 
                  "Should provide clear error message for comma-only list");

        // Only whitespace and commas
        let result = FileExtensions::from_str(" , , , ");
        assert!(result.is_err(), "Should reject lists with only whitespace and commas");
        let err = result.unwrap_err();
        assert_eq!(err.to_string(), "Invalid argument: No file extensions specified", 
                  "Should provide clear error message for whitespace and comma list");
    }
}