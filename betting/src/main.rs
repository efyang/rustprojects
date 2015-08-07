/*Here is a simple game involving coin tosses: A player is given a starting amount of cash (perhaps $50) and repeatedly bets an amount of money (perhaps $1 or $2) on the outcome of a fair coin toss. Each time the coin is tossed, the player decides how much money to bet on that toss. The player will continue to play the game until one of three events happens:

    He reaches a predefined goal ($250) and walks away feeling happy (Win).
    He loses all of his cash ($0) and walks away feeling sad (Lose).
    He plays too many rounds (20000) and walks away feeling bored (Lose). (For the purposes of this assignment, we will always consider the failure to reach the target amount in the stipulated number of bets as a loss.)*/
extern crate rand;

struct Player{
    cash: u64,
    target_amount: u64,
    rounds: u64,
    round_history: Vec<bool>, //true = win, false = loss
    round_limit: u64,
}

impl Player {

    fn get_win_ratio(&self) -> u64 {
       //iterate over the round history 
    }

    fn decide_bet_amount(&self) -> u8 {
        //enter bet strategy here
    }

    fn is_in_play(&self) -> bool {
        //decides whether player has lost or not
    }
}

fn generate_coin_toss() -> bool {
    return rand::random();
}

fn main() {
    
}
