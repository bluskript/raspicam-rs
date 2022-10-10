use opencv::core::Vector;
use raspicam_rs::{
    bindings::{RASPICAM_EXPOSURE, RASPICAM_FORMAT},
    RaspiCam,
};

fn main() {
    RaspiCam::new();
    let mut raspicam = raspicam_rs::RaspiCam::new();
    raspicam.obj.pin_mut().startCapture();
    raspicam
        .set_brightness(50)
        .set_contrast(0)
        .set_exposure(RASPICAM_EXPOSURE::RASPICAM_EXPOSURE_OFF)
        .set_format(RASPICAM_FORMAT::RASPICAM_FORMAT_RGB)
        .open(true);
    let frame = raspicam.grab_image_mat().unwrap();
    opencv::imgcodecs::imwrite("./frame.png", &frame, &Vector::default()).unwrap();
}
