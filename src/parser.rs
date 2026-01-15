use anyhow::{Context, Result};
use ruby_prism::{parse, ParseResult};
use std::fs;
use std::path::Path;

/// Parse Ruby source code and return ruby-prism AST
///
/// Note: Uses Box::leak internally to ensure 'static lifetime
pub fn parse_ruby_file(file_path: &Path) -> Result<ParseResult<'static>> {
    let source = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path.display()))?;

    parse_ruby_source(&source, file_path.to_string_lossy().to_string())
}

/// Parse Ruby source code string
pub fn parse_ruby_source(source: &str, file_name: String) -> Result<ParseResult<'static>> {
    // ruby-prism accepts &[u8]
    // Use Box::leak to ensure 'static lifetime (memory leak is acceptable for analysis tools)
    let source_bytes: &'static [u8] = Box::leak(source.as_bytes().to_vec().into_boxed_slice());
    let parse_result = parse(source_bytes);

    // Check parse errors
    let error_messages: Vec<String> = parse_result
        .errors()
        .map(|e| {
            format!(
                "Parse error at offset {}: {}",
                e.location().start_offset(),
                e.message()
            )
        })
        .collect();

    if !error_messages.is_empty() {
        anyhow::bail!(
            "Failed to parse Ruby source in {}:\n{}",
            file_name,
            error_messages.join("\n")
        );
    }

    Ok(parse_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_ruby() {
        let source = r#"x = 1
puts x"#;
        let result = parse_ruby_source(source, "test.rb".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_string_literal() {
        let source = r#""hello".upcase"#;
        let result = parse_ruby_source(source, "test.rb".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_array_literal() {
        let source = r#"[1, 2, 3].map { |x| x * 2 }"#;
        let result = parse_ruby_source(source, "test.rb".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_method_definition() {
        let source = r#"def test_method
  x = "hello"
  x.upcase
end"#;
        let result = parse_ruby_source(source, "test.rb".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_invalid_ruby() {
        let source = "def\nend end";
        let result = parse_ruby_source(source, "test.rb".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_method_call() {
        let source = r#"user = User.new
user.save"#;
        let result = parse_ruby_source(source, "test.rb".to_string());
        assert!(result.is_ok());
    }
}
