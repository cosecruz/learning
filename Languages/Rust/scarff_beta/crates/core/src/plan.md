Target{
Language
ProjectType
Framework
Architecture
//TODO: Post-MVP others like flavour, technologies, features and addons
}

// Target must be built to a fully constructed object no Options there

Target can has presets
Presets help provide for faster scaffolding
so instead creating target from user entries
then engine orchestrating to resolving to template and rendering project structure then writing to filesystem
presets shortcuts the process; from target we can match to presets if they exist; if the preset exist it means the template and project structure has already been created and is saved then it skips to writing to filesystem on disk

i can further make preset a cache that will cache user scaffold results for them;

TargetBuilder -> builds Target

// So to build a Target builder must do the following
// validate entries for: - type - support via if the option is provided - support via compatibility

// infer non compulsory target entries
infer based on entries amd or defaults

//
