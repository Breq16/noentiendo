mod execute;
mod fetch;
mod graphics;
mod memory;
mod registers;
mod system;

use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  match args.len() {
    2 => {}
    _ => panic!("Usage: {} <file>", args[0]),
  }
  let filename = &args[1];

  let graphics = Box::new(graphics::WinitGraphicsProvider::new());
  let memory = memory::systems::easy(graphics, filename);

  let mut system = system::System::new(memory);

  system.reset();

  loop {
    system.tick();
  }
}
