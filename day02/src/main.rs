use itertools::Itertools;
use std::fs;

fn parse_report(line: &str) -> Vec<i32> {
    line
        .split_whitespace()
        .map(|num| num.parse().expect("Couldn't parse number"))
        .collect()
}

fn is_safe_report(levels: &Vec<i32>) -> bool {
    levels
        .iter()
        .tuple_windows()
        .all(|(prev, next)| prev > next && prev - next <= 3)
        || levels
            .iter()
            .tuple_windows()
            .all(|(prev, next)| prev < next && next - prev <= 3)
}

fn is_safe_report_after_removal(report: &Vec<i32>) -> bool {
    (0..report.len()).any(|index_to_remove| {
        let report_with_removal = report
            .iter()
            .enumerate()
            .filter(|&(index, _)| index != index_to_remove)
            .map(|(_, level)| *level)
            .collect::<Vec<i32>>();

        // If the report is safe after removing the given level, then return true
        is_safe_report(&report_with_removal)
    })
}

/// Main function to read report data from a file, analyze the safety of reports,
/// and print the count of safe reports both before and after potential level removal.
fn main() {
    // Read the report data from the input file.
    let report_data = fs::read_to_string("input.txt").expect("Couldn't read input file.");

    // Count the number of reports that are considered safe.
    let safe_reports_count = report_data
        .lines()
        .map(parse_report)
        .filter(is_safe_report)
        .count();

    // Print the number of safe reports.
    println!("Number of safe reports: {}", safe_reports_count);

    // Count the number of reports that are considered safe after removing one level.
    let safe_reports_after_removal_count = report_data
        .lines()
        .map(parse_report)
        .filter(is_safe_report_after_removal)
        .count();

    // Print the number of safe reports after possible level removal.
    println!("Number of safe reports after removal: {}", safe_reports_after_removal_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_safe_report() {
        assert!(is_safe_report(&vec![7, 6, 4, 2, 1])); // Safe: Levels all decreasing by 1 or 2
        assert!(!is_safe_report(&vec![1, 2, 7, 8, 9])); // Unsafe: 2 to 7 is an increase of 5
        assert!(!is_safe_report(&vec![9, 7, 6, 2, 1])); // Unsafe: 6 to 2 is a decrease of 4
        assert!(!is_safe_report(&vec![1, 3, 2, 4, 5])); // Unsafe: 1 to 3 is increasing, 3 to 2 is decreasing
        assert!(!is_safe_report(&vec![8, 6, 4, 4, 1])); // Unsafe: 4 to 4 is neither an increase nor a decrease
        assert!(is_safe_report(&vec![1, 3, 6, 7, 9])); // Safe: Levels all increasing by 1, 2, or 3
    }

    #[test]
    fn test_is_safe_report_after_removal() {
        assert!(is_safe_report_after_removal(&vec![7, 6, 4, 2, 1])); // Safe without removing any level
        assert!(!is_safe_report_after_removal(&vec![1, 2, 7, 8, 9])); // Unsafe regardless of which level is removed
        assert!(!is_safe_report_after_removal(&vec![9, 7, 6, 2, 1])); // Unsafe regardless of which level is removed
        assert!(is_safe_report_after_removal(&vec![1, 3, 2, 4, 5])); // Safe by removing the second level, 3
        assert!(is_safe_report_after_removal(&vec![8, 6, 4, 4, 1])); // Safe by removing the third level, 4
        assert!(is_safe_report_after_removal(&vec![1, 3, 6, 7, 9])); // Safe without removing any level
    }

    #[test]
    fn test_with_test_file() {
        let test_contents = fs::read_to_string("test.txt").expect("Couldn't read test file.");

        let num_safe = test_contents
            .lines()
            .filter(|line| is_safe_report(&parse_report(line)))
            .count();

        let num_safe_after_removal = test_contents
            .lines()
            .filter(|line| is_safe_report_after_removal(&parse_report(line)))
            .count();

        assert_eq!(num_safe, 2);
        assert_eq!(num_safe_after_removal, 4);
    }
}