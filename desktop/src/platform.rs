use glium::{Display, Program, ProgramCreationError};
use wedge_core::Platform;

pub struct DesktopPlatform {}

impl Platform for DesktopPlatform {
    fn create() -> Self {
        DesktopPlatform {}
    }

    type GlslCompileContext = Display;
    type GlslProgram = Program;
    type GlslError = ProgramCreationError;
    fn compile_glsl_program<'a>(
        &self,
        context: &'a Self::GlslCompileContext,
        vert: impl AsRef<str>,
        frag: impl AsRef<str>,
    ) -> Result<Self::GlslProgram, Self::GlslError> {
        let vert = vert.as_ref();
        let frag = frag.as_ref();

        Program::from_source(context, vert, frag, None)
    }
}
