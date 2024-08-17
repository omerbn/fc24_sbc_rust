use glob::glob;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

use crate::player::Player;
use crate::println_with_timestamp;


pub struct LocalStore<'a> {
    pub players: &'a Vec<Player>,
    name_to_player: HashMap<String, &'a Player>,
}

impl<'a> LocalStore<'a> {
    pub fn get_player_by_name(&self, name: &str) -> Option<&Player> {
        match self.name_to_player.get(name) {
            Some(player) => Some(*player),
            None => None,
        }
    }

    pub fn create_slice(&self, si: usize, ei: usize) -> &'a [Player] {
        &self.players[si..ei]
    }

    fn init_name_to_player(&mut self) {
        for player in self.players.iter() {
            self.name_to_player
                .insert(player.__fullname.clone(), player);
            if player.__nickname != "" {
                self.name_to_player
                    .insert(player.__nickname.clone(), player);
            }
        }
    }
}

pub struct GlobalStore {
    players: Vec<Player>,
    meta_data: HashMap<u64, PlayersMetaData>,
}

impl GlobalStore {
    pub fn new() -> Self {
        let mut store = GlobalStore {
            players: Vec::new(),
            meta_data: HashMap::new(),
        };
        store.load().unwrap();
        store
    }

    pub fn get_local_store(&self) -> LocalStore {
        let mut lstore = LocalStore {
            players: &self.players,
            name_to_player: HashMap::new(),
        };
        lstore.init_name_to_player();
        lstore
    }

    fn add_player(&mut self, mut player: Player) {
        let hash_value = self.meta_data.get(&player.assetId).unwrap();

        player.__fullname = format!("{} {}", hash_value.f, hash_value.l);
        player.__name = player.__fullname.clone();
        if hash_value.c != "" {
            player.__nickname = hash_value.c.clone();
            player.__name = player.__nickname.clone();
        }
        self.players.push(player);
    }

    fn load_players_file(&mut self, filename: &str) -> Result<(), Box<dyn Error>> {
        let json: ClubJsonFormat = crate::utils::load_json_file(filename)?;
        for player in json.itemData.iter() {
            self.add_player(player.clone());
        }
        Ok(())
    }

    fn load_all_players(&mut self) -> Result<(), Box<dyn Error>> {
        // Use glob to match files with the pattern
        for entry in glob("./data_files/club*.json")? {
            match entry {
                Ok(path) => {
                    match path.to_str() {
                        Some(path_str) => {
                            self.load_players_file(path_str)?;
                        }
                        None => (),
                    }
                    // println!("loading file: {:?}", path);
                }
                Err(e) => eprintln!("No player files to load in:{:?}", e),
            }
        }

        Ok(())
    }

    fn load_meta_data(&mut self) -> Result<(), Box<dyn Error>> {
        let json: PlayersJsonFormat = crate::utils::load_json_file("./data_files/players.json")?;
        for meta in json.LegendsPlayers.iter() {
            self.meta_data.insert(meta.id, meta.clone());
        }
        for meta in json.Players.iter() {
            self.meta_data.insert(meta.id, meta.clone());
        }

        Ok(())
    }

    fn load(&mut self) -> Result<(), Box<dyn Error>> {
        println_with_timestamp!("Loading global store:");
        print!("Loading meta data... ");
        self.load_meta_data()?;
        println!("Done");
        print!("Loeading players... ");
        self.load_all_players()?;
        self.players.sort_by(|a, b| a.rating.cmp(&b.rating)); // sorting by rating ascending
        println!("Done: {} players", self.players.len());
        println_with_timestamp!("Finished loading global store");
        Ok(())
    }
}

#[derive(Debug, Deserialize, Clone)]
struct ClubJsonFormat {
    itemData: Vec<Player>,
}

#[derive(Debug, Deserialize, Clone)]
struct PlayersJsonFormat {
    LegendsPlayers: Vec<PlayersMetaData>,
    Players: Vec<PlayersMetaData>,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
struct PlayersMetaData {
    #[serde(default)]
    c: String,

    f: String,
    id: u64,
    l: String,
    r: u8,
}
