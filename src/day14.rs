use std::error::Error;
use std::io;

fn get_input() -> usize {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => buffer.trim().parse::<usize>().unwrap_or(0),
        Err(error) => {
            println!("Error: {}", error);
            0
        }
    }
}

pub fn first_star() -> Result<(), Box<Error + 'static>> {
    let mut recipes_score: Vec<usize> = vec![3, 7];
    let mut first_index: usize = 0;
    let mut second_index: usize = 1;
    let input = get_input();

    while recipes_score.len() < input + 10 {
        let (first_score, second_score) = (recipes_score[first_index], recipes_score[second_index]);
        let result = first_score + second_score;
        if result >= 10 {
            recipes_score.push(result / 10);
        }
        recipes_score.push(result % 10);
        first_index = (first_index + first_score + 1) % recipes_score.len();
        second_index = (second_index + second_score + 1) % recipes_score.len();
    }

    let mut answer = String::new();
    for score in recipes_score.iter().skip(input).take(10) {
        answer.push_str(&score.to_string());
    }

    println!("Score is: {}", answer);

    Ok(())
}

fn get_input_vec() -> Vec<usize> {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => buffer
            .trim()
            .chars()
            .map(|x| x.to_string().parse::<usize>().unwrap_or(0))
            .collect(),
        Err(error) => {
            println!("Error: {}", error);
            vec![0]
        }
    }
}

pub fn second_star() -> Result<(), Box<Error + 'static>> {
    let input = get_input_vec();

    let mut recipes_score: Vec<usize> = vec![3, 7];
    let mut first_index: usize = 0;
    let mut second_index: usize = 1;

    // NB: VecDeque doesn't work properly in that case: it contains two independant slice for looping effect and can't do eq! properly
    let mut score_matching = 0;

    loop {
        let (first_score, second_score) = (recipes_score[first_index], recipes_score[second_index]);
        let result = first_score + second_score;
        if result >= 10 {
            recipes_score.push(result / 10);
            if result / 10 == input[score_matching] {
                score_matching += 1;
                if score_matching == input.len() {
                    break;
                }
            } else if result / 10 == input[0] {
                score_matching = 1;
            } else {
                score_matching = 0;
            }
        }
        recipes_score.push(result % 10);

        if result % 10 == input[score_matching] {
            score_matching += 1;
            if score_matching == input.len() {
                break;
            }
        } else if result % 10 == input[0] {
            score_matching = 1;
        } else {
            score_matching = 0;
        }

        first_index = (first_index + first_score + 1) % recipes_score.len();
        second_index = (second_index + second_score + 1) % recipes_score.len();
    }

    println!(
        "{:?} shows after {} recipes",
        input,
        recipes_score.len() - input.len()
    );

    Ok(())
}
