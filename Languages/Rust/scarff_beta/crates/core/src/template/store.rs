use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{
    domain::{Target, Template, TemplateId},
    errors::CoreResult,
    template::{built_in_templates, errors::TemplateError},
};

/// Abstract store trait that can be implemented by both in memory or remote Template Registry
pub trait Store {
    /// Find templates matching the given target criteria
    fn find(&self, target: &Target) -> CoreResult<Vec<Template>>;

    /// Get a specific template by ID
    fn get(&self, id: &TemplateId) -> CoreResult<Template>;

    // Insert a template into the store
    fn insert(&self, template: Template) -> CoreResult<()>;

    /// List all available templates
    fn list(&self) -> CoreResult<Vec<Template>>;

    /// Check if a template exists
    fn contains(&self, id: &TemplateId) -> bool;
}

/// Thread-safe in-memory store implementation
pub struct InMemoryStore {
    inner: Arc<RwLock<TemplateStore>>,
}

impl InMemoryStore {
    /// Create a new empty in-memory store
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(TemplateStore::default())),
        }
    }

    /// Create a store with pre-loaded templates
    pub fn with_templates(templates: Vec<Template>) -> CoreResult<Self> {
        let store = Self::new();
        for template in templates {
            store.insert(template)?;
        }
        Ok(store)
    }

    /// Load built-in templates into the store
    pub fn load_builtin(&self) -> CoreResult<()> {
        let builtin_templates = vec![
            built_in_templates::rust_cli_default(),
            // Add more built-in templates here
        ];

        for template in builtin_templates {
            // Use insert which handles duplicates gracefully
            let _ = self.insert(template);
        }

        Ok(())
    }

    /// Clear all templates from the store
    pub fn clear(&self) -> CoreResult<()> {
        let mut store = self.inner.write().map_err(|_| TemplateError::LockError)?;

        store.templates.clear();
        Ok(())
    }

    // Get the number of templates in the store
    pub fn len(&self) -> usize {
        self.inner
            .read()
            .map(|store| store.templates.len())
            .unwrap_or(0)
    }

    /// Check if the store is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for InMemoryStore {
    fn default() -> Self {
        Self::new()
    }
}

impl Store for InMemoryStore {
    fn find(&self, target: &Target) -> CoreResult<Vec<Template>> {
        let store = self.inner.read().map_err(|_| TemplateError::LockError)?;

        let matching_templates: Vec<Template> = store
            .templates
            .values()
            .filter(|t| t.matcher.matches(target))
            .cloned()
            .collect();

        Ok(matching_templates)
    }

    fn get(&self, id: &TemplateId) -> CoreResult<Template> {
        let store = self.inner.read().map_err(|_| TemplateError::LockError)?;

        let result = store
            .templates
            .get(id)
            .cloned()
            .ok_or_else(|| TemplateError::NotFound(id.clone()))?;

        Ok(result)
    }

    fn contains(&self, id: &TemplateId) -> bool {
        self.inner
            .read()
            .map(|store| store.templates.contains_key(id))
            .unwrap_or(false)
    }

    fn insert(&self, template: Template) -> CoreResult<()> {
        let mut store = self.inner.write().map_err(|_| TemplateError::LockError)?;

        // Validate template before insertion
        validate_template(&template)?;

        // Insert or update (idempotent operation)
        store.templates.insert(template.id.clone(), template);

        Ok(())
    }

    fn list(&self) -> CoreResult<Vec<Template>> {
        let store = self.inner.read().map_err(|_| TemplateError::LockError)?;

        Ok(store.templates.values().cloned().collect())
    }
}

/// Internal storage structure
#[derive(Debug, Clone, Default)]
struct TemplateStore {
    templates: HashMap<TemplateId, Template>,
}

/// Validate a template before insertion
fn validate_template(template: &Template) -> CoreResult<()> {
    // Check that ID is not empty
    if template.id.0.is_empty() {
        Err(TemplateError::InvalidTemplate(
            "Template ID cannot be empty".to_string(),
        ))?
    }

    // Validate metadata
    if template.metadata.name.is_empty() {
        Err(TemplateError::InvalidTemplate(
            "Template name cannot be empty".to_string(),
        ))?
    }

    // Validate tree has at least one node
    if template.tree.nodes.is_empty() {
        Err(TemplateError::InvalidTemplate(
            "Template tree must have at least one node".to_string(),
        ))?
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        Architecture, Language, ProjectKind,
        domain::{Framework, RustFramework, TargetMatcher, TemplateMetadata, TemplateTree},
        template::built_in_templates,
    };

    use super::*;

    fn rust_cli_layered_target() -> Target {
        Target::builder()
            .language(Language::Rust)
            .kind(ProjectKind::Cli)
            .unwrap()
            .architecture(Architecture::Layered)
            .unwrap()
            .build()
            .unwrap()
    }

    #[test]
    fn test_new_store_is_empty() {
        let store = InMemoryStore::new();
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn test_insert_and_get() {
        let store = InMemoryStore::new();
        let template = built_in_templates::rust_cli_default();
        let id = template.id.clone();

        store.insert(template.clone()).unwrap();

        let retrieved = store.get(&id).unwrap();
        assert_eq!(retrieved.id, id);
    }

    #[test]
    fn test_find_matching_templates() {
        let store = InMemoryStore::new();
        store.load_builtin().unwrap();

        let target = Target::builder()
            .language(Language::Rust)
            .kind(ProjectKind::Cli)
            .unwrap()
            .architecture(Architecture::Layered)
            .unwrap()
            .build()
            .unwrap();

        let results = store.find(&target).unwrap();
        assert!(!results.is_empty());
    }

    #[test]
    fn test_idempotent_insert() {
        let store = InMemoryStore::new();
        let template = built_in_templates::rust_cli_default();

        store.insert(template.clone()).unwrap();
        store.insert(template.clone()).unwrap(); // Should not error

        assert_eq!(store.len(), 1);
    }

    #[test]
    fn test_validate_invalid_template() {
        let invalid_template = Template {
            id: TemplateId(""), // Empty ID
            metadata: TemplateMetadata::new("Test"),
            matcher: TargetMatcher {
                language: Language::Rust,
                framework: None,
                kind: ProjectKind::Cli,
                architecture: Architecture::Layered,
            },
            tree: TemplateTree::new(),
        };

        let result = validate_template(&invalid_template);
        assert!(result.is_err());
    }

    #[test]
    fn matcher_matches_exact_target() {
        let matcher = TargetMatcher {
            language: Language::Rust,
            framework: None,
            kind: ProjectKind::Cli,
            architecture: Architecture::Layered,
        };

        let target = rust_cli_layered_target();

        assert!(matcher.matches(&target));
    }

    #[test]
    fn matcher_rejects_wrong_language() {
        let matcher = TargetMatcher {
            language: Language::Python,
            framework: None,
            kind: ProjectKind::Cli,
            architecture: Architecture::Layered,
        };

        let target = rust_cli_layered_target();

        assert!(!matcher.matches(&target));
    }

    #[test]
    fn matcher_rejects_wrong_architecture() {
        let matcher = TargetMatcher {
            language: Language::Rust,
            framework: None,
            kind: ProjectKind::Cli,
            architecture: Architecture::MVC,
        };

        let target = rust_cli_layered_target();

        assert!(!matcher.matches(&target));
    }

    #[test]
    fn matcher_framework_none_matches_any_framework() {
        let matcher = TargetMatcher {
            language: Language::Rust,
            framework: None,
            kind: ProjectKind::Cli,
            architecture: Architecture::Layered,
        };

        let mut target = rust_cli_layered_target();
        target.framework = Some(Framework::Rust(RustFramework::Axum));

        println!("{target:?} {matcher:?}");

        assert!(!matcher.matches(&target));
    }

    #[test]
    fn matcher_framework_some_requires_exact_match() {
        let matcher = TargetMatcher {
            language: Language::Rust,
            framework: Some(Framework::Rust(RustFramework::Axum)),
            kind: ProjectKind::Cli,
            architecture: Architecture::Layered,
        };

        let target = rust_cli_layered_target();

        assert!(!matcher.matches(&target));
    }
}
