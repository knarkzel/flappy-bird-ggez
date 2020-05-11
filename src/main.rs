use game_prototype::bird::*;
use game_prototype::pipe::*;
use ggez::event::{self, EventHandler};
use ggez::input::keyboard;
use ggez::input::keyboard::KeyCode;
use ggez::{graphics, mint, Context, ContextBuilder, GameResult};
use rand::Rng;
use std::{env, path};

fn main() -> GameResult {
    let mut cb = ContextBuilder::new("Flappy Birds", "Knarkzel");

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        cb = cb.add_resource_path(path);
    }

    let (mut ctx, mut event_loop) = cb.build()?;

    let mut my_game = Game::new(&mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
    Ok(())
}

struct Game {
    background: (f32, f32),
    meshes: Vec<graphics::Image>,
    birds: Vec<Bird>,
    pipes: Vec<Pipes>,
    saved_birds: Vec<Bird>,
    amount: i32,
}

impl Game {
    pub fn new(ctx: &mut Context) -> Game {
        // meshes
        // let bird_mesh = graphics::Rect::new(0., 0., 48., 48.);
        // let bird_mesh = graphics::Mesh::new_rectangle(
        //     ctx,
        //     graphics::DrawMode::fill(),
        //     bird_mesh,
        //     graphics::Color::new(255., 0., 0., 1.),
        // )
        // .unwrap();
        let bird_mesh = graphics::Image::new(ctx, "/bird.png").unwrap();
        let bird_jump = graphics::Image::new(ctx, "/bird-jump.png").unwrap();
        let background = graphics::Image::new(ctx, "/background.png").unwrap();
        let mut meshes: Vec<graphics::Image> = vec![];
        meshes.push(bird_mesh);
        meshes.push(background);
        meshes.push(bird_jump);

        // other
        let amount = 100;
        let birds = (0..amount).map(|_| Bird::new()).collect();
        let saved_birds: Vec<Bird> = vec![];
        let pipes = vec![
            Pipes::new(ctx, 200.),
            Pipes::new(ctx, 400.),
            Pipes::new(ctx, 600.),
            Pipes::new(ctx, 800.),
        ];
        Game {
            background: (0., 799.),
            meshes,
            birds,
            pipes,
            saved_birds,
            amount,
        }
    }
    fn restart(&mut self, ctx: &mut Context) {
        let pipes = vec![
            Pipes::new(ctx, 200.),
            Pipes::new(ctx, 400.),
            Pipes::new(ctx, 600.),
            Pipes::new(ctx, 800.),
        ];
        let mut rng = rand::thread_rng();
        let take: i32 = 5;
        let best_competors: Vec<Bird> = self
            .saved_birds
            .clone()
            .drain(self.saved_birds.len() - take as usize..)
            .collect();
        for bird in best_competors {
            let mut clones: Vec<Bird> = (0..self.amount / take).map(|_| bird.clone()).collect();
            for new_bird in clones.iter_mut() {
                new_bird.brain.mutate();
                new_bird.y = rng.gen_range(48. * 2., 600. - (48. * 3.));
            }
            self.birds.extend(clones);
        }
        self.pipes = pipes;
        self.background = (0., 799.);
        self.saved_birds = vec![];
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let iterations = if keyboard::is_key_pressed(ctx, KeyCode::Space) {
            20
        } else {
            1
        };
        for _ in 0..iterations {
            if self.background.0 <= -800. {
                self.background.0 = 799.;
            }
            if self.background.1 <= -800. {
                self.background.1 = 799.;
            }
            self.background.0 -= 1.;
            self.background.1 -= 1.;
            for pipes in self.pipes.iter_mut() {
                pipes.update();
            }
            if self.pipes[0].pipe[0].x < 0. {
                self.pipes.remove(0);
                self.pipes.push(Pipes::new(ctx, 800.));
            }
            for bird in self.birds.iter_mut() {
                bird.update(&self.pipes[0].pipe[0], &self.pipes[0].pipe[1]);
            }
            // get stuff that collides
            let mut to_remove: Vec<usize> = vec![];
            for (i, bird) in self.birds.iter().enumerate() {
                let first_pipe = &self.pipes[0].pipe[0];
                let second_pipe = &self.pipes[0].pipe[1];
                if do_collide(bird, first_pipe) || do_collide(bird, second_pipe) {
                    to_remove.push(i);
                }
            }
            // remove stuff that collides
            for i in to_remove.iter().rev() {
                self.saved_birds.push(self.birds.remove(*i));
            }
            if self.saved_birds.len() >= self.amount as usize {
                self.restart(ctx);
            }
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::new(0., 0., 200., 1.));
        graphics::draw(
            ctx,
            &self.meshes[1],
            graphics::DrawParam::default().dest(mint::Point2 {
                x: self.background.0,
                y: 0.,
            }),
        )?;
        graphics::draw(
            ctx,
            &self.meshes[1],
            graphics::DrawParam::default().dest(mint::Point2 {
                x: self.background.1,
                y: 0.,
            }),
        )?;
        for pipes in self.pipes.iter() {
            pipes.pipe[0].render(ctx)?;
            pipes.pipe[1].render(ctx)?;
        }
        for bird in self.birds.iter() {
            if bird.vel > 0. {
                bird.render(ctx, &self.meshes[0])?;
            } else {
                bird.render(ctx, &self.meshes[2])?;
            }
        }

        graphics::present(ctx)
    }
}

fn do_collide(rect1: &Bird, rect2: &Pipe) -> bool {
    if rect1.x < rect2.x + rect2.width
        && rect1.x + rect1.width > rect2.x
        && rect1.y < rect2.y + rect2.height
        && rect1.y + rect1.height > rect2.y
        || rect1.y + rect1.height > 600.
        || rect1.y < 0.
    {
        return true;
    }
    false
}
