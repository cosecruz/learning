// //! Integration tests for the complete Scarff core system.
// //!
// //! These tests verify the entire flow from Target → Template → ProjectStructure → Filesystem.

// use scarff_core::{
//     Target,
//     domain::{Architecture, Language, ProjectKind},
//     scaffold::{Engine, filesystem::MockFilesystem},
// };
// use std::path::Path;

// // ============================================================================
// // End-to-End Scaffolding Tests
// // ============================================================================

// #[test]
// fn test_scaffold_rust_cli_complete_flow() {
//     // Setup
//     let mock_fs = Box::new(MockFilesystem::new());
//     let fs_clone = mock_fs.clone();
//     let engine = Engine::with_filesystem(mock_fs);

//     // Create target
//     let target = Target::builder()
//         .language(Language::Rust)
//         .kind(ProjectKind::Cli)
//         .unwrap()
//         .architecture(Architecture::Layered)
//         .unwrap()
//         .build()
//         .unwrap();

//     // Execute scaffolding
//     let result = engine.scaffold(target, "my-cli-app", ".");

//     // Verify success
//     assert!(
//         result.is_ok(),
//         "Scaffolding should succeed: {:?}",
//         result.err()
//     );

//     // Verify project directory created
//     assert!(fs_clone.exists(Path::new("./my-cli-app")));

//     // Verify source directory created
//     assert!(fs_clone.exists(Path::new("./my-cli-app/src")));

//     // Verify files created (at minimum)
//     assert!(fs_clone.exists(Path::new("./my-cli-app/src/main.rs")));
//     assert!(fs_clone.exists(Path::new("./my-cli-app/Cargo.toml")));

//     // Verify file count
//     assert!(fs_clone.file_count() >= 2);
//     assert!(fs_clone.directory_count() >= 1);
// }

// #[test]
// fn test_scaffold_rust_backend_axum() {
//     use scarff_core::{Framework, RustFramework};

//     let mock_fs = Box::new(MockFilesystem::new());
//     let fs_clone = mock_fs.clone();
//     let engine = Engine::with_filesystem(mock_fs);

//     let target = Target::builder()
//         .language(Language::Rust)
//         .kind(ProjectKind::WebBackend)
//         .unwrap()
//         .framework(Framework::Rust(RustFramework::Axum))
//         .unwrap()
//         .architecture(Architecture::Layered)
//         .unwrap()
//         .build()
//         .unwrap();

//     let result = engine.scaffold(target, "my-api", ".");

//     assert!(result.is_ok());
//     assert!(fs_clone.exists(Path::new("./my-api")));
//     assert!(fs_clone.exists(Path::new("./my-api/src")));
// }

// #[test]
// fn test_scaffold_python_backend_fastapi() {
//     use scarff_core::{Framework, PythonFramework};

//     let mock_fs = Box::new(MockFilesystem::new());
//     let fs_clone = mock_fs.clone();
//     let engine = Engine::with_filesystem(mock_fs);

//     let target = Target::builder()
//         .language(Language::Python)
//         .kind(ProjectKind::WebBackend)
//         .unwrap()
//         .framework(Framework::Python(PythonFramework::FastApi))
//         .unwrap()
//         .architecture(Architecture::Layered)
//         .unwrap()
//         .build()
//         .unwrap();

//     let result = engine.scaffold(target, "my-python-api", ".");

//     assert!(result.is_ok());
//     assert!(fs_clone.exists(Path::new("./my-python-api")));
// }

// #[test]
// fn test_scaffold_typescript_frontend_react() {
//     use scarff_core::{Framework, TypeScriptFramework};

//     let mock_fs = Box::new(MockFilesystem::new());
//     let fs_clone = mock_fs.clone();
//     let engine = Engine::with_filesystem(mock_fs);

//     let target = Target::builder()
//         .language(Language::TypeScript)
//         .kind(ProjectKind::WebFrontend)
//         .unwrap()
//         .framework(Framework::TypeScript(TypeScriptFramework::React))
//         .unwrap()
//         .architecture(Architecture::Layered)
//         .unwrap()
//         .build()
//         .unwrap();

//     let result = engine.scaffold(target, "my-react-app", ".");

//     assert!(result.is_ok());
//     assert!(fs_clone.exists(Path::new("./my-react-app")));
// }

// // ============================================================================
// // Variable Substitution Tests
// // ============================================================================

// #[test]
// fn test_variable_substitution_in_rendered_files() {
//     let mock_fs = Box::new(MockFilesystem::new());
//     let fs_clone = mock_fs.clone();
//     let engine = Engine::with_filesystem(mock_fs);

//     let target = Target::builder()
//         .language(Language::Rust)
//         .kind(ProjectKind::Cli)
//         .unwrap()
//         .build()
//         .unwrap();

//     engine
//         .scaffold(target, "awesome-cli", ".")
//         .expect("Should succeed");

//     // Read a file and check variable substitution occurred
//     // Note: This assumes the template uses {{PROJECT_NAME}}
//     let files = fs_clone.list_files();
//     assert!(!files.is_empty());

//     // Check that at least one file contains the project name
//     let has_project_name = files.iter().any(|path| {
//         if let Ok(content) = fs_clone.read_file(path) {
//             content.contains("awesome-cli") || content.contains("awesome_cli")
//         } else {
//             false
//         }
//     });

//     assert!(
//         has_project_name,
//         "At least one file should contain the project name"
//     );
// }

// // ============================================================================
// // Template Resolution Tests
// // ============================================================================

// #[test]
// fn test_engine_lists_all_builtin_templates() {
//     let engine = Engine::new();
//     let templates = engine.list_templates().unwrap();

//     // Should have at least 5 built-in templates
//     assert!(templates.len() >= 5);

//     // Check for expected templates
//     let template_names: Vec<_> = templates.iter().map(|t| t.name.as_str()).collect();

//     assert!(template_names.contains(&"Rust CLI (Default)"));
//     assert!(template_names.contains(&"Rust CLI (Layered)"));
//     assert!(template_names.contains(&"Rust Web Backend (Axum)"));
// }

// #[test]
// fn test_engine_finds_matching_templates() {
//     let engine = Engine::new();

//     let target = Target::builder()
//         .language(Language::Rust)
//         .kind(ProjectKind::Cli)
//         .unwrap()
//         .build()
//         .unwrap();

//     let matches = engine.find_templates(&target).unwrap();

//     // Should find at least the Rust CLI templates
//     assert!(matches.len() >= 2);

//     // All should be Rust
//     assert!(matches.iter().all(|t| t.language == "rust"));

//     // All should be CLI
//     assert!(matches.iter().all(|t| t.kind == "cli"));
// }

// // ============================================================================
// // Error Handling Tests
// // ============================================================================

// #[test]
// fn test_scaffold_fails_if_project_exists() {
//     let mock_fs = Box::new(MockFilesystem::new());
//     let engine = Engine::with_filesystem(mock_fs.clone());

//     // Pre-create the project directory
//     mock_fs
//         .create_dir_all(Path::new("./existing-project"))
//         .unwrap();

//     let target = Target::builder()
//         .language(Language::Rust)
//         .kind(ProjectKind::Cli)
//         .unwrap()
//         .build()
//         .unwrap();

//     let result = engine.scaffold(target, "existing-project", ".");

//     assert!(result.is_err());
// }

// #[test]
// fn test_scaffold_with_invalid_target_fails() {
//     use scarff_core::{Framework, PythonFramework};

//     // Create a target with mismatched framework
//     let result = Target::builder()
//         .language(Language::Rust)
//         .framework(Framework::Python(PythonFramework::Django)) // Wrong framework!
//         .unwrap_err(); // Should error in builder

//     // Builder should catch this
//     assert!(format!("{:?}", result).contains("FrameworkLanguageMismatch"));
// }

// // ============================================================================
// // Validation Tests
// // ============================================================================

// #[test]
// fn test_target_validation_rejects_incompatible_combinations() {
//     use scarff_core::{Framework, RustFramework};

//     // Try to create a CLI with a web framework (should fail during inference)
//     let result = Target::builder()
//         .language(Language::Rust)
//         .kind(ProjectKind::Cli)
//         .unwrap()
//         .framework(Framework::Rust(RustFramework::Axum)) // Web framework for CLI
//         .unwrap_err();

//     assert!(format!("{:?}", result).contains("mismatch"));
// }

// // ============================================================================
// // Filesystem Tests
// // ============================================================================

// #[test]
// fn test_mock_filesystem_isolation() {
//     let fs1 = MockFilesystem::new();
//     let fs2 = MockFilesystem::new();

//     fs1.create_dir_all(Path::new("/test")).unwrap();
//     fs1.write_file(Path::new("/test/file.txt"), "content")
//         .unwrap();

//     // fs2 should not see fs1's files
//     assert!(!fs2.exists(Path::new("/test/file.txt")));
// }

// #[test]
// fn test_rollback_on_partial_failure() {
//     use scarff_core::domain::{Permissions, ProjectStructure};
//     use scarff_core::scaffold::{filesystem::MockFilesystem, writer::FileWriter, writer::Writer};

//     let fs = Box::new(MockFilesystem::new());
//     let fs_clone = fs.clone();
//     let writer = FileWriter::new(fs);

//     // Create a structure that will fail validation (duplicate paths)
//     let mut structure = ProjectStructure::new("/test-project");
//     structure.add_file("main.rs", "content".to_string(), Permissions::read_write());
//     structure.add_file(
//         "main.rs",
//         "duplicate".to_string(),
//         Permissions::read_write(),
//     );

//     let result = writer.write(&structure);

//     // Should fail
//     assert!(result.is_err());

//     // Filesystem should be clean (no partial writes)
//     assert!(!fs_clone.exists(Path::new("/test-project")));
// }

// // ============================================================================
// // Performance Tests
// // ============================================================================

// #[test]
// fn test_scaffold_multiple_projects_sequentially() {
//     let mock_fs = Box::new(MockFilesystem::new());
//     let fs_clone = mock_fs.clone();
//     let engine = Engine::with_filesystem(mock_fs);

//     let target = Target::builder()
//         .language(Language::Rust)
//         .kind(ProjectKind::Cli)
//         .unwrap()
//         .build()
//         .unwrap();

//     // Create multiple projects
//     for i in 0..5 {
//         let project_name = format!("project-{}", i);
//         let result = engine.scaffold(target.clone(), &project_name, ".");
//         assert!(result.is_ok());
//     }

//     // All should exist
//     for i in 0..5 {
//         let path = format!("./project-{}", i);
//         assert!(fs_clone.exists(Path::new(&path)));
//     }
// }

// // ============================================================================
// // Tracing Tests
// // ============================================================================

// #[test]
// #[cfg(feature = "logging")]
// fn test_tracing_spans_are_created() {
//     use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

//     // Setup tracing subscriber for test
//     let subscriber =
//         tracing_subscriber::registry().with(tracing_subscriber::fmt::layer().with_test_writer());

//     tracing::subscriber::set_global_default(subscriber).ok();

//     let mock_fs = Box::new(MockFilesystem::new());
//     let engine = Engine::with_filesystem(mock_fs);

//     let target = Target::builder()
//         .language(Language::Rust)
//         .kind(ProjectKind::Cli)
//         .unwrap()
//         .build()
//         .unwrap();

//     // This should create tracing spans
//     let _ = engine.scaffold(target, "test-project", ".");

//     // If we got here without panicking, tracing is working
// }

// // ============================================================================
// // Integration with Real Types Tests
// // ============================================================================

// #[test]
// fn test_all_preset_targets_work() {
//     let mock_fs = Box::new(MockFilesystem::new());
//     let engine = Engine::with_filesystem(mock_fs.clone());

//     // Test all preset methods
//     let presets = vec![
//         Target::rust_cli().unwrap(),
//         Target::rust_backend_axum().unwrap(),
//         Target::python_backend_fastapi().unwrap(),
//         Target::typescript_frontend_react().unwrap(),
//     ];

//     for (idx, target) in presets.into_iter().enumerate() {
//         let project_name = format!("preset-project-{}", idx);
//         let result = engine.scaffold(target, &project_name, ".");

//         assert!(result.is_ok(), "Preset target {} should work", project_name);
//     }
// }
