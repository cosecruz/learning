//! Template storage and retrieval.
//!
//! This module provides storage abstractions for templates:
//! - In-memory store (for built-in templates)
//! - Extensible trait for future stores (filesystem, remote registry)

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use tracing::{debug, info, instrument, warn};

use crate::{
    domain::Target,
    errors::CoreResult,
    template::{Template, TemplateError, TemplateId, TemplateRecord},
};

// ============================================================================
// Store Trait
// ============================================================================

/// Abstract store trait for template storage.
///
/// This trait can be implemented by:
/// - In-memory stores (for built-in templates)
/// - Filesystem stores (for user templates in ~/.scarff/templates)
/// - Remote registries (for community templates)
///
/// ## Thread Safety
///
/// Implementations must be thread-safe (Send + Sync).
///
/// ## Example
///
/// ```rust,ignore
/// let store: Box<dyn Store> = Box::new(InMemoryStore::new());
/// store.insert(template)?;
/// let template = store.get(&id)?;
/// ```
pub trait Store: Send + Sync {
    /// Find templates matching the given target criteria.
    ///
    /// Returns all templates whose matcher matches the target.
    /// The caller is responsible for selecting the best match
    /// (typically using specificity scoring).
    fn find(&self, target: &Target) -> CoreResult<Vec<Template>>;

    /// Get a specific template by ID.
    ///
    /// # Errors
    ///
    /// Returns `TemplateError::NotFound` if the template doesn't exist.
    fn get(&self, id: &TemplateId) -> CoreResult<Template>;

    /// Insert a template into the store.
    ///
    /// If a template with the same ID already exists, it is replaced.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Template validation fails
    /// - Store lock acquisition fails
    fn insert(&self, template: Template) -> CoreResult<()>;

    /// List all available templates.
    fn list(&self) -> CoreResult<Vec<Template>>;

    /// Check if a template exists by ID.
    fn contains(&self, id: &TemplateId) -> bool;

    /// Remove a template by ID.
    ///
    /// # Errors
    ///
    /// Returns `TemplateError::NotFound` if the template doesn't exist.
    fn remove(&self, id: &TemplateId) -> CoreResult<()>;
}

// ============================================================================
// InMemoryStore
// ============================================================================

/// Thread-safe in-memory template store.
///
/// This is the default store for built-in templates. It stores templates
/// in a HashMap protected by an RwLock for thread-safe access.
///
/// ## Example
///
/// ```rust,ignore
/// let store = InMemoryStore::new();
/// store.load_builtin()?;
///
/// let templates = store.find(&target)?;
/// ```
#[derive(Clone)]
pub struct InMemoryStore {
    inner: Arc<RwLock<TemplateStore>>,
}

impl InMemoryStore {
    /// Create a new empty in-memory store.
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(TemplateStore::default())),
        }
    }

    /// Create a store with pre-loaded templates.
    ///
    /// # Errors
    ///
    /// Returns an error if any template fails validation.
    pub fn with_templates(templates: Vec<Template>) -> CoreResult<Self> {
        let store = Self::new();
        for template in templates {
            store.insert(template)?;
        }
        Ok(store)
    }

    /// Load built-in templates into the store.
    ///
    /// This loads all templates from the `built_in_templates` module.
    ///
    /// # Errors
    ///
    /// Returns an error if any template fails validation or insertion.
    #[instrument(skip(self))]
    pub fn load_builtin(&self) -> CoreResult<()> {
        info!("Loading built-in templates");

        let builtin_templates = crate::template::built_in_templates::all_templates();

        for template in builtin_templates {
            debug!(template_id = %template.metadata.name, "Loading template");
            self.insert(template)?; // ✅ Propagate errors
        }

        info!(count = self.len(), "Built-in templates loaded successfully");
        Ok(())
    }

    /// Clear all templates from the store.
    pub fn clear(&self) -> CoreResult<()> {
        let mut store = self.inner.write().map_err(|_| TemplateError::LockError)?;

        store.templates.clear();
        Ok(())
    }

    /// Get the number of templates in the store.
    pub fn len(&self) -> usize {
        self.inner
            .read()
            .map(|store| store.templates.len())
            .unwrap_or(0)
    }

    /// Check if the store is empty.
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
    #[instrument(skip(self, target), fields(target = %target))]
    fn find(&self, target: &Target) -> CoreResult<Vec<Template>> {
        let store = self.inner.read().map_err(|_| TemplateError::LockError)?;

        let matching_templates: Vec<Template> = store
            .templates
            .values()
            .filter(|t| t.matcher.matches(target))
            .cloned()
            .collect();

        debug!(count = matching_templates.len(), "Found matching templates");

        Ok(matching_templates)
    }

    #[instrument(skip(self), fields(template_id = %id))]
    fn get(&self, id: &TemplateId) -> CoreResult<Template> {
        let store = self.inner.read().map_err(|_| TemplateError::LockError)?;

        store
            .templates
            .get(id)
            .cloned()
            .ok_or_else(|| TemplateError::NotFound(id.clone()).into())
    }

    fn contains(&self, id: &TemplateId) -> bool {
        self.inner
            .read()
            .map(|store| store.templates.contains_key(id))
            .unwrap_or(false)
    }

    #[instrument(skip(self, template), fields(template_id = %template.metadata.name))]
    fn insert(&self, template: Template) -> CoreResult<()> {
        // Validate template before insertion
        validate_template(&template)?;

        let mut store = self.inner.write().map_err(|_| TemplateError::LockError)?;

        // Create TemplateId from metadata
        let id = TemplateId::new(
            template.metadata.name,
            template.metadata.version.to_string(),
        );

        // Insert or update (idempotent operation)
        store.templates.insert(id, template);

        debug!("Template inserted successfully");
        Ok(())
    }

    fn list(&self) -> CoreResult<Vec<Template>> {
        let store = self.inner.read().map_err(|_| TemplateError::LockError)?;

        Ok(store.templates.values().cloned().collect())
    }

    #[instrument(skip(self), fields(template_id = %id))]
    fn remove(&self, id: &TemplateId) -> CoreResult<()> {
        let mut store = self.inner.write().map_err(|_| TemplateError::LockError)?;

        store
            .templates
            .remove(id)
            .ok_or_else(|| TemplateError::NotFound(id.clone()))?;

        debug!("Template removed successfully");
        Ok(())
    }
}

// ============================================================================
// Internal Storage
// ============================================================================

///TODO: use Template record instead
/// Internal storage structure.
#[derive(Debug, Clone, Default)]
struct TemplateStore {
    templates: HashMap<TemplateId, Template>,
}

// ============================================================================
// Validation
// ============================================================================

/// Validate a template before insertion.
///
/// # Errors
///
/// Returns an error if:
/// - Template metadata is invalid (empty name, etc.)
/// - Template tree is empty
/// - Template tree contains invalid paths
/// TODO: inserts TemplateRecord instead of Template
/// TODO: validate that
fn validate_template(template: &Template) -> CoreResult<()> {
    // Check that name is not empty
    if template.metadata.name.is_empty() {
        return Err(
            TemplateError::InvalidTemplate("Template name cannot be empty".to_string()).into(),
        );
    }

    // Check that version is not empty
    if template.metadata.version.is_empty() {
        return Err(
            TemplateError::InvalidTemplate("Template version cannot be empty".to_string()).into(),
        );
    }

    // Validate tree has at least one node
    if template.tree.is_empty() {
        return Err(TemplateError::InvalidTemplate(
            "Template tree must have at least one node".to_string(),
        )
        .into());
    }

    // Additional validation can be added here:
    // - Check for duplicate paths
    // - Validate path characters
    // - Check permissions

    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        domain::{Architecture, Language, ProjectKind, TargetMatcher},
        template::{
            DirectorySpec, FileSpec, TemplateContent, TemplateMetadata, TemplateSource,
            TemplateTree,
        },
    };

    fn create_test_template(name: &'static str) -> Template {
        Template {
            matcher: TargetMatcher::builder()
                .language(Language::Rust)
                .kind(ProjectKind::Cli)
                .architecture(Architecture::Layered)
                .build(),
            metadata: TemplateMetadata::new(name).version("1.0.0"),
            tree: TemplateTree::new().with_node(crate::template::TemplateNode::File(
                FileSpec::new(
                    "main.rs",
                    TemplateContent::Literal(TemplateSource::Static("fn main() {}")),
                ),
            )),
        }
    }

    fn rust_cli_target() -> Target {
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
    fn new_store_is_empty() {
        let store = InMemoryStore::new();
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn insert_and_get() {
        let store = InMemoryStore::new();
        let template = create_test_template("test");

        store.insert(template.clone()).unwrap();

        let id = TemplateId::new("test", "1.0.0".to_string());
        let retrieved = store.get(&id).unwrap();
        assert_eq!(retrieved.metadata.name, "test");
    }

    #[test]
    fn find_matching_templates() {
        let store = InMemoryStore::new();
        let template = create_test_template("rust-cli");

        store.insert(template).unwrap();

        let target = rust_cli_target();
        let results = store.find(&target).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].metadata.name, "rust-cli");
    }

    #[test]
    fn idempotent_insert() {
        let store = InMemoryStore::new();
        let template = create_test_template("test");

        store.insert(template.clone()).unwrap();
        store.insert(template.clone()).unwrap(); // Should not error

        assert_eq!(store.len(), 1);
    }

    #[test]
    fn validate_invalid_template_empty_name() {
        let mut template = create_test_template("test");
        template.metadata.name = "";

        let result = validate_template(&template);
        assert!(result.is_err());
    }

    #[test]
    fn validate_invalid_template_empty_tree() {
        let template = Template {
            matcher: TargetMatcher::builder().build(),
            metadata: TemplateMetadata::new("test"),
            tree: TemplateTree::new(),
        };

        let result = validate_template(&template);
        assert!(result.is_err());
    }

    #[test]
    fn contains_check() {
        let store = InMemoryStore::new();
        let template = create_test_template("test");

        let id = TemplateId::new("test", "1.0.0".to_string());
        assert!(!store.contains(&id));

        store.insert(template).unwrap();
        assert!(store.contains(&id));
    }

    #[test]
    fn list_all_templates() {
        let store = InMemoryStore::new();

        store.insert(create_test_template("test1")).unwrap();
        store.insert(create_test_template("test2")).unwrap();

        let templates = store.list().unwrap();
        assert_eq!(templates.len(), 2);
    }

    #[test]
    fn remove_template() {
        let store = InMemoryStore::new();
        let template = create_test_template("test");

        store.insert(template).unwrap();

        let id = TemplateId::new("test", "1.0.0".to_string());
        assert!(store.contains(&id));

        store.remove(&id).unwrap();
        assert!(!store.contains(&id));
    }

    #[test]
    fn remove_nonexistent_template_errors() {
        let store = InMemoryStore::new();
        let id = TemplateId::new("nonexistent", "1.0.0".to_string());

        let result = store.remove(&id);
        assert!(result.is_err());
    }

    #[test]
    fn clear_store() {
        let store = InMemoryStore::new();

        store.insert(create_test_template("test1")).unwrap();
        store.insert(create_test_template("test2")).unwrap();

        assert_eq!(store.len(), 2);

        store.clear().unwrap();
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn with_templates_constructor() {
        let templates = vec![create_test_template("test1"), create_test_template("test2")];

        let store = InMemoryStore::with_templates(templates).unwrap();
        assert_eq!(store.len(), 2);
    }

    #[test]
    fn store_is_thread_safe() {
        use std::sync::Arc;
        use std::thread;

        let store = Arc::new(InMemoryStore::new());

        let mut handles = vec![];

        // Spawn threads to insert templates
        for i in 0..10 {
            let store_clone = Arc::clone(&store);
            let handle = thread::spawn(move || {
                let template = create_test_template("test");
                store_clone.insert(template).unwrap();
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // Should have successfully inserted (some may be duplicates due to same ID)
        assert!(store.len() > 0);
    }
}
