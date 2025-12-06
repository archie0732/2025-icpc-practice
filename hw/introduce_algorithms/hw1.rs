use std::io::{self, BufRead};

const INF: i32 = 1_000_000_000;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n_line = lines
        .next()
        .expect("No input provided")
        .expect("Failed to read line");
    let n: usize = n_line.trim().parse().expect("Invalid number for n");

    let mut dist = vec![vec![0; n]; n];

    for i in 0..n {
        let line = lines
            .next()
            .expect("Unexpected end of input")
            .expect("Failed to read line");

        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Parse error"))
            .collect();

        for j in 0..n {
            if nums[j] == -1 {
                dist[i][j] = INF;
            } else {
                dist[i][j] = nums[j];
            }
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dist[i][k] != INF && dist[k][j] != INF {
                    if dist[i][k] + dist[k][j] < dist[i][j] {
                        dist[i][j] = dist[i][k] + dist[k][j];
                    }
                }
            }
        }
    }

    for i in 0..n {
        let row_strings: Vec<String> = dist[i].iter().map(|&val| val.to_string()).collect();
        println!("{}", row_strings.join(" "));
    }
}
