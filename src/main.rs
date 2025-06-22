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
use agb::display::object::SpriteVram;
use agb::fixnum::vec2;
use agb::rng;

include_aseprite!(
            mod sprites,
            "gfx/snake_sprites.aseprite"
        );

pub fn game1(mut gba: agb::Gba) -> !
{
    //gba hardware access
    let mut gfx = gba.graphics.get();
    let mut input = agb::input::ButtonController::new();

    //sprite list
    let snake_head_v = SpriteVram::from(sprites::SNAKE_HEAD_VERTICAL.sprite(0));
    let snake_head_h = SpriteVram::from(sprites::SNAKE_HEAD_HORIZONTAL.sprite(0));
    let snake_body_straight_v = SpriteVram::from(sprites::SNAKE_BODY_STRAIGHT_VERTICAL.sprite(0));
    let snake_body_straight_h = SpriteVram::from(sprites::SNAKE_BODY_STRAIGHT_HORIZONTAL.sprite(0));
    let snake_body_turn = SpriteVram::from(sprites::SNAKE_BODY_TURN.sprite(0));
    let snake_body_end_v = SpriteVram::from(sprites::SNAKE_BODY_END_VERTICAL.sprite(0));
    let snake_body_end_h = SpriteVram::from(sprites::SNAKE_BODY_END_HORIZONTAL.sprite(0));

    //snake starting values
    let mut head_position_x = 112;
    let mut head_position_y = 64;
    let mut body_positions_x = [0; 100];
    let mut body_positions_y = [0; 100];
    body_positions_x[0] = 112;
    body_positions_x[1] = 112;
    body_positions_y[0] = 80;
    body_positions_y[1] = 96;
    let mut snake_length = 2;

    //starting movement
    let mut movement:[i32; 2] = [0, -16];
    let mut horizontal = false;

    //snake objects
    let mut snake_head = Object::new(snake_head_v.clone());
    let mut body = core::array::from_fn::<_, 119, _>(|_| Object::new(snake_body_straight_v.clone()));

    //set snake default position
    snake_head.set_pos((head_position_x, head_position_y));
    body[1].set_sprite(snake_body_end_v.clone());
    body[0].set_pos((body_positions_x[0], body_positions_y[0]));
    body[1].set_pos((body_positions_x[1], body_positions_y[1]));

    //snake body orientations
    enum BodySprites {
        StraightVertical,
        StraightHorizontal,
        Turn
    }


    let mut snake_body_type = core::array::from_fn::<_, 119, _>(|_| BodySprites::StraightVertical);

    let apple = SpriteVram::from(sprites::APPLE.sprite(0));
    let banana = SpriteVram::from(sprites::BANANA.sprite(0));
    let cherry = SpriteVram::from(sprites::CHERRY.sprite(0));
    let kiwi = SpriteVram::from(sprites::KIWI.sprite(0));
    let orange = SpriteVram::from(sprites::ORANGE.sprite(0));
    let lemon = SpriteVram::from(sprites::LEMON.sprite(0));



    let fruit_list = [apple, banana, cherry, kiwi, orange, lemon];

    let mut fruit_amount = 0;
    let mut fruits = core::array::from_fn::<_, 8, _>(|_| Object::new(fruit_list[0].clone()));

    let mut wait = 0;

    loop {
        input.update();
        if wait == 15 { //4 times per second
            let turn; // player input

            //determine head direction and movement
            match horizontal {
                false => {
                    turn = input.x_tri() as i32;
                    if turn != 0 {
                        snake_body_type[0] = BodySprites::Turn;
                        horizontal = !horizontal;
                        movement = [movement[1].abs() * turn, movement[0].abs() * turn];
                        snake_head.set_sprite(snake_head_h.clone());
                        snake_head.set_hflip(turn > 0);
                        snake_head.set_vflip(!snake_head.vflip());
                    }
                    else { snake_body_type[0] = BodySprites::StraightVertical}
                },
                true => {
                    turn = input.y_tri() as i32;
                    if turn != 0 {
                        snake_body_type[0] = BodySprites::Turn;
                        horizontal = !horizontal;
                        movement = [movement[1].abs() * turn, movement[0].abs() * turn];
                        snake_head.set_sprite(snake_head_v.clone());
                        snake_head.set_vflip(turn > 0);
                        snake_head.set_hflip(!snake_head.hflip());
                    }
                    else { snake_body_type[0] = BodySprites::StraightHorizontal}
                },
            };
            head_position_x = head_position_x + movement[0];
            head_position_y = head_position_y + movement[1];

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


            for pos in (0..snake_length).rev() {

                if pos == 0 { //first body part
                    let new_sprite;
                    match snake_body_type[pos] {
                        BodySprites::StraightVertical => {
                            new_sprite = snake_body_straight_v.clone();
                            snake_body_type[pos +1] = BodySprites::StraightVertical;
                            body[pos].set_hflip(snake_head.hflip());
                            body[pos].set_vflip(snake_head.vflip());
                        },
                        BodySprites::StraightHorizontal => {
                            new_sprite = snake_body_straight_h.clone();
                            snake_body_type[pos +1] = BodySprites::StraightHorizontal;
                            body[pos].set_hflip(snake_head.hflip());
                            body[pos].set_vflip(snake_head.vflip());
                        },
                        BodySprites::Turn => {
                            new_sprite = snake_body_turn.clone();
                            snake_body_type[pos +1] = BodySprites::Turn;
                            body[pos].set_hflip(snake_head.hflip());
                            body[pos].set_vflip(!snake_head.vflip());
                        }
                    };
                    body[pos].set_sprite(new_sprite);
                    body[pos].set_pos(snake_head.pos());
                }

                else if pos == snake_length -1 { //tail
                    let new_sprite;
                    match snake_body_type[pos] {
                        BodySprites::StraightVertical => {
                            new_sprite = snake_body_end_v.clone();
                            body[pos].set_hflip(!body[pos].hflip());
                            body[pos].set_vflip(body[pos -1].vflip());
                        },
                        BodySprites::StraightHorizontal => {
                            new_sprite = snake_body_end_h.clone();
                            body[pos].set_hflip(body[pos -1].hflip());
                            body[pos].set_vflip(!body[pos].vflip());
                        },
                        BodySprites::Turn => {
                            if pos == 1 && body[pos -1].pos().x == snake_head.pos().x{
                                new_sprite = snake_body_end_v.clone();
                                body[pos].set_hflip(!body[pos].hflip());
                                body[pos].set_vflip(!body[pos -1].vflip());
                            }
                            else if pos > 1 && body[pos -1].pos().x == body[pos -2].pos().x {
                                new_sprite = snake_body_end_v.clone();
                                body[pos].set_hflip(!body[pos].hflip());
                                body[pos].set_vflip(!body[pos -1].vflip());
                            }
                            else {
                                new_sprite = snake_body_end_h.clone();
                                body[pos].set_hflip(body[pos -1].hflip());
                                body[pos].set_vflip(!body[pos].vflip());
                            }
                        }
                    };
                    body[pos].set_sprite(new_sprite);
                    body[pos].set_pos(body[pos -1].pos());
                }

                else { //other body parts
                    let new_sprite;
                    match snake_body_type[pos] {
                        BodySprites::StraightVertical => {
                            new_sprite = snake_body_straight_v.clone();
                            snake_body_type[pos +1] = BodySprites::StraightVertical;
                        },
                        BodySprites::StraightHorizontal => {
                            new_sprite = snake_body_straight_h.clone();
                            snake_body_type[pos +1] = BodySprites::StraightHorizontal;
                        },
                        BodySprites::Turn => {
                            new_sprite = snake_body_turn.clone();
                            snake_body_type[pos +1] = BodySprites::Turn;
                        }
                    };
                    body[pos].set_sprite(new_sprite);
                    body[pos].set_pos(body[pos -1].pos());
                    body[pos].set_hflip(body[pos -1].hflip());
                    body[pos].set_vflip(body[pos -1].vflip());

                }

            }


            snake_head.set_pos((head_position_x, head_position_y));

            if fruit_amount == 0 {
                fruit_amount = rng::next_i32().rem_euclid(8);
                agb::println!("{fruit_amount}");
                for fruit in 0..fruit_amount {
                    fruits[fruit as usize].set_sprite(fruit_list[rng::next_i32().rem_euclid(5) as usize].clone());
                    let mut search_free_pos = true;
                    while search_free_pos {
                        agb::println!("still searching");
                        let xpos = rng::next_i32().rem_euclid(14) * 16;
                        let ypos = rng::next_i32().rem_euclid(9) * 16;
                        let mut free = true;
                        if snake_head.pos() != vec2(xpos, ypos) {
                            for pos in 0..snake_length {
                                if body[pos].pos() == vec2(xpos, ypos) {
                                    free = false;
                                    break;
                                };
                            }
                            if free == true {
                                fruits[fruit as usize].set_pos(vec2(xpos, ypos));
                                search_free_pos = false;
                            }
                        }
                    }
                }

            }
            wait = 0;
        }
        else {
            wait += 1;
        }


        let mut frame = gfx.frame();

        snake_head.show(&mut frame);
        for pos in 0..snake_length {
            body[pos].show(&mut frame);
        }

        for fruit in 0..fruit_amount {
            fruits[fruit as usize].show(&mut frame);
        }



        frame.commit();

        //make rng harder to predict
        let _ = rng::next_i32();
    }
}