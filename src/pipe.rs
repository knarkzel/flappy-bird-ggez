use ggez::{graphics, mint, Context, GameResult};
use rand::Rng;

pub struct Pipe {
    pub x: f32,
    pub y: f32,
    pub mesh: graphics::Image,
    pub width: f32,
    pub height: f32,
}

impl Pipe {
    pub fn new(ctx: &mut Context, x: f32, y: f32, height: f32) -> Pipe {
        let width = 48.;
        let mesh = if y == 0. {
            graphics::Image::new(ctx, "/pipe-top.png").unwrap()
        } else {
            graphics::Image::new(ctx, "/pipe-bottom.png").unwrap()
        };
        Pipe {
            x,
            y,
            mesh,
            width,
            height,
        }
    }
    pub fn render(&self, ctx: &mut Context) -> GameResult<()> {
        let offset = if self.y == 0. { 600. - self.height } else { 0. };
        graphics::draw(
            ctx,
            &self.mesh,
            graphics::DrawParam::default().dest(mint::Point2 {
                x: self.x,
                y: self.y - offset,
            }),
        )?;
        Ok(())
    }
}

pub struct Pipes {
    pub pipe: Vec<Pipe>,
    speed: f32,
}

impl Pipes {
    pub fn new(ctx: &mut Context, x: f32) -> Pipes {
        let mut rng = rand::thread_rng();
        let gap = 250.;
        let height = 32. + rng.gen_range(0., 600. - gap);
        Pipes {
            pipe: vec![
                Pipe::new(ctx, x, 0., height),
                Pipe::new(ctx, x, height + gap - 64., 600.),
            ],
            speed: 2.,
        }
    }
    pub fn update(&mut self) {
        self.pipe[0].x -= self.speed;
        self.pipe[1].x -= self.speed;
    }
}
