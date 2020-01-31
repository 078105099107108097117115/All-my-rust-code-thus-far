use ggez::event::EventHandler;
use ggez::{GameResult, Context, event};
use ggez::conf::Conf;

struct GameState {
}

impl GameState{
    fn new() -> Self{
        GameState{}
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _context : &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, _context : &mut Context) -> GameResult {
        Ok(())
    }
}

fn main() {
    let game_state = &mut GameState::new();
    let configuration = Conf::new();
    //let context = Context::load_from_conf("random_walkers", author : &'static str, default_config : conf::Conf);
    let context = &mut Context::load_from_conf("random_walker", "Nicholas njihia", configuration).unwrap();
    event::run(context, game_state).unwrap();
    //println!("Hello, world!");
}
