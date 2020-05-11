use super::pipe::*;
use super::neuralnetwork::*;
use rand::Rng;
use ggez::{graphics, Context, GameResult};

#[derive(Clone, Debug)]
pub struct Bird {
    pub x: f32,
    pub y: f32,
    pub vel: f32,
    pub width: f32,
    pub height: f32,
    pub fitness: i32,
    pub exist: bool,
    pub brain: NeuralNetwork,
}

impl Bird {
    pub fn new() -> Bird {
        let width = 48.;
        let height = 48.;
        let mut rng = rand::thread_rng();
        let y = rng.gen_range(48. * 2., 600. - (48. * 3.));
        Bird {
            x: 30.,
            y,
            vel: 0.,
            width,
            height,
            fitness: 0,
            exist: true,
            brain: NeuralNetwork::new(4, 6, 1),
        }
    }
    pub fn jump(&mut self) {
        self.vel = 6.;
    }
    pub fn update(&mut self, pipe_top: &Pipe, pipe_bottom: &Pipe) {
        if self.vel > -13. {
            self.vel -= 0.30;
        }
        self.y -= self.vel;
        self.fitness += 1;

        // neural network stuff
        //  set inputs
        self.brain.set(0, (self.y / 600.).abs());
        self.brain.set(1, (pipe_top.y + pipe_top.height) / 600.);
        self.brain.set(2, pipe_bottom.y / 600.);
        self.brain.set(3, (self.vel + 13.) / 19.);

        // process through network
        self.brain.process();

        // get output
        if self.brain.get(2, 0) >= 0.5 {
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
