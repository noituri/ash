use once_cell::sync::OnceCell;
use regex::Regex;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

static COMMENT_REGEX: OnceCell<Regex> = OnceCell::new();

pub struct Source {
    inner: String,
}

impl ToString for Source {
    fn to_string(&self) -> String {
        self.inner.clone()
    }
}

impl Source {
    pub fn from_file(path: PathBuf) -> Result<Self, Box<dyn Error>> {
        let source = fs::read_to_string(path)?;
        Ok(Self::from_string(source))
    }

    pub fn from_string<S: Into<String>>(src: S) -> Self {
        let src = Self { inner: src.into() };

        src.prepare()
    }

    fn prepare(mut self) -> Self {
        let comments = COMMENT_REGEX.get_or_init(|| Regex::new(r"(//.*)|(/\*(\n|.)*\*/)").unwrap());

        // Hack for making sure dedent is detected
        self.inner += "\n";
        self.inner = comments.replace_all(&self.inner, "").to_string();

        self
    }
}
