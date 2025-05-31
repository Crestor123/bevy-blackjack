mod card;

use std::io;
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum GameState {
    #[default]
    Setup,
    Input,
    Checking,
    GameOver,
}

#[derive(Component)]
struct Score(i32);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Computer;

#[derive(Component)]
struct Bust(bool);


#[derive(Resource)]
struct PlayerCount(usize);


//Creates the players as a bundle including a player tag, a score, and a deck
//Executed on startup, and directs to the Input state
fn create_players(
    mut cmd: Commands,
    mut next_state: ResMut<NextState<GameState>>
) {
    cmd.spawn((Player, Bust(false), Score(0), card::Deck(Vec::new())));
    cmd.spawn((Computer, Bust(false), Score(0), card::Deck(Vec::new())));

    next_state.set(GameState::Input);
}

//Queries for decks with scores, and cards
//Adds up each player's total score, and checks for aces
fn tally_score(
    mut query: Query<(&card::Deck, &mut Score, &mut Bust)>, 
    qcard: Query<&card::Card>, 
) {
    for (deck, mut score, mut bust) in query.iter_mut() {
        score.0 = 0;
        let mut aces: i32 = 0;

        for card in &deck.0 {
            let  c = qcard.get(*card);
            match c {
                Ok(k) => {
                    match k.value {
                        1 => aces += 1,
                        2..=10 => score.0 += k.value,
                        11..=13 => score.0 += 10,
                        _ => println!("Error")
                    }
                },
                Err(_) => println!("Error")
            }
        }

        //Ace check
        if aces != 0 {
            for _ in 0..aces {
                if score.0 <= 10 {
                    score.0 += 11;
                }
                else { score.0 += 1 }
            }
        }

        //Check if the score is over 21; if so, the player busts
        if score.0 > 21 {
            println!("Score is over 21");
            bust.0 = true
        }
    }
}

//Checks if the player has busted
//Changes state to game over if so
fn check_score(
    qplayer: Query<&Bust, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>
) {
    let bust = qplayer.single();

    if bust.0 {
        next_state.set(GameState::GameOver)
    }
}

//Queries for the player
//Prints out each card in the player's hand
fn look_at_cards(
    qdeck: Query<&card::Deck, With<Player>>, 
    qcard: Query<&card::Card>,
) {
    for deck in qdeck.iter() {
            for card in &deck.0 {
            let c = qcard.get(*card);
            match c {
                //Ok(k) => println!("{} {}", k.suit, k.value),
                Ok(k) => print!("{} ", k),
                Err(_) => println!("Error"),
            }
        }
    }
    println!();
}

//Prints out the player's score
//Set the state to Input
fn print_score(
    qscore: Query<&Score, With<Player>>, 
    mut next_state: ResMut<NextState<GameState>>
) {
    let score = qscore.single();
    println!("Score: {}", score.0);

    next_state.set(GameState::Input);
}

//Queries for the dealer and the player
//Gets the player input, and changes state based on that input
fn get_input(
    mut qdealer: Query<&mut card::Deck, With<card::Dealer>>,
    mut qplayer: Query<&mut card::Deck, (With<Player>, Without<card::Dealer>)>,
    mut next_state: ResMut<NextState<GameState>>
) {
    let mut input = String::new();
    let dealer = qdealer.single_mut().into_inner();
    let player = qplayer.single_mut().into_inner();

    println!("Enter 'draw' to draw a card");
    println!("Enter 'stand' to end the game");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("You input: {}", input);
    match input.as_str().trim() {
        "draw" => {
            card::draw_card(dealer, player);
             next_state.set(GameState::Checking);
        }
        "stand" => next_state.set(GameState::GameOver),
        _ => println!("Error")
    }   
}

//Runs when entering the game over state
fn game_over(
    qplayer: Query<(&Score, &Bust), With<Player>>
) {
    //print final score
    //If you busted, you lose
    let (score, bust) = qplayer.single();
    println!("\nFinal Score: {}", score.0);
    if bust.0 {
        println!("You lose!");
    }
    else { println!("You win!") }
}

//Closes the application
fn end_game(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit::Success);
}

fn main() {
    App::new()
        .add_plugins((StatesPlugin, MinimalPlugins))
        .init_state::<GameState>()
        .add_systems(Startup, (card::create_dealer_deck, create_players).chain())
        .add_systems(OnEnter(GameState::Input), get_input)
        .add_systems(OnEnter(GameState::Checking), (tally_score, look_at_cards, print_score, check_score).chain())
        .add_systems(OnEnter(GameState::GameOver), (game_over, end_game).chain())
        .run();
}