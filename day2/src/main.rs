use std::fs;

fn main() {
    let f = fs::read_to_string("input.txt").unwrap();
    let f = f.trim();
    let mut ans = 0;
    for line in f.split('\n') {
        let report: Vec<_> = line.split(' ').collect();
        if part1(&report) {
            ans += 1;
        }
    }
    dbg!(ans);
}

fn part1(report: &Vec<&str>) -> bool {
    let mut valid = true;
    let report = report
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    if report[0] < report[report.len() - 1] {
        for i in 0..report.len() - 1 {
            if report[i + 1] - report[i] >= 1 && report[i + 1] - report[i] <= 3 {
                continue;
            } else {
                valid = false;
                break;
            }
        }
    } else {
        for i in 0..report.len() - 1 {
            if report[i] - report[i + 1] >= 1 && report[i] - report[i + 1] <= 3 {
                continue;
            } else {
                valid = false;
                break;
            }
        }
    }
    //dbg!(report);
    return valid;
}
