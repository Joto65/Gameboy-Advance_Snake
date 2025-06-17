// Games made using `agb` are no_std which means you don't have access to the standard
// rust library. This is because the game boy advance doesn't really have an operating
// system, so most of the content of the standard library doesn't apply.
#![no_std]
// `agb` defines its own `main` function, so you must declare your game's main function
// using the #[agb::entry] proc macro. Failing to do so will cause failure in linking
// which won't be a particularly clear error message.
#![no_main]
// This is required to allow writing tests
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

// By default no_std crates don't get alloc, so you won't be able to use things like Vec
// until you declare the extern crate. `agb` provides an allocator so it will all work
extern crate alloc;

// The main function must take 1 arguments and never returns, and must be marked with
// the #[agb::entry] macro.
#[agb::entry]
fn main(gba: agb::Gba) -> ! {
    game1(gba);
}

use agb::include_aseprite;
use agb::display::object::Object;

include_aseprite!(
    mod sprites,
    "gfx/snake_sprites.aseprite"
);

pub fn game1(mut gba: agb::Gba) -> !
{
    let mut gfx = gba.graphics.get();

    let mut snake_head = Object::new(sprites::SNAKE_HEAD.sprite(0));
    let mut snake_body_straight = Object::new(sprites::SNAKE_BODY_STRAIGHT.sprite(0));
    let mut snake_body_turn = Object::new(sprites::SNAKE_BODY_TURN.sprite(0));
    let mut snake_body_turn2 = Object::new(sprites::SNAKE_BODY_TURN.sprite(0));
    let mut snake_body_end_v = Object::new(sprites::SNAKE_BODY_END_VERTICAL.sprite(0));
    let mut snake_body_end_h = Object::new(sprites::SNAKE_BODY_END_HORIZONTAL.sprite(0));


    let mut head_position_x = 112;
    let mut head_position_y = 72;
    let mut body_positions_x = [0; 100];
    let mut body_positions_y = [0; 100];
    body_positions_x[0] = 112;
    body_positions_x[1] = 112;
    body_positions_y[0] = 88;
    body_positions_y[1] = 104;
    let mut snake_length = 3;
    let mut x_velocity = 0;
    let mut y_velocity = -16;

    let mut wait = 0;

    snake_head.set_pos((head_position_x, head_position_y));
    snake_body_straight.set_pos((body_positions_x[0], body_positions_y[0]));
    snake_body_end_v.set_pos((body_positions_x[1], body_positions_y[1]));

    loop {

        if wait == 15 {
            head_position_x = (head_position_x + x_velocity);
            head_position_y = (head_position_y + y_velocity);

            if head_position_x < 0 {
                head_position_x = agb::display::WIDTH - 16;
            } else if head_position_x > agb::display::WIDTH - 16 {
                head_position_x = 0;
            }
            if head_position_y < 0 {
                head_position_y = agb::display::HEIGHT - 16;
            } else if head_position_y > agb::display::HEIGHT - 16 {
                head_position_y = 0;
            }

            //temporary
            snake_body_end_v.set_hflip((!snake_body_end_v.hflip()));
            snake_body_end_v.set_pos(snake_body_straight.pos());
            snake_body_straight.set_pos(snake_head.pos());


            snake_head.set_pos((head_position_x, head_position_y));
            wait = 0;
        }
        else {
            wait += 1;
        }


        let mut frame = gfx.frame();

        snake_head.show(&mut frame);
        snake_body_straight.show(&mut frame);
        snake_body_end_v.show(&mut frame);

        frame.commit();

    }
}