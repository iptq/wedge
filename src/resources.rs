use std::collections::HashMap;

use glium::{Display, Program, ProgramCreationError, Texture2d};

#[derive(Default)]
pub struct Resources {
    textures: HashMap<String, Texture2d>,
    shaders: HashMap<String, Program>,
}

impl Resources {
    pub fn load_shader(
        &mut self,
        display: &Display,
        name: impl AsRef<str>,
        vertex: &str,
        fragment: &str,
    ) -> Result<(), ProgramCreationError> {
        let name = name.as_ref().to_owned();
        let program = Program::from_source(display, vertex, fragment, None)?;
        self.shaders.insert(name, program);
        Ok(())
    }

    pub fn get_shader(&self, name: impl AsRef<str>) -> Option<&Program> {
        self.shaders.get(name.as_ref())
    }
}
