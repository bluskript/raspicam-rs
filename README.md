# raspicam-rs

[![docs.rs](https://img.shields.io/docsrs/raspicam-rs?style=flat-square)](https://docs.rs/raspicam-rs/latest)
[![Crates.io](https://img.shields.io/crates/d/raspicam-rs?style=flat-square)](https://crates.io/crates/raspicam-rs)

Rust bindings to the amazing C++ [raspicam](https://github.com/cedricve/raspicam) library (with optional OpenCV utilities)!

This is a followup to a Rust-based robotics project I worked where high FPS low latency and full color
support was a requirement: [ez-aquarii](https://github.com/Orion-Robotics/ez-aquarii/)

## Prerequisites

**You MUST be on a 32-bit install. The raspberry pi camera libraries do NOT work on a 64-bit installation. See: https://github.com/raspberrypi/userland/issues/688**

---

A prerequisite is you have to compile the `raspicam` library and install it. This repo has a install script for convenience that'll do everything for you:

```
curl https://raw.githubusercontent.com/bluskript/raspicam-rs/master/install.sh | sudo bash
```

Install dependencies:

```
sudo apt install clang libclang-dev
```

If you are using opencv integration, also install `libopencv-dev`
and enable the opencv feature in the crate:

```rs
raspicam-rs = { version = "0.1.2", features = ["opencv"] }
opencv = "0.70.0"
```

---

Example usage showing how to capture a single image and save it:

```rs
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
```

### How to configure

These bindings use something similar to a builder pattern to configure the camera. You can chain configuration commands together:

```rs
let mut raspicam = raspicam_rs::RaspiCam::new();
raspicam
    .set_brightness(50)
    .set_contrast(0)
    .set_exposure(RASPICAM_EXPOSURE::RASPICAM_EXPOSURE_OFF)
    .set_format(RASPICAM_FORMAT::RASPICAM_FORMAT_RGB)
    .set_sensor_mode(7)
    .open(true);
```

If there is a method that isn't available directly, you can call methods on the internal C++ object like so:

```rs
let mut camera = raspicam_rs::RaspiCam::new();
raspicam.obj.pin_mut().startCapture();
```

### Achieving high FPS low latency fullcolor

In my experience, high FPS fullcolor realtime processing can be achieved using low resolutions (480x480 as an example), sensor mode 7, shutter speed 15000, and setting the framerate 90.

A real-world demo of low latency camera capturing (predates this library but same result can be achieved): [Check Here](https://github.com/Orion-Robotics/ez-aquarii/blob/master/controller/src/modules/camera/mod.rs#L167)
