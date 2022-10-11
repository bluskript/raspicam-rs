use std::time::{Duration, Instant};

fn main() {
    let mut camera = raspicam_rs::RaspiCam::new();
    camera
        .set_format(raspicam_rs::bindings::RASPICAM_FORMAT::RASPICAM_FORMAT_RGB)
        .set_exposure(raspicam_rs::bindings::RASPICAM_EXPOSURE::RASPICAM_EXPOSURE_OFF)
        .set_capture_size(480, 480)
        .set_frame_rate(90)
        .set_sensor_mode(7)
        .set_shutter_speed(15000)
        .open(true)
        .unwrap();
    let mut start = Instant::now();
    let mut counter = 0;
    loop {
        let data = camera.grab().unwrap();
        counter += 1;
        if Instant::now().duration_since(start) > Duration::from_secs(1) {
            println!("{counter} FPS");
            counter = 0;
            start = Instant::now();
        }
    }
}
