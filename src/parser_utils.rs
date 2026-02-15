pub fn parse_positive_f32(s: &str) -> Result<f32, String> {
    let v: f32 = s.parse().map_err(|_| "must be a number")?;
    if v > 0.0 {
        Ok(v)
    } else {
        Err("must be greater than zero".to_string())
    }
}
