#![allow(unused_imports)]
use engine::system::System;
use serial_test::serial;

use crate::simple_data;

#[test]
#[serial]
#[should_panic(expected = "No main menu game has been defined")]
fn no_main_menu() {
    let mut system = System::new("Test No Main", 1);

    system.init();

    let core = &mut system.core;

    let main_game = core
        .game_registry
        .get_main_menu_gameid()
        .expect("No main menu game has been defined"); // required text for test to fail/succeed
    core.set_game(main_game);

    core.go();

    core.deinit();
}

#[test]
#[serial]
fn all_but_go() {
    let mut system = System::new("Test No Run", 1);

    system.init();

    let core = &mut system.core;

    let _main_game_id = core.register_game(
        "Main Menu",
        true,
        Box::new(simple_data::SimpleOneFrameGame {}),
    );

    let main_game = core
        .game_registry
        .get_main_menu_gameid()
        .expect("No main menu game has been defined");
    core.set_game(main_game);

    // core.go();

    core.deinit();
}

#[test]
#[serial]
fn base_main_menu() {
    let mut system = System::new("Test One Frame", 1);

    system.init();

    let core = &mut system.core;

    let _main_game_id = core.register_game(
        "Main Menu",
        true,
        Box::new(simple_data::SimpleOneFrameGame {}),
    );

    let main_game = core
        .game_registry
        .get_main_menu_gameid()
        .expect("No main menu game has been defined");
    core.set_game(main_game);

    core.go();

    core.deinit();
}
