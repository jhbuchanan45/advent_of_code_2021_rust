use std::char;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut reports = Vec::new();

    // count ones
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            reports.push(line.unwrap().into_bytes());
        }
    }

    let reports = reports; // make immutable

    // find oxygen rating
    let mut oxygen_reports = reports.clone();
    for i in 0..oxygen_reports[0].len() {
        if oxygen_reports.len() <= 1 {
            break;
        }

        let match_char = most_common_value(&oxygen_reports, i).unwrap();

        let oxygen_test_results = oxygen_reports.clone().into_iter().filter(|report| report[i] == match_char).collect::<Vec<Vec<u8>>>();
    
        if oxygen_test_results.len() == 0 {
            continue;
        }

        oxygen_reports = oxygen_test_results;
    }

    if oxygen_reports.len() > 1 {
        panic!("Oxygen report impossible to find")
    }

    let oxygen_report = std::str::from_utf8(&oxygen_reports[0]).unwrap();

    // find co2 rating
    let mut co2_reports = reports.clone();
    for i in 0..co2_reports[0].len() {
        if co2_reports.len() <= 1 {
            break;
        }

        let match_char = most_common_value(&co2_reports, i).unwrap();

        let co2_test_results = co2_reports.clone().into_iter().filter(|report| report[i] != match_char).collect::<Vec<Vec<u8>>>();
    
        if co2_test_results.len() == 0 {
            continue;
        }

        co2_reports = co2_test_results;
    }

    if co2_reports.len() > 1 {
        panic!("CO2 report impossible to find")
    }

    let co2_report = std::str::from_utf8(&co2_reports[0]).unwrap();

    println!("Final Oxygen: {}", oxygen_report);
    println!("Final CO2: {}", co2_report);
    println!("Rating: {}", isize::from_str_radix(oxygen_report, 2).unwrap() * isize::from_str_radix(co2_report, 2).unwrap());
}

fn most_common_value(reports: &Vec<Vec<u8>>, index: usize) -> io::Result<u8> {
    let mut ones_count = 0;
    let mut total = 0;

    for report in reports {
        let character = report[index] as char;
        match character {
            '1' => ones_count += 1,
            '0' => (),
            _ => panic!("Character not '0' or '1'"),
        }
        total += 1;
    }

    Ok(match ones_count >= total - ones_count {
        true => '1' as u8,
        false => '0' as u8,
    })
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
