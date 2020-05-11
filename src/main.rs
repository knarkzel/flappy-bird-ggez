use game_prototype::bird::*;
use game_prototype::pipe::*;
use ggez::event::{self, EventHandler};
use ggez::{graphics, Context, ContextBuilder, GameResult};
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
    amount: i32,
    dead: i32,
}

impl Game {
    pub fn new(_ctx: &mut Context) -> Game {
        let amount = 100;
        let birds = (0..amount).map(|_| Bird::new()).collect();
        let pipes = vec![
            Pipes::new(200.),
            Pipes::new(400.),
            Pipes::new(600.),
            Pipes::new(800.),
        ];
        Game { birds, pipes, amount, dead: 0 }
    }
    fn restart(&mut self) {
        let pipes = vec![
            Pipes::new(200.),
            Pipes::new(400.),
            Pipes::new(600.),
            Pipes::new(800.),
        ];
        let mut rng = rand::thread_rng();
        for bird in self.birds.iter_mut() {
            bird.exist = true;
            println!("{:?}", bird.brain);
            // bird.brain.mutate();
            bird.y = rng.gen_range(48. * 2., 600. - (48. * 3.));
        }
        self.pipes = pipes;
        self.dead = 0;
    }
}

impl EventHandler for Game {
    fn update(&mut self, _: &mut Context) -> GameResult<()> {
        for i in 0..self.birds.len() {
            if do_collide(&self.birds[0], &self.pipes[0].pipe[0]) || do_collide(&self.birds[0], &self.pipes[0].pipe[1])
                || self.birds[i].y + self.birds[i].height > 600. || self.birds[i].y < 0.
            {
                if self.birds[i].exist {
                    self.dead += 1;
                    self.birds[i].exist = false;
                }
            }
        }
        for bird in self.birds.iter_mut() {
            if bird.exist {
                bird.update(&self.pipes[0].pipe[0], &self.pipes[0].pipe[1]);
            }
        }
        if self.dead >= self.amount {
           // take 10 best birds, make 10 copies of each then mutate slightly
           // restart level then
            // let birds = (0..self.amount).map(|_| Bird::new()).collect();
            // self.birds.sort_by(|a, b| a.fitness.cmp(&b.fitness));
            // println!("{:#?}", self.birds);
            self.restart();
        }
        for pipes in self.pipes.iter_mut() {
            pipes.update();
        }
        if self.pipes[0].pipe[0].x < 0. {
            self.pipes.remove(0);
            self.pipes.push(Pipes::new(800.));
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
            if bird.exist {
                bird.render(ctx)?;
            }
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
