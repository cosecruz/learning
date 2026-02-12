use std::fmt;
use std::marker::PhantomData;

use crate::domain::{
    error::DomainError,
    value_objects::{Architecture, Framework, Language, ProjectKind},
};

/// Fully validated project configuration.
///
/// Aggregate Root: Consistent boundary for target configuration.
/// All invariants enforced at construction via typestate builder.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Target {
    language: Language,
    kind: ProjectKind,
    framework: Option<Framework>,
    architecture: Architecture,
}

impl Target {
    /// Start building a new Target.
    pub fn builder() -> TargetBuilder<NoLanguage> {
        TargetBuilder::new()
    }

    // Getters
    pub const fn language(&self) -> Language {
        self.language
    }
    pub const fn kind(&self) -> ProjectKind {
        self.kind
    }
    pub const fn framework(&self) -> Option<Framework> {
        self.framework
    }
    pub const fn architecture(&self) -> Architecture {
        self.architecture
    }

    /// Check if this target requires a framework.
    pub fn requires_framework(&self) -> bool {
        self.kind.requires_framework() && self.framework.is_none()
    }

    /// Validate target consistency.
    ///
    /// Called automatically by builder, but available for re-validation.
    pub fn validate(&self) -> Result<(), DomainError> {
        // Language supports kind
        if !self.language.supports(self.kind) {
            return Err(DomainError::IncompatibleLanguageKind {
                language: self.language.to_string(),
                kind: self.kind.to_string(),
            });
        }

        // Framework compatibility
        if let Some(fw) = self.framework {
            if !fw.is_compatible_with(self.language, self.kind) {
                return Err(DomainError::IncompatibleFramework {
                    framework: fw.to_string(),
                    context: format!("{} {}", self.language, self.kind),
                });
            }
        } else if self.kind.requires_framework() {
            return Err(DomainError::MissingRequiredField { field: "framework" });
        }

        // Architecture compatibility
        if !self
            .architecture
            .is_compatible_with(self.language, self.kind, self.framework)
        {
            return Err(DomainError::InvalidArchitecture {
                architecture: self.architecture.to_string(),
                reason: format!("incompatible with {} {}", self.language, self.kind),
            });
        }

        Ok(())
    }
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} ({})", self.language, self.kind, self.architecture)?;
        if let Some(fw) = self.framework {
            write!(f, " + {}", fw)?;
        }
        Ok(())
    }
}

// Typestate markers
pub struct NoLanguage;
pub struct HasLanguage;

/// Typestate builder for Target.
///
/// Enforces at compile time:
/// 1. Language must be set first
/// 2. Framework must be compatible with language
/// 3. Kind must be compatible with language
pub struct TargetBuilder<L> {
    language: Option<Language>,
    kind: Option<ProjectKind>,
    framework: Option<Framework>,
    architecture: Option<Architecture>,
    _marker: PhantomData<L>,
}

impl TargetBuilder<NoLanguage> {
    pub fn new() -> Self {
        Self {
            language: None,
            kind: None,
            framework: None,
            architecture: None,
            _marker: PhantomData,
        }
    }

    pub fn language(self, language: Language) -> TargetBuilder<HasLanguage> {
        TargetBuilder {
            language: Some(language),
            kind: self.kind,
            framework: self.framework,
            architecture: self.architecture,
            _marker: PhantomData,
        }
    }
}

impl Default for TargetBuilder<NoLanguage> {
    fn default() -> Self {
        Self::new()
    }
}

impl TargetBuilder<HasLanguage> {
    pub fn kind(mut self, kind: ProjectKind) -> Result<Self, DomainError> {
        let lang = self.language.unwrap(); // Safe: typestate guarantees this

        if !lang.supports(kind) {
            return Err(DomainError::IncompatibleLanguageKind {
                language: lang.to_string(),
                kind: kind.to_string(),
            });
        }

        self.kind = Some(kind);
        Ok(self)
    }

    pub fn framework(mut self, framework: Framework) -> Result<Self, DomainError> {
        let lang = self.language.unwrap();

        if framework.language() != lang {
            return Err(DomainError::IncompatibleFramework {
                framework: framework.to_string(),
                context: format!("language {}", lang),
            });
        }

        self.framework = Some(framework);
        Ok(self)
    }

    pub fn architecture(mut self, architecture: Architecture) -> Self {
        self.architecture = Some(architecture);
        self
    }

    pub fn build(self) -> Result<Target, DomainError> {
        let language = self.language.unwrap();

        // Infer defaults
        let kind = self
            .kind
            .unwrap_or_else(|| ProjectKind::default_for(language));
        let framework = self.framework.or_else(|| {
            if kind.requires_framework() {
                Framework::infer(language, kind)
            } else {
                None
            }
        });
        let architecture = self
            .architecture
            .unwrap_or_else(|| Architecture::infer(language, kind, framework));

        let target = Target {
            language,
            kind,
            framework,
            architecture,
        };

        target.validate()?;
        Ok(target)
    }
}
