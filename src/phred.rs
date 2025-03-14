#[cfg(test)]
mod test {

    use super::calculate_phred;

    #[test]
    fn test_calculate_phred() {
        let qual: char = '&';
        let expected: f32 = 5.0; // Phred-Score für das Zeichen '&'
        let res = calculate_phred(qual).expect("Invalid Phred Char");
        assert_eq!(expected, res);
    }

    // Alternativ können viele Ergebnisse auf einmal getestet werden:
    #[test]
    fn test_calculate_phred_other() {
        let tests: Vec<(char, f32)> = vec![('&', 5.0), ('+', 10.0)];
        for test in tests {
            let res = calculate_phred(test.0).expect("Invalid Phred Char");
            assert_eq!(test.1, res);
        }
    }
    // ggf. andere Testfunktionen
}

pub fn avg_qual(qual_str: &String) -> Option<f32> {
    let qu_len = qual_str.len() as f32;
    let mut qu_sum = 0.0;

    // Going over every char in the string
    for qu in qual_str.chars() {
        // Adding every associated phred score to qu_sum
        let qu_evaluated = calculate_phred(qu)?;
        qu_sum += qu_evaluated;
    }

    Some(qu_sum / qu_len)
}

fn calculate_phred(qual: char) -> Option<f32> {
    let phsc = qual as usize;
    if (33..=126).contains(&phsc) {
        Some((phsc as f32) - 33.0)
    } else {
        None
    }
}
