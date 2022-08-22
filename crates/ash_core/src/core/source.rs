use std::path::PathBuf;
use std::{fs, io};

use once_cell::sync::OnceCell;
use regex::Regex;

static COMMENT_REGEX: OnceCell<Regex> = OnceCell::new();

pub struct Source {
    location: Option<String>,
    inner: String,
}

impl Source {
    pub fn from_file(path: PathBuf) -> Result<Self, io::Error> {
        let source = fs::read_to_string(&path)?;
        let location = path.as_os_str().to_str().map(ToOwned::to_owned);
        let source = Self {
            location,
            inner: source,
        };
        Ok(source.prepare())
    }

    pub fn from_string<S: Into<String>>(src: S) -> Self {
        let src = Self {
            location: None,
            inner: src.into(),
        };

        src.prepare()
    }

    pub fn inner(&self) -> &str {
        return &self.inner;
    }

    pub fn location(&self) -> String {
        return self
            .location
            .as_ref()
            .unwrap_or(&String::from("<unknown>"))
            .to_owned();
    }

    fn prepare(mut self) -> Self {
        let comments = COMMENT_REGEX.get_or_init(|| Regex::new(r"(//.*)|(/\*(\n|.)*\*/)").unwrap());

        // Hack for making sure dedent is detected
        self.inner += "\n";
        self.inner = comments.replace_all(&self.inner, "").to_string();

        self
    }
}
