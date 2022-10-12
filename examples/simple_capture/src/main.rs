use image::RgbImage;
use raspicam_rs::{
    bindings::{RASPICAM_EXPOSURE, RASPICAM_FORMAT},
    RaspiCam,
};

fn main() {
    RaspiCam::new();
    let mut raspicam = raspicam_rs::RaspiCam::new();
    raspicam
        .set_capture_size(480, 480)
        .set_frame_rate(90)
        .set_format(RASPICAM_FORMAT::RASPICAM_FORMAT_RGB)
        .open(true)
        .unwrap();
    let img = RgbImage::from_raw(480, 480, raspicam.grab().unwrap().to_vec()).unwrap();
    img.save("frame.png").unwrap();
}
