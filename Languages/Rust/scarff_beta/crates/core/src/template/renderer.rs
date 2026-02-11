//! Template rendering - converting templates into project structures.
//!
//! The renderer takes a template and a render context (variables) and
//! produces a `ProjectStructure` ready to be written to disk.

use std::path::PathBuf;
use tracing::{debug, info, instrument, warn};

use crate::{
    domain::{
        ProjectStructure, RenderContext, TemplateContent, TemplateNode, TemplateSource, validator,
    },
    errors::CoreResult,
    template::{Template, TemplateError},
};

// ============================================================================
// TemplateRenderer
// ============================================================================

/// Renders templates into project structures.
///
/// The renderer is responsible for:
/// 1. Variable substitution ({{PROJECT_NAME}}, etc.)
/// 2. Converting template nodes into filesystem entries
/// 3. Validating the resulting structure
///
/// ## Example
///
/// ```rust,ignore
/// let renderer = TemplateRenderer::new();
/// let context = RenderContext::new("my-project");
/// let structure = renderer.render(&template, &context, output_path)?;
/// ```
pub struct TemplateRenderer;

impl TemplateRenderer {
    /// Create a new template renderer.
    pub fn new() -> Self {
        Self
    }

    /// Render a template into a project structure.
    ///
    /// # Arguments
    ///
    /// * `template` - The template to render
    /// * `ctx` - Render context containing variables
    /// * `output_root` - Root directory for the output
    ///
    /// # Returns
    ///
    /// A validated `ProjectStructure` ready to be written to disk.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Template has invalid content
    /// - Variable substitution fails
    /// - Resulting structure is invalid (duplicates, absolute paths, etc.)
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let renderer = TemplateRenderer::new();
    /// let context = RenderContext::new("my-cli-app");
    /// let structure = renderer.render(
    ///     &template,
    ///     &context,
    ///     PathBuf::from("/tmp/output")
    /// )?;
    /// ```
    #[instrument(
        skip(self, template, ctx),
        fields(
            template_name = template.metadata.name,
            output_root = %output_root.display()
        )
    )]
    pub fn render(
        &self,
        template: &Template,
        ctx: &RenderContext,
        output_root: PathBuf,
    ) -> CoreResult<ProjectStructure> {
        info!("Starting template rendering");

        // TODO: validate Template
        // 1. Validate template record
        validator::validate_template(template)
            .map_err(|e| TemplateError::RenderingFailed(format!("{e}")))?;
        // TODO: change to return validated ProjectrStructure
        // need a validator to validate arguments
        // validate that template is not empty and has valid matcher, metadata and tree nodes

        let mut structure = ProjectStructure::new(output_root);

        // Process each node in the template tree
        for (idx, node) in template.tree.nodes.iter().enumerate() {
            debug!(
                progress = format!("{}/{}", idx + 1, template.tree.len()),
                "Processing template node"
            );

            match node {
                TemplateNode::File(spec) => {
                    // Render file content
                    let content = self.render_content(&spec.content, ctx)?;

                    debug!(
                        path = %spec.path,
                        size = content.len(),
                        "Rendered file content"
                    );

                    structure.add_file(spec.path.as_path(), content, spec.permissions);
                }
                TemplateNode::Directory(spec) => {
                    debug!(
                        path = %spec.path,
                        "Adding directory"
                    );

                    structure.add_directory(spec.path.as_path(), spec.permissions);
                }
            }
        }

        // Validate the structure before returning

        info!(
            files = structure.file_count(),
            directories = structure.directory_count(),
            "Template rendering complete, validating structure"
        );

        // 4. Validate structure
        validator::validate_project_structure(&structure)
            .map_err(|e| TemplateError::RenderingFailed(format!("{e}")))?;

        info!("Structure validation successful");

        Ok(structure)
    }

    /// Render template content with variable substitution.
    ///
    /// # Content Types
    ///
    /// - **Literal**: Return as-is (no substitution)
    /// - **Parameterized**: Replace {{VARIABLE}} placeholders
    /// - **External**: Not yet supported (post-MVP)
    fn render_content(&self, content: &TemplateContent, ctx: &RenderContext) -> CoreResult<String> {
        match content {
            TemplateContent::Literal(source) => {
                // No variable substitution
                Ok(source.as_str().to_string())
            }

            TemplateContent::Parameterized(source) => {
                // Perform variable substitution
                let template_str = source.as_str();
                let rendered = ctx.render(template_str);

                debug!(
                    original_len = template_str.len(),
                    rendered_len = rendered.len(),
                    "Variable substitution complete"
                );

                Ok(rendered)
            }

            TemplateContent::External(template_id) => {
                // External template engines (Tera, Handlebars, etc.)
                // Not implemented in MVP
                warn!(
                    template_id = %template_id.0,
                    "External template rendering not yet implemented"
                );

                Err(crate::template::TemplateError::InvalidTemplate(format!(
                    "External template '{}' not supported in MVP",
                    template_id.0
                ))
                .into())
            }
        }
    }
}

impl Default for TemplateRenderer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        domain::{
            Language, ProjectKind, TargetMatcher, TemplateId,
            common::{Permissions, RelativePath},
        },
        template::{
            DirectorySpec, FileSpec, TemplateContent, TemplateMetadata, TemplateNode,
            TemplateSource, TemplateTree,
        },
    };

    fn create_test_template(name: &'static str) -> Template {
        Template {
            id: TemplateId::new(name, "0.1.0".to_string()),
            matcher: TargetMatcher::builder()
                .language(Language::Rust)
                .kind(ProjectKind::Cli)
                .build(),
            metadata: TemplateMetadata::new(name).version("1.0.0"),
            tree: TemplateTree::new()
                .with_node(TemplateNode::Directory(DirectorySpec::new("src")))
                .with_node(TemplateNode::File(FileSpec::new(
                    "src/main.rs",
                    TemplateContent::Parameterized(TemplateSource::Static(
                        "// Project: {{PROJECT_NAME}}\nfn main() {}",
                    )),
                )))
                .with_node(TemplateNode::File(FileSpec::new(
                    "README.md",
                    TemplateContent::Literal(TemplateSource::Static("# My Project")),
                ))),
        }
    }

    #[test]
    fn render_simple_template() {
        let renderer = TemplateRenderer::new();
        let template = create_test_template("test");
        let context = RenderContext::new("my-cli");

        let structure = renderer
            .render(&template, &context, PathBuf::from("/tmp/test"))
            .unwrap();

        assert_eq!(structure.file_count(), 2);
        assert_eq!(structure.directory_count(), 1);
    }

    #[test]
    fn render_with_variable_substitution() {
        let renderer = TemplateRenderer::new();
        let template = create_test_template("test");
        let context = RenderContext::new("awesome-cli");

        let structure = renderer
            .render(&template, &context, PathBuf::from("/tmp/test"))
            .unwrap();

        // Find the main.rs file
        let main_file = structure
            .files()
            .find(|f| f.path.ends_with("main.rs"))
            .unwrap();

        // Check that variable was substituted
        assert!(main_file.content.contains("awesome-cli"));
        assert!(!main_file.content.contains("{{PROJECT_NAME}}"));
    }

    #[test]
    fn render_literal_content_no_substitution() {
        let renderer = TemplateRenderer::new();
        let template = create_test_template("test");
        let context = RenderContext::new("my-cli");

        let structure = renderer
            .render(&template, &context, PathBuf::from("/tmp/test"))
            .unwrap();

        // Find the README.md file (literal content)
        let readme = structure
            .files()
            .find(|f| f.path.ends_with("README.md"))
            .unwrap();

        // Should be unchanged
        assert_eq!(readme.content, "# My Project");
    }

    #[test]
    fn render_validates_structure() {
        let renderer = TemplateRenderer::new();

        // Create a template with duplicate paths
        let mut tree = TemplateTree::new();
        tree.push(TemplateNode::File(FileSpec::new(
            "main.rs",
            TemplateContent::Literal(TemplateSource::Static("")),
        )));
        tree.push(TemplateNode::File(FileSpec::new(
            "main.rs", // Duplicate!
            TemplateContent::Literal(TemplateSource::Static("")),
        )));

        let template = Template {
            id: TemplateId::new("test", "0.1.0".to_string()),
            matcher: TargetMatcher::builder().build(),
            metadata: TemplateMetadata::new("test"),
            tree,
        };

        let context = RenderContext::new("test");

        let result = renderer.render(&template, &context, PathBuf::from("/tmp/test"));

        // Should fail validation
        assert!(result.is_err());
    }

    #[test]
    fn render_empty_template_fails() {
        let renderer = TemplateRenderer::new();

        let template = Template {
            id: TemplateId::new("test", "0.1.0".to_string()),
            matcher: TargetMatcher::builder().build(),
            metadata: TemplateMetadata::new("test"),
            tree: TemplateTree::new(), // Empty!
        };

        let context = RenderContext::new("test");

        let result = renderer.render(&template, &context, PathBuf::from("/tmp/test"));

        println!("{result:?}");

        // Should fail validation (empty tree)
        assert!(result.is_err());
    }

    #[test]
    fn render_preserves_permissions() {
        let renderer = TemplateRenderer::new();

        let tree = TemplateTree::new().with_node(TemplateNode::File(
            FileSpec::new(
                "script.sh",
                TemplateContent::Literal(TemplateSource::Static("#!/bin/bash")),
            )
            .executable(), // Mark as executable
        ));

        let template = Template {
            id: TemplateId::new("test", "0.1.0".to_string()),
            matcher: TargetMatcher::builder().build(),
            metadata: TemplateMetadata::new("test"),
            tree,
        };

        let context = RenderContext::new("test");

        let structure = renderer
            .render(&template, &context, PathBuf::from("/tmp/test"))
            .unwrap();

        let script = structure
            .files()
            .find(|f| f.path.ends_with("script.sh"))
            .unwrap();

        assert!(script.permissions.executable_flag());
    }

    #[test]
    fn render_multiple_variables() {
        let renderer = TemplateRenderer::new();

        let tree = TemplateTree::new().with_node(TemplateNode::File(FileSpec::new(
            "config.toml",
            TemplateContent::Parameterized(TemplateSource::Static(
                r#"
[package]
name = "{{PROJECT_NAME_KEBAB}}"
version = "0.1.0"

[info]
year = {{YEAR}}
"#,
            )),
        )));

        let template = Template {
            id: TemplateId::new("test", "0.1.0".to_string()),
            matcher: TargetMatcher::builder().build(),
            metadata: TemplateMetadata::new("test"),
            tree,
        };

        let context = RenderContext::new("MyAwesomeProject");

        let structure = renderer
            .render(&template, &context, PathBuf::from("/tmp/test"))
            .unwrap();

        println!("{structure:?}");

        let config = structure
            .files()
            .find(|f| f.path.ends_with("config.toml"))
            .unwrap();

        println!("{config:?}");

        assert!(config.content.contains("my-awesome-project")); // kebab-case
        assert!(config.content.contains("2026")); // current year
    }

    #[test]
    fn render_external_template_not_supported() {
        let renderer = TemplateRenderer::new();

        let tree = TemplateTree::new().with_node(TemplateNode::File(FileSpec::new(
            "main.rs",
            TemplateContent::External(crate::template::ContentTemplateId("tera:main")),
        )));

        let template = Template {
            id: TemplateId::new("test", "0.1.0".to_string()),
            matcher: TargetMatcher::builder().build(),
            metadata: TemplateMetadata::new("test"),
            tree,
        };

        let context = RenderContext::new("test");

        let result = renderer.render(&template, &context, PathBuf::from("/tmp/test"));

        // Should error for external templates in MVP
        assert!(result.is_err());
    }
}
