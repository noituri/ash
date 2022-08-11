use super::Env;

pub struct Context {
    env: Env,
}

impl Context {
    pub fn new() -> Self {
        let env = Env::default();
        Self { env }
    }

    pub fn get_env(&self) -> &Env {
        &self.env
    }

    pub fn set_env(&mut self, new_env: Env) {
        self.env = new_env;
    }
}
