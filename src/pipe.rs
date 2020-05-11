use ggez::{graphics, mint, Context, GameResult};
use rand::Rng;

pub struct Pipe {
    pub x: f32,
    pub y: f32,
    pub mesh: graphics::Mesh,
    pub width: f32,
    pub height: f32,
}

impl Pipe {
    pub fn new(ctx: &mut Context, x: f32, y: f32, height: f32) -> Pipe {
        let width = 48.;
        let mesh = graphics::Rect::new(0., 0., width, height);
        let mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            mesh,
            graphics::Color::new(0., 255., 0., 1.),
        )
        .unwrap();
        Pipe {
            x,
            y,
            mesh,
            width,
            height,
        }
    }
    pub fn render(&self, ctx: &mut Context) -> GameResult<()> {
        graphics::draw(
            ctx,
            &self.mesh,
            graphics::DrawParam::default().dest(mint::Point2 {
                x: self.x,
                y: self.y,
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
        let height = 32. + rng.gen_range(0., 600. - 32. * 10.);
        Pipes {
            pipe: vec![
                Pipe::new(ctx, x, 0., height),
                Pipe::new(ctx, x, height + 32. * 8., 600.),
            ],
            speed: 2.,
        }
    }
    pub fn update(&mut self) {
        self.pipe[0].x -= self.speed;
        self.pipe[1].x -= self.speed;
    }
}
