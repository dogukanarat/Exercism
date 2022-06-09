use std::collections::HashMap;

struct ScoreTable {
    matches_played: i32,
    won: i32,
    draw: i32,
    lost: i32,
    points: i32,
}

pub fn tally(match_results: &str) -> String {
    let score_lines_raw = match_results.split("\n").collect::<Vec<&str>>();

    let mut score_map: HashMap<&str, ScoreTable> = HashMap::new();

    for line in score_lines_raw {
        let mut score_line: Vec<&str> = Vec::new();
        for info in line.split(";") {
            score_line.push(info);
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

        if let Some(score) = score_map.get_mut(left_side_name) {
            score.matches_played += 1;

            if left_side_score > right_side_score {
                score.won += 1;
            } else if left_side_score < right_side_score  {
                score.lost += 1;
            } else {
                score.draw += 1;
            }

            score.points += left_side_score;

        } else {
            let new_score = ScoreTable {
                matches_played: 1,
                won: (left_side_score > right_side_score) as i32,
                draw: (left_side_score == right_side_score) as i32,
                lost: (left_side_score < right_side_score) as i32,
                points: left_side_score,
            };

            score_map.insert(left_side_name, new_score);
        }

        if let Some(score) = score_map.get_mut(right_side_name) {
            score.matches_played += 1;

            if right_side_score > left_side_score {
                score.won += 1;
            } else if right_side_score < left_side_score  {
                score.lost += 1;
            } else {
                score.draw += 1;
            }

            score.points += right_side_score;

        } else {
            let new_score = ScoreTable {
                matches_played: 1,
                won: (right_side_score > left_side_score) as i32,
                draw: (right_side_score == left_side_score) as i32,
                lost: (right_side_score < left_side_score) as i32,
                points: right_side_score,
            };

            score_map.insert(right_side_name, new_score);
        }
    }

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

    return_score[return_score_size - 1].pop();

    return_score.insert(0, "Team                           | MP |  W |  D |  L |  P\n".to_string());

    return_score.join("")
}
