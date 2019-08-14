pub trait Platform {
    fn create() -> Self;

    type GlslCompileContext;
    type GlslProgram;
    type GlslError;
    fn compile_glsl_program<'a>(
        &self,
        context: &'a Self::GlslCompileContext,
        vert: impl AsRef<str>,
        frag: impl AsRef<str>,
    ) -> Result<Self::GlslProgram, Self::GlslError>;
}
