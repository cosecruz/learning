//! - TemplateResolver: resolves Target to a Template. This is where the matching happens
//! Goal is to find the right template for target

use anyhow::Context;

use crate::{
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
        let matches = self
            .store
            .find(target)
            .context("Failed to search template store")?;

        match matches.len() {
            // if no matches
            0 => Err(TemplateError::NoMatch {
                target: target.clone(),
            })
            .context(format!("No template found for target: {}", target))?,

            //if 1 match
            1 => Ok(matches[0].clone()),

            // if >1 matches
            _ => {
                // Multiple matches: pick most specific or use scoring algorithm
                let best = matches
                    .into_iter()
                    .max_by_key(|t| t.matcher.specificity())
                    .context("Failed to select best template")?;
                Ok(best)
            }
        }
    }

    // TODO: POST MVP
    ///list all templates available from store
    pub fn list(&self) -> CoreResult<Vec<Template>> {
        todo!("post mvp")
    }

    ///return templates that is available that targets resolve to
    pub fn find_all(&self, target: &Target) -> CoreResult<Vec<Template>> {
        todo!("post mvp")
    }
}
