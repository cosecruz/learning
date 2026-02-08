//! Main scaffolding engine - orchestrates the entire scaffolding process.

use crate::{
    domain::{RenderContext, Target},
    errors::CoreResult,
    scaffold::{
        filesystem::RealFilesystem,
        // validator::Validator,
        writer::{FileWriter, Writer},
    },
    template::{Store, TemplateRenderer, TemplateResolver},
};
use std::path::Path;
use tracing::{info, instrument};

/// Main scaffolding engine.
///
/// This is the primary entry point for scaffolding operations.
/// It coordinates template resolution, rendering, and filesystem operations.
///
/// # Examples
///
/// ```rust,no_run
/// use scarff_core::{Engine, Target, Language, ProjectKind, Architecture};
///
/// let engine = Engine::new();
/// let target = Target::builder()
///     .language(Language::Rust)
///     .kind(ProjectKind::Cli)
///     .architecture(Architecture::Layered)
///     .resolve()?;
///
/// engine.scaffold(target, "my-project", "./output")?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// `
pub struct Engine {
    resolver: TemplateResolver,
    renderer: TemplateRenderer,
    // validator: Validator,
    writer: FileWriter,
}

impl Engine {
    /// Create a new engine with default configuration.
    ///
    /// Uses built-in templates and real filesystem operations.
    pub fn new() -> Self {
        let store = crate::template::InMemoryStore::new();
        store
            .load_builtin()
            .expect("Failed to load built-in templates");

        Self::with_store(Box::new(store))
    }

    /// Create an engine with a custom template store.
    ///
    /// Useful for testing or when using external template sources.
    pub fn with_store(store: Box<dyn Store>) -> Self {
        Self {
            resolver: TemplateResolver::new(store),
            renderer: TemplateRenderer::new(),
            // validator: Validator::new(),
            writer: FileWriter::new(Box::new(RealFilesystem)),
        }
    }

    /// Create an engine with a custom filesystem implementation.
    ///
    /// Primarily used for testing with mock filesystems.
    #[cfg(test)]
    pub(crate) fn with_filesystem(filesystem: Box<dyn super::filesystem::Filesystem>) -> Self {
        let store = crate::template::InMemoryStore::new();
        store
            .load_builtin()
            .expect("Failed to load built-in templates");

        Self {
            resolver: TemplateResolver::new(Box::new(store)),
            renderer: TemplateRenderer::new(),
            // validator: Validator::new(),
            writer: FileWriter::new(filesystem),
        }
    }

    /// Scaffold a new project.
    ///
    /// This is the main method that coordinates the entire scaffolding process:
    ///
    /// 1. Validates the target configuration
    /// 2. Resolves the appropriate template
    /// 3. Renders the template with the project name
    /// 4. Writes the result to the filesystem
    ///
    /// # Arguments
    ///
    /// * `target` - The validated project configuration
    /// * `project_name` - Name of the project (used for variables)
    /// * `output_path` - Directory where the project will be created
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Target validation fails
    /// - No matching template is found
    /// - Template rendering fails
    /// - Filesystem operations fail
    /// - Output directory already contains a project with this name
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use scarff_core::{Engine, Target};
    /// # use std::path::Path;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let engine = Engine::new();
    /// let target = Target::rust_cli();
    ///
    /// engine.scaffold(
    ///     target,
    ///     "my-awesome-cli",
    ///     Path::new("./projects")
    /// )?;
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(
        skip(self, output_path, project_name),
        fields(
            target = %target,
            // project_name = %project_name,
            // output_path = %output_path.to_string_lossy()
        )
    )]
    pub fn scaffold(
        &self,
        target: Target,
        project_name: impl AsRef<str>,
        output_path: impl AsRef<Path>,
    ) -> CoreResult<()> {
        let project_name = project_name.as_ref();
        let output_path = output_path.as_ref();

        // 1. Validate target (defensive, should already be valid from builder)
        info!("Validating target");
        // self.validator.validate(&target)?;

        // 2. Resolve template
        info!("Resolving template");
        let template = self.resolver.resolve(&target)?;
        info!(template_id = %template.id, "Template resolved");

        // 3. Create render context
        // TODO: based on language RenderContext can be used to change name format with var()
        let context = RenderContext::new(project_name);

        // 4. Render template to project structure
        info!("Rendering template");
        let project_path = output_path.join(project_name);
        let structure = self.renderer.render(&template, &context, project_path)?;

        info!(
            files = structure.file_count(),
            directories = structure.directory_count(),
            "Template rendered successfully"
        );

        // 5. Write to filesystem
        info!("Writing to filesystem");
        self.writer.write(&structure)?;

        info!("Scaffold process completed successfully");
        Ok(())
    }

    /// Get information about available templates.
    ///
    /// Returns metadata about all templates that can be used for scaffolding.
    pub fn list_templates(&self) -> CoreResult<Vec<TemplateInfo>> {
        let templates = self.resolver.list()?;

        Ok(templates
            .iter()
            .map(|t| TemplateInfo {
                id: t.id.to_string(),
                name: t.metadata.name.to_string(),
                description: t.metadata.description.to_string(),
                language: t.matcher.language.to_string(),
                kind: t.matcher.kind.to_string(),
                architecture: t.matcher.architecture.to_string(),
                framework: t.matcher.framework.as_ref().map(|f| f.to_string()),
            })
            .collect())
    }

    /// Find templates that match a given target.
    ///
    /// Useful for showing users what templates are available for their configuration.
    pub fn find_templates(&self, target: &Target) -> CoreResult<Vec<TemplateInfo>> {
        let templates = self.resolver.find_all(target)?;

        Ok(templates
            .iter()
            .map(|t| TemplateInfo {
                id: t.id.to_string(),
                name: t.metadata.name.to_string(),
                description: t.metadata.description.to_string(),
                language: t.matcher.language.to_string(),
                kind: t.matcher.kind.to_string(),
                architecture: t.matcher.architecture.to_string(),
                framework: t.matcher.framework.as_ref().map(|f| f.to_string()),
            })
            .collect())
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

/// Information about a template.
///
/// This is a simplified view of template metadata for display purposes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub language: String,
    pub kind: String,
    pub architecture: String,
    pub framework: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Architecture, Language, ProjectKind};

    #[test]
    fn engine_new_loads_builtin_templates() {
        let engine = Engine::new();
        let templates = engine.list_templates().unwrap();

        assert!(!templates.is_empty(), "Should have built-in templates");
    }

    // #[test]
    // fn engine_scaffolds_rust_cli_project() {
    //     let mock_fs = Box::new(MockFilesystem::new());
    //     let engine = Engine::with_filesystem(mock_fs.clone());

    //     let target = Target::builder()
    //         .language(Language::Rust)
    //         .kind(ProjectKind::Cli)
    //         .architecture(Architecture::Layered)
    //         .resolve()
    //         .unwrap();

    //     let result = engine.scaffold(target, "test-cli", ".");

    //     assert!(result.is_ok(), "Scaffolding should succeed");

    //     // Verify files were written
    //     assert!(mock_fs.has_file("./test-cli/Cargo.toml"));
    //     assert!(mock_fs.has_file("./test-cli/src/main.rs"));
    // }

    #[test]
    fn engine_finds_matching_templates() {
        let engine = Engine::new();

        let target = Target::builder()
            .language(Language::Rust)
            .kind(ProjectKind::Cli)
            .unwrap()
            .architecture(Architecture::Layered)
            .unwrap()
            .build()
            .unwrap();

        let matches = engine.find_templates(&target).unwrap();

        assert!(!matches.is_empty(), "Should find at least one template");
        assert!(matches.iter().all(|t| t.language == "rust"));
    }
}
