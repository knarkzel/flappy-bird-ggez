use ggez::{graphics, Context, GameResult};
use rand::Rng;

pub struct Pipe {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Pipe {
    pub fn new(x: f32, y: f32, height: f32) -> Pipe {
        let width = 48.;
        Pipe {
            x,
            y,
            width,
            height,
        }
    }
    pub fn render(&self, ctx: &mut Context) -> GameResult<()> {
        let rect = graphics::Rect::new(self.x, self.y, self.width, self.height);
        let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            rect,
            graphics::Color::new(0., 255., 0., 1.),
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

pub struct Pipes {
    pub pipe: Vec<Pipe>,
    speed: f32,
}

impl Pipes {
    pub fn new(x: f32) -> Pipes {
        let mut rng = rand::thread_rng();
        let height = 32. + rng.gen_range(0., 600. - 32. * 7.);
        Pipes {
            pipe: vec![
                Pipe::new(x, 0., height),
                Pipe::new(x, height + 32. * 5., 600.),
            ],
            speed: 2.,
        }
    }
    pub fn update(&mut self) {
        self.pipe[0].x -= self.speed;
        self.pipe[1].x -= self.speed;
    }
}
