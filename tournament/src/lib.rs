use std::vec::Vec;
use std::cmp::Ordering;

struct ScoreTable {
    matches_played: i32,
    won: i32,
    draw: i32,
    lost: i32,
    points: i32,
}

pub fn tally(match_results: &str) -> String {
    let score_lines_raw = match_results.split("\n").collect::<Vec<&str>>();

    let mut score_map: Vec<(&str, ScoreTable)> = Vec::new();

    for line in score_lines_raw {
        let mut score_line: Vec<&str> = Vec::new();
        for info in line.split(";") {
            score_line.push(info);
        }

        if score_line.len() != 3 {
            break;
        }

        let left_side_name = score_line[0];
        let right_side_name = score_line[1];
        let result_name = score_line[2];

        let mut left_side_score = 0;
        let mut right_side_score = 0;
        match result_name {
            "win" => {
                left_side_score = 3;
                right_side_score = 0;
            }
            "loss" => {
                left_side_score = 0;
                right_side_score = 3;
            }
            "draw" => {
                left_side_score = 1;
                right_side_score = 1;
            }
            _ => {
                println!("{} wins", "error");
            }
        }

        if let Some(score) = score_map.iter_mut().find(|x| x.0 == left_side_name) {
            score.1.matches_played += 1;

            if left_side_score > right_side_score {
                score.1.won += 1;
            } else if left_side_score < right_side_score  {
                score.1.lost += 1;
            } else {
                score.1.draw += 1;
            }

            score.1.points += left_side_score;

        } else {
            let new_score = ScoreTable {
                matches_played: 1,
                won: (left_side_score > right_side_score) as i32,
                draw: (left_side_score == right_side_score) as i32,
                lost: (left_side_score < right_side_score) as i32,
                points: left_side_score,
            };

            score_map.push((left_side_name, new_score));
        }

        if let Some(score) =  score_map.iter_mut().find(|x| x.0 == right_side_name) {
            score.1.matches_played += 1;

            if right_side_score > left_side_score {
                score.1.won += 1;
            } else if right_side_score < left_side_score  {
                score.1.lost += 1;
            } else {
                score.1.draw += 1;
            }

            score.1.points += right_side_score;

        } else {
            let new_score = ScoreTable {
                matches_played: 1,
                won: (right_side_score > left_side_score) as i32,
                draw: (right_side_score == left_side_score) as i32,
                lost: (right_side_score < left_side_score) as i32,
                points: right_side_score,
            };

            score_map.push((right_side_name, new_score));
        }
    }

    score_map.sort_by(|a, b| {
        match b.1.points.cmp(&a.1.points) {
            Ordering::Equal => b.0.cmp(&a.0).reverse(),
            other => other,
        }
    });

    // @todo sort hash map by points

    let mut return_score = score_map.iter().map(|(key, value)| {
        let mut key_copy = String::from(*key);
        
        while key_copy.len() != 30 {
            key_copy.push(' ');
        }

        format!("{} |  {} |  {} |  {} |  {} |  {}\n",
        key_copy,
        value.matches_played,
        value.won,
        value.draw,
        value.lost,
        value.points)
    }).collect::<Vec<String>>();

    let return_score_size = return_score.len();

    if return_score_size > 0 {
        return_score[return_score_size - 1].pop();
        return_score.insert(0, "Team                           | MP |  W |  D |  L |  P\n".to_string());
    }
    else
    {
        return_score.insert(0, "Team                           | MP |  W |  D |  L |  P".to_string());
    }

    

    return_score.join("")
}
