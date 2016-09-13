
use piston_window::*;
use common::{ParamType, MoveDestination, GameEndState};
use chrono::Duration;

struct Block<'a> {
    name: &'a str,
    num: u32,
    old_num: u32,
    hotkey: char,
    selected: bool,
    min_val: u32,
    max_val: u32,
    param_type: ParamType,
}

impl<'a> Block<'a> {
    pub fn new(name: &'a str,
               num: u32,
               hotkey: char,
               min_val: u32,
               max_val: u32,
               param_type: ParamType)
               -> Block<'a> {
        Block {
            name: name,
            num: num,
            old_num: num,
            hotkey: hotkey,
            selected: false,
            min_val: min_val,
            max_val: max_val,
            param_type: param_type,
        }
    }

    pub fn draw(&self,
                context: Context,
                graphics: &mut G2d,
                rect: &mut [u32; 4],
                glyps: &mut Glyphs) {
        let margin = 10;
        let text_height = 20;
        let block_height = 30;
        let text_padding = 7;
        let color = if self.selected {
            [0.0, 1.0, 0.0, 1.0]
        } else {
            [1.0, 1.0, 1.0, 1.0]
        };
        rect[1] += 20;
        rectangle(color,
                  [(rect[0] + margin) as f64,
                   (rect[1]) as f64,
                   (rect[2] - 2 * margin) as f64,
                   block_height as f64],
                  context.transform,
                  graphics);
        rect[1] += block_height;
        let transform = context.transform.trans((rect[0] + margin + text_padding) as f64,
                                                (rect[1] - text_padding) as f64);
        text([0.0, 0.0, 0.0, 1.0],
             text_height,
             &*format!("{}: {}", self.name, self.num),
             glyps,
             transform,
             graphics);
        rect[1] += margin + text_height / 2;
        let transform = context.transform.trans((rect[0] + margin) as f64, (rect[1]) as f64);
        let s = if self.selected {
            format!("Use arrows, press '{}' to apply changes", self.hotkey)
        } else {
            format!("Press '{}' to change", self.hotkey)
        };
        text([1.0, 1.0, 1.0, 1.0],
             text_height,
             &*s,
             glyps,
             transform,
             graphics);
    }

    pub fn inc_safe(&mut self) {
        if self.num < self.max_val {
            self.num += 1;
        }
    }

    pub fn dec_safe(&mut self) {
        if self.num > self.min_val {
            self.num -= 1;
        }
    }

    pub fn apply_changes(&mut self) {
        self.old_num = self.num;
    }

    pub fn discard_changes(&mut self) {
        self.num = self.old_num;
    }
}

pub struct UI<'a> {
    blocks: Vec<Block<'a>>,
    selected_block: i32,
}

impl<'a> UI<'a> {
    pub fn new(height: u32, width: u32, mines: u32) -> UI<'a> {
        UI {
            blocks: vec![Block::new("Field width", width, 'W', 5, 50, ParamType::Width),
                         Block::new("Field height", height, 'H', 5, 50, ParamType::Height),
                         Block::new("Mines", mines, 'M', 1, 2500, ParamType::Mines),
            ],
            selected_block: -1,
        }
    }

    pub fn draw(&mut self,
                context: Context,
                graphics: &mut G2d,
                mut rect: [u32; 4],
                glyps: &mut Glyphs,
                mines_total: u32,
                mines_marked: u32,
                duration: Duration) {
        for b in self.blocks.iter() {
            b.draw(context, graphics, &mut rect, glyps);
        }

        let transform = context.transform.trans((rect[0] + 10) as f64, (rect[1] + 27) as f64);
        text([1.0, 1.0, 1.0, 1.0],
             20,
             &*format!("{} marked {} remaining",
                       mines_marked,
                       mines_total as i32 - mines_marked as i32),
             glyps,
             transform,
             graphics);

        let transform = context.transform.trans((rect[0] + 10) as f64, (rect[1] + 27 * 2) as f64);
        let total_seconds = duration.num_seconds();
        let mins = total_seconds / 60;
        let rem_seconds = total_seconds - mins * 60;

        text([1.0, 1.0, 1.0, 1.0],
             20,
             &*format!("{}:{:02}", mins, rem_seconds),
             glyps,
             transform,
             graphics);
    }

    pub fn proc_key(&mut self, block: ParamType) -> Option<u32> {
        if self.selected_block >= 0 {
            let selected = self.blocks.get_mut(self.selected_block as usize).unwrap();
            if selected.param_type == block {
                selected.selected = false;
                selected.apply_changes();
                self.selected_block = -1;
                return Some(selected.num);
            }
        }
        for i in 0..self.blocks.len() {
            let item = self.blocks.get_mut(i).unwrap();
            let b = item.param_type == block;
            item.selected = b;
            if b {
                self.selected_block = i as i32;
            } else {
                item.discard_changes();
            }
        }
        None
    }

    pub fn change_selected(&mut self, dest: MoveDestination) {
        assert!(self.selected_block >= 0);
        let selected = self.blocks.get_mut(self.selected_block as usize).unwrap();
        match dest {
            MoveDestination::Up | MoveDestination::Right => selected.inc_safe(),
            MoveDestination::Down | MoveDestination::Left => selected.dec_safe(),
        }
    }
}

pub struct EndMessage<'a> {
    visible: bool,
    win: bool,
    message: &'a str,
}

static EM_TEXT_BIG: u32 = 40;
static EM_TEXT_SMALL: u32 = 20;
static EM_TEXT_PADDING: u32 = 5;
static EM_BORDER_WIDTH: u32 = 3;
static EM_BORDER_WIDTH_2: u32 = 6;

static RETRY_MSG: &'static str = "Press 'R' to play again";

impl<'a> EndMessage<'a> {
    pub fn new() -> EndMessage<'a> {
        EndMessage {
            visible: false,
            win: false,
            message: "Ready",
        }
    }

    pub fn show(&mut self, end_state: GameEndState) {
        self.win = match end_state {
            GameEndState::Win => true,
            _ => false,
        };
        self.visible = true;
        self.message = if self.win {
            "You WIN :)"
        } else {
            "Game over :("
        };
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    /// Message Box's size. (width,height).
    pub fn size() -> (u32, u32) {
        (350, (EM_TEXT_BIG + EM_TEXT_SMALL + (EM_TEXT_PADDING * 4) + (EM_BORDER_WIDTH * 2)))
    }

    pub fn draw(&self, context: Context, graphics: &mut G2d, rect: [u32; 4], glyps: &mut Glyphs) {
        if !self.visible {
            return;
        }

        let border_color = if self.win {
            [0.0, 0.0, 1.0, 1.0]    // Win : Blue
        } else {
            [1.0, 1.0, 0.0, 1.0]    // Lose: yellow
        };

        // draw border
        rectangle(border_color,
                  [(rect[0]) as f64, (rect[1]) as f64, (rect[2]) as f64, (rect[3]) as f64],
                  context.transform,
                  graphics);

        // draw background
        rectangle([1.0, 1.0, 1.0, 1.0],
                  [(rect[0] + EM_BORDER_WIDTH) as f64,
                   (rect[1] + EM_BORDER_WIDTH) as f64,
                   (rect[2] - EM_BORDER_WIDTH_2) as f64,
                   (rect[3] - EM_BORDER_WIDTH_2) as f64],
                  context.transform,
                  graphics);

        // Draw game result message
        let trans_msg = context.transform
            .trans((rect[0] + EM_BORDER_WIDTH + EM_TEXT_PADDING) as f64,
                   (rect[1] + EM_BORDER_WIDTH + EM_TEXT_PADDING + EM_TEXT_BIG) as f64);

        text([0.0, 0.0, 0.0, 1.0],
             EM_TEXT_BIG,
             &*self.message,
             glyps,
             trans_msg,
             graphics);

        // Draw 'play again' message
        let trans_retry = context.transform
            .trans((rect[0] + EM_BORDER_WIDTH + EM_TEXT_PADDING) as f64,
                   (rect[1] + EM_BORDER_WIDTH + EM_TEXT_PADDING * 2 + EM_TEXT_BIG +
                    EM_TEXT_SMALL) as f64);
        text([0.0, 0.0, 0.0, 1.0],
             EM_TEXT_SMALL,
             &*RETRY_MSG,
             glyps,
             trans_retry,
             graphics);
    }
}
