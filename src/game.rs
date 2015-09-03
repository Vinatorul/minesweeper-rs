use field::{Field, Content};
use ui::UI;
use piston_window::*;
use common::{ParamType, MoveDestination};

pub struct Game<'a> {
    field: Field,
    ui: UI<'a>,
    glyphs: Glyphs,
    mouse_x: f64,
    mouse_y: f64,
    game_ended: bool,
    panel_width: u32,
    in_ui: bool
}

impl<'a> Game<'a> {
    pub fn new(glyphs: Glyphs, width: u32, height: u32, mines: u32) -> Game<'a> {
        Game {
            field: Field::new(width, height, mines),
            ui: UI::new(width, height, mines),
            glyphs: glyphs,
            mouse_x: 0.0,
            mouse_y: 0.0,
            game_ended: false,
            panel_width: 350,
            in_ui: false
        }
    }

    pub fn render(&mut self, window: &PistonWindow) {      
        window.draw_2d(|c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            let field_rect = self.get_field_rect(window);
            self.field.draw(c, g, field_rect, &mut self.glyphs);
            let ui_rect = self.get_ui_rect(window);
            self.ui.draw(c, g, ui_rect, &mut self.glyphs);
        });
    }

    fn get_field_rect(&self, window: &PistonWindow) -> [u32; 4] {
        let mut w = window.size().width - self.panel_width;
        w = (w /self.field.get_width()) * self.field.get_width();
        let mut h = window.size().height;
        h = (h /self.field.get_height()) * self.field.get_height();
        [0, 0, w, h]
    }

    fn get_ui_rect(&self, window: &PistonWindow) -> [u32; 4] {
        let mut field_w = window.size().width - self.panel_width;
        field_w = (field_w /self.field.get_width()) * self.field.get_width();
        let w = window.size().width - field_w;
        let h = window.size().height;
        [field_w, 0, w, h]
    }

    pub fn proc_key(&mut self, button: Button, window: &PistonWindow) {
        if self.in_ui {
            match button {
                Button::Keyboard(key) => {
                    match  key {
                        Key::H => {
                            match self.ui.proc_key(ParamType::Height) {
                                Some(h) => {
                                    self.in_ui = false;
                                    self.field.reinit_field(h, ParamType::Height);
                                }
                                _ => {}
                            }
                        },
                        Key::M => {
                            match self.ui.proc_key(ParamType::Mines) {
                                Some(m) => {
                                    self.in_ui = false;
                                    self.field.reinit_field(m, ParamType::Mines);
                                }
                                _ => {}
                            }
                        },
                        Key::W => {
                            match self.ui.proc_key(ParamType::Width) {
                                Some(w) => {
                                    self.in_ui = false;
                                    self.field.reinit_field(w, ParamType::Width);
                                }
                                _ => {}
                            }
                        },
                        Key::Up => self.ui.change_selected(MoveDestination::Up),
                        Key::Down => self.ui.change_selected(MoveDestination::Down),
                        Key::Left => self.ui.change_selected(MoveDestination::Left),
                        Key::Right => self.ui.change_selected(MoveDestination::Right),
                        _ => {}
                    }
                }
                _ => {}
            }
        } else {
            match button {
                Button::Keyboard(key) => {
                    match key {
                        Key::R => self.restart(),
                        Key::Up => self.field.move_selection(MoveDestination::Up),
                        Key::Down => self.field.move_selection(MoveDestination::Down),
                        Key::Left => self.field.move_selection(MoveDestination::Left),
                        Key::Right => self.field.move_selection(MoveDestination::Right),
                        Key::Space => {
                            let ind = self.field.get_selected_ind();
                            self.open_cell(ind);
                        },
                        Key::LCtrl | Key::RCtrl => {
                            let ind = self.field.get_selected_ind();
                            self.toggle_mark(ind);
                        }
                        Key::H => {
                            self.ui.proc_key(ParamType::Height);
                            self.in_ui = true;
                        },
                        Key::M =>{
                            self.ui.proc_key(ParamType::Mines);
                            self.in_ui = true;
                        },
                        Key::W => {
                            self.ui.proc_key(ParamType::Width);
                            self.in_ui = true;
                        },
                        _ => {}
                    }
                },
                Button::Mouse(btn) => {
                    let field_rect = self.get_field_rect(window);
                    let cell_w = field_rect[2] / self.field.get_width();
                    let cell_h = field_rect[3] / self.field.get_height();
                    let mouse_x = self.mouse_x.floor() as u32;
                    let mouse_y = self.mouse_y.floor() as u32;
                    if (mouse_x < field_rect[0]) || (mouse_x > field_rect[0] + field_rect[2]) ||
                       (mouse_y < field_rect[1]) || (mouse_y > field_rect[1] + field_rect[3]) {
                        return;
                    }
                    let x = (mouse_x - field_rect[0]) / cell_w;
                    let y = (mouse_y - field_rect[1]) / cell_h;
                    let w = self.field.get_width();
                    match btn {
                        MouseButton::Left => {
                            self.open_cell(x + y*w);
                        },
                        MouseButton::Right => {

                            self.toggle_mark(x + y*w);
                        },
                        _ => {}
                    }
                }
            }
        }
    }

    fn toggle_mark(&mut self, i: u32) {
        if self.game_ended || self.field.revealed(i) {
            return;
        }
        self.field.toggle_mark(i);
    }

    fn open_cell(&mut self, i: u32) {
        if self.game_ended {
            return;
        }
        
        if !self.field.revealed(i) {
            self.check_reveal(i);
        } else {
            let on_left_edge = i % self.field.get_width() == 0;
            let on_right_edge = (i+1) % self.field.get_width() == 0; 

            for rdelta in -1..2i32 {
                for cdelta in -1..2i32 {
                    if on_left_edge && (cdelta == -1) {
                        continue;
                    }
                    if on_right_edge && (cdelta == 1) {
                        continue;
                    }
                    let tgt = (i as i32) + rdelta*(self.field.get_width() as i32) + cdelta;
                    if (tgt < 0) || (tgt >= self.field.get_size() as i32) {
                        continue;
                    }
                    self.check_reveal(tgt as u32);
                }
            }
        }
    }

    pub fn check_reveal(&mut self, i : u32) {
        if self.field.marked(i) {
            return;
        }
        match *self.field.reveal(i) {
            Content::Mine(_) => {
                self.field.reveal_all();
                self.game_ended = true;
                self.field.set_killer(i); 
                println!("Game over :(");
            },
            Content::None => {
                self.field.chain_reveal(i);
                if self.field.is_victory() {
                    println!("You win :)");
                    self.game_ended = true;
                }
            }
            Content::Number(_i) => {
                if self.field.is_victory() {
                    println!("You win :)");
                    self.game_ended = true;
                } 
            }
        }
    }

    pub fn mouse_move(&mut self, mouse_rel: [f64; 2]) {
        self.mouse_x = mouse_rel[0];
        self.mouse_y = mouse_rel[1];
    }

    fn restart(&mut self) {
        self.game_ended = false;
        self.field.restart();
    }
}
