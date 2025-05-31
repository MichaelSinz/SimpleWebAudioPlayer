#[cfg(test)]
mod tests {
    use crate::color::Rgba;
    use std::str::FromStr;

    // Test RGB (3-digit) format
    #[test]
    fn test_rgb_format() {
        // Basic red color
        let color = Rgba::from_str("F00").unwrap();
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 0);
        assert_eq!(color.blue, 0);
        assert_eq!(color.alpha, 255, "Alpha should default to 255 for RGB format");
        
        // Basic green color
        let color = Rgba::from_str("0F0").unwrap();
        assert_eq!(color.red, 0);
        assert_eq!(color.green, 255);
        assert_eq!(color.blue, 0);
        
        // Basic blue color
        let color = Rgba::from_str("00F").unwrap();
        assert_eq!(color.red, 0);
        assert_eq!(color.green, 0);
        assert_eq!(color.blue, 255);
        
        // Mixed color
        let color = Rgba::from_str("123").unwrap();
        assert_eq!(color.red, 17);
        assert_eq!(color.green, 34);
        assert_eq!(color.blue, 51);
        
        // All zeros
        let color = Rgba::from_str("000").unwrap();
        assert_eq!(color.red, 0);
        assert_eq!(color.green, 0);
        assert_eq!(color.blue, 0);
        
        // All max values
        let color = Rgba::from_str("FFF").unwrap();
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 255);
        assert_eq!(color.blue, 255);
        
        // Case insensitivity
        let color = Rgba::from_str("aBc").unwrap();
        assert_eq!(color.red, 170);
        assert_eq!(color.green, 187);
        assert_eq!(color.blue, 204);
    }
    
    // Test RRGGBB (6-digit) format
    #[test]
    fn test_rrggbb_format() {
        // Basic red color
        let color = Rgba::from_str("FF0000").unwrap();
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 0);
        assert_eq!(color.blue, 0);
        assert_eq!(color.alpha, 255, "Alpha should default to 255 for RRGGBB format");
        
        // Basic green color
        let color = Rgba::from_str("00FF00").unwrap();
        assert_eq!(color.red, 0);
        assert_eq!(color.green, 255);
        assert_eq!(color.blue, 0);
        
        // Basic blue color
        let color = Rgba::from_str("0000FF").unwrap();
        assert_eq!(color.red, 0);
        assert_eq!(color.green, 0);
        assert_eq!(color.blue, 255);
        
        // Mixed color
        let color = Rgba::from_str("123456").unwrap();
        assert_eq!(color.red, 18);
        assert_eq!(color.green, 52);
        assert_eq!(color.blue, 86);
        
        // All zeros
        let color = Rgba::from_str("000000").unwrap();
        assert_eq!(color.red, 0);
        assert_eq!(color.green, 0);
        assert_eq!(color.blue, 0);
        
        // All max values
        let color = Rgba::from_str("FFFFFF").unwrap();
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 255);
        assert_eq!(color.blue, 255);
        
        // Case insensitivity
        let color = Rgba::from_str("aAbBcC").unwrap();
        assert_eq!(color.red, 170);
        assert_eq!(color.green, 187);
        assert_eq!(color.blue, 204);
    }
    
    // Test RRGGBBAA (8-digit) format
    #[test]
    fn test_rrggbbaa_format() {
        // Red with half transparency
        let color = Rgba::from_str("FF000080").unwrap();
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 0);
        assert_eq!(color.blue, 0);
        assert_eq!(color.alpha, 128);
        
        // Green with full transparency
        let color = Rgba::from_str("00FF0000").unwrap();
        assert_eq!(color.red, 0);
        assert_eq!(color.green, 255);
        assert_eq!(color.blue, 0);
        assert_eq!(color.alpha, 0);
        
        // Blue with full opacity
        let color = Rgba::from_str("0000FFFF").unwrap();
        assert_eq!(color.red, 0);
        assert_eq!(color.green, 0);
        assert_eq!(color.blue, 255);
        assert_eq!(color.alpha, 255);
        
        // Mixed color with mixed alpha
        let color = Rgba::from_str("12345678").unwrap();
        assert_eq!(color.red, 18);
        assert_eq!(color.green, 52);
        assert_eq!(color.blue, 86);
        assert_eq!(color.alpha, 120);
        
        // All zeros
        let color = Rgba::from_str("00000000").unwrap();
        assert_eq!(color.red, 0);
        assert_eq!(color.green, 0);
        assert_eq!(color.blue, 0);
        assert_eq!(color.alpha, 0);
        
        // All max values
        let color = Rgba::from_str("FFFFFFFF").unwrap();
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 255);
        assert_eq!(color.blue, 255);
        assert_eq!(color.alpha, 255);
        
        // Case insensitivity
        let color = Rgba::from_str("aAbBcCdD").unwrap();
        assert_eq!(color.red, 170);
        assert_eq!(color.green, 187);
        assert_eq!(color.blue, 204);
        assert_eq!(color.alpha, 221);
    }
    
    // Test invalid formats and error handling
    #[test]
    fn test_invalid_formats() {
        // Non-hex characters
        assert!(Rgba::from_str("XYZ").is_err(), "Should reject non-hex characters");
        assert!(Rgba::from_str("GHIJKL").is_err(), "Should reject non-hex characters");
        
        // Invalid lengths
        assert!(Rgba::from_str("").is_err(), "Should reject empty string");
        assert!(Rgba::from_str("1").is_err(), "Should reject 1-character input");
        assert!(Rgba::from_str("12").is_err(), "Should reject 2-character input");
        assert!(Rgba::from_str("1234").is_err(), "Should reject 4-character input");
        assert!(Rgba::from_str("12345").is_err(), "Should reject 5-character input");
        assert!(Rgba::from_str("1234567").is_err(), "Should reject 7-character input");
        assert!(Rgba::from_str("123456789").is_err(), "Should reject 9-character input");
        
        // Check error message for invalid hex characters
        let err = Rgba::from_str("XYZ").unwrap_err();
        assert!(err.to_string().contains("Invalid color format"), 
               "Error for non-hex chars should indicate invalid format");
        
        // Check error message for empty string
        let err = Rgba::from_str("").unwrap_err();
        assert!(err.to_string().contains("Invalid color format: cannot parse integer from empty string"), 
               "Error for empty string should indicate empty string issue");
        
        // Check error message for valid hex but wrong length
        let err = Rgba::from_str("1234").unwrap_err();
        assert!(err.to_string().contains("Color must be in RGB, RRGGBB, or RRGGBBAA format"), 
               "Error for valid hex but wrong length should mention the supported formats");
        
        // Check longer valid hex but wrong length
        let err = Rgba::from_str("1234567").unwrap_err();
        assert!(err.to_string().contains("Color must be in RGB, RRGGBB, or RRGGBBAA format"), 
               "Error for valid hex but wrong length should mention the supported formats");
    }
    
    // Test direct constructors
    #[test]
    fn test_constructors() {
        // Test new() constructor
        let color = Rgba::new(10, 20, 30, 40);
        assert_eq!(color.red, 10);
        assert_eq!(color.green, 20);
        assert_eq!(color.blue, 30);
        assert_eq!(color.alpha, 40);
        
        // Test rgb() constructor (should have alpha = 255)
        let color = Rgba::rgb(50, 60, 70);
        assert_eq!(color.red, 50);
        assert_eq!(color.green, 60);
        assert_eq!(color.blue, 70);
        assert_eq!(color.alpha, 255);
    }
    
    // Test whitespace handling
    #[test]
    fn test_whitespace_handling() {
        // Leading whitespace
        let color = Rgba::from_str(" F00").unwrap();
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 0);
        assert_eq!(color.blue, 0);
        
        // Trailing whitespace
        let color = Rgba::from_str("00FF00 ").unwrap();
        assert_eq!(color.red, 0);
        assert_eq!(color.green, 255);
        assert_eq!(color.blue, 0);
        
        // Both leading and trailing whitespace
        let color = Rgba::from_str(" 0000FFFF ").unwrap();
        assert_eq!(color.red, 0);
        assert_eq!(color.green, 0);
        assert_eq!(color.blue, 255);
        assert_eq!(color.alpha, 255);
    }
}