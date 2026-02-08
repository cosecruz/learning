// ============================================================================
// region: Built-in Template Definitions
// ============================================================================

// region: template macros

// ====================template v1==========================
// macro_rules! template {
//     (
//         id: $id:literal,
//         name: $name:literal,
//         version: $version:literal,
//         matcher:{
//             language: $lang:ident,
//             framework: $fw:expr,
//             kind: $ptype:ident,
//             architecture: $arch:ident $(,)?
//         },
//         tree: $tree:expr $(,)?
//     ) => {
//         Template {
//             id: TemplateId($id),
//             metadata: TemplateMetadata::new($name).version($version),
//             matcher: TargetMatcher {
//                 language: Language::$lang,
//                 framework: $fw,
//                 kind: ProjectKind::$ptype,
//                 architecture: Architecture::$arch,
//             },
//             tree: $tree,
//         }
//     };
// }

use crate::domain::{
    Architecture, DirectorySpec, FileSpec, Language, ProjectKind, TargetMatcher, Template,
    TemplateContent, TemplateId, TemplateMetadata, TemplateNode, TemplateTree,
};

// =======================template tree========================
macro_rules! template_tree {
    (
        $(
            dir $dir:literal;
        )*
        $(
            file $path:literal => $tpl:literal;
        )*
    ) => {{
        let mut tree = TemplateTree::new();

        $(
            tree = tree.with_node(
                TemplateNode::Directory(DirectorySpec::new($dir))
            );
        )*

        $(
            tree = tree.with_node(
                TemplateNode::File(FileSpec::new(
                    $path,
                    TemplateContent::Template(include_str!($tpl)),
                ))
            );
        )*

        tree
    }};
}

// ===================template v2=======================================

macro_rules! template {
    (
        id: $id:literal,
        name: $name:literal,
        version: $version:literal,

        matcher {
            language: $lang:ident,
            framework: $fw:expr,
            kind: $ptype:ident,
            architecture: $arch:ident $(,)?
        }

        tree {
            $($tree:tt)*
        }
    ) => {
        Template {
            id: TemplateId($id),
            metadata: TemplateMetadata::new($name).version($version),
            matcher: TargetMatcher {
                language: Language::$lang,
                framework: $fw,
                kind: ProjectKind::$ptype,
                architecture: Architecture::$arch,
            },
            tree: template_tree! {
                $($tree)*
            },
        }
    };
}

// endregion: template macros

/// Rust CLI application default
pub fn rust_cli_default() -> Template {
    // Template {
    //     id: TemplateId("rust_cli_default_v001"),
    //     metadata: TemplateMetadata::new("Rust CLI (Default)").version("1.0.0"),
    //     matcher: TargetMatcher {
    //         language: Language::Rust,
    //         framework: None,
    //         kind: ProjectKind::Cli,
    //         architecture: Architecture::Layered,
    //     },
    //     tree: TemplateTree::new()
    //         .with_node(TemplateNode::Directory(DirectorySpec::new("src")))
    //         .with_node(TemplateNode::File(FileSpec::new(
    //             "src/main.rs",
    //             TemplateContent::Template(include_str!(
    //                 "templates/rust/cli/_default/main.rs.template"
    //             )),
    //         )))
    //         .with_node(TemplateNode::File(FileSpec::new(
    //             "Cargo.toml",
    //             TemplateContent::Template(include_str!(
    //                 "templates/rust/cli/_default/Cargo.toml.template"
    //             )),
    //         ))),
    // }

    // template! {
    //     id: "rust_cli_default_v001",
    //     name: "Rust CLI (Default)",
    //     version: "1.0.0",
    //     matcher: {
    //         language: Rust,
    //         framework: None,
    //         kind: Cli,
    //         architecture: Layered,
    //     },
    //     tree: TemplateTree::new()
    //         .with_node(...)
    // }

    template! {
        id: "rust_cli_default_v001",
        name: "Rust CLI (Default)",
        version: "1.0.0",

        matcher {
            language: Rust,
            framework: None,
            kind: Cli,
            architecture: Layered,
        }

        tree {
            dir "src";
            file "src/main.rs"
                => "templates/rust/cli/_default/main.rs.template";
            file "Cargo.toml"
                => "templates/rust/cli/_default/Cargo.toml.template";
        }
    }
}

// Rust CLI application with layered architecture
// fn rust_cli_layered() -> Template {
//     Template {
//         id: TemplateId("rust_cli_layered_v001".to_string()),
//         metadata: TemplateMetadata::new("Rust CLI (Layered)")
//             .version("1.0.0")
//             .description("A command-line application with layered architecture")
//             .tags(vec!["rust", "cli", "layered"]),
//         matcher: TargetMatcher {
//             language: Language::Rust,
//             framework: None,
//             kind: ProjectKind::Cli,
//             architecture: Architecture::Layered,
//         },
//         tree: TemplateTree::new()
//             // Source directories
//             .with_node(TemplateNode::Directory(DirectorySpec::new("src")))
//             .with_node(TemplateNode::Directory(DirectorySpec::new("src/domain")))
//             .with_node(TemplateNode::Directory(DirectorySpec::new(
//                 "src/application",
//             )))
//             .with_node(TemplateNode::Directory(DirectorySpec::new(
//                 "src/infrastructure",
//             )))
//             .with_node(TemplateNode::Directory(DirectorySpec::new(
//                 "src/presentation",
//             )))
//             // Main entry point
//             .with_node(TemplateNode::File(FileSpec::new(
//                 "src/main.rs",
//                 TemplateContent::Template(include_str!(
//                     "templates/rust_cli_layered/main.rs.template"
//                 )),
//             )))
//             // Domain layer
//             .with_node(TemplateNode::File(FileSpec::new(
//                 "src/domain/mod.rs",
//                 TemplateContent::Template(include_str!(
//                     "templates/rust_cli_layered/domain_mod.rs.template"
//                 )),
//             )))
//             // Application layer
//             .with_node(TemplateNode::File(FileSpec::new(
//                 "src/application/mod.rs",
//                 TemplateContent::Template(include_str!(
//                     "templates/rust_cli_layered/application_mod.rs.template"
//                 )),
//             )))
//             // Infrastructure layer
//             .with_node(TemplateNode::File(FileSpec::new(
//                 "src/infrastructure/mod.rs",
//                 TemplateContent::Template(include_str!(
//                     "templates/rust_cli_layered/infrastructure_mod.rs.template"
//                 )),
//             )))
//             // Presentation layer
//             .with_node(TemplateNode::File(FileSpec::new(
//                 "src/presentation/mod.rs",
//                 TemplateContent::Template(include_str!(
//                     "templates/rust_cli_layered/presentation_mod.rs.template"
//                 )),
//             )))
//             // Configuration files
//             .with_node(TemplateNode::File(FileSpec::new(
//                 "Cargo.toml",
//                 TemplateContent::Template(include_str!(
//                     "templates/rust_cli_layered/Cargo.toml.template"
//                 )),
//             )))
//             .with_node(TemplateNode::File(FileSpec::new(
//                 "README.md",
//                 TemplateContent::Template(include_str!(
//                     "templates/rust_cli_layered/README.md.template"
//                 )),
//             )))
//             .with_node(TemplateNode::File(FileSpec::new(
//                 ".gitignore",
//                 TemplateContent::Static(include_str!("templates/common/rust.gitignore")),
//             ))),
//     }
// }

// Rust library with layered architecture
// fn rust_lib_layered() -> Template {
//     Template {
//         id: TemplateId("rust_lib_layered_v001".to_string()),
//         metadata: TemplateMetadata::new("Rust Library (Layered)")
//             .version("1.0.0")
//             .description("A library crate with layered architecture")
//             .tags(vec!["rust", "library", "layered"]),
//         matcher: TargetMatcher {
//             language: Language::Rust,
//             framework: None,
//             kind: ProjectKind::Library,
//             architecture: Architecture::Layered,
//         },
//         tree: TemplateTree::new()
//             .with_node(TemplateNode::Directory(DirectorySpec::new("src")))
//             .with_node(TemplateNode::File(FileSpec::new(
//                 "src/lib.rs",
//                 TemplateContent::Template(include_str!("templates/rust_lib_layered/lib.rs.template")),
//             )))
//             .with_node(TemplateNode::File(FileSpec::new(
//                 "Cargo.toml",
//                 TemplateContent::Template(include_str!("templates/rust_lib_layered/Cargo.toml.template")),
//             )))
//             .with_node(TemplateNode::File(FileSpec::new(
//                 "README.md",
//                 TemplateContent::Template(include_str!("templates/rust_lib_layered/README.md.template")),
//             ))),
//     }
// }

// Rust web API with clean architecture
// fn rust_web_api_clean() -> Template {
//     Template {
//         id: TemplateId("rust_web_api_clean_v001".to_string()),
//         metadata: TemplateMetadata::new("Rust Web API (Clean)")
//             .version("1.0.0")
//             .description("A web API with clean architecture using Axum")
//             .tags(vec!["rust", "web", "api", "clean", "axum"]),
//         matcher: TargetMatcher {
//             language: Language::Rust,
//             framework: Some("axum".to_string()),
//             kind: ProjectKind::WebApi,
//             architecture: Architecture::Clean,
//         },
//         tree: TemplateTree::new()
//             // Core directories
//             .with_node(TemplateNode::Directory(DirectorySpec::new("src")))
//             .with_node(TemplateNode::Directory(DirectorySpec::new("src/domain")))
//             .with_node(TemplateNode::Directory(DirectorySpec::new("src/application")))
//             .with_node(TemplateNode::Directory(DirectorySpec::new("src/infrastructure")))
//             .with_node(TemplateNode::Directory(DirectorySpec::new("src/presentation")))

//             // Main files
//             .with_node(TemplateNode::File(FileSpec::new(
//                 "src/main.rs",
//                 TemplateContent::Template(include_str!("templates/rust_web_api_clean/main.rs.template")),
//             )))

//             // Config
//             .with_node(TemplateNode::File(FileSpec::new(
//                 "Cargo.toml",
//                 TemplateContent::Template(include_str!("templates/rust_web_api_clean/Cargo.toml.template")),
//             )))
//             .with_node(TemplateNode::File(FileSpec::new(
//                 ".env.example",
//                 TemplateContent::Static("PORT=3000\nDATABASE_URL=postgres://localhost/mydb"),
//             ))),
//     }
// }
// endregion: Built0in Template Definitions
