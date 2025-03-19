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

pub fn calculate_phred(qual: u8) -> Option<f32> {
    if (33..=126).contains(&qual) {
        Some((qual as f32) - 33.0)
    } else {
        None
    }
}
