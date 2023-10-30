#[derive(Clone)]
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
///
///
///
#[derive(Clone)]
pub struct ProjectConfig {
    name: String,
    driver: String,
    citations: bool,
    graphics: bool,
    doctype: DocumentType,
}

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
            citations: false,
            graphics: false,
            doctype: DocumentType::Letter
        }
    }
}