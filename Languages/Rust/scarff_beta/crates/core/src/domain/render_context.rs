//! Context for rendering templates with variables.

use std::collections::HashMap;

/// Context containing variables for template rendering.
///
/// Provides standard variables (project name, year, etc.) plus custom variables.
#[derive(Debug, Clone)]
pub struct RenderContext {
    variables: HashMap<String, String>,
}

impl RenderContext {
    /// Create a new render context with a project name.
    ///
    /// Standard variables are automatically populated:
    /// - `PROJECT_NAME`: Original project name
    /// - `PROJECT_NAME_SNAKE`: snake_case version
    /// - `PROJECT_NAME_KEBAB`: kebab-case version
    /// - `PROJECT_NAME_PASCAL`: PascalCase version
    /// - `YEAR`: Current year (for copyright notices)
    pub fn new(project_name: impl Into<String>) -> Self {
        let mut variables = HashMap::new();
        let project_name = project_name.into();

        // Core variables
        variables.insert("PROJECT_NAME".to_string(), project_name.clone());
        variables.insert(
            "PROJECT_NAME_SNAKE".to_string(),
            to_snake_case(&project_name),
        );
        variables.insert(
            "PROJECT_NAME_KEBAB".to_string(),
            to_kebab_case(&project_name),
        );
        variables.insert(
            "PROJECT_NAME_PASCAL".to_string(),
            to_pascal_case(&project_name),
        );
        variables.insert("YEAR".to_string(), current_year());

        Self { variables }
    }

    /// Add a custom variable.
    ///
    /// Builder-style method for chaining.
    pub fn with_var(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.variables.insert(key.into(), value.into());
        self
    }

    /// Set a custom variable (mutable method).
    pub fn set_var(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.variables.insert(key.into(), value.into());
    }

    /// Get a variable value.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.variables.get(key).map(|s| s.as_str())
    }

    /// Check if a variable exists.
    pub fn has(&self, key: &str) -> bool {
        self.variables.contains_key(key)
    }

    /// Get all variables as a map (for template engines).
    pub fn all(&self) -> &HashMap<String, String> {
        &self.variables
    }

    /// Render a template string by replacing {{VARIABLE}} placeholders.
    ///
    /// Simple implementation for MVP. Can be replaced with a proper template engine later.
    pub fn render(&self, template: &str) -> String {
        let mut result = template.to_string();

        for (key, value) in &self.variables {
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, value);
        }

        result
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Convert a string to snake_case.
///
/// Rules:
/// - Replace hyphens and spaces with underscores
/// - Convert to lowercase
fn to_snake_case(s: &str) -> String {
    s.replace('-', "_")
        .replace(' ', "_")
        .chars()
        .map(|c| c.to_lowercase().to_string())
        .collect()
}

/// Convert a string to kebab-case.
///
/// Rules:
/// - Replace underscores and spaces with hyphens
/// - Convert to lowercase
fn to_kebab_case(s: &str) -> String {
    s.replace('_', "-")
        .replace(' ', "-")
        .chars()
        .map(|c| c.to_lowercase().to_string())
        .collect()
}

/// Convert a string to PascalCase.
///
/// Rules:
/// - Split on hyphens, underscores, and spaces
/// - Capitalize first letter of each word
/// - Join without separators
fn to_pascal_case(s: &str) -> String {
    s.split(|c: char| c == '-' || c == '_' || c.is_whitespace())
        .filter(|word| !word.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    let mut result = String::new();
                    result.push_str(&first.to_uppercase().to_string());
                    result.push_str(&chars.collect::<String>().to_lowercase());
                    result
                }
            }
        })
        .collect()
}

/// Get the current year as a string.
///
/// For now, returns a placeholder. In production, use `chrono` or `time` crate.
fn current_year() -> String {
    // TODO: Use chrono::Utc::now().year() in production
    "2026".to_string()
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_context_standard_variables() {
        let ctx = RenderContext::new("my-awesome-project");

        assert_eq!(ctx.get("PROJECT_NAME"), Some("my-awesome-project"));
        assert_eq!(ctx.get("PROJECT_NAME_SNAKE"), Some("my_awesome_project"));
        assert_eq!(ctx.get("PROJECT_NAME_KEBAB"), Some("my-awesome-project"));
        assert_eq!(ctx.get("PROJECT_NAME_PASCAL"), Some("MyAwesomeProject"));
        assert_eq!(ctx.get("YEAR"), Some("2026"));
    }

    #[test]
    fn render_context_custom_variables() {
        let ctx = RenderContext::new("test-project")
            .with_var("AUTHOR", "John Doe")
            .with_var("LICENSE", "MIT");

        assert_eq!(ctx.get("AUTHOR"), Some("John Doe"));
        assert_eq!(ctx.get("LICENSE"), Some("MIT"));
    }

    #[test]
    fn render_context_set_var_mutable() {
        let mut ctx = RenderContext::new("test");
        ctx.set_var("CUSTOM", "value");
        assert_eq!(ctx.get("CUSTOM"), Some("value"));
    }

    #[test]
    fn render_context_has() {
        let ctx = RenderContext::new("test");
        assert!(ctx.has("PROJECT_NAME"));
        assert!(!ctx.has("NONEXISTENT"));
    }

    #[test]
    fn render_simple_template() {
        let ctx = RenderContext::new("my-project");
        let template = "Project: {{PROJECT_NAME}}, Year: {{YEAR}}";
        let result = ctx.render(template);
        assert_eq!(result, "Project: my-project, Year: 2026");
    }

    #[test]
    fn render_multiple_occurrences() {
        let ctx = RenderContext::new("test");
        let template = "{{PROJECT_NAME}} {{PROJECT_NAME}} {{PROJECT_NAME}}";
        let result = ctx.render(template);
        assert_eq!(result, "test test test");
    }

    #[test]
    fn to_snake_case_conversions() {
        assert_eq!(to_snake_case("my-project"), "my_project");
        assert_eq!(to_snake_case("my project"), "my_project");
        assert_eq!(to_snake_case("MyProject"), "myproject");
        assert_eq!(to_snake_case("my_project"), "my_project");
    }

    #[test]
    fn to_kebab_case_conversions() {
        assert_eq!(to_kebab_case("my_project"), "my-project");
        assert_eq!(to_kebab_case("my project"), "my-project");
        assert_eq!(to_kebab_case("MyProject"), "myproject");
        assert_eq!(to_kebab_case("my-project"), "my-project");
    }

    #[test]
    fn to_pascal_case_conversions() {
        assert_eq!(to_pascal_case("my-project"), "MyProject");
        assert_eq!(to_pascal_case("my_project"), "MyProject");
        assert_eq!(to_pascal_case("my project"), "MyProject");
        assert_eq!(to_pascal_case("MyProject"), "Myproject"); // Normalizes
    }

    #[test]
    fn to_pascal_case_multiple_words() {
        assert_eq!(to_pascal_case("hello-world-app"), "HelloWorldApp");
        assert_eq!(to_pascal_case("foo_bar_baz"), "FooBarBaz");
    }
}
