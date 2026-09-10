mod game_board;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::widgets::Paragraph;

fn main() -> std::io::Result<()> {
    let gb = game_board::game_board::GameBoard::new(1);

    ratatui::run(|mut terminal| {
        let mut input = String::new();
        let mut key_presses = String::new();

        loop {
            if let Event::Key(key_event) = event::read()? {
                terminal.draw(|frame| {
                    frame.render_widget(
                        Paragraph::new(input.as_str()),
                        frame.area(),
                    );
                })?;
                
                if key_event.kind != KeyEventKind::Press {
                    continue;
                }

                match key_event.code {
                    KeyCode::Esc => break,
                    KeyCode::Enter => {
                        key_presses.push_str(&input);
                        key_presses.push('\n');
                        input.clear();

                        terminal.draw(|frame| {
                            frame.render_widget(
                                Paragraph::new(format!("keys pressed are {:?}", key_presses)),
                                frame.area(),
                            );
                        })?;

                    }
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Char(character) => {
                        input.push(character);
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    })
}
