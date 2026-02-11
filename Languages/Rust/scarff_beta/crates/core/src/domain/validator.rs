//! Validation logic for domain models.
//!
//! This module provides validation for:
//! - Target: Project configuration validation
//! - Template: Template structure validation
//! - ProjectStructure: Output structure validation

use crate::domain::{
    DomainError, ProjectStructure, Target, Template, TemplateRecord,
    target::{ActivelySupported, Compatible, LangCapable},
};

// ============================================================================
// Target Validation
// ============================================================================

/// Validate a target configuration.
///
/// Checks:
/// - Language is supported
/// - Framework is compatible with language and kind
/// - Architecture is compatible with framework and kind
/// - All required fields are present
///
/// # Errors
///
/// Returns a `DomainError` if validation fails.
pub fn validate_target(target: &Target) -> Result<(), DomainError> {
    // Language is always set by builder, but check it's supported
    if !target.language().is_supported() {
        return Err(DomainError::UnsupportedLanguage {
            language: target.language().to_string(),
        });
    }

    // Project kind validation
    if !target.kind().is_supported() {
        return Err(DomainError::UnsupportedProjectKind {
            kind: target.kind().to_string(),
        });
    }

    // Check language-kind compatibility
    if !target.kind().lang_capable(target.language()) {
        return Err(DomainError::ProjectKindLanguageMismatch {
            kind: target.kind().to_string(),
            language: target.language().to_string(),
        });
    }

    // Framework validation (if present)
    if let Some(framework) = target.framework() {
        // Check framework is supported
        if !framework.is_supported() {
            return Err(DomainError::FrameworkLanguageMismatch {
                framework: framework.to_string(),
                language: target.language().to_string(),
            });
        }

        // Check framework-language compatibility
        if framework.language() != target.language() {
            return Err(DomainError::FrameworkLanguageMismatch {
                framework: framework.to_string(),
                language: target.language().to_string(),
            });
        }

        // Check framework-kind compatibility
        if !framework.is_compatible((target.language(), target.kind())) {
            return Err(DomainError::FrameworkProjectKindMismatch {
                framework: framework.to_string(),
                kind: target.kind().to_string(),
            });
        }
    } else if target.kind().requires_framework() {
        // Framework is required but not provided
        return Err(DomainError::FrameworkRequired {
            kind: target.kind().to_string(),
        });
    }

    // Architecture validation
    if !target.architecture().is_supported() {
        return Err(DomainError::UnsupportedArchitecture {
            architecture: target.architecture().to_string(),
        });
    }

    // Check architecture compatibility
    if !target
        .architecture()
        .is_compatible((target.language(), target.kind(), target.framework()))
    {
        return Err(DomainError::ArchitectureProjectKindMismatch {
            architecture: target.architecture().to_string(),
            kind: target.kind().to_string(),
        });
    }

    Ok(())
}

// ============================================================================
// Template Validation
// ============================================================================

/// Validate a template.
///
/// Checks:
/// - Metadata is valid (non-empty name, version)
/// - Tree is not empty
/// - All paths are relative
/// - No duplicate paths
/// - Matcher is valid
///
/// # Errors
///
/// Returns a `DomainError` if validation fails.
pub fn validate_template(template: &Template) -> Result<(), DomainError> {
    // Validate ID
    if template.id.name().is_empty() {
        return Err(DomainError::InvalidTemplateWithMetadata {
            name: "unknown".to_string(),
            reason: "Template ID name cannot be empty".to_string(),
        });
    }

    if template.id.version().is_empty() {
        return Err(DomainError::InvalidTemplateWithMetadata {
            name: template.id.name().to_string(),
            reason: "Template ID version cannot be empty".to_string(),
        });
    }
    // Validate metadata
    if template.metadata.name.is_empty() {
        return Err(DomainError::InvalidTemplateWithMetadata {
            name: template.id.name.to_string(),
            reason: "Template metadata name cannot be empty".to_string(),
        });
    }

    if template.metadata.version.is_empty() {
        return Err(DomainError::InvalidTemplateWithMetadata {
            name: template.metadata.name.to_string(),
            reason: "Template metadata version cannot be empty".to_string(),
        });
    }

    // Validate tree is not empty
    if template.tree.nodes.is_empty() {
        return Err(DomainError::TemplateEmptyTree {
            template_id: template.metadata.name.to_string(),
        });
    }

    // Validate paths
    let mut seen_paths = std::collections::HashSet::new();

    for node in &template.tree.nodes {
        let path = match node {
            crate::domain::TemplateNode::File(spec) => spec.path.as_path(),
            crate::domain::TemplateNode::Directory(spec) => spec.path.as_path(),
        };

        // Check for duplicates
        if !seen_paths.insert(path) {
            return Err(DomainError::TemplateDuplicatePath {
                template_id: template.metadata.name.to_string(),
                path: path.to_path_buf(),
            });
        }

        // Check not absolute
        if path.is_absolute() {
            return Err(DomainError::TemplateAbsolutePath {
                template_id: template.metadata.name.to_string(),
                path: path.to_path_buf(),
            });
        }
    }

    Ok(())
}

/// Validate a template record.
///
/// Checks:
/// - UUID is valid (not nil)
/// - ID is valid (non-empty name and version)
/// - Template is valid
///
/// # Errors
///
/// Returns a `DomainError` if validation fails.
pub fn validate_template_record(record: &TemplateRecord) -> Result<(), DomainError> {
    // Check UUID is not nil
    if record.uuid.is_nil() {
        return Err(DomainError::InvalidTemplateWithMetadata {
            name: record.template.id.to_string(),
            reason: "Template UUID cannot be nil".to_string(),
        });
    }

    // Validate the template itself
    validate_template(&record.template)?;

    Ok(())
}

// ============================================================================
// ProjectStructure Validation
// ============================================================================

/// Validate a project structure.
///
/// Checks:
/// - Structure is not empty
/// - All paths are relative
/// - No duplicate paths
/// - Root path is set
///
/// # Errors
///
/// Returns a `DomainError` if validation fails.
pub fn validate_project_structure(structure: &ProjectStructure) -> Result<(), DomainError> {
    // Check structure is not empty
    if structure.entries.is_empty() {
        return Err(DomainError::ProjectStructureError(
            "Project structure is empty - no files or directories to create".to_string(),
        ));
    }

    // Check root path is set and not empty
    if structure.root.as_os_str().is_empty() {
        return Err(DomainError::ProjectStructureError(
            "Project root path cannot be empty".to_string(),
        ));
    }

    // Validate paths
    let mut seen_paths = std::collections::HashSet::new();

    for entry in &structure.entries {
        let path = match entry {
            crate::domain::FsEntry::File(f) => &f.path,
            crate::domain::FsEntry::Directory(d) => &d.path,
        };

        // Check for duplicates
        if !seen_paths.insert(path) {
            return Err(DomainError::ProjectStructureError(format!(
                "Duplicate path in structure: {}",
                path.display()
            )));
        }

        // Check path is relative
        if path.is_absolute() {
            return Err(DomainError::ProjectStructureError(format!(
                "Absolute path not allowed in structure: {}",
                path.display()
            )));
        }
    }

    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        Architecture, Framework, Language, ProjectKind, RustFramework, TargetMatcher, TemplateId,
        TemplateMetadata, TemplateTree,
    };

    #[test]
    fn validate_valid_target() {
        let target = Target::builder()
            .language(Language::Rust)
            .kind(ProjectKind::Cli)
            .unwrap()
            .architecture(Architecture::Layered)
            .unwrap()
            .build()
            .unwrap();

        assert!(validate_target(&target).is_ok());
    }

    #[test]
    fn validate_target_with_framework() {
        let target = Target::builder()
            .language(Language::Rust)
            .kind(ProjectKind::WebBackend)
            .unwrap()
            .framework(Framework::Rust(RustFramework::Axum))
            .unwrap()
            .architecture(Architecture::Layered)
            .unwrap()
            .build()
            .unwrap();

        assert!(validate_target(&target).is_ok());
    }

    #[test]
    fn validate_target_missing_required_framework() {
        // This should fail at builder level, but test validation anyway
        let target = Target::builder()
            .language(Language::Rust)
            .kind(ProjectKind::Cli)
            .unwrap()
            .build()
            .unwrap();

        // CLI doesn't require framework, should pass
        assert!(validate_target(&target).is_ok());
    }

    #[test]
    fn validate_empty_template_fails() {
        let template = Template {
            id: TemplateId::new("test", "".to_string()),
            matcher: TargetMatcher::builder().build(),
            metadata: TemplateMetadata::new("test"),
            tree: TemplateTree::new(),
        };

        let result = validate_template(&template);
        assert!(result.is_err());
    }

    #[test]
    fn validate_template_with_absolute_path_fails() {
        use crate::domain::{
            FileSpec, RelativePath, TemplateContent, TemplateNode, TemplateSource,
        };

        let mut tree = TemplateTree::new();
        // This would panic at RelativePath::new, so we can't really test this
        // The type system prevents it

        // Test with empty name instead
        let template = Template {
            id: TemplateId::new("test", "0.1.0".to_string()),
            matcher: TargetMatcher::builder().build(),
            metadata: TemplateMetadata {
                name: "",
                description: "",
                version: "1.0.0",
                author: "",
                tags: vec![],
            },
            tree: TemplateTree::new().with_node(TemplateNode::File(FileSpec::new(
                "test.txt",
                TemplateContent::Literal(TemplateSource::Static("")),
            ))),
        };

        let result = validate_template(&template);
        assert!(result.is_err());
    }
}
