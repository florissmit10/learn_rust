use std::fs;

fn main() {
    let input: Vec<String> =
        fs::read_to_string("input.txt")
            .expect("Cannot read input.txt")
            .lines()
            .map(|s| s.to_string())
            .collect();
    let report = DiagnosticReport::new(input);
    println!("{:?}",report);
}

#[derive(Debug)]
struct DiagnosticReport {
    data: Vec<DiagnosticsReportLine>,
    line_len: usize,
}

#[derive(Debug)]
struct DiagnosticsReportLine {
    data: Vec<i8>,
}

impl DiagnosticsReportLine {
    fn new(data: String) -> Self {
        let data:Vec<i8> = data.chars().map(|c| c.to_digit(2).unwrap() as i8).collect();
        return DiagnosticsReportLine{data}
    }

    fn get(self, i: usize) -> i8 {
        return self.data[i];
    }
 }

impl DiagnosticReport {
    fn new(data: Vec<String>) -> Self{
        let data: Vec<DiagnosticsReportLine> =
            data.into_iter().map(|l| DiagnosticsReportLine::new(l)).collect();
        let line_len: usize = *data.iter().map(|drl| drl.data.len()).collect::<Vec<usize>>().iter().max().unwrap();
        return DiagnosticReport{data, line_len}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dr_basic() {
        let input = vec![String::from("00000")];

        let result = DiagnosticReport::new(input);

        assert_eq!(result.data.len(), 1)
    }
}