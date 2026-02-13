pub fn parse_f32_in_range(min: f32, max: f32) -> impl Fn(&str) -> Result<f32, String> + Clone {
    move |s: &str| {
        let v: f32 = s.parse().map_err(|_| "must be a floating point number")?;
        if (min..=max).contains(&v) {
            Ok(v)
        } else {
            Err(format!("must be between {} and {} (inclusive)", min, max))
        }
    }
}
