// A list of scores (one per line) of a soccer match is given. Each line is of
// the form "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: "England,France,4,2" (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, the total
// number of goals the team scored, and the total number of goals the team
// conceded.

use std::collections::HashMap;

// A structure to store the goal details of a team.
#[derive(Default, Debug)]
// deriving Debug here so i can print the result
struct TeamScores {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: &str) -> HashMap<&str, TeamScores> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores = HashMap::new();

    for line in results.lines() {
        let mut split_iterator = line.split(',');
        // NOTE: We use `unwrap` because we didn't deal with error handling yet.
        let team_1_name = split_iterator.next().unwrap();
        // looks until the first comma for the first value
        let team_2_name = split_iterator.next().unwrap();
        // jumps after the comma after the first one is created
        // the second value is team_2_name
        let team_1_score: u8 = split_iterator.next().unwrap().parse().unwrap();
        // after team_2_name is created the third value is team_1_score
        let team_2_score: u8 = split_iterator.next().unwrap().parse().unwrap();
        // jumps after that comma and then gets the fourth value

        // each call to next moves to the next value
        // this is how we can use the same code and get different values

        // TODO: Populate the scores table with the extracted details.
        // Keep in mind that goals scored by team 1 will be the number of goals
        // conceded by team 2. Similarly, goals scored by team 2 will be the
        // number of goals conceded by team 1.

        // let team_scores = TeamScores {
        //     goals_scored: team_1_score,
        //     goals_conceded: team_2_score,
        // };
        //
        // scores.insert(team_1_name, team_scores);
        // initial solution, fails test

        let team_1_entry = scores.entry(team_1_name).or_insert(TeamScores::default());
        // add team_1_entry to the table as value of team_1_name which is a key
        // if no value is found for team_1_name, it will be inserted with the default
        team_1_entry.goals_scored += team_1_score;
        // for every match the goals scored by team 1 will be the number of goals
        team_1_entry.goals_conceded += team_2_score;
        // for every match the goals team 2 scores are the number of goals conceded

        let team_2_entry = scores.entry(team_2_name).or_insert(TeamScores::default());
        team_2_entry.goals_conceded += team_1_score;
        team_2_entry.goals_scored += team_2_score;
        // same as above
    }

    scores
}

fn main() {
    // You can optionally experiment here.
    const RESULTS: &str = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";

    let scores = build_scores_table(RESULTS);
    println!("{:?}", scores);
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESULTS: &str = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";

    #[test]
    fn build_scores() {
        let scores = build_scores_table(RESULTS);

        assert!(["England", "France", "Germany", "Italy", "Poland", "Spain"]
            .into_iter()
            .all(|team_name| scores.contains_key(team_name)));
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 6);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 3);
    }
}
