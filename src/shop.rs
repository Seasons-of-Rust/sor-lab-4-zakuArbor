use std::fmt;

use crate::{card::Card, FightResult};

pub struct Shop {
    pub cards: Vec<Card>,
}

impl Shop {
    /// Get the price of the most expensive card in the shop
    pub fn most_expensive(&self) -> u32 {
        self.cards.iter().map(|card| card.price).max().unwrap()
    }

    /// Get the total damage of all cards in the shop
    pub fn total_damage(&self) -> u32 {
        self.cards.iter().map(|card| card.damage).sum()
    }

    /// Get the total health of all cards in the shop
    pub fn total_health(&self) -> u32 {
        self.cards.iter().map(|card| card.health).sum()
    }

    /// Simulate a fight against another store. Returns a FightResult::Win if
    /// this store wins, FightResult::Loss if this store loses, and a
    /// FightResult::Tie if both stores win the same number of battles.
    pub fn fight_store(&self, other: &Shop) -> FightResult {
        let mut shop1: u64 = 0;
        let mut shop2: u64 = 0;

        for card1 in self.cards.iter() {
            for card2 in other.cards.iter() {
                match card1.fight(card2) {
                    FightResult::Win => shop1 += 1,
                    FightResult::Loss => shop2 += 1,
                    _ => (),
                }
            }
        }

        match (shop1 > shop2, shop1 == shop2) {
            //why can I not use if statement instead i.e. cargo clippy complains (warning)
            (true, _) => FightResult::Win,
            (false, false) => FightResult::Loss,
            (false, true) => FightResult::Tie,
        }
        /*
        //Preferred Method
        if shop1 > shop2 {
            return FightResult::Win;
        } else if shop1 < shop2 {
            return FightResult::Loss;
        }
        FightResult::Tie
        */
    }
}

// Implement the Display trait for Shop so that it can be printed. Print the
// shop's stats, including the most expensive card, the total damage, and the
// total health.
impl fmt::Display for Shop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "|Shop: {}/{}/{}|",
            self.most_expensive(),
            self.total_damage(),
            self.total_health()
        )
    }
}
