use game::{State, World};
use raylib::prelude::*;

pub mod game;

const SIZE: usize = 80;
const SCREEN: i32 = 800;
const PIXEL: i32 = 10;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN, SCREEN)
        .title("Conways Game of Life - Mikel Apeseguia")
        .build();
    let mut w = World::new(SIZE);
    w.randomize();

    rl.set_target_fps(3);

    while !rl.window_should_close() {
        w.update();
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        for i in 1..SIZE - 1 {
            for j in 1..SIZE - 1 {
                if w.cells[i][j].state == State::Alive {
                    d.draw_rectangle(
                        (i as i32 * PIXEL) as i32,
                        (j as i32 * PIXEL) as i32,
                        PIXEL as i32,
                        PIXEL as i32,
                        Color::WHITE,
                    );
                }
            }
        }

        d.draw_text(&format!("Alive: {}", w.alive), 12, 12, 12, Color::GREEN);
        d.draw_text(
            &format!("Generation: {}", w.generation),
            12,
            25,
            12,
            Color::GREEN,
        );
    }
}
