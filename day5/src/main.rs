use std::collections::HashMap;
use std::fs;

fn main() {
    let f = fs::read_to_string("ex.txt").unwrap();
    let pg_odr_rules: Vec<&str> = f.split("\n\n").collect::<Vec<_>>()[0].lines().collect();
    let validate_pgs: Vec<&str> = f.split("\n\n").collect::<Vec<_>>()[1].lines().collect();

    let mut map = HashMap::new();
    for pg_odr_rule in pg_odr_rules {
        let mut rule = pg_odr_rule.split("\n").collect::<Vec<_>>();
        let pgl = rule.remove(0);
        let pgr = rule.remove(0);
    }

    dbg!(map);

    //dbg!(pg_odr_rules);
    //dbg!(validate_pgs);
}
