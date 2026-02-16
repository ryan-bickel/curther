use crate::curther::Reverb;

pub fn parse_positive_f32(s: &str) -> Result<f32, String> {
    let v: f32 = s.parse().map_err(|_| "must be a number")?;
    if v > 0.0 {
        Ok(v)
    } else {
        Err("must be greater than zero".to_string())
    }
}

pub fn parse_reverb(values: Option<Vec<String>>) -> Result<Option<Reverb>, String>{
    match values {
        Some(values) => {
            if values.len() != 2 {
                Err("must pass two values to reverb".to_string())
            } else {
                let millis: u64 = values[0].parse().map_err(|_| "reverb duration must be an integer")?;
                let amplitude: f32 = values[0].parse().map_err(|_| "reverb amplitude must be a number")?;
                let reverb = Reverb::new(millis, amplitude).map_err(|_| "invalid reverb vals")?;
                Ok(Some(reverb))
            }
        }
        None => Ok(None)
    }
}
