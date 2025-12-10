use std::io::{self, Read};

const INF: i32 = 999999;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("讀取失敗");
    let mut tokens = input.split_whitespace();

    let n_str = tokens.next().expect("找不到頂點數");
    let n: usize = n_str.parse().expect("頂點數格式錯誤");

    let mut w = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            let val_str = tokens.next().expect("矩陣資料不足");
            let val: i32 = val_str.parse().expect("矩陣內容格式錯誤");
            w[i][j] = if val == -1 { INF } else { val };
        }
    }

    let mut touch = vec![0; n];
    let mut length = vec![0; n];
    let mut final_distances = vec![0; n];
    let mut f_set: Vec<(usize, usize)> = Vec::new();

    for i in 1..n {
        touch[i] = 0;
        length[i] = w[0][i];
    }

    for _ in 0..(n - 1) {
        let mut min = INF;
        let mut vnear = 0;

        for i in 1..n {
            if length[i] >= 0 && length[i] < min {
                min = length[i];
                vnear = i;
            }
        }

        if min == INF {
            break;
        }

        f_set.push((touch[vnear], vnear));

        final_distances[vnear] = min;

        let dist_to_vnear = length[vnear];
        length[vnear] = -1;

        for i in 1..n {
            if length[i] != -1 {
                if w[vnear][i] != INF {
                    if dist_to_vnear + w[vnear][i] < length[i] {
                        length[i] = dist_to_vnear + w[vnear][i];
                        touch[i] = vnear;
                    }
                }
            }
        }
    }

    print!("F = {{");
    for (idx, edge) in f_set.iter().enumerate() {
        print!("<v{}, v{}>", edge.0 + 1, edge.1 + 1);
        if idx < f_set.len() - 1 {
            print!(", ");
        }
    }
    println!("}}");
    for target in 1..n {
        let dist = final_distances[target];

        let mut path = Vec::new();
        let mut curr = target;
        path.push(curr);

        let mut steps = 0;
        while curr != 0 && steps < n {
            curr = touch[curr];
            path.push(curr);
            steps += 1;
        }
        path.reverse();

        let path_str: Vec<String> = path.iter().map(|&x| format!("v{}", x + 1)).collect();
        println!("{} (length = {})", path_str.join(", "), dist);
    }
}
