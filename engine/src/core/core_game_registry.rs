use std::collections::HashMap;
use crate::core::core_game::*;


pub type GameId = usize;

#[derive(Default)]
pub struct CoreGameRegistry {
    main_menu: Option<GameId>,  // references self.games
    games: Vec<CoreGame>,   // or some sort of references
    game_names: HashMap<String, GameId>,   // references self.games
}


// NOTE: no way to remove entries
impl CoreGameRegistry {
    pub fn register_game ( &mut self, entry: CoreGame, main_game: bool ) -> GameId {
        // add game
        let game_num = self.games.len() as GameId;
        self.game_names.insert(entry.name.clone(), game_num);
        self.games.push(entry);

        // note its number, and use
        if main_game {
            assert!(self.main_menu.is_none(), "trying to assign new main menu game over existing one in registry");
            self.main_menu = Some(game_num);
        }

        game_num
    }

    pub fn get_game_name ( &self, game_num: GameId ) -> Option<&str> {
        if let Some(entry) = self.games.get(game_num) {
            Some(entry.name.as_str())
        } else { None }
    }

    pub fn get_main_menu_name ( &self ) -> Option<&str> {
        if let Some(game_num) = self.main_menu {
            self.get_game_name(game_num)
        } else { None }
    }

    // TODO: do we need to find the name from the game itself?

    pub fn get_main_menu_gameid ( &mut self ) -> Option<GameId> {
        self.main_menu
    }

    pub fn find_gameid ( &mut self, name: &str ) -> Option<GameId> {
        self.game_names.get(name).copied()
    }

    pub fn get_main_menu_game ( &mut self ) -> Option<&mut CoreGame> {
        if let Some(game_num) = self.main_menu {
            self.get_game(game_num)
        } else { None }
    }

    pub fn find_game ( &mut self, name: &str ) -> Option<&mut CoreGame> {
        if let Some(game_num) = self.game_names.get(name) {
            self.get_game(*game_num)
        } else { None }
    }

    pub fn get_game ( &mut self, game_num: GameId ) -> Option<&mut CoreGame> {
        if let Some(entry) = self.games.get_mut(game_num) {
            Some(entry)
        } else { None }
    }

    pub fn get_game_count ( &self ) -> usize {
        self.games.len()
    }
}