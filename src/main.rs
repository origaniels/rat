use std::{io::{stdout, Result}, vec};


use rat::{Bullet, Invader};
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, KeyCode, KeyEventKind}, style::Color, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand
    },
    style::Stylize,
    widgets::{Block, Paragraph, Widget},
    Terminal,
};


fn main() -> Result<()>{
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut term = Terminal::new(CrosstermBackend::new(stdout()))?;
    term.clear()?;

    let mut player = Invader::Invader::new(42, 42);
    let mut bullets: Vec<Bullet::Bullet> = vec![];
    loop {
        term.draw(|frame| {
            let area = frame.size();

            let mut to_be_removed: Vec<usize> = vec![];
            for (i, bullet) in bullets.iter_mut().enumerate() {
                bullet.up();
                if bullet.offscreen() {
                    to_be_removed.push(i);
                } else {
                    frame.render_widget(bullet.get_widget(), area);
                }
            }
            for (i, pos) in to_be_removed.iter().enumerate() {
                bullets.remove(pos-i);
            }

            player.set_frame(area.height, area.width);
            frame.render_widget(
                player.get_widget(),
                area
            );
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key)= event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => break,
                        KeyCode::Char('z')=>player.up(),
                        KeyCode::Char('s')=>player.down(),
                        KeyCode::Char('d')=>player.right(),
                        KeyCode::Char('q')=>player.left(),
                        KeyCode::Tab=>player.shoot(&mut bullets),
                        _ => ()
                    };
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
