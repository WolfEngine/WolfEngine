#![allow(unused_crate_dependencies)]
#![allow(clippy::cast_possible_truncation)]

#[cfg(feature = "media_ffmpeg")]
#[test]
fn test_av_frame_convert() -> Result<(), wolf::error::WError> {
    use image::GenericImageView;
    use wolf::error::WError;
    use wolf::media::ffmpeg::av_config::AvConfig;
    use wolf::media::{bindgen::ffmpeg::AVPixelFormat, ffmpeg::av_frame::AvFrame};

    let mut current_dir = std::env::current_dir().unwrap();
    if current_dir.ends_with("wolf") {
        current_dir = current_dir.join("..");
    }
    let file_path = current_dir.join("content/texture/rgb.png");

    let img = image::open(file_path).unwrap();
    let img_size = img.dimensions();
    let pixels = img.as_rgba8().unwrap().to_vec();

    // create a source frame from image
    let src_config =
        AvConfig::new_video(AVPixelFormat::AV_PIX_FMT_RGBA, img_size.0, img_size.1, 1)?;
    let src_frame = AvFrame::new(src_config, Vec::new(), pixels)?;

    // convert to bgra
    let dst_config =
        AvConfig::new_video(AVPixelFormat::AV_PIX_FMT_BGRA, img_size.0, img_size.1, 1)?;
    let dst_frame = src_frame.convert_video(&dst_config)?;

    let out_path = current_dir.join("sample_bgra.png");
    let dst_pixels = dst_frame.get_video_data_ptr(0)?;

    let width = u32::try_from(dst_config.width).map_err(|_| WError::IntCastError)?;
    let height = u32::try_from(dst_config.height).map_err(|_| WError::IntCastError)?;

    let image_res = image::save_buffer_with_format(
        out_path,
        dst_pixels,
        width,
        height,
        image::ColorType::Rgba8,
        image::ImageFormat::Png,
    );
    if image_res.is_err() {
        return Err(WError::InvalidParameter);
    }
    Ok(())
}