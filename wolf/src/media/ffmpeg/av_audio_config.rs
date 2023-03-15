use crate::{
    error::WError,
    media::bindgen::ffmpeg::{
        av_channel_layout_default, av_channel_layout_uninit, AVChannelLayout, AVSampleFormat,
    },
};

#[derive(Debug, Clone)]
pub struct AvAudioConfig {
    // the sample rate of the audio
    pub sample_rate: u32,
    // the sample format of the audio
    pub sample_fmt: AVSampleFormat,
    // the channel layout of the audio
    pub channel_layout: *mut AVChannelLayout,
}

impl Drop for AvAudioConfig {
    fn drop(&mut self) {
        if self.channel_layout.is_null() {
            return;
        }
        unsafe { av_channel_layout_uninit(self.channel_layout) };

        #[cfg(debug_assertions)]
        println!("AvAudioConfig dropped");
    }
}

impl Default for AvAudioConfig {
    fn default() -> Self {
        Self {
            sample_rate: 44100,
            sample_fmt: AVSampleFormat::AV_SAMPLE_FMT_S16,
            channel_layout: std::ptr::null_mut(),
        }
    }
}

impl AvAudioConfig {
    /// # Errors
    ///
    /// returns an `WError`
    pub fn new(
        p_channels: u32,
        p_sample_rate: u32,
        p_sample_fmt: AVSampleFormat,
    ) -> Result<Self, WError> {
        let channels = i32::try_from(p_channels).map_err(|_| WError::IntCastError)?;
        let config = Self {
            sample_rate: p_sample_rate,
            sample_fmt: p_sample_fmt,
            channel_layout: std::ptr::null_mut(),
        };
        unsafe { av_channel_layout_default(config.channel_layout, channels) };
        Ok(config)
    }
}
