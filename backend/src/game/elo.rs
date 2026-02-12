/// Calculate the K-factor based on player's total games and current Elo.
/// K=40 for new players (<30 games), K=20 standard, K=10 for 2400+ Elo.
fn k_factor(elo: i32, total_games: i32) -> f64 {
    if total_games < 30 {
        40.0
    } else if elo >= 2400 {
        10.0
    } else {
        20.0
    }
}

/// Calculate expected score (probability of winning) for player A against player B.
fn expected_score(rating_a: f64, rating_b: f64) -> f64 {
    1.0 / (1.0 + 10_f64.powf((rating_b - rating_a) / 400.0))
}

/// Calculate new Elo ratings for two players after a match.
/// Returns (new_elo_p1, new_elo_p2).
/// `outcome` is 1.0 for player1 win, 0.0 for player2 win, 0.5 for draw.
pub fn calculate_elo(
    p1_elo: i32,
    p1_games: i32,
    p2_elo: i32,
    p2_games: i32,
    outcome: f64, // 1.0 = p1 wins, 0.0 = p2 wins, 0.5 = draw
) -> (i32, i32) {
    let e1 = expected_score(p1_elo as f64, p2_elo as f64);
    let e2 = 1.0 - e1;

    let k1 = k_factor(p1_elo, p1_games);
    let k2 = k_factor(p2_elo, p2_games);

    let new_p1 = (p1_elo as f64 + k1 * (outcome - e1)).round() as i32;
    let new_p2 = (p2_elo as f64 + k2 * ((1.0 - outcome) - e2)).round() as i32;

    (new_p1.max(0), new_p2.max(0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal_elo_win() {
        let (new_p1, new_p2) = calculate_elo(1000, 0, 1000, 0, 1.0);
        assert_eq!(new_p1, 1020);
        assert_eq!(new_p2, 980);
    }

    #[test]
    fn test_equal_elo_draw() {
        let (new_p1, new_p2) = calculate_elo(1000, 0, 1000, 0, 0.5);
        assert_eq!(new_p1, 1000);
        assert_eq!(new_p2, 1000);
    }

    #[test]
    fn test_higher_elo_loses() {
        let (new_p1, new_p2) = calculate_elo(1400, 50, 1000, 50, 0.0);
        // Higher rated player loses, loses fewer points
        assert!(new_p1 < 1400);
        assert!(new_p2 > 1000);
    }
}
