use std::fs::read_to_string;
fn main() {
    let contents = read_to_string("inputs/day1.txt").unwrap();
    let (mut lvalues, mut rvalues): (Vec<i32>, Vec<i32>) = contents
        .lines()
        .map(|line| {
            let (lstr, rstr) = line.split_once(' ').unwrap();
            let left = lstr.trim().parse::<i32>().unwrap();
            let right = rstr.trim().parse::<i32>().unwrap();
            (left, right)
        })
        .unzip();

    lvalues.sort();
    rvalues.sort();

    let total_distance: i32 = lvalues
        .iter()
        .zip(rvalues.iter())
        .map(|(l, r)| (r - l).abs())
        .sum();

    println!("total distance => {}", total_distance);
}
