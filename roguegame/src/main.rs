#![allow(unused)]

use rltk::{GameState, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;
//use specs_derive::Component;
use std::cmp::{max, min};

struct Position {
    x: i32,
    y: i32,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

struct Renderbisa {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}

impl Component for Renderbisa {
    type Storage = VecStorage<Self>;
}

struct State {
    ecs: World,
}

impl GameState for State {
    fn tick(&mut self, ct: &mut Rltk) {
        ct.cls();
        let positions = self.ecs.read_storage::<Position>();
        let render = self.ecs.read_storage::<Renderbisa>();

        for (pos, ren) in (&positions, &render).join() {
            ct.set(pos.x, pos.y, ren.fg, ren.bg, ren.glyph)
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let ctx = RltkBuilder::simple80x50()
        .with_title("Rogue Game")
        .build()?;
    let mut gs = State { ecs: World::new() }; //zaa warudo

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderbisa>();
    gs.ecs
        .create_entity()
        .with(Position { x: 20, y: 20 })
        .with(Renderbisa {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
        })
        .build();

        for i in 0..=7 {
            gs.ecs.create_entity().with(Position {x: 9 * i, y: 5 * i}).with(Renderbisa {glyph: rltk::to_cp437('A'), 
        fg: RGB::named(rltk::BLUE), bg: RGB::named(rltk::BLACK)}).build();
        }
    rltk::main_loop(ctx, gs)
}
