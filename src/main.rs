use notan::draw::*;
use notan::math::{vec2, Mat3, Vec2};
use notan::prelude::*;
use notan::log::debug;

const WIDTH: f32 = 600.0;
const HEIGHT: f32 = 600.0;
const MARGIN: f32 = 50.0;


#[derive(Default, Copy, Clone, PartialEq, Debug)]
enum Player {
    #[default]
    Red,
    Yellow,
}

enum Tied {
    Yes,
}

#[derive(AppState)]
struct State {
    rng: Random,
    font: Font,
    turn: Player,
    table: [[Option<Player>; 7];6],
    winner: Option<Player>,
    tie: Option<Tied>,
}

impl State {
    fn new(gfx: &mut Graphics) -> Self {
        let font = gfx
            .create_font(include_bytes!("assets/Ubuntu-B.ttf"))
            .unwrap();

        let mut rng = Random::default();
        let turn = if rng.gen_bool(0.5) {
            Player::Yellow
        } else {
            Player::Red
        };

        State {
            rng,
            font,
            turn,
            table: Default::default(),
            winner: None,
            tie: None,
        }
    }

    fn reset(&mut self) {
        self.turn = if self.rng.gen_bool(0.5) {
            Player::Yellow
        } else {
            Player::Red
        };

        self.table = Default::default();
        self.winner = None;
        self.tie = None;
    }
}

pub fn main() -> Result<(), String> {
    let win = WindowConfig::default()
        .set_multisampling(8)
        .set_size(WIDTH as _, HEIGHT as _)
        .set_vsync(true);

    notan::init_with(State::new)
        .add_config(win)
        .add_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
}

fn update(app: &mut App, state: &mut State) {
    if state.winner.is_some() {
        if app.keyboard.was_pressed(KeyCode::Space) {
            state.reset();
        }
        return;
    }

    let x = MARGIN;
    let y = MARGIN;
    let width = WIDTH - MARGIN * 2.0;
    let height = HEIGHT - MARGIN * 2.0;

    let tile_width = width / 7.0;

    let (mx, my) = app.mouse.position();

    if app.mouse.was_pressed(MouseButton::Left) {
        // check bounds
        if mx < x || mx > x + width {
            return;
        }

        if my < y || my > y + height {
            return;
        }

        // inside the table
        let col = ((mx - x) / tile_width).floor();
        let index = index_from_pos(col as _);
        // debug!("{} index", index);

        debug!("table cols and rows: {}", state.table.len(), );
        // set piece
        let is_empty = matches!(state.table[0][col as usize], None);
        debug!("is_empty index: {:?}", state.table[0][index]);
        if !is_empty {
            return;
        }
        
        let mut row = 0;

        for i in (0..6).rev() {
            if state.table[i][col as usize].is_none() {
                row = i;
                break;
            }
        }     
            state.table[row][col as usize] = Some(state.turn);

        // change turn
        state.turn = match state.turn {
            Player::Red => Player::Yellow,
            Player::Yellow => Player::Red,
        };

        // game over

        if let Some(winner) = check_winner(&state.table) {
            state.winner = Some(winner);

        } 
        if is_full(&state.table) {
            state.tie = Some(Tied::Yes);
        }
    }
}

fn is_full(table: &[[Option<Player>; 7]; 6]) -> bool {
    for j in 0..=6{//move horizontally
        if table[0][j] == None { //go through the top row and find if any spot is open
            return false
        }
    }
    true
}

fn check_winner(table: &[[Option<Player>; 7];6]) -> Option<Player> {
    let mut winner: Option<Player> = None;
    //horizontal check
    
    for i in 0..=3{
        for j in 0..=5{
            if table[j][i]== table[j][i+1] && table[j][i]== table[j][i+2] && table[j][i]== table[j][i+3] {
                if table[j][i] != None {
                    winner = table[j][i]
                }
            }
        }
    }

    //vertical check

    for i in 0..=2{
        for j in 0..=6{
            if table[i][j]== table[i+1][j] && table[i][j]== table[i+2][j] && table[i][j]== table[i+3][j] {
                if table[i][j] != None {
                    winner = table[i][j]
                }
            }
        }
    }

    //ascending diagonal check

    for i in 3..=5{
        for j in 0..=3{
            if table[i][j]== table[i-1][j+1] && table[i][j]== table[i-2][j+2] && table[i][j]== table[i-3][j+3] {
                if table[i][j] != None {
                    winner = table[i][j]
                }
            }
        }
    }

    //descending diagonal check

    for i in 3..=5{
        for j in 3..=6{
            if table[i][j]== table[i-1][j-1] && table[i][j]== table[i-2][j-2] && table[i][j]== table[i-3][j-3] {
                if table[i][j] != None {
                    winner = table[i][j]
                }
            }
        }
    }
    
    winner
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::new(0.68, 0.84, 0.90, 0.1));

    let x = MARGIN;
    let y = MARGIN;
    let width = WIDTH - MARGIN * 2.0;
    let height = HEIGHT - MARGIN * 2.0;

    let tile_width = width / 7.0;
    let tile_height = height / 6.0;

    // draw "who is playing"
    let size = vec2(tile_width, tile_height);
    draw_text(
        &mut draw,
        &state.font,
        size,
        state.turn,
        "Playing: ",
        24.0,
        vec2(300.0, MARGIN * 0.5),
        1.0,
    );

    // drawing board
    draw.rect((x, y), (width, height))
        .stroke_color(Color::BLUE)
        .stroke(6.0);

    for index in 0..7 {
            draw.line(
                (x + tile_width * index as f32, y),
                (x + tile_width * index as f32, y + height),
            )
            .width(2.0).color(Color::BLUE);
            draw.line(
                (x, y + tile_height * index as f32),
                (x + width, y + tile_height * index as f32),
            )
            .width(2.0).color(Color::BLUE);
    // }      
    }

    // // drawing pieces
    state.table.iter().enumerate().for_each(|(i, p)| {
        let pos_y = i as f32 * size.y + y *1.8;

        for col_i in 0..7 {
            let pos_x = col_i as f32 * size.x + x*1.72;

            match p[col_i] {
                Some(Player::Red) => draw_red(&mut draw, Vec2{x: size.x * 1.75, y: size.y * 1.75}, vec2(pos_x, pos_y)),
                Some(Player::Yellow) =>  draw_yellow(&mut draw, Vec2{x: size.x * 1.75, y: size.y * 1.75}, vec2(pos_x, pos_y)),
                None => {}
            }
        }
    });



    // draw final menu
    if let Some(winner) = state.winner {
        draw.rect((0.0, 0.0), (WIDTH, HEIGHT))
            .color(Color::GRAY)
            .alpha(0.8);

        let (text, x_offet) = ("Winner: ", size.x * 0.3);

        draw_text(
            &mut draw,
            &state.font,
            size,
            winner,
            text,
            48.0,
            vec2(WIDTH * 0.5 - x_offet, HEIGHT * 0.5),
            0.6,
        );

        draw.text(&state.font, "Press SPACE to reset")
            .position(WIDTH * 0.5, HEIGHT * 0.75)
            .size(32.0)
            .h_align_center()
            .v_align_middle()
            .color(Color::BLACK);
    }

    if let Some(_tie) = &state.tie {
        draw.rect((0.0, 0.0), (WIDTH, HEIGHT))
            .color(Color::GRAY)
            .alpha(0.8);


        draw.text(&state.font, "Tie")
            .position(WIDTH * 0.5, HEIGHT * 0.5)
            .size(48.0)
            .h_align_center()
            .v_align_middle()
            .color(Color::BLACK);


        draw.text(&state.font, "Press SPACE to reset")
            .position(WIDTH * 0.5, HEIGHT * 0.75)
            .size(32.0)
            .h_align_center()
            .v_align_middle()
            .color(Color::BLACK);
    }

    gfx.render(&draw);
}

#[allow(clippy::too_many_arguments)]
fn draw_text(
    draw: &mut Draw,
    font: &Font,
    size: Vec2,
    player: Player,
    text: &str,
    font_size: f32,
    pos: Vec2,
    scale: f32,
) {
    // drawing text
    draw.text(font, text)
        .color(Color::BLACK)
        .size(font_size)
        .v_align_middle()
        .h_align_center()
        .position(pos.x, pos.y);

    let bounds = draw.last_text_bounds();

    let pos = vec2(bounds.max_x() + 30.0, bounds.center_y());
    let mm = Mat3::from_translation(pos)
        * Mat3::from_scale(Vec2::splat(scale))
        * Mat3::from_translation(-pos);
    draw.transform().push(mm);
    match player {
        Player::Red => draw_red(draw, size, pos),
        Player::Yellow => draw_yellow(draw, size, pos),
    }
    draw.transform().pop();
}

fn draw_red(draw: &mut Draw, size: Vec2, pos: Vec2) {
    let radius = size.x/4.0 ;
    draw.circle(radius)
        .position(pos.x, pos.y)
        .fill_color(Color::RED)
        .fill();
}

fn draw_yellow(draw: &mut Draw, size: Vec2, pos: Vec2) {
    let radius = size.x/4.0;
    draw.circle(radius)
        .position(pos.x, pos.y)
        .fill_color(Color::YELLOW)
        .fill();
}

fn index_from_pos(x: usize) -> usize {
    x
}