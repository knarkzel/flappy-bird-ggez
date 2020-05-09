use super::pipe::*;
use super::neuralnetwork::*;
use ggez::{graphics, Context, GameResult};

pub struct Bird<'a> {
    pub x: f32,
    pub y: f32,
    pub vel: f32,
    pub width: f32,
    pub height: f32,
    pub brain: NeuralNetwork<'a>,
}

impl<'a> Bird<'a> {
    pub fn new() -> Bird<'a> {
        let width = 48.;
        let height = 48.;
        Bird {
            x: 30.,
            y: 600. / 2. - width,
            vel: 0.,
            width,
            height,
            brain: NeuralNetwork::new(3, 4, 1),
        }
    }
    pub fn jump(&mut self) {
        self.vel = 6.;
    }
    pub fn update(&mut self, pipe: &Pipe) {
        if self.vel > -13. {
            self.vel -= 0.30;
        }
        self.y -= self.vel;
        if self.y + self.height / 2. > pipe.y - 48. {
            self.jump();
        }
    }
    pub fn render(&self, ctx: &mut Context) -> GameResult<()> {
        let rect = graphics::Rect::new(self.x, self.y, self.width, self.height);
        let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            rect,
            graphics::Color::new(255., 0., 0., 1.),
        )?;
        graphics::draw(ctx, &rect, graphics::DrawParam::default())?;
        let rect = graphics::Rect::new(self.x, self.y, self.width, self.height);
        let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(2.),
            rect,
            graphics::BLACK,
        )?;
        graphics::draw(ctx, &rect, graphics::DrawParam::default())?;
        Ok(())
    }
}
