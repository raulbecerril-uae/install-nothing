use std::fs;
use std::path::Path;

/// Manages Deno compilation logs (both success and error cases)
pub struct DenoLogs {
    success_logs: Vec<String>,
    error_logs: Vec<String>,
}

impl DenoLogs {
    /// Load Deno logs from files
    pub fn load() -> Self {
        let success_path = Path::new("data/deno.log");
        let error_path = Path::new("data/error/deno.log");

        let success_logs = if success_path.exists() {
            match fs::read_to_string(success_path) {
                Ok(content) => content
                    .lines()
                    .filter(|line| !line.trim().is_empty())
                    .map(String::from)
                    .collect(),
                Err(_) => Self::default_success_logs(),
            }
        } else {
            Self::default_success_logs()
        };

        let error_logs = if error_path.exists() {
            match fs::read_to_string(error_path) {
                Ok(content) => content
                    .lines()
                    .filter(|line| !line.trim().is_empty())
                    .map(String::from)
                    .collect(),
                Err(_) => Self::default_error_logs(),
            }
        } else {
            Self::default_error_logs()
        };

        Self {
            success_logs,
            error_logs,
        }
    }

    /// Get success logs
    pub fn success_logs(&self) -> &[String] {
        &self.success_logs
    }

    /// Get error logs
    pub fn error_logs(&self) -> &[String] {
        &self.error_logs
    }

    /// Default fallback logs if no success file is present
    fn default_success_logs() -> Vec<String> {
        vec![
            "   Compiling proc-macro2 v1.0.101".to_string(),
            "   Compiling unicode-ident v1.0.12".to_string(),
            "   Compiling libc v0.2.172".to_string(),
            "   Compiling serde v1.0.228".to_string(),
            "    Finished `release` profile [optimized] target(s) in 2m 31s".to_string(),
        ]
    }

    /// Default fallback error logs if no error file is present
    fn default_error_logs() -> Vec<String> {
        vec![
            "   Compiling proc-macro2 v1.0.101".to_string(),
            "   Compiling libc v0.2.172".to_string(),
            "error: linking with `cc` failed: exit status: 1".to_string(),
            "  = note: clang: error: invalid linker name in argument '-fuse-ld=lld'".to_string(),
            "error: could not compile `proc-macro2` (build script) due to 1 previous error"
                .to_string(),
        ]
    }
}
