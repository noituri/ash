#[derive(Debug, Clone)]
pub(crate) struct Annotation {
    name: String,
}

impl Annotation {
    const BUILT_IN_NAME: &'static str = "builtin";

    pub fn new<S: ToString>(name: S) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub fn is_builtin(&self) -> bool {
        self.name == Self::BUILT_IN_NAME
    }
}
