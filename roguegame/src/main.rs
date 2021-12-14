// #![allow(unused)]

// use rltk::{GameState, Rltk, VirtualKeyCode, RGB};
// use specs::prelude::*;
// //use specs_derive::Component;
// use std::cmp::{max, min};

// struct Position {
//     x: i32,
//     y: i32,
// }

// impl Component for Position {
//     type Storage = VecStorage<Self>;
// }

// struct Renderbisa {
//     glyph: rltk::FontCharType,
//     fg: RGB,
//     bg: RGB,
// }

// impl Component for Renderbisa {
//     type Storage = VecStorage<Self>;
// }

// struct RightMove {}
// impl Component for RightMove {
//     type Storage = VecStorage<Self>;
// }

// struct State {
//     ecs: World,
// }

// impl GameState for State {
//     fn tick(&mut self, ct: &mut Rltk) {
//         ct.cls();
//         self.systems_run();
//         let positions = self.ecs.read_storage::<Position>();
//         let render = self.ecs.read_storage::<Renderbisa>();

//         for (pos, ren) in (&positions, &render).join() {
//             ct.set(pos.x, pos.y, ren.fg, ren.bg, ren.glyph)
//         }
//     }
// }
// struct RightWalk {}
// impl<'a> System<'a> for RightWalk {
//     type SystemData = (ReadStorage<'a, RightMove>, WriteStorage<'a, Position>);

//     fn run(&mut self, (right, mut pos): Self::SystemData) {
//         for (_right, pos) in (&right, &mut pos).join() {
//             pos.x += 1;
//             if pos.x > 79 {
//                 pos.x = 0;
//             }
//         }
//     }
// }
// impl State {
//     fn systems_run(&mut self) {
//         let mut right = RightWalk {};
//         right.run_now(&self.ecs);
//         self.ecs.maintain();
//     }
// }
// fn main() -> rltk::BError {
//     use rltk::RltkBuilder;
//     let ctx = RltkBuilder::simple80x50()
//         .with_title("Rogue Game")
//         .build()?;
//     let mut gs = State { ecs: World::new() }; //zaa warudo

//     gs.ecs.register::<Position>();
//     gs.ecs.register::<Renderbisa>();
//     gs.ecs.register::<RightMove>();

//     gs.ecs
//         .create_entity()
//         .with(Position { x: 20, y: 20 })
//         .with(Renderbisa {
//             glyph: rltk::to_cp437('@'),
//             fg: RGB::named(rltk::RED),
//             bg: RGB::named(rltk::BLACK),
//         })
//         .build();

//     for i in 0..=7 {
//         gs.ecs
//             .create_entity()
//             .with(Position { x: 9 * i, y: 5 * i })
//             .with(Renderbisa {
//                 glyph: rltk::to_cp437('A'),
//                 fg: RGB::named(rltk::BLUE),
//                 bg: RGB::named(rltk::BLACK),
//             })
//             .with(RightMove {})
//             .build();
//     }
//     rltk::main_loop(ctx, gs)
// }
use rltk::{GameState, Rltk};

struct State {}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        ctx.print(1, 1, "Hello Rust World");
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let gs = State {};
    rltk::main_loop(context, gs)
}
