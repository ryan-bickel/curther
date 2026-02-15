pub fn parse_f32_at_least(min: f32) -> impl Fn(&str) -> Result<f32, String> + Clone {
    move |s: &str| {
        let v: f32 = s.parse().map_err(|_| "must be a floating point number")?;
        if v >= min {
            Ok(v)
        } else {
            Err(format!("must be greater than or equal to {}", min))
        }
    }
}
