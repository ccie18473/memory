use crate::prelude::*;

pub struct TGame {
    pub deck: TDeck,
    pub selected: usize,
    pub tile1: usize,
    pub tile2: usize,
    pub duration: i32,
    pub moves: usize,
    pub counter: usize,
    pub eog_flag: bool,
}

impl TGame {
    pub fn new_game() -> Self {
        Self {
            deck: TDeck::new_shuffled_deck(),
            selected: 0,
            tile1: 0,
            tile2: 0,
            duration: 0,
            moves: 0,
            counter: 0,
            eog_flag: false,
        }
    }
    pub fn play(&mut self, tile: usize) {
        if tile < 52 {
            if self.selected == 0 && !self.deck.cards[tile].selected {
                self.tile1 = tile;
                self.deck.cards[tile].image =
                    self.deck.cards[tile].value + self.deck.cards[tile].suit * 13;
                self.selected += 1;
                self.deck.cards[tile].selected = true;
            } else if self.selected == 1 && !self.deck.cards[tile].selected {
                self.tile2 = tile;
                self.deck.cards[tile].image =
                    self.deck.cards[tile].value + self.deck.cards[tile].suit * 13;
                self.selected += 1;
                self.deck.cards[tile].selected = true;
            }
        }
    }
    pub fn win_check(&mut self) {
        if self.selected == 2 {
            let index1 = self.deck.cards[self.tile1].value + self.deck.cards[self.tile1].suit * 13;
            let index2 = self.deck.cards[self.tile2].value + self.deck.cards[self.tile2].suit * 13;
            if (index1 as isize - index2 as isize).abs() == 26 {
                self.deck.cards[self.tile1].image = FERRIS_IMG;
                self.deck.cards[self.tile2].image = FERRIS_IMG;
                self.tile1 = 0;
                self.tile2 = 0;
                self.selected = 0;
                self.counter += 1;
            } else {
                self.deck.cards[self.tile1].image = RUST_IMG;
                self.deck.cards[self.tile2].image = RUST_IMG;
                self.deck.cards[self.tile1].selected = false;
                self.deck.cards[self.tile2].selected = false;
                self.tile1 = 0;
                self.tile2 = 0;
                self.selected = 0;
            }
            self.moves += 1;
        }
        if self.counter == 26 {
            self.eog_flag = true;
        }
    }
}
