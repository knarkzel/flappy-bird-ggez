use game_prototype::bird::*;
use game_prototype::pipe::*;
use ggez::input::keyboard;
use ggez::input::keyboard::KeyCode;
use ggez::event::{self, EventHandler};
use ggez::{graphics, Context, ContextBuilder, GameResult};

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
}

impl Game {
    pub fn new(_ctx: &mut Context) -> Game {
        let mut birds = vec![Bird::new()];
        let pipes = vec![
            Pipes::new(200.),
            Pipes::new(400.),
            Pipes::new(600.),
            Pipes::new(800.),
        ];
        birds[0].jump();
        Game { birds, pipes }
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        for bird in self.birds.iter_mut() {
            bird.update(&self.pipes[0].pipe[1]);
            // if keyboard::is_key_pressed(ctx, KeyCode::Space) {
            //     bird.jump();
            // }
        }
        for pipes in self.pipes.iter_mut() {
            pipes.update();
        }
        if self.pipes[0].pipe[0].x < 0. {
            self.pipes.remove(0);
            self.pipes.push(Pipes::new(800.));
        }
        for i in 0..self.birds.len() {
            if do_collide(&self.birds[0], &self.pipes[0].pipe[0]) || do_collide(&self.birds[0], &self.pipes[0].pipe[1])
                || self.birds[i].y + self.birds[i].height > 600. || self.birds[i].y < 0.
            {
                self.birds.remove(i);
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
