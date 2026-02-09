//! - TemplateResolver: resolves Target to a Template. This is where the matching happens
//!   Goal is to find the right template for target

use crate::{
    CoreError,
    domain::{Target, Template},
    errors::CoreResult,
    template::{errors::TemplateError, store::Store},
};

pub struct TemplateResolver {
    store: Box<dyn Store>,
}

impl TemplateResolver {
    pub fn new(store: Box<dyn Store>) -> Self {
        Self { store }
    }

    pub fn resolve(&self, target: &Target) -> CoreResult<Template> {
        let matches = self.store.find(target)?;

        match matches.len() {
            // No matches
            0 => Err(TemplateError::NoMatch {
                target: target.clone(),
            })?,

            // Exactly one match
            1 => Ok(matches.into_iter().next().unwrap()),

            // Multiple matches: choose most specific
            _ => Ok(matches
                .into_iter()
                .max_by_key(|t| t.matcher.specificity())
                .ok_or_else(|| {
                    TemplateError::InvalidTemplate("Template matcher returned empty result".into())
                })?),
        }
    }

    // TODO: POST MVP
    ///list all templates available from store
    pub fn list(&self) -> CoreResult<Vec<Template>> {
        todo!("post mvp")
    }

    ///return templates that is available that targets resolve to
    pub fn find_all(&self, _target: &Target) -> CoreResult<Vec<Template>> {
        todo!("post mvp")
    }
}
