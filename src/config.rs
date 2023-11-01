use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fs::File;
use std::io::Write;
use toml;

#[derive(Clone, Serialize, Deserialize)]
pub enum DocumentType {
    Article,
    Book,
    Letter,
}

/// The ProjectConfig struct stores five pieces of
/// information about the project: what name, driver,
/// whether citations or graphics are used, & what
/// type of document it is.
///
/// ## Structure
/// pub name: String,
/// pub driver: String,
/// pub citations: bool,
/// pub graphics: bool,
/// pub doctype: DocumentType
#[derive(Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub driver: String,
    pub citations: bool,
    pub graphics: bool,
    pub doctype: DocumentType,
} // TODO: I don't think these need to be `pub` now...

impl ProjectConfig {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_driver(&self) -> String {
        self.driver.clone()
    }

    pub fn get_citations(&self) -> bool {
        self.citations
    }

    pub fn get_graphics(&self) -> bool {
        self.graphics
    }

    pub fn get_doctype(&self) -> DocumentType {
        self.doctype.clone()
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }

    pub fn set_driver(&mut self, driver: &str) {
        self.driver = driver.to_owned();
    }

    pub fn set_citations(&mut self, citations: bool) {
        self.citations = citations;
    }

    pub fn set_graphics(&mut self, graphics: bool) {
        self.graphics = graphics;
    }

    pub fn set_doctype(&mut self, doctype: DocumentType) {
        self.doctype = doctype;
    }

    pub fn new() -> ProjectConfig {
        ProjectConfig {
            name: "document1".to_owned(),
            driver: "pdflatex".to_owned(),
            citations: true,
            graphics: true,
            doctype: DocumentType::Letter,
        }
    }
}

pub fn write_project_config(config: &ProjectConfig) -> Result<(), toml::ser::Error> {
    let root_dir = config.get_name() + "/";
    let toml_str = toml::to_string(&config)?;
    let mut file = File::create(root_dir + "config.toml").expect("Location must be writable.");
    file.write_all(toml_str.as_bytes())
        .expect("Location must be writable.");
    Ok(())
}
