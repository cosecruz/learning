//! - TemplateRenderer: renders Template to a ProjectStructure.
//!   The project structure it outputs is what will be written to the file system.
//!

use std::path::PathBuf;

use tracing::{debug, info, instrument};

use crate::{
    domain::{ProjectStructure, RenderContext, Template, TemplateContent, TemplateNode},
    errors::CoreResult,
};

pub(crate) struct TemplateRenderer;

impl TemplateRenderer {
    pub(crate) fn new() -> Self {
        Self
    }

    #[instrument(skip(self, template, ctx), fields(template_id = %template.id))]
    pub(crate) fn render(
        &self,
        template: &Template,
        ctx: &RenderContext,
        output_root: PathBuf,
    ) -> CoreResult<ProjectStructure> {
        info!("[scaffold] rendering template");
        let mut structure = ProjectStructure::new(output_root);

        for node in &template.tree.nodes {
            match node {
                TemplateNode::File(spec) => {
                    let content = match &spec.content {
                        TemplateContent::Static(s) => s.to_string(),
                        TemplateContent::Template(t) => ctx.render(t),
                        TemplateContent::Rendered { template_id } => {
                            // complex rendering PST MVP
                            debug!(template_id = ?template_id);
                            todo!("Complex template rendering")
                        }
                    };
                    structure.add_file(spec.path.as_path(), content, spec.permissions);
                }
                TemplateNode::Directory(spec) => {
                    structure.add_directory(spec.path.as_path(), spec.permissions)
                }
            }
        }
        structure.validate()?;
        Ok(structure)
    }
}
