use std::{error::Error, io, time::Duration, sync::mpsc, thread};

use crossterm::{terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, cursor::{Hide, Show}, ExecutableCommand, event::{self, Event, KeyCode}};
use invaders::{frame::{new_frame, Drawable}, render::render, player::Player};
use rusty_audio::Audio;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");
    audio.add("move", "move.wav");
    audio.add("pew", "pew.wav");
    audio.add("startup", "startup.wav");
    audio.add("win", "win.wav");

    audio.play("startup");
    
    //Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    //Render loop in seperate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handler = thread::spawn(move || {
        let mut last_frame = new_frame();
        let mut stdout = io::stdout();
        render(&mut stdout, &last_frame, &last_frame, true);

        loop{
            let curr_frame = match render_rx.recv(){
                Ok(x) => x,
                Err(_) => break
            };
            render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    }); 

    //Game Loop
    let mut player = Player::new();
    'gameloop: loop{
        //Per frame init
        let mut curr_fram = new_frame();
        //Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }

                    _ => {

                    }
                }
            }
        }

        //Draw & Render
        player.draw(&mut  curr_fram);
        let _ = render_tx.send(curr_fram);
        thread::sleep(Duration::from_millis(1));
    }
    
    //Cleanup
    drop(render_tx);
    render_handler.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
