use std::collections::HashSet;
use std::fmt;
use uuid::Uuid;

use super::{project_structure::ProjectStructure, target::Target};

use crate::domain::{
    entities::common::{Permissions, RelativePath},
    error::DomainError,
    value_objects::{Architecture, Framework, Language, ProjectKind},
};

/// Template engine port (trait).
///
/// This is a **driven port** in hexagonal terms.
/// The domain defines the interface, infrastructure implements it.
pub trait TemplateEngine: Send + Sync {
    /// Resolve template for target.
    fn resolve(&self, target: &Target) -> Result<TemplateRecord, DomainError>;

    /// Render template to project structure.
    fn render(
        &self,
        record: &TemplateRecord,
        ctx: &RenderContext,
    ) -> Result<ProjectStructure, DomainError>;
}

/// Context for template rendering.
///
/// Value Object: Immutable data for rendering.
#[derive(Debug, Clone)]
pub struct RenderContext {
    project_name: String,
    variables: std::collections::HashMap<String, String>,
}

impl RenderContext {
    // Create a new render context with a project name.
    ///
    /// Standard variables are automatically populated:
    /// - `PROJECT_NAME`: Original project name
    /// - `PROJECT_NAME_SNAKE`: snake_case version
    /// - `PROJECT_NAME_KEBAB`: kebab-case version
    /// - `PROJECT_NAME_PASCAL`: PascalCase version
    /// - `YEAR`: Current year (for copyright notices)
    pub fn new(project_name: impl Into<String>) -> Self {
        let name = project_name.into();
        let mut vars = std::collections::HashMap::new();

        // Standard variables
        vars.insert("PROJECT_NAME".to_string(), name.clone());
        vars.insert("PROJECT_NAME_SNAKE".to_string(), to_snake_case(&name));
        vars.insert("PROJECT_NAME_KEBAB".to_string(), to_kebab_case(&name));
        vars.insert("PROJECT_NAME_PASCAL".to_string(), to_pascal_case(&name));
        vars.insert("YEAR".to_string(), "2026".to_string());

        Self {
            project_name: name,
            variables: vars,
        }
    }

    pub fn with_variable(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.variables.insert(key.into(), value.into());
        self
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.variables.get(key).map(|s| s.as_str())
    }

    /// Render a template string by replacing {{VARIABLE}} placeholders.
    ///
    /// Simple implementation for MVP. Can be replaced with a proper template engine later.
    pub fn render(&self, template: &str) -> String {
        let mut result = template.to_string();

        for (key, value) in &self.variables {
            let placeholder = format!("{{{{{key}}}}}");
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
    split_words(s).join("_")
}

/// Convert a string to kebab-case.
///
/// Rules:
/// - Replace underscores and spaces with hyphens
/// - Convert to lowercase
fn to_kebab_case(s: &str) -> String {
    split_words(s).join("-")
}

/// Convert a string to PascalCase.
///
/// Rules:
/// - Split on hyphens, underscores, and spaces
/// - Capitalize first letter of each word
/// - Join without separators
fn to_pascal_case(s: &str) -> String {
    split_words(s)
        .into_iter()
        .map(|w| {
            let mut chars = w.chars();
            match chars.next() {
                Some(first) => {
                    let mut out = String::new();
                    out.extend(first.to_uppercase());
                    out.push_str(chars.as_str());
                    out
                }
                None => String::new(),
            }
        })
        .collect()
}

fn split_words(input: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();

    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '_' || c == '-' || c.is_whitespace() {
            if !current.is_empty() {
                words.push(current.to_lowercase());
                current.clear();
            }
            continue;
        }

        // Word boundary: lower -> upper (myAwesome)
        if let Some(next) = chars.peek() {
            if c.is_lowercase() && next.is_uppercase() {
                current.push(c);
                words.push(current.to_lowercase());
                current.clear();
                continue;
            }

            // Acronym boundary: HTTPServer -> HTTP + Server
            if c.is_uppercase()
                && next.is_uppercase()
                && chars.clone().nth(1).map_or(false, |n| n.is_lowercase())
            {
                current.push(c);
                words.push(current.to_lowercase());
                current.clear();
                continue;
            }
        }

        current.push(c);
    }

    if !current.is_empty() {
        words.push(current.to_lowercase());
    }

    words
}

/// Template identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TemplateId {
    name: String,
    version: String,
}

impl TemplateId {
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        let name = name.into();
        let version = version.into();
        assert!(!name.contains('@'), "Template name cannot contain @");
        Self { name, version }
    }

    pub fn parse(s: &str) -> Result<Self, DomainError> {
        let parts: Vec<&str> = s.split('@').collect();
        if parts.len() != 2 {
            return Err(DomainError::InvalidTemplate(format!(
                "Invalid template ID format: {}. Expected 'name@version'",
                s
            )));
        }
        Ok(Self::new(parts[0], parts[1]))
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn version(&self) -> &str {
        &self.version
    }
}

impl fmt::Display for TemplateId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{}", self.name, self.version)
    }
}

/// Storage wrapper for templates.
#[derive(Debug, Clone)]
pub struct TemplateRecord {
    pub uuid: Uuid,
    pub template: Template,
}

impl TemplateRecord {
    pub fn new(template: Template) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            template,
        }
    }

    pub fn with_uuid(uuid: Uuid, template: Template) -> Self {
        Self { uuid, template }
    }

    pub fn validate(&self) -> Result<(), DomainError> {
        if self.uuid.is_nil() {
            return Err(DomainError::InvalidTemplate("UUID cannot be nil".into()));
        }
        self.template.validate()
    }
}

/// Core template aggregate.
#[derive(Debug, Clone)]
pub struct Template {
    pub id: TemplateId,
    pub matcher: TargetMatcher,
    pub metadata: TemplateMetadata,
    pub tree: TemplateTree,
}

impl Template {
    pub fn builder() -> TemplateBuilder {
        TemplateBuilder::default()
    }

    pub fn validate(&self) -> Result<(), DomainError> {
        // ID validation
        if self.id.name().is_empty() {
            return Err(DomainError::InvalidTemplate(
                "Template name cannot be empty".into(),
            ));
        }

        // Metadata validation
        if self.metadata.name.is_empty() {
            return Err(DomainError::InvalidTemplate(
                "Metadata name cannot be empty".into(),
            ));
        }

        // Tree validation
        if self.tree.is_empty() {
            return Err(DomainError::EmptyTemplate {
                template_id: self.id.to_string(),
            });
        }

        // Path validation
        let mut seen = HashSet::new();
        for node in &self.tree.nodes {
            let path = match node {
                TemplateNode::File(f) => f.path.as_str(),
                TemplateNode::Directory(d) => d.path.as_str(),
            };

            if !seen.insert(path.to_string()) {
                return Err(DomainError::DuplicatePath {
                    path: path.to_string(),
                });
            }
        }

        Ok(())
    }

    /// Check if this template matches a target.
    pub fn matches(&self, target: &Target) -> bool {
        self.matcher.matches(target)
    }

    /// Calculate specificity score for conflict resolution.
    pub fn specificity(&self) -> u8 {
        self.matcher.specificity()
    }
}

/// Builder for templates.
#[derive(Default)]
pub struct TemplateBuilder {
    id: Option<TemplateId>,
    matcher: Option<TargetMatcher>,
    metadata: Option<TemplateMetadata>,
    tree: TemplateTree,
}

impl TemplateBuilder {
    pub fn id(mut self, id: TemplateId) -> Self {
        self.id = Some(id);
        self
    }

    pub fn matcher(mut self, matcher: TargetMatcher) -> Self {
        self.matcher = Some(matcher);
        self
    }

    pub fn metadata(mut self, metadata: TemplateMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn tree(mut self, tree: TemplateTree) -> Self {
        self.tree = tree;
        self
    }

    pub fn add_node(mut self, node: TemplateNode) -> Self {
        self.tree.push(node);
        self
    }

    pub fn build(self) -> Result<Template, DomainError> {
        // Early validation
        if self.tree.is_empty() {
            return Err(DomainError::InvalidTemplate(
                "Template tree cannot be empty".into(),
            ));
        }

        Ok(Template {
            id: self
                .id
                .ok_or_else(|| DomainError::MissingRequiredField { field: "id" })?,
            matcher: self
                .matcher
                .ok_or_else(|| DomainError::MissingRequiredField { field: "matcher" })?,
            metadata: self
                .metadata
                .ok_or_else(|| DomainError::MissingRequiredField { field: "metadata" })?,
            tree: self.tree,
        })
    }
}

// ============================================================================
// TargetMatcher - When to Apply a Template
// ============================================================================

/// Describes when a template applies to a [`Target`].
///
/// ## Matching Semantics
///
/// - `None` → **wildcard** (matches any value)
/// - `Some(x)` → **constraint** (must equal target field)
///
/// ## Specificity
///
/// When multiple templates match, the most specific wins:
/// - 4 fields set > 3 fields set > 2 fields set > 1 field set
/// - All else equal, first match wins (deterministic)
///
/// ## Example
///
/// ```rust,ignore
/// // Matches ANY Rust project
/// let broad = TargetMatcher {
///     language: Some(Language::Rust),
///     framework: None,
///     kind: None,
///     architecture: None,
/// };
///
/// // Matches ONLY Rust CLI with Layered architecture
/// let specific = TargetMatcher {
///     language: Some(Language::Rust),
///     framework: None,
///     kind: Some(ProjectKind::Cli),
///     architecture: Some(Architecture::Layered),
/// };
///
/// assert_eq!(broad.specificity(), 1);
/// assert_eq!(specific.specificity(), 3);
/// ```
/// Matcher for target applicability.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TargetMatcher {
    pub language: Option<Language>,
    pub framework: Option<Framework>,
    pub kind: Option<ProjectKind>,
    pub architecture: Option<Architecture>,
}

impl TargetMatcher {
    /// Create a new matcher builder.
    pub fn builder() -> TargetMatcherBuilder {
        TargetMatcherBuilder::default()
    }

    /// Check whether this matcher applies to a target.
    ///
    /// A matcher matches if all non-None fields equal the target's corresponding field.
    pub fn matches(&self, target: &Target) -> bool {
        self.language.map_or(true, |l| l == target.language())
            && self
                .framework
                .map_or(true, |f| Some(f) == target.framework())
            && self.kind.map_or(true, |k| k == target.kind())
            && self
                .architecture
                .map_or(true, |a| a == target.architecture())
    }

    /// Calculate specificity score (higher = more specific).
    ///
    /// Specificity is the count of non-None fields.
    /// Used to resolve conflicts when multiple templates match.
    pub fn specificity(&self) -> u8 {
        u8::try_from(
            [
                self.language.is_some(),
                self.framework.is_some(),
                self.kind.is_some(),
                self.architecture.is_some(),
            ]
            .into_iter()
            .filter(|b| *b)
            .count(),
        )
        .expect("specificity count should fit in u8")
    }
}

#[derive(Default)]
pub struct TargetMatcherBuilder {
    language: Option<Language>,
    framework: Option<Framework>,
    kind: Option<ProjectKind>,
    architecture: Option<Architecture>,
}

impl TargetMatcherBuilder {
    pub fn language(mut self, lang: Language) -> Self {
        self.language = Some(lang);
        self
    }

    pub fn framework(mut self, fw: Framework) -> Self {
        self.framework = Some(fw);
        self
    }

    pub fn kind(mut self, kind: ProjectKind) -> Self {
        self.kind = Some(kind);
        self
    }

    pub fn architecture(mut self, arch: Architecture) -> Self {
        self.architecture = Some(arch);
        self
    }

    pub fn build(self) -> TargetMatcher {
        TargetMatcher {
            language: self.language,
            framework: self.framework,
            kind: self.kind,
            architecture: self.architecture,
        }
    }
}

/// Human-readable metadata describing a template.
///
/// All fields are `&'static str` for simplicity in MVP.
/// Post-MVP: Can be changed to `String` for dynamic templates.
#[derive(Debug, Clone)]
pub struct TemplateMetadata {
    pub name: &'static str,
    pub description: &'static str,
    pub version: &'static str,
    pub author: &'static str,
    pub tags: Vec<&'static str>,
    // / Architecture pattern tag (auto-populated)
    // pub architecture_tag: Option<ArchitecturePattern>,
}

impl TemplateMetadata {
    /// Create new metadata with just a name.
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            description: "",
            version: "0.1.0",
            author: "Scarff",
            tags: Vec::new(),
            // architecture_tag: None,
        }
    }

    pub fn description(mut self, desc: &'static str) -> Self {
        self.description = desc;
        self
    }

    pub fn version(mut self, ver: &'static str) -> Self {
        self.version = ver;
        self
    }

    pub fn author(mut self, author: &'static str) -> Self {
        self.author = author;
        self
    }

    pub fn tags(mut self, tags: Vec<&'static str>) -> Self {
        self.tags = tags;
        self
    }

    // /// Set architecture pattern tag.
    // pub fn architecture_tag(mut self, pattern: ArchitecturePattern) -> Self {
    //     self.architecture_tag = Some(pattern);
    //     self
    // }
}

// Declarative description of a project's filesystem structure.
///
/// Contains an ordered list of files and directories to create.
/// Order matters for dependencies (e.g., create directories before files).
#[derive(Debug, Clone, Default)]
pub struct TemplateTree {
    pub nodes: Vec<TemplateNode>,
}
impl TemplateTree {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, node: TemplateNode) {
        self.nodes.push(node);
    }

    pub fn with_node(mut self, node: TemplateNode) -> Self {
        self.push(node);
        self
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

#[derive(Debug, Clone)]
pub enum TemplateNode {
    File(FileSpec),
    Directory(DirectorySpec),
}

#[derive(Debug, Clone)]
pub struct FileSpec {
    pub path: RelativePath,
    pub content: TemplateContent,
    pub permissions: Permissions,
}

impl FileSpec {
    pub fn new(path: impl Into<RelativePath>, content: TemplateContent) -> Self {
        Self {
            path: path.into(),
            content,
            permissions: Permissions::read_write(),
        }
    }

    pub fn executable(mut self) -> Self {
        self.permissions = Permissions::executable();
        self
    }
}

#[derive(Debug, Clone)]
pub struct DirectorySpec {
    pub path: RelativePath,
    pub permissions: Permissions,
}

impl DirectorySpec {
    pub fn new(path: impl Into<RelativePath>) -> Self {
        Self {
            path: path.into(),
            permissions: Permissions::read_write(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TemplateContent {
    Literal(TemplateSource),
    Parameterized(TemplateSource),
    External(ContentTemplateId),
}

#[derive(Debug, Clone)]
pub enum TemplateSource {
    Static(&'static str),
    Owned(String),
}

impl From<&'static str> for TemplateSource {
    fn from(s: &'static str) -> Self {
        Self::Static(s)
    }
}

impl From<String> for TemplateSource {
    fn from(s: String) -> Self {
        Self::Owned(s)
    }
}

impl TemplateSource {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Static(s) => s,
            Self::Owned(s) => s,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContentTemplateId(pub &'static str);
