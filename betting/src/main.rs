/*Here is a simple game involving coin tosses: A player is given a starting amount of cash (perhaps $50) and repeatedly bets an amount of money (perhaps $1 or $2) on the outcome of a fair coin toss. Each time the coin is tossed, the player decides how much money to bet on that toss. The player will continue to play the game until one of three events happens:

    He reaches a predefined goal ($250) and walks away feeling happy (Win).
    He loses all of his cash ($0) and walks away feeling sad (Lose).  He plays too many rounds (20000) and walks away feeling bored (Lose). (For the purposes of this assignment, we will always consider the failure to reach the target amount in the stipulated number of bets as a loss.)*/
extern crate rand;
use std::env;

struct Player{
    cash: i64,
    target: i64,
    bet_limit: i64,
    rounds: i64,
    round_history: Vec<bool>, //true = win, false = loss
    round_limit: i64,
} 

impl Player {

    fn get_win_ratio(&self) -> f64 {
       //iterate over the round history
       let mut wins: f64 = 0 as f64;
       let mut losses: f64 = 0 as f64;
        
       for &round in self.round_history.iter(){
           if round{
           wins = wins + 1.0;
           }else{
           losses = losses + 1.0;
           }
       }

       return wins/losses;
    }

    fn decide_bet_amount(&self) -> i64 {
        //bet strategy
        let bet: i64;
        return 2;
    }

    fn is_in_play(&self) -> bool {
        //decides whether player has finished playing or not
        if self.cash <= 0{
            println!("\nThe player has played {} rounds and is now out of cash.", self.rounds);
            println!("Win/Loss Ratio: {}", self.get_win_ratio());
            return false;
        }else if self.rounds >= self.round_limit{
            println!("\nThe player has played at least {} rounds, and has given up with {} cash.", self.rounds, self.cash);
            println!("Win/Loss Ratio: {}", self.get_win_ratio());
            return false;
        }else if self.cash >= self.target{
            println!("\nThe player has obtained at least the target amount, {}.", self.target);
            println!("Win/Loss Ratio: {}", self.get_win_ratio());
            return false;
        }else{
            return true;
        }
    }
}

fn generate_coin_toss() -> bool {
    return rand::random();
}

fn bool_to_status (input : bool) -> &'static str{
    if input {
        return "Won";
    }else{
        return "Lost";
    }
}

fn main_game(cash: i64, target: i64,bet_limit: i64, round_limit: i64, logging: bool) {
    
   let mut player = Player { cash: cash, 
                             target: target,
                             bet_limit: bet_limit,
                             rounds: 0,
                             round_history: Vec::new(),
                             round_limit: round_limit,};
   loop {
        let mut bet: i64 = player.decide_bet_amount();
        let round: bool;
        
        if !generate_coin_toss() {
            bet = bet * -1;
            round = false;
        }else{
            round = true;
        }
        player.cash = player.cash + bet;
        player.rounds = player.rounds + 1;
        player.round_history.push(round);
        if logging {
            println!("Cash: {} Bet: {} Bet Amount: {} Rounds: {}", player.cash, bool_to_status(round), bet.abs() , player.rounds);
        }
        if !player.is_in_play(){
            break;
        }
    }
}

fn string_to_bool (input : &String) -> bool {
    let lower_input: String = input.to_lowercase();
    if lower_input == "true" || lower_input == "yes" {
        return true;
    }else{
        return false;
    }
}


fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() == 6 {
        //cash target bet_limit round_limit logging
        main_game(args[1].parse().ok().expect("Invalid Argument."), 
                  args[2].parse().ok().expect("Invalid Argument."), 
                  args[3].parse().ok().expect("Invalid Argument."),
                  args[4].parse().ok().expect("Invalid Argument."),
                  string_to_bool(&args[4]));
    }else{
        println!("Your arguments were invalid, going with default values.");
        main_game(50, 250, 20000, false);
    }
}
