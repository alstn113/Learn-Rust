pub struct FileType {
    name: String,
    hl_opts: HighLightOptions,
}

#[derive(Default, Copy, Clone)]
pub struct HighLightOptions {
    numbers: bool,
    strings: bool,
}

impl Default for FileType {
    fn default() -> Self {
        Self {
            name: String::from("None"),
            hl_opts: HighLightOptions::default(),
        }
    }
}

impl FileType {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn highlight_options(&self) -> HighLightOptions {
        self.hl_opts
    }

    pub fn from(filename: &str) -> Self {
        if filename.ends_with(".rs") {
            return Self {
                name: String::from("Rust"),
                hl_opts: HighLightOptions {
                    numbers: true,
                    strings: true,
                },
            };
        }
        Self::default()
    }
}

impl HighLightOptions {
    pub fn numbers(&self) -> bool {
        self.numbers
    }

    pub fn strings(&self) -> bool {
        self.strings
    }
}
