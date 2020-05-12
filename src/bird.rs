use super::neuralnetwork::*;
use super::pipe::*;
use ggez::{graphics, mint, Context, GameResult};
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Bird {
    pub x: f32,
    pub y: f32,
    pub alive: i32,
    pub vel: f32,
    pub width: f32,
    pub height: f32,
    pub brain: NeuralNetwork,
}

impl Bird {
    pub fn new() -> Bird {
        let mut rng = rand::thread_rng();
        let x = 30.;
        let y = rng.gen_range(48. * 2., 600. - (48. * 3.));
        let width = 48.;
        let height = 48.;
        Bird {
            x,
            y,
            alive: 0,
            vel: 0.,
            width,
            height,
            brain: NeuralNetwork::new(4, 6, 1),
        }
    }
    pub fn jump(&mut self) {
        self.vel = 6.;
    }
    pub fn update(&mut self, pipe_top: &Pipe, pipe_bottom: &Pipe) {
        self.alive += 1;
        if self.vel > -13. {
            self.vel -= 0.30;
        }
        self.y -= self.vel;

        // neural network stuff
        //  set inputs
        self.brain.set(0, (self.y / 600.).abs(), 0.);
        self.brain.set(1, (self.vel + 13.) / 19., 0.);
        self.brain.set(2, pipe_top.y / 600., 0.);
        self.brain.set(3, pipe_bottom.y / 600., 0.);

        // process through network
        self.brain.process();

        // get output
        if self.brain.get(2, 0) > 0.5 {
            self.jump();
        }
    }
    pub fn render(&self, ctx: &mut Context, mesh: &graphics::Image) -> GameResult<()> {
        let angle = (-(self.vel + 13.) / 19. + 1.) * 2.;
        let offset = mint::Point2 {x: 0.5, y: 0.5};
        graphics::draw(
            ctx,
            mesh,
            graphics::DrawParam::default().dest(mint::Point2 {
                x: self.x + self.width / 2.,
                y: self.y + self.height / 2.,
            }).rotation(angle).offset(offset),
        )?;
        Ok(())
    }
}
