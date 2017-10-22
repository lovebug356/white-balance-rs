use image;

pub fn image_format_from_string(extension: &str) -> Option<image::ImageFormat> {
    match extension {
        "png" |"PNG" => Some(image::ImageFormat::PNG),
        "jpg" | "jpeg" | "JPEG" | "JPG" => Some(image::ImageFormat::JPEG),
        _ => None
    }
}

#[cfg(test)]
mod gray_test {
    use super::*;

    #[test]
    fn test_from_string() {
        assert_eq!(image_format_from_string("png").unwrap(), image::ImageFormat::PNG);
        assert_eq!(image_format_from_string("PNG").unwrap(), image::ImageFormat::PNG);
        assert_eq!(image_format_from_string("jpg").unwrap(), image::ImageFormat::JPEG);
        assert_eq!(image_format_from_string("blabla").is_some(), false);
    }
}