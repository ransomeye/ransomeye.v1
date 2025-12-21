// Path and File Name : /home/ransomeye/rebuild/ransomeye_dpi_probe/config/validation.rs
// Author: nXxBku0CKFAJCBN3X1g3bQk7OxYQylg8CMw1iGsq7gU
// Details of functionality of this file: Configuration validation - validates environment variable values and constraints

use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Invalid value for {0}: {1}")]
    InvalidValue(String, String),
    #[error("Value out of range for {0}: {1} (min: {2}, max: {3})")]
    ValueOutOfRange(String, usize, usize, usize),
    #[error("Path does not exist: {0}")]
    PathNotFound(String),
    #[error("Path is not absolute: {0}")]
    PathNotAbsolute(String),
    #[error("Path is not readable: {0}")]
    PathNotReadable(String),
    #[error("Path is not writable: {0}")]
    PathNotWritable(String),
}

pub struct ConfigValidator;

impl ConfigValidator {
    /// Validate buffer size (MB)
    pub fn validate_buffer_size_mb(size: usize) -> Result<(), ValidationError> {
        const MIN: usize = 1;
        const MAX: usize = 10000;
        
        if size < MIN || size > MAX {
            return Err(ValidationError::ValueOutOfRange(
                "MAX_BUFFER_SIZE_MB".to_string(),
                size,
                MIN,
                MAX,
            ));
        }
        
        Ok(())
    }
    
    /// Validate backpressure threshold
    pub fn validate_backpressure_threshold(threshold: usize) -> Result<(), ValidationError> {
        const MIN: usize = 1024;
        const MAX: usize = 1048576; // 1MB
        
        if threshold < MIN || threshold > MAX {
            return Err(ValidationError::ValueOutOfRange(
                "BACKPRESSURE_THRESHOLD".to_string(),
                threshold,
                MIN,
                MAX,
            ));
        }
        
        Ok(())
    }
    
    /// Validate flow timeout (seconds)
    pub fn validate_flow_timeout(timeout: u64) -> Result<(), ValidationError> {
        const MIN: u64 = 60;
        const MAX: u64 = 3600;
        
        if timeout < MIN || timeout > MAX {
            return Err(ValidationError::ValueOutOfRange(
                "FLOW_TIMEOUT_SECONDS".to_string(),
                timeout as usize,
                MIN as usize,
                MAX as usize,
            ));
        }
        
        Ok(())
    }
    
    /// Validate health report interval (seconds)
    pub fn validate_health_interval(interval: u64) -> Result<(), ValidationError> {
        const MIN: u64 = 10;
        const MAX: u64 = 3600;
        
        if interval < MIN || interval > MAX {
            return Err(ValidationError::ValueOutOfRange(
                "HEALTH_REPORT_INTERVAL_SECONDS".to_string(),
                interval as usize,
                MIN as usize,
                MAX as usize,
            ));
        }
        
        Ok(())
    }
    
    /// Validate that path is absolute
    pub fn validate_absolute_path(path: &str) -> Result<(), ValidationError> {
        let path_obj = Path::new(path);
        if !path_obj.is_absolute() {
            return Err(ValidationError::PathNotAbsolute(path.to_string()));
        }
        Ok(())
    }
    
    /// Validate that file path exists and is readable
    pub fn validate_readable_file(path: &str) -> Result<(), ValidationError> {
        Self::validate_absolute_path(path)?;
        
        let path_obj = Path::new(path);
        if !path_obj.exists() {
            return Err(ValidationError::PathNotFound(path.to_string()));
        }
        
        if !path_obj.is_file() {
            return Err(ValidationError::InvalidValue(
                path.to_string(),
                "Path is not a file".to_string(),
            ));
        }
        
        // Check read permissions
        if std::fs::metadata(path)
            .map_err(|e| ValidationError::PathNotReadable(format!("{}: {}", path, e)))?
            .permissions()
            .readonly()
            && !std::fs::File::open(path).is_ok()
        {
            return Err(ValidationError::PathNotReadable(path.to_string()));
        }
        
        Ok(())
    }
    
    /// Validate that directory path exists and is writable
    pub fn validate_writable_directory(path: &str) -> Result<(), ValidationError> {
        Self::validate_absolute_path(path)?;
        
        let path_obj = Path::new(path);
        
        // If directory doesn't exist, check if parent is writable
        if !path_obj.exists() {
            if let Some(parent) = path_obj.parent() {
                if !parent.exists() {
                    return Err(ValidationError::PathNotFound(parent.display().to_string()));
                }
                
                // Check if parent is writable
                let test_file = parent.join(".write_test");
                if std::fs::write(&test_file, b"test").is_ok() {
                    let _ = std::fs::remove_file(&test_file);
                } else {
                    return Err(ValidationError::PathNotWritable(parent.display().to_string()));
                }
            }
        } else {
            if !path_obj.is_dir() {
                return Err(ValidationError::InvalidValue(
                    path.to_string(),
                    "Path is not a directory".to_string(),
                ));
            }
            
            // Check write permissions
            let test_file = path_obj.join(".write_test");
            if std::fs::write(&test_file, b"test").is_ok() {
                let _ = std::fs::remove_file(&test_file);
            } else {
                return Err(ValidationError::PathNotWritable(path.to_string()));
            }
        }
        
        Ok(())
    }
    
    /// Validate URL format
    pub fn validate_url(url: &str) -> Result<(), ValidationError> {
        url::Url::parse(url)
            .map_err(|e| ValidationError::InvalidValue(
                "CORE_API_URL".to_string(),
                format!("Invalid URL: {}", e),
            ))?;
        
        Ok(())
    }
}
