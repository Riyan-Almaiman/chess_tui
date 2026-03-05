use crossterm::event::KeyCode;

pub enum Color {
    Black, 
    White
}
pub struct  Player {
    pub color: Color
}
pub struct App {
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub selected: Option<(usize, usize)>,
    pub player: Player
    
}

impl App {
    pub fn new(player: Player) -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            player,
            selected: None,
        }
    }

    pub fn handle_input(&mut self, key: KeyCode) -> bool {

         match key {
                    KeyCode::Left => {
                        if self.cursor_x > 0 {
                            self.cursor_x -= 1;
                        }
                    }
                    KeyCode::Char(' ') => {
                        self.selected = Some((self.cursor_x, self.cursor_y));
                    }
                    KeyCode::Right => {
                        if self.cursor_x < 7 {
                            self.cursor_x += 1;
                        }
                    }
                    KeyCode::Up => {
                        if self.cursor_y > 0 {
                            self.cursor_y -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if self.cursor_y < 7 {
                            self.cursor_y += 1;
                        }
                    }
                    KeyCode::Char('q') => return false,
                    _ => ()
                }
                true
    }
}