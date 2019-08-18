use glium::draw_parameters::{Blend, DrawParameters};
use glium::index::{NoIndices, PrimitiveType};
use glium::{Display, Frame, Program, Surface, Texture2d, VertexBuffer};
use nalgebra::{Matrix4, Vector4};

use crate::color::Color;
use crate::enums::{Orientation, Shape};
use crate::game::Game;
use crate::platform::GlslProgram;

pub struct Renderer<'a, 'b> {
    pub window: (f32, f32),
    target: &'a mut Frame,
    display: &'b Display,
    cell_program: &'b GlslProgram,
    segment_program: &'b GlslProgram,
    segment_texture: &'b Texture2d,
}

impl<'a, 'b> Renderer<'a, 'b> {
    pub fn new(game: &'b Game, target: &'a mut Frame) -> Self {
        Renderer {
            window: (
                game.resources.window_dimensions.0 as f32,
                game.resources.window_dimensions.1 as f32,
            ),
            target,
            display: &game.display,
            cell_program: game.resources.get_shader("cell").unwrap(),
            segment_program: game.resources.get_shader("segment").unwrap(),
            segment_texture: game.resources.get_texture("segment").unwrap(),
        }
    }

    pub fn render_cell(&mut self, location: (i32, i32), scale: i32) {
        #[derive(Copy, Clone)]
        struct Vertex {
            point: [f32; 2],
        }
        implement_vertex!(Vertex, point);

        let indices = NoIndices(PrimitiveType::TrianglesList);
        let mut vertices = Vec::<Vertex>::new();
        vertices.push(Vertex { point: [0.0, 0.0] });
        vertices.push(Vertex { point: [1.0, 0.0] });
        vertices.push(Vertex { point: [0.0, 1.0] });
        vertices.push(Vertex { point: [1.0, 1.0] });
        vertices.push(Vertex { point: [0.0, 1.0] });
        vertices.push(Vertex { point: [1.0, 0.0] });
        let vertex_buffer = VertexBuffer::new(self.display, &vertices).unwrap();

        let projection = glm::ortho::<f32>(
            0.0,
            self.window.0 as f32,
            self.window.1 as f32,
            0.0,
            -1.0,
            1.0,
        );
        let mut matrix = Matrix4::<f32>::identity();
        matrix = matrix.append_nonuniform_scaling(&[scale as f32, scale as f32, 1.0].into());
        matrix = matrix.append_translation(&[location.0 as f32, location.1 as f32, 0.0].into());

        let color = Vector4::from([0.6, 0.6, 0.8, 1.0f32]);

        let uniforms = uniform! {
            target: *matrix.as_ref(),
            projection: *projection.as_ref(),
            color: *color.as_ref(),
        };

        self.target
            .draw(
                &vertex_buffer,
                &indices,
                &self.cell_program,
                &uniforms,
                &DrawParameters {
                    blend: Blend::alpha_blending(),
                    ..Default::default()
                },
            )
            .unwrap();
    }

    pub fn render_segment(
        &mut self,
        location: (i32, i32),
        scale: i32,
        color: Color,
        orientation: Orientation,
        shape: Shape,
    ) {
        #[derive(Copy, Clone)]
        struct Vertex {
            pos: [f32; 2],
            tex: [f32; 2],
        }
        implement_vertex!(Vertex, pos, tex);

        let indices = NoIndices(PrimitiveType::TrianglesList);
        let mut vertices = Vec::new();
        match shape {
            Shape::BottomLeft => {
                vertices.push(Vertex {
                    pos: [1.0, 1.0],
                    tex: [1.0, 1.0],
                });
                vertices.push(Vertex {
                    pos: [1.0, 0.0],
                    tex: [1.0, 0.0],
                });
                vertices.push(Vertex {
                    pos: [0.0, 0.0],
                    tex: [0.0, 0.0],
                });
            }
            Shape::TopLeft => {
                vertices.push(Vertex {
                    pos: [0.0, 1.0],
                    tex: [0.0, 1.0],
                });
                vertices.push(Vertex {
                    pos: [1.0, 1.0],
                    tex: [1.0, 1.0],
                });
                vertices.push(Vertex {
                    pos: [1.0, 0.0],
                    tex: [1.0, 0.0],
                });
            }
            Shape::TopRight => {
                vertices.push(Vertex {
                    pos: [1.0, 1.0],
                    tex: [1.0, 1.0],
                });
                vertices.push(Vertex {
                    pos: [0.0, 1.0],
                    tex: [0.0, 1.0],
                });
                vertices.push(Vertex {
                    pos: [0.0, 0.0],
                    tex: [0.0, 0.0],
                });
            }
            Shape::BottomRight => {
                vertices.push(Vertex {
                    pos: [0.0, 1.0],
                    tex: [0.0, 1.0],
                });
                vertices.push(Vertex {
                    pos: [1.0, 0.0],
                    tex: [1.0, 0.0],
                });
                vertices.push(Vertex {
                    pos: [0.0, 0.0],
                    tex: [0.0, 0.0],
                });
            }
            _ => {
                vertices.push(Vertex {
                    pos: [0.0, 1.0],
                    tex: [0.0, 1.0],
                });
                vertices.push(Vertex {
                    pos: [1.0, 0.0],
                    tex: [1.0, 0.0],
                });
                vertices.push(Vertex {
                    pos: [0.0, 0.0],
                    tex: [0.0, 0.0],
                });
                vertices.push(Vertex {
                    pos: [0.0, 1.0],
                    tex: [0.0, 1.0],
                });
                vertices.push(Vertex {
                    pos: [1.0, 1.0],
                    tex: [1.0, 1.0],
                });
                vertices.push(Vertex {
                    pos: [1.0, 0.0],
                    tex: [1.0, 0.0],
                });
            }
        }
        let vertex_buffer = VertexBuffer::new(self.display, &vertices).unwrap();
        let tint = Vector4::from([color.0, color.1, color.2, 1.0f32]);

        let projection = glm::ortho::<f32>(
            0.0,
            self.window.0 as f32,
            self.window.1 as f32,
            0.0,
            -1.0,
            1.0,
        );
        let mut matrix = Matrix4::<f32>::identity();
        matrix = matrix.append_nonuniform_scaling(&[scale as f32, scale as f32, 1.0].into());
        matrix = matrix.append_translation(&[location.0 as f32, location.1 as f32, 0.0].into());

        let rotate_texture = match orientation {
            Orientation::Both => false,
            Orientation::None => false,
            Orientation::Vertical => true,
            Orientation::Horizontal => false,
        };

        let uniforms = uniform! {
            target: *matrix.as_ref(),
            rotate_texture: rotate_texture,
            projection: *projection.as_ref(),
            tint: *tint.as_ref(),
            tex: self.segment_texture,
        };

        self.target
            .draw(
                &vertex_buffer,
                &indices,
                &self.segment_program,
                &uniforms,
                &DrawParameters {
                    blend: Blend::alpha_blending(),
                    ..Default::default()
                },
            )
            .unwrap();
    }
}
