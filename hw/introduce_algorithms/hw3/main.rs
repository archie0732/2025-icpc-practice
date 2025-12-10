use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut tokens = input.split_whitespace();

    let n_str = match tokens.next() {
        Some(s) => s,
        None => return,
    };
    let n: usize = n_str.parse().unwrap();

    let mut w = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            let val_str = tokens.next().unwrap();
            let val: i32 = val_str.parse().unwrap();
            w[i][j] = if val == -1 { i32::MAX } else { val };
        }
    }

    let mut nearest = vec![0; n];
    let mut distance = vec![0; n];
    let mut f_set: Vec<(usize, usize)> = Vec::new();

    for i in 1..n {
        nearest[i] = 0;
        distance[i] = w[0][i];
    }

    for _ in 0..(n - 1) {
        let mut min = i32::MAX;
        let mut vnear = 0;

        for i in 1..n {
            if distance[i] >= 0 && distance[i] < min {
                min = distance[i];
                vnear = i;
            }
        }

        f_set.push((nearest[vnear], vnear));
        distance[vnear] = -1;

        for i in 1..n {
            if distance[i] != -1 {
                if w[i][vnear] < distance[i] {
                    distance[i] = w[i][vnear];
                    nearest[i] = vnear;
                }
            }
        }
    }

    print!("F = {{");
    for (idx, edge) in f_set.iter().enumerate() {
        let u = edge.0 + 1;
        let v = edge.1 + 1;
        print!("<v{}, v{}>", u, v);
        if idx < f_set.len() - 1 {
            print!(", ");
        }
    }
    println!("}}");
}
