//! Template resolution - finding the right template for a target.
//!
//! The resolver's job is to:
//! 1. Find all templates that match a target
//! 2. Select the most specific match
//! 3. Handle ambiguous matches (multiple templates with same specificity)

use tracing::{debug, info, instrument};

use crate::{
    domain::{Target, validator},
    errors::CoreResult,
    template::{Store, Template, TemplateError},
};

// ============================================================================
// TemplateResolver
// ============================================================================

/// Resolves targets into templates.
///
/// The resolver uses a specificity-based matching algorithm:
/// 1. Find all templates whose matcher matches the target
/// 2. Score each match by specificity (number of constrained fields)
/// 3. Return the most specific match
/// 4. Error if multiple matches have the same highest specificity (ambiguous)
///
/// ## Example
///
/// ```rust,ignore
/// let resolver = TemplateResolver::new(store);
/// let template = resolver.resolve(&target)?;
/// ```
pub struct TemplateResolver {
    store: Box<dyn Store>,
}

impl TemplateResolver {
    /// Create a new resolver with the given store.
    pub fn new(store: Box<dyn Store>) -> Self {
        Self { store }
    }

    /// Resolve a target into a template.
    ///
    /// # Algorithm
    ///
    /// validate target first
    /// 1. Find all matching templates from store
    /// 2. If no matches → error
    /// 3. If one match → return it
    /// 4. If multiple matches:
    ///    - Find highest specificity score
    ///    - Count templates with that score
    ///    - If count == 1 → return that template
    ///    - If count > 1 → error (ambiguous)
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No matching template is found
    /// - Multiple templates match with equal specificity (ambiguous)
    /// - Store access fails
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let target = Target::builder()
    ///     .language(Language::Rust)
    ///     .kind(ProjectKind::Cli)
    ///     .build()?;
    ///
    /// let template = resolver.resolve(&target)?;
    /// ```
    #[instrument(skip(self), fields(target = %target))]
    pub fn resolve(&self, target: &Target) -> CoreResult<Template> {
        info!("Resolving template for target");

        // TODO: validate target
        // 1. Validate target
        validator::validate_target(target)
            .map_err(|e| TemplateError::InvalidTarget(format!("{}", e)))?;
        // TODO: change to return validated Template
        // TODO: TO STORE CONVERT IT TO A Template Record
        // Step 1: Find all matching templates
        let matches = self.store.find(target)?;

        debug!(count = matches.len(), "Found matching templates");

        // Step 2: Handle no matches
        if matches.is_empty() {
            return Err(TemplateError::NoMatch {
                target: target.to_string(),
            }
            .into());
        }

        // Step 3: Handle single match
        if matches.len() == 1 {
            let template = matches.into_iter().next().unwrap();
            info!(
                template_name = template.metadata.name,
                "Resolved to single matching template"
            );
            return Ok(template);
        }

        // Step 4: Handle multiple matches - select most specific
        let max_specificity = matches
            .iter()
            .map(|t| t.matcher.specificity())
            .max()
            .unwrap(); // Safe: we know there's at least one match

        debug!(
            max_specificity = max_specificity,
            "Selecting most specific template"
        );

        let most_specific: Vec<_> = matches
            .into_iter()
            .filter(|t| t.matcher.specificity() == max_specificity)
            .collect();

        // Step 5: Check for ambiguous matches
        if most_specific.len() > 1 {
            let template_names: Vec<_> = most_specific.iter().map(|t| t.metadata.name).collect();

            debug!(
                ?template_names,
                "Multiple templates with same specificity - ambiguous match"
            );

            return Err(TemplateError::AmbiguousMatch {
                target: target.to_string(),
                count: most_specific.len(),
            }
            .into());
        }

        // Step 6: Return the winner
        let template = most_specific.into_iter().next().unwrap();

        validator::validate_template(&template)?;
        info!(
            template_name = template.metadata.name,
            specificity = max_specificity,
            "Resolved to most specific template"
        );

        Ok(template)
    }

    /// List all available templates.
    ///
    /// Useful for showing users what templates are available.
    pub fn list(&self) -> CoreResult<Vec<Template>> {
        self.store.list()
    }

    /// Find all templates that match a target.
    ///
    /// Unlike `resolve()`, this returns ALL matching templates,
    /// not just the most specific one.
    ///
    /// Useful for:
    /// - Showing users alternative templates
    /// - Debugging template matching
    /// - Building interactive template selectors
    #[instrument(skip(self), fields(target = %target))]
    pub fn find_all(&self, target: &Target) -> CoreResult<Vec<Template>> {
        validator::validate_target(target)?;
        let matches = self.store.find(target)?;

        debug!(count = matches.len(), "Found matching templates");

        Ok(matches)
    }

    /// Get a specific template by name and version.
    ///
    /// This bypasses matching and directly retrieves a template by ID.
    pub fn get(&self, name: &str, version: &str) -> CoreResult<Template> {
        let id = crate::template::TemplateId::new(name, version.to_string());
        self.store.get(&id)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        domain::{Architecture, Language, ProjectKind, TargetMatcher, TemplateId},
        template::{
            DirectorySpec, FileSpec, InMemoryStore, TemplateContent, TemplateMetadata,
            TemplateSource, TemplateTree,
        },
    };

    fn create_template(
        name: &'static str,
        language: Option<Language>,
        kind: Option<ProjectKind>,
        architecture: Option<Architecture>,
    ) -> Template {
        Template {
            id: TemplateId::new(name, "0.1.0".to_string()),
            matcher: TargetMatcher {
                language,
                framework: None,
                kind,
                architecture,
            },
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
    fn resolve_single_match() {
        let store = InMemoryStore::new();
        let template = create_template(
            "test",
            Some(Language::Rust),
            Some(ProjectKind::Cli),
            Some(Architecture::Layered),
        );

        store.insert(template).unwrap();

        let resolver = TemplateResolver::new(Box::new(store));
        let target = rust_cli_target();

        let result = resolver.resolve(&target);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().metadata.name, "test");
    }

    #[test]
    fn resolve_no_match() {
        let store = InMemoryStore::new();
        let template = create_template(
            "test",
            Some(Language::Python), // Wrong language
            Some(ProjectKind::Cli),
            Some(Architecture::Layered),
        );

        store.insert(template).unwrap();

        let resolver = TemplateResolver::new(Box::new(store));
        let target = rust_cli_target();

        let result = resolver.resolve(&target);
        assert!(result.is_err());
        // assert!(matches!(
        //     result.unwrap_err().template_error(),
        //     Some(TemplateError::NoMatch { .. })
        // ));
    }

    #[test]
    fn resolve_most_specific() {
        let store = InMemoryStore::new();

        // Less specific (1 field)
        let broad = create_template("broad", Some(Language::Rust), None, None);

        // More specific (3 fields)
        let specific = create_template(
            "specific",
            Some(Language::Rust),
            Some(ProjectKind::Cli),
            Some(Architecture::Layered),
        );

        store.insert(broad).unwrap();
        store.insert(specific).unwrap();

        let resolver = TemplateResolver::new(Box::new(store));
        let target = rust_cli_target();

        let result = resolver.resolve(&target).unwrap();
        assert_eq!(result.metadata.name, "specific");
    }

    #[test]
    fn resolve_ambiguous_match() {
        let store = InMemoryStore::new();

        // Both have specificity 2
        let template1 = create_template(
            "template1",
            Some(Language::Rust),
            Some(ProjectKind::Cli),
            None,
        );

        let template2 = create_template(
            "template2",
            Some(Language::Rust),
            None,
            Some(Architecture::Layered),
        );

        store.insert(template1).unwrap();
        store.insert(template2).unwrap();

        let resolver = TemplateResolver::new(Box::new(store));
        let target = rust_cli_target();

        let result = resolver.resolve(&target);
        assert!(result.is_err());

        // Should be ambiguous match error
        let err = result.unwrap_err();
        // assert!(matches!(
        //     err.template_error(),
        //     Some(TemplateError::AmbiguousMatch { .. })
        // ));
    }

    #[test]
    fn find_all_returns_all_matches() {
        let store = InMemoryStore::new();

        let template1 = create_template("template1", Some(Language::Rust), None, None);
        let template2 = create_template(
            "template2",
            Some(Language::Rust),
            Some(ProjectKind::Cli),
            None,
        );

        store.insert(template1).unwrap();
        store.insert(template2).unwrap();

        let resolver = TemplateResolver::new(Box::new(store));
        let target = rust_cli_target();

        let matches = resolver.find_all(&target).unwrap();
        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn list_all_templates() {
        let store = InMemoryStore::new();

        store
            .insert(create_template("test1", Some(Language::Rust), None, None))
            .unwrap();
        store
            .insert(create_template("test2", Some(Language::Python), None, None))
            .unwrap();

        let resolver = TemplateResolver::new(Box::new(store));
        let templates = resolver.list().unwrap();

        assert_eq!(templates.len(), 2);
    }

    #[test]
    fn get_specific_template() {
        let store = InMemoryStore::new();
        let template = create_template("test", Some(Language::Rust), None, None);

        store.insert(template).unwrap();

        let resolver = TemplateResolver::new(Box::new(store));
        let result = resolver.get("test", "1.0.0");

        assert!(result.is_ok());
        assert_eq!(result.unwrap().metadata.name, "test");
    }

    #[test]
    fn get_nonexistent_template() {
        let store = InMemoryStore::new();
        let resolver = TemplateResolver::new(Box::new(store));

        let result = resolver.get("nonexistent", "1.0.0");
        assert!(result.is_err());
    }
}
