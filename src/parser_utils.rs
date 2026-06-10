use rodio::{
    Device, DeviceTrait,
    cpal::{self, traits::HostTrait},
};

pub fn parse_positive_f32(s: &str) -> Result<f32, String> {
    let v: f32 = s.parse().map_err(|_| "must be a number")?;
    if v > 0.0 {
        Ok(v)
    } else {
        Err("must be greater than zero".to_string())
    }
}

pub fn get_devices() -> Result<Vec<Device>, String> {
    let host = *cpal::ALL_HOSTS.first().ok_or("No audio host found.")?;
    let devices = cpal::host_from_id(host)
        .map_err(|_| "Failed to create host from default host ID.")?
        .output_devices()
        .map_err(|_| "Failed to list devices on default audio host.")?
        .collect();

    Ok(devices)
}

pub fn parse_device_name(s: &str) -> Result<rodio::Device, String> {
    for device in get_devices()? {
        if let Ok(name) = device.name()
            && name == s
        {
            return Ok(device);
        }
    }

    Err(format!("No matching output device found: {s}"))
}
