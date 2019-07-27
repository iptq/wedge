use glium::draw_parameters::{Blend, DrawParameters};
use glium::index::{NoIndices, PrimitiveType};
use glium::{Display, Frame, Program, Surface, VertexBuffer};
use nalgebra::Matrix4;

use crate::enums::Shape;
use crate::game::Game;
use crate::{GAME_HEIGHT, GAME_WIDTH};

pub struct Renderer<'a, 'b> {
    target: &'a mut Frame,
    display: &'b Display,
    program: &'b Program,
}

impl<'a, 'b> Renderer<'a, 'b> {
    pub fn new(game: &'b Game, target: &'a mut Frame) -> Self {
        let program = game.resources.get_shader("cell").unwrap();
        Renderer {
            target,
            display: &game.display,
            program,
        }
    }

    pub fn render_cell(&mut self, location: (u32, u32), scale: u32, shape: Shape) {
        #[derive(Copy, Clone)]
        struct Vertex {
            point: [f32; 2],
        }
        implement_vertex!(Vertex, point);

        let indices = NoIndices(PrimitiveType::TrianglesList);
        let mut vertices = Vec::<Vertex>::new();
        match shape {
            Shape::Full => {
                vertices.push(Vertex { point: [0.0, 0.0] });
                vertices.push(Vertex { point: [1.0, 0.0] });
                vertices.push(Vertex { point: [0.0, 1.0] });
                vertices.push(Vertex { point: [1.0, 1.0] });
                vertices.push(Vertex { point: [0.0, 1.0] });
                vertices.push(Vertex { point: [1.0, 0.0] });
            }
            _ => {
                vertices.push(Vertex { point: [0.0, 0.0] });
                vertices.push(Vertex { point: [1.0, 0.0] });
                vertices.push(Vertex { point: [0.0, 1.0] });
            }
        }
        let vertex_buffer = VertexBuffer::new(self.display, &vertices).unwrap();

        let projection =
            glm::ortho::<f32>(0.0, GAME_WIDTH as f32, GAME_HEIGHT as f32, 0.0, -1.0, 1.0);
        let mut matrix = Matrix4::<f32>::identity();
        matrix = matrix.append_nonuniform_scaling(&[scale as f32, scale as f32, 1.0].into());
        matrix = matrix.append_translation(&[location.0 as f32, location.1 as f32, 0.0].into());

        let uniforms = uniform! {
            target: *matrix.as_ref(),
            projection: *projection.as_ref(),
        };

        self.target
            .draw(
                &vertex_buffer,
                &indices,
                &self.program,
                &uniforms,
                &DrawParameters {
                    blend: Blend::alpha_blending(),
                    ..Default::default()
                },
            )
            .unwrap();
    }
}
