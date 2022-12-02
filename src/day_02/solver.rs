use crate::input_helper;
use crate::day_02::model::RockPaperScissors;

pub fn solve()-> String {
    let input = input_helper::read_input::<RockPaperScissors>("input/22_02_test");
    let score:u32 = input.into_iter().map(|rps| rps.calculate_score()).sum();

    return format!("The total score is {}", score);
}

pub fn solve_two() -> String {
    let input = input_helper::read_input::<RockPaperScissors>("input/22_02_test");
    let sum: u32 = input.into_iter().map(|rps| rps.calculate_score_with_second_strat()).sum();
    return format!("With the new strategy the total score is {}", sum);
}

