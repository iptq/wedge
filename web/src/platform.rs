// https://rustwasm.github.io/wasm-bindgen/examples/webgl.html

use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};
use wedge_core::Platform;

pub struct WebPlatform {}

impl Platform for WebPlatform {
    fn create() -> Self {
        WebPlatform {}
    }

    // TODO: don't use strings lol
    type GlslCompileContext = WebGlRenderingContext;
    type GlslProgram = WebGlProgram;
    type GlslError = String;
    fn compile_glsl_program<'a>(
        &self,
        context: &'a Self::GlslCompileContext,
        vert: impl AsRef<str>,
        frag: impl AsRef<str>,
    ) -> Result<Self::GlslProgram, Self::GlslError> {
        let program = context
            .create_program()
            .ok_or_else(|| String::from("Unable to create shader object"))?;

        let vert = vert.as_ref();
        let frag = frag.as_ref();

        let vert_shader = compile_shader(context, WebGlRenderingContext::VERTEX_SHADER, vert)?;
        let frag_shader = compile_shader(context, WebGlRenderingContext::FRAGMENT_SHADER, frag)?;

        context.attach_shader(&program, &vert_shader);
        context.attach_shader(&program, &frag_shader);
        context.link_program(&program);
        if context
            .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(program)
        } else {
            Err(context
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating program object")))
        }
    }
}

fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}
