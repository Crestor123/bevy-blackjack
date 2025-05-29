use std::fmt;
use rand::rng;
use rand::seq::SliceRandom;
use bevy::prelude::*;

pub enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Suit::Spade => write!(f, "Spade"),
            Suit::Heart => write!(f, "Heart"),
            Suit::Diamond => write!(f, "Diamond"),
            Suit::Club => write!(f, "Club"),
            _ => write!(f, "Error")
        }
    }
}

#[derive(Component)]
pub struct Card{
    pub suit: Suit,
    pub value: i32
}

#[derive(Component)]
pub struct Deck(pub Vec<Entity>);

#[derive(Component)]
pub struct Dealer;

pub fn create_dealer_deck(mut cmd: Commands) {
    //Creates a deck and populates it with the standard 52 cards
    let mut deck = Vec::new();
    let mut rng = rng();

    for i in 1..14 {
        let spade = cmd.spawn(Card{suit: Suit::Spade, value: i}).id();
        let heart = cmd.spawn(Card{suit: Suit::Heart, value: i}).id();
        let diamond = cmd.spawn(Card{suit: Suit::Diamond, value: i}).id();
        let club = cmd.spawn(Card{suit: Suit::Club, value: i}).id();

        deck.push(spade);
        deck.push(heart);
        deck.push(diamond);
        deck.push(club);
    }

    deck.shuffle(&mut rng);
    cmd.spawn((Dealer, Deck(deck)));
}

pub fn draw_card(
    dealer: &mut Deck, 
    deck: &mut Deck,
) {
    println!("Drawing card\n");
    let card = dealer.0.pop();
    deck.0.push(card.unwrap());
}