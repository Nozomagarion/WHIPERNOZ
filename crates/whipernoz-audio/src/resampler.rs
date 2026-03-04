use anyhow::Result;

/// Resample audio to Whisper's required format: 16kHz mono f32
pub fn resample_to_16khz(samples: &[f32], source_rate: u32, source_channels: u16) -> Result<Vec<f32>> {
    // If already 16kHz mono, return as-is
    if source_rate == 16000 && source_channels == 1 {
        return Ok(samples.to_vec());
    }

    let mut mono = if source_channels > 1 {
        // Downmix to mono by averaging channels
        samples
            .chunks(source_channels as usize)
            .map(|frame| frame.iter().sum::<f32>() / source_channels as f32)
            .collect::<Vec<f32>>()
    } else {
        samples.to_vec()
    };

    // Simple linear interpolation resampling
    if source_rate != 16000 {
        let ratio = 16000.0 / source_rate as f64;
        let new_len = (mono.len() as f64 * ratio) as usize;
        let mut resampled = Vec::with_capacity(new_len);

        for i in 0..new_len {
            let src_idx = i as f64 / ratio;
            let idx = src_idx as usize;
            let frac = src_idx - idx as f64;

            let sample = if idx + 1 < mono.len() {
                mono[idx] as f64 * (1.0 - frac) + mono[idx + 1] as f64 * frac
            } else {
                mono[idx.min(mono.len() - 1)] as f64
            };
            resampled.push(sample as f32);
        }

        mono = resampled;
    }

    Ok(mono)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passthrough_16khz_mono() {
        let samples = vec![0.1, 0.2, 0.3];
        let result = resample_to_16khz(&samples, 16000, 1).unwrap();
        assert_eq!(result, samples);
    }

    #[test]
    fn downmix_stereo() {
        let stereo = vec![0.2, 0.4, 0.6, 0.8];
        let result = resample_to_16khz(&stereo, 16000, 2).unwrap();
        assert_eq!(result.len(), 2);
        assert!((result[0] - 0.3).abs() < 0.001);
        assert!((result[1] - 0.7).abs() < 0.001);
    }
}
