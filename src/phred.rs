#[cfg(test)]
mod test {

    use super::calculate_phred;

    #[test]
    fn test_calculate_phred() {
        let qual: u8 = '&' as u8;
        let expected: f32 = 5.0; // Phred-Score für das Zeichen '&'
        let res = calculate_phred(qual).expect("Invalid Phred Char");
        assert_eq!(expected, res);
    }

    // Alternativ können viele Ergebnisse auf einmal getestet werden:
    #[test]
    fn test_calculate_phred_other() {
        let tests: Vec<(u8, f32)> = vec![('&' as u8, 5.0), ('+' as u8, 10.0)];
        for test in tests {
            let res = calculate_phred(test.0).expect("Invalid Phred Char");
            assert!(
                (res - test.1).abs() < f32::EPSILON,
                "Expected {}, but got {}",
                test.1,
                res
            );
        }
    }
    // ggf. andere Testfunktionen
}

pub fn avg_qual(qual_str: &[u8]) -> Option<f32> {
    
    if qual_str.is_empty() {
        return None;
    }
    
    let qu_sum: f32 = qual_str
    .iter()
    .map(|&qu| calculate_phred(qu))
    .collect::<Option<Vec<f32>>>()?
    .iter()
    .sum();

    Some(qu_sum/ qual_str.len() as f32)

}

fn calculate_phred(qual: u8) -> Option<f32> {
    if (33..=126).contains(&qual) {
        Some((qual as f32) - 33.0)
    } else {
        None
    }
}
