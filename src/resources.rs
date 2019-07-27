use std::collections::HashMap;

use glium::texture::RawImage2d;
use glium::{Display, Program, ProgramCreationError, Texture2d};
use image::{DynamicImage, GenericImageView, ImageBuffer, ImageError, Rgba};

#[derive(Default)]
pub struct Resources {
    pub window_dimensions: (u32, u32),
    textures: HashMap<String, Texture2d>,
    shaders: HashMap<String, Program>,
}

impl Resources {
    pub fn load_image_from_memory(
        &mut self,
        display: &Display,
        name: impl AsRef<str>,
        buffer: &[u8],
        alpha: bool,
    ) -> Result<(), ImageError> {
        let image = image::load_from_memory(buffer)?;
        self.load_image(display, name, image, alpha);
        Ok(())
    }

    pub fn load_image(
        &mut self,
        display: &Display,
        name: impl AsRef<str>,
        image: DynamicImage,
        alpha: bool,
    ) {
        let name = name.as_ref().to_owned();
        let dimensions = image.dimensions();
        let image = if alpha {
            RawImage2d::from_raw_rgba_reversed(&image.raw_pixels(), dimensions)
        } else {
            RawImage2d::from_raw_rgb_reversed(&image.raw_pixels(), dimensions)
        };
        // TODO: don't unwrap
        let texture = Texture2d::new(display, image).unwrap();
        self.textures.insert(name, texture);
    }

    pub fn get_texture(&self, name: impl AsRef<str>) -> Option<&Texture2d> {
        self.textures.get(name.as_ref())
    }

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
