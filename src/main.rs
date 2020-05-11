use game_prototype::bird::*;
use game_prototype::pipe::*;
use ggez::event::{self, EventHandler};
use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::input::keyboard;
use ggez::input::keyboard::KeyCode;
use rand::Rng;

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("Title", "Game Author")
        .build()
        .expect("Unable to build context.");

    let mut my_game = Game::new(&mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct Game {
    birds: Vec<Bird>,
    pipes: Vec<Pipes>,
    saved_birds: Vec<Bird>,
    amount: i32,
}

impl Game {
    pub fn new(_ctx: &mut Context) -> Game {
        let amount = 100;
        let birds = (0..amount).map(|_| Bird::new()).collect();
        let saved_birds: Vec<Bird> = vec![];
        let pipes = vec![
            Pipes::new(200.),
            Pipes::new(400.),
            Pipes::new(600.),
            Pipes::new(800.),
        ];
        Game { birds, pipes, saved_birds, amount }
    }
    fn restart(&mut self) {
        let pipes = vec![
            Pipes::new(200.),
            Pipes::new(400.),
            Pipes::new(600.),
            Pipes::new(800.),
        ];
        let mut rng = rand::thread_rng();
        let take: i32 = 5;
        let best_competors: Vec<Bird> = self.saved_birds.clone().drain(self.saved_birds.len() - take as usize ..).collect();
        for bird in best_competors {
            let mut clones: Vec<Bird> = (0..self.amount / take).map(|_| bird.clone()).collect();
            for new_bird in clones.iter_mut() {
                new_bird.brain.mutate();
                new_bird.y = rng.gen_range(48. * 2., 600. - (48. * 3.));
            }
            self.birds.extend(clones);
        }
        self.pipes = pipes;
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
            for bird in self.birds.iter_mut() {
                bird.update(&self.pipes[0].pipe[0], &self.pipes[0].pipe[1]);
            }
            let mut total = self.birds.len();
            for i in 0..total {
                if i <= total - 1 {
                    if do_collide(&self.birds[0], &self.pipes[0].pipe[0]) || do_collide(&self.birds[0], &self.pipes[0].pipe[1])
                        || self.birds[i].y + self.birds[i].height > 600. || self.birds[i].y < 0.
                    {
                        self.saved_birds.push(self.birds.remove(i));
                        total -= 1;
                    }
                }
            }
            if self.saved_birds.len() >= self.amount as usize {
                self.restart();
            }
            for pipes in self.pipes.iter_mut() {
                pipes.update();
            }
            if self.pipes[0].pipe[0].x < 0. {
                self.pipes.remove(0);
                self.pipes.push(Pipes::new(800.));
            }
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        for pipes in self.pipes.iter() {
            pipes.pipe[0].render(ctx)?;
            pipes.pipe[1].render(ctx)?;
        }
        for bird in self.birds.iter() {
            bird.render(ctx)?;
        }
        graphics::present(ctx)
    }
}

fn do_collide(rect1: &Bird, rect2: &Pipe) -> bool {
    if rect1.x < rect2.x + rect2.width &&
        rect1.x + rect1.width > rect2.x &&
        rect1.y < rect2.y + rect2.height &&
        rect1.y + rect1.height > rect2.y {
            return true;
    }
    false
}
