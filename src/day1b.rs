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

    let similarity_score: i32 = lvalues
        .iter()
        .map(|e| e * rvalues.iter().filter(|v| *v == e).count() as i32)
        .sum();

    println!("total similarity score => {}", similarity_score);
}
