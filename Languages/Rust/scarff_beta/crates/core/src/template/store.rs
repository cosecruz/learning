use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::domain::{
    Architecture, DirectorySpec, FileSpec, Language, ProjectType, Target, TargetMatcher, Template,
    TemplateContent, TemplateId, TemplateMetadata, TemplateNode, TemplateTree,
};

/// Abstract template registry
pub trait Store {
    fn find(&self, target: &Target) -> Vec<Template>;
}

/// In-memory implementation
#[derive(Clone)]
pub struct InMemoryStore {
    inner: Rc<RefCell<TemplateStore>>,
}

impl InMemoryStore {
    pub fn new(store: TemplateStore) -> Self {
        Self {
            inner: Rc::new(RefCell::new(store)),
        }
    }

    pub fn load(&self, templates: Vec<Template>) {
        let mut store = self.inner.borrow_mut();

        for template in templates {
            store.templates.insert(template.id.clone(), template);
        }
    }
}

impl Store for InMemoryStore {
    fn find(&self, target: &Target) -> Vec<Template> {
        let store = self.inner.borrow();

        store
            .templates
            .values()
            .filter(|template| template.matcher.matches(target))
            .cloned()
            .collect()
    }
}

#[derive(Debug, Default)]
pub struct TemplateStore {
    pub templates: HashMap<TemplateId, Template>,
}

// Template definitions
pub fn rust_cli_layered() -> Template {
    Template {
        id: TemplateId("rust_cli_layered_v001"),
        metadata: TemplateMetadata::new("Rust CLI (Layered)").version("1.0.0"),
        matcher: TargetMatcher {
            language: Language::Rust,
            framework: None,
            project_type: ProjectType::Cli,
            architecture: Architecture::Layered,
        },
        tree: TemplateTree::new()
            .with_node(TemplateNode::Directory(DirectorySpec::new("src")))
            .with_node(TemplateNode::File(FileSpec::new(
                "src/main.rs",
                TemplateContent::Template(include_str!("templates/rust_cli/main.rs.template")),
            )))
            .with_node(TemplateNode::File(FileSpec::new(
                "Cargo.toml",
                TemplateContent::Template(include_str!("templates/rust_cli/Cargo.toml.template")),
            ))),
    }
}

#[cfg(test)]
mod matcher_tests {
    use crate::domain::{Framework, RustFramework};

    use super::*;

    fn rust_cli_layered_target() -> Target {
        Target::builder()
            .language(Language::Rust)
            .project_type(ProjectType::Cli)
            .architecture(Architecture::Layered)
            .resolve()
            .unwrap()
    }

    #[test]
    fn matcher_matches_exact_target() {
        let matcher = TargetMatcher {
            language: Language::Rust,
            framework: None,
            project_type: ProjectType::Cli,
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
            project_type: ProjectType::Cli,
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
            project_type: ProjectType::Cli,
            architecture: Architecture::Mvc,
        };

        let target = rust_cli_layered_target();

        assert!(!matcher.matches(&target));
    }

    #[test]
    fn matcher_framework_none_matches_any_framework() {
        let matcher = TargetMatcher {
            language: Language::Rust,
            framework: None,
            project_type: ProjectType::Cli,
            architecture: Architecture::Layered,
        };

        let mut target = rust_cli_layered_target();
        target.framework = Some(Framework::Rust(RustFramework::Axum));

        assert!(!matcher.matches(&target));
    }

    #[test]
    fn matcher_framework_some_requires_exact_match() {
        let matcher = TargetMatcher {
            language: Language::Rust,
            framework: Some(Framework::Rust(RustFramework::Axum)),
            project_type: ProjectType::Cli,
            architecture: Architecture::Layered,
        };

        let target = rust_cli_layered_target();

        assert!(!matcher.matches(&target));
    }
}
