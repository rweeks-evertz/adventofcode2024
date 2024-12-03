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
            let okay_values = test_report(report);
            if okay_values == report.len() {
                return 1;
            }

            // else test other combos
            for i in 0..report.len() {
                let mut modified_report = report.to_vec();
                modified_report.remove(i);
                let okay_values = test_report(&modified_report);
                if okay_values == modified_report.len() {
                    return 1;
                }
            }
            return 0;
        })
        .sum();

    println!("safe reports total => {:?}", safe_reports);
}

fn test_report(report: &Vec<i32>) -> usize {
    let mut prev: i32 = -1;
    let mut increasing: Option<bool> = None;
    report
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
        .count()
}
