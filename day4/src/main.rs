use std::fs;

fn main() {
    let f = fs::read_to_string("ex.txt").unwrap();
    let mut wgrid = Vec::new();
    for line in f.lines() {
        let mut l = Vec::new();
        for c in line.chars() {
            l.push(c);
        }
        wgrid.push(l);
    }
    dbg!(part1(&wgrid));
}

fn part1(wgrid: &Vec<Vec<char>>) -> i32 {
    let mut ans = 0;

    let dirs: Vec<[i32; 2]> = vec![
        [-1, -1],
        [-1, 0],
        [-1, 1],
        [0, -1],
        [0, 1],
        [1, -1],
        [1, 0],
        [1, 1],
    ];

    let word = "XMAS".to_string();

    for y in 0..wgrid.len() {
        for x in 0..wgrid[y].len() {
            let mut idx = 0;
            if wgrid[y][x] == word.chars().nth(idx).unwrap() {
                idx += 1;
                for dir in &dirs {
                    let i = dir[0];
                    let j = dir[1];
                    if y as i32 + j < 0
                        || y as i32 + j >= wgrid.len() as i32
                        || x as i32 + i < 0
                        || x as i32 + i >= wgrid[y].len() as i32
                    {
                        continue;
                    }
                    let curr = wgrid[y + j as usize][x + i as usize];
                    if curr == word.chars().nth(idx).unwrap() {
                        idx += 1;
                    }
                }
            }
        }
    }

    ans
}
