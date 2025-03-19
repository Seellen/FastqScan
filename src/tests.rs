use crate::phred;

#[cfg(test)]
mod test {

    use crate::phred::avg_qual;

    use super::phred::calculate_phred;

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

    #[test]
    fn test_avg_qual(){
        const TEST_STRING: &[u8] = b"/&%/&)/%%";
        const EXPECT: f32 = 8.0;
        let res = avg_qual(TEST_STRING).expect("Expected Some(f32) but got none");
        assert!(
            (res - EXPECT).abs() < f32::EPSILON,
            "Expected {}, but got {}",
            EXPECT,
            res
        );
    }
}