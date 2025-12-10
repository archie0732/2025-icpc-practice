use std::cmp::min;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let seq1_str = lines.next().unwrap().unwrap();
    let seq1: Vec<char> = seq1_str.trim().chars().collect();

    let seq2_str = lines.next().unwrap().expect("Failed to read line 2");
    let seq2: Vec<char> = seq2_str.trim().chars().collect();

    let m = seq1.len();
    let n = seq2.len();

    let gap_penalty = 2;
    let mismatch_penalty = 1;
    let match_penalty = 0;

    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 0..=m {
        dp[i][0] = i as i32 * gap_penalty;
    }
    for j in 0..=n {
        dp[0][j] = j as i32 * gap_penalty;
    }

    for i in 1..=m {
        for j in 1..=n {
            let cost = if seq1[i - 1] == seq2[j - 1] {
                match_penalty
            } else {
                mismatch_penalty
            };

            let diagonal = dp[i - 1][j - 1] + cost;
            let up = dp[i - 1][j] + gap_penalty;
            let left = dp[i][j - 1] + gap_penalty;

            dp[i][j] = min(diagonal, min(up, left));
        }
    }

    println!("{}", dp[m][n]);

    let mut align1 = String::new();
    let mut align2 = String::new();
    let mut i = m;
    let mut j = n;

    while i > 0 || j > 0 {
        let current_score = dp[i][j];

        let diag_score = if i > 0 && j > 0 {
            let cost = if seq1[i - 1] == seq2[j - 1] {
                match_penalty
            } else {
                mismatch_penalty
            };
            dp[i - 1][j - 1] + cost
        } else {
            -1
        };

        let up_score = if i > 0 {
            dp[i - 1][j] + gap_penalty
        } else {
            -1
        };

        let left_score = if j > 0 {
            dp[i][j - 1] + gap_penalty
        } else {
            -1
        };

        if i > 0 && current_score == up_score {
            align1.push(seq1[i - 1]);
            align2.push('-');
            i -= 1;
        } else if j > 0 && current_score == left_score {
            align1.push('-');
            align2.push(seq2[j - 1]);
            j -= 1;
        } else if i > 0 && j > 0 && current_score == diag_score {
            align1.push(seq1[i - 1]);
            align2.push(seq2[j - 1]);
            i -= 1;
            j -= 1;
        } else {
            break;
        }
    }

    let final_seq1: String = align1.chars().rev().collect();
    let final_seq2: String = align2.chars().rev().collect();

    println!("{}", final_seq1);
    println!("{}", final_seq2);
}
