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

impl Output {
    pub fn output(&self, object: Box<dyn show::Show>) -> String {
        match self {
            Self::Json => object.json(),
            Self::Text => object.text(),
            Self::Table => object.table(),
            Self::Yaml => object.yaml(),
            Self::YamlStream => object.yaml_stream(),
        }
    }
}
