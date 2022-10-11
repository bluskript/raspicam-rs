#![warn(clippy::pedantic, clippy::nursery)]

//! # raspicam.rs
//! Abstraction documentation available at [`RaspiCam`](raspicam_rs::RaspiCam)
//! Example usage:
//! ```rs
//! use opencv::core::Vector;
//! use raspicam_rs::{
//!     bindings::{RASPICAM_EXPOSURE, RASPICAM_FORMAT},
//!     RaspiCam,
//! };
//!
//! fn main() {
//!     RaspiCam::new();
//!     let mut raspicam = raspicam_rs::RaspiCam::new();
//!     raspicam
//!         .set_brightness(50)
//!         .set_contrast(0)
//!         .set_format(RASPICAM_FORMAT::RASPICAM_FORMAT_RGB)
//!         .open(true);
//!     let frame = raspicam.grab_image_mat().unwrap();
//!     opencv::imgcodecs::imwrite("./frame.png", &frame, &Vector::default()).unwrap();
//! }
//! ```
//!

use autocxx::prelude::*;
pub mod autocxx_ffi_default_gen;
pub use autocxx_ffi_default_gen::ffi::raspicam as bindings;
use core::slice;
#[cfg(feature = "opencv")]
use opencv::prelude::Mat;
use thiserror::Error;
pub mod binding_generator;

pub struct RaspiCam {
    pub obj: UniquePtr<bindings::RaspiCam>,
}

#[derive(Error, Debug)]
pub enum CameraError {
    #[error("camera must be open before capturing")]
    CameraNotOpen,
    #[error("camera open fail")]
    CameraOpenFail,
    #[cfg(feature = "opencv")]
    #[error("opencv error")]
    OpenCVError(#[from] opencv::Error),
}

impl RaspiCam {
    /// Creates a new `RaspiCam` instance
    pub fn new() -> Self {
        Self {
            obj: bindings::RaspiCam::new().within_unique_ptr(),
        }
    }

    /// Opens the `RaspiCam` camera
    ///
    /// `start_capture` determines if camera must start capture or not
    /// # Errors
    /// This function can fail if the camera is already open
    pub fn open(&mut self, start_capture: bool) -> Result<(), CameraError> {
        if self.obj.pin_mut().open(start_capture) {
            Ok(())
        } else {
            Err(CameraError::CameraOpenFail)
        }
    }

    /// Sets the camera format
    /// Available formats are:
    /// - YUV420
    /// - GRAY
    /// - BGR
    /// - RGB
    /// - IGNORE
    pub fn set_format(&mut self, fmt: bindings::RASPICAM_FORMAT) -> &mut Self {
        self.obj.pin_mut().setFormat(fmt);
        self
    }

    /// Sets the dimensions of the camera capture
    pub fn set_capture_size(&mut self, width: u32, height: u32) -> &mut Self {
        self.obj
            .pin_mut()
            .setCaptureSize(width.into(), height.into());
        self
    }

    /// Sets the brightness of the camera
    ///
    /// `brightness` - range from 0-100
    pub fn set_brightness(&mut self, brightness: u32) -> &mut Self {
        self.obj.pin_mut().setBrightness(brightness.into());
        self
    }

    /// Sets the sharpness of the camera
    ///
    /// `sharpness` - range from -100-100
    pub fn set_sharpness(&mut self, sharpness: i32) -> &mut Self {
        self.obj.pin_mut().setSharpness(sharpness.into());
        self
    }

    /// Sets the contrast of the camera
    ///
    /// `contrast` - range from -100-100
    pub fn set_contrast(&mut self, contrast: i32) -> &mut Self {
        self.obj.pin_mut().setContrast(contrast.into());
        self
    }

    /// Sets the ISO of the camera
    ///
    /// `iso` - range from 100-800
    pub fn set_isoo(&mut self, iso: i32) -> &mut Self {
        self.obj.pin_mut().setISO(iso.into());
        self
    }

    /// Sets the saturation of the camera
    ///
    /// `saturation` - range from -100-100
    pub fn set_saturation(&mut self, saturation: i32) -> &mut Self {
        self.obj.pin_mut().setSaturation(saturation.into());
        self
    }

    /// Toggles video stabilization of the camera
    ///
    /// `v` - on or off
    pub fn set_video_stabilization(&mut self, v: bool) -> &mut Self {
        self.obj.pin_mut().setVideoStabilization(v);
        self
    }

    /// Set EV compensation of the camera
    ///
    /// `val` - range from -10 to 10
    pub fn set_exposure_compensation(&mut self, val: i32) -> &mut Self {
        self.obj.pin_mut().setExposureCompensation(val.into());
        self
    }

    pub fn set_rotation(&mut self, rotation: i32) -> &mut Self {
        self.obj.pin_mut().setRotation(rotation.into());
        self
    }

    pub fn set_exposure(&mut self, exposure: bindings::RASPICAM_EXPOSURE) -> &mut Self {
        self.obj.pin_mut().setExposure(exposure);
        self
    }

    pub fn set_shutter_speed(&mut self, ss: u32) -> &mut Self {
        self.obj.pin_mut().setShutterSpeed(ss.into());
        self
    }

    pub fn set_awb_rb(&mut self, r: f32, b: f32) -> &mut Self {
        self.obj.pin_mut().setAWB_RB(r, b);
        self
    }

    pub fn set_image_effect(&mut self, imageEffect: bindings::RASPICAM_IMAGE_EFFECT) -> &mut Self {
        self.obj.pin_mut().setImageEffect(imageEffect);
        self
    }

    pub fn set_metering(&mut self, metering: bindings::RASPICAM_METERING) -> &mut Self {
        self.obj.pin_mut().setMetering(metering);
        self
    }

    pub fn set_horizontal_flip(&mut self, hFlip: bool) -> &mut Self {
        self.obj.pin_mut().setHorizontalFlip(hFlip);
        self
    }

    pub fn set_vertical_flip(&mut self, vFlip: bool) -> &mut Self {
        self.obj.pin_mut().setVerticalFlip(vFlip);
        self
    }

    pub fn set_frame_rate(&mut self, frames_per_second: i32) -> &mut Self {
        self.obj.pin_mut().setFrameRate(frames_per_second.into());
        self
    }

    pub fn set_sensor_mode(&mut self, mode: i32) -> &mut Self {
        self.obj.pin_mut().setSensorMode(mode.into());
        self
    }

    /// # Errors
    /// `grab_raw` can fail if the camera is not opened
    /// This function can fail if the camera is not capturing
    pub fn grab_raw(&mut self) -> Result<(*mut u8, usize), CameraError> {
        if !self.obj.pin_mut().grab() {
            return Err(CameraError::CameraNotOpen);
        }
        Ok((self.obj.getImageBufferData(), self.obj.getImageBufferSize()))
    }

    /// # Errors
    /// `grab` can fail if the camera is not opened
    pub fn grab(&mut self) -> Result<&mut [u8], CameraError> {
        let (data, len) = self.grab_raw()?;
        Ok(unsafe { slice::from_raw_parts_mut(data, len) })
    }

    #[cfg(feature = "opencv")]
    /// # Errors
    /// `grab_image_mat` can return any error that [`Mat::new_nd_with_data`](https://docs.rs/opencv/latest/opencv/core/struct.Mat.html#method.new_nd_with_data) could return
    /// It could also fail if the camera is not opened
    pub fn grab_image_mat(&mut self) -> Result<Mat, CameraError> {
        use opencv::core::CV_8UC3;

        unsafe {
            Mat::new_nd_with_data(
                &[self.obj.getWidth().0 as i32, self.obj.getHeight().0 as i32],
                CV_8UC3,
                self.grab()?.as_mut_ptr().cast::<std::ffi::c_void>(),
                None,
            )
            .map_err(CameraError::OpenCVError)
        }
    }
}

impl Default for RaspiCam {
    fn default() -> Self {
        Self::new()
    }
}
