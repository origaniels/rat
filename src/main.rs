use std::{io::{stdout, Result}, time::Instant, vec};


use geo::Point;
use rat::{Bullet, Invader};
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, KeyCode, KeyEventKind, MouseEvent, MouseEventKind}, style::Color, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand
    },
    style::Stylize,
    widgets::{Block, Paragraph, Widget},
    Terminal,
};


fn main() -> Result<()>{
    stdout().execute(EnterAlternateScreen)?;
    stdout().execute(event::EnableMouseCapture)?;
    enable_raw_mode()?;
    let mut term = Terminal::new(CrosstermBackend::new(stdout()))?;
    term.clear()?;

    let mut player = Invader::Invader::new(42., 42.);
    let mut bullets: Vec<Bullet::Bullet> = vec![];

    let mut input_events: Vec<KeyCode> = vec![];
    let mut last_frame: Instant = Instant::now();

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

        if last_frame.elapsed().as_millis()>16 {
            for input in input_events.iter() {
                match input {
                    
                    KeyCode::Char('z')=>player.up(),
                    KeyCode::Char('s')=>player.down(),
                    KeyCode::Char('d')=>player.right(),
                    KeyCode::Char('q')=>player.left(),
                    KeyCode::Tab=>player.shoot(&mut bullets),
                    _ => ()
                }
            }
            input_events.clear();
            last_frame = Instant::now();
            player.gravitate();
        }

        if event::poll(std::time::Duration::from_millis(1))? {
            match event::read()? {
                event::Event::Key(key) => {
                    println!("key");
                    if key.kind == KeyEventKind::Press {
                        if key.code == KeyCode::Esc {
                            break;
                        }
                        input_events.push(key.code);
                    }
                },
                event::Event::Mouse(evt) => {
                    println!("mouse");
                    if evt.kind == MouseEventKind::Moved {
                        player.set_target(evt.column as f64,evt.row as f64);
                    }
                },
                _=>()
            }
        }
    }

    stdout().execute(event::DisableMouseCapture)?;
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
