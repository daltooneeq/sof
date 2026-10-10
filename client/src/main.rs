mod game_tui;

use color_eyre::Result;
use crossterm::event;

fn main() -> Result<()>{
   color_eyre::install()?;
   ratatui::run(|terminal| {
       loop {
           terminal.draw(game_tui::render)?;
           if event::read()?.is_key_press() {
               break Ok(())
           }
       }
   })
}
