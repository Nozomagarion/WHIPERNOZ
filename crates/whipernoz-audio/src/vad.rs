/// Voice Activity Detection - detects speech boundaries
pub struct VoiceActivityDetector {
    threshold: f32,
    silence_frames: u32,
    silence_threshold_frames: u32,
    is_speaking: bool,
}

impl VoiceActivityDetector {
    pub fn new() -> Self {
        Self {
            threshold: 0.01,
            silence_frames: 0,
            // ~1.5 seconds of silence at 16kHz with 480-sample frames
            silence_threshold_frames: 50,
            is_speaking: false,
        }
    }

    /// Check if a frame contains speech based on RMS energy
    pub fn is_speech(&mut self, samples: &[f32]) -> bool {
        let rms = (samples.iter().map(|s| s * s).sum::<f32>() / samples.len() as f32).sqrt();

        if rms > self.threshold {
            self.is_speaking = true;
            self.silence_frames = 0;
            true
        } else {
            if self.is_speaking {
                self.silence_frames += 1;
            }
            false
        }
    }

    /// Check if enough silence has passed to consider speech ended
    pub fn speech_ended(&self) -> bool {
        self.is_speaking && self.silence_frames >= self.silence_threshold_frames
    }

    pub fn reset(&mut self) {
        self.is_speaking = false;
        self.silence_frames = 0;
    }
}

impl Default for VoiceActivityDetector {
    fn default() -> Self {
        Self::new()
    }
}
