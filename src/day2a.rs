use std::fs::read_to_string;
fn main() {
    let contents = read_to_string("inputs/day2.txt").unwrap();
    let reports: Vec<Vec<i32>> = contents
        .lines()
        .map(|line| {
            let numbers: Vec<i32> = line
                .split(' ')
                .map(str::to_string)
                .map(|f| f.parse::<i32>().unwrap())
                .collect();
            return numbers;
        })
        .collect();

    let safe_reports: usize = reports
        .iter()
        .map(|report| {
            let mut increasing: Option<bool> = Option::None;
            let mut prev: i32 = -1;
            let okay_values = report
                .iter()
                .map_while(|curr| {
                    if prev == -1 {
                        prev = *curr;
                        return Some(());
                    }
                    let t = curr - prev;
                    let matching_direction = match increasing {
                        None => {
                            increasing = Some(t.is_positive());
                            true
                        }
                        Some(inc) => inc == t.is_positive(),
                    };
                    if !matching_direction {
                        prev = *curr;
                        return None;
                    }
                    if t.abs() > 0 && t.abs() < 4 {
                        prev = *curr;
                        return Some(());
                    }
                    prev = *curr;
                    return None;
                })
                .count();
            if okay_values == report.len() {
                return 1;
            }
            return 0;
        })
        .sum();

    println!("safe reports total => {:?}", safe_reports);
}
