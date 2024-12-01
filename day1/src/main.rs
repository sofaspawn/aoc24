use std::collections::HashMap;
use std::fs;

fn main() {
    let f = fs::read_to_string("input.txt").unwrap();
    let mut l1 = Vec::new();
    let mut l2 = Vec::new();
    for line in f.split("\n") {
        //println!("{:?}",line.split("   ").collect::<Vec<_>>());
        let v = line.split("   ").collect::<Vec<_>>();
        if v.len() > 1 {
            l1.push(v[0]);
            l2.push(v[1]);
        }
    }
    //part1(&mut l1, &mut l2);
    part2(&l1, &l2);
}

fn part2(l1: &Vec<&str>, l2: &Vec<&str>) {
    //let mut c1 = HashMap::new();
    let mut c2 = HashMap::new();
    /*
    for v in l1 {
        c1.entry(v).and_modify(|counter| *counter += 1).or_insert(1);
    }
    */
    for v in l2 {
        c2.entry(v).and_modify(|counter| *counter += 1).or_insert(1);
    }
    let mut ans = 0;
    for v in l1 {
        ans += v.to_string().parse::<i32>().unwrap() * c2.get(v).unwrap_or(&0);
    }
    dbg!(ans);
}

fn _part1(l1: &mut Vec<&str>, l2: &mut Vec<&str>) {
    l1.sort();
    l2.sort();
    let mut ans = 0;
    for i in 0..l1.len() {
        let snd = l2[i].to_string().parse::<i32>().unwrap();
        let fst = l1[i].to_string().parse::<i32>().unwrap();
        ans += (snd - fst).abs();
    }
    println!("{ans}");
}
