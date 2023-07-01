use core::num;
use std::{env, process::exit};
use rand::prelude::*;
use crate::player::player::Player;
use console::Term;


mod player;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4
    {
        println!("Incorrect arguments");
        println!("Program invocation: ./main [guess] [player1_nickname] [player2_nickname] ... [playern_nickname]");
        exit(0);
    }

    let guess: i32 = args[1].parse().unwrap();
    let mut players: Vec<Player> = Vec::new();

    for i in 2..args.len()
    {
        players.push(Player::new(args[i].clone(),0 ));
    }

    for player in &players 
    {
        println!("{}", player.get_nickname());

    }

    let number_of_players: usize = players.len();
    let start: usize = rand::thread_rng().gen_range(0..number_of_players);
    println!("{} starts!", players[start].get_nickname());
    let mut index: usize = start;
    let term = Term::stdout();
    println!("Numebr of players: {}", number_of_players);

    loop 
    {

        println!("{} press any key to roll the dice...", players[index].get_nickname());
        term.read_char().unwrap();
        let roll: usize = rand::thread_rng().gen_range(1..=6);
        println!("{} rolled {}", players[index].get_nickname(), roll.to_string());
        players[index].increment_score(roll.try_into().unwrap());
        println!("{}'s score: {}", players[index].get_nickname(), players[index].get_score());
        if index == number_of_players - 1
        {
            index = 0;
        } 
        else
        {
            index += 1;
        } 
    }

}
