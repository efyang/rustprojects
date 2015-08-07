/*Here is a simple game involving coin tosses: A player is given a starting amount of cash (perhaps $50) and repeatedly bets an amount of money (perhaps $1 or $2) on the outcome of a fair coin toss. Each time the coin is tossed, the player decides how much money to bet on that toss. The player will continue to play the game until one of three events happens:

    He reaches a predefined goal ($250) and walks away feeling happy (Win).
    He loses all of his cash ($0) and walks away feeling sad (Lose).
    He plays too many rounds (20000) and walks away feeling bored (Lose). (For the purposes of this assignment, we will always consider the failure to reach the target amount in the stipulated number of bets as a loss.)*/
extern crate rand;

struct Player{
    mut cash: i64,
    target: u64,
    rounds: u64,
    round_history: Vec<bool>, //true = win, false = loss
    round_limit: u64,
}

impl Player {

    fn get_win_ratio(&self) -> f64 {
       //iterate over the round history
       return 10.0;
    }

    fn decide_bet_amount(&self) -> i8 {
        //bet strategy
        return 2;
    }

    fn change_cash_amount(&self, i8: change) {
        //changes the cash amount of the player
        self.cash = self.cash + change;
    }

    fn is_in_play(&self) -> bool {
        //decides whether player has finished playing or not
        if self.cash <= 0{
            println!("The player has played {} rounds and is now out of cash.", self.rounds);
            println!("Win/Loss Ratio: {}", self.get_win_ratio());
            return false;
        }else if self.rounds >= self.round_limit{
            println!("The player has played at least {} rounds, and has given up with {} cash.", self.rounds, self.cash);
            println!("Win/Loss Ratio: {}", self.get_win_ratio());
            return false;
        }else if self.cash >= target{
            println!("The player has obtained at least the target amount, {}.", self.target);
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

fn main() {
    let mut player = Player { cash: 50, 
                              target: 25,
                              rounds: 0,
                              round_history: vec![],
                              round_limit: 20000,};
    let mut bet: i8 = player.decide_bet_amount();
    
    if !generate_coin_toss() {
        bet = bet * -1;
    }
    player.change_cash_amount(bet);
    
    if player.is_in_play() {
        main();
    }
}
