
#[cfg(test)]
mod tests {

    use parchis::*;

    #[test]
    fn board_creation() {

        let mut b: Board = Board::new();
        assert_eq!(0, b.players.len());
        println!("{:#?}", b);

        let p: Player = Player::new(String::from("Green"));
        assert_eq!("Green", p.name);
        println!("{:#?}", p);

        b.add_player(p);
        assert_eq!(1, b.players.len());
        println!("{:#?}", b);

    }
}

pub mod parchis {

#[derive(Debug)]
pub struct Board {
    pub players: Vec<Player>,
}

impl Board {

    pub fn new() -> Board {
        Board {
            players: vec![],
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }
}



#[derive(Debug)]
pub struct Player {
    pub name: String,
}

impl Player {

    pub fn new(name: String) -> Player {
        Player {
            name
        }
    }
}

}
