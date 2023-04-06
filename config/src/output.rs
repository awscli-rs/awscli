use clap::ValueEnum;

#[derive(Clone, Copy, Debug, Default, PartialEq, ValueEnum)]
pub enum Output {
    /// JSON document (machine readable)
    Json,
    /// Normal Text (human readable)
    #[default]
    Text,
    /// Text table (machine readable)
    Table,
    /// Single YAML document (machine readable)
    Yaml,
    /// Multiple YAML documents (machine readable)
    YamlStream,
}
