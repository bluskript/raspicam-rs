#![warn(clippy::pedantic, clippy::nursery)]

use autocxx::prelude::*;
pub mod autocxx_ffi_default_gen;
pub use autocxx_ffi_default_gen::ffi::raspicam as bindings;
use core::slice;
#[cfg(feature = "opencv")]
use opencv::prelude::Mat;
pub mod binding_generator;

pub struct RaspiCam {
    pub obj: UniquePtr<bindings::RaspiCam>,
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
    pub fn open(&mut self, start_capture: bool) {
        self.obj.pin_mut().open(start_capture);
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

    pub fn grab_raw(&mut self) -> (*mut u8, usize) {
        self.obj.pin_mut().grab();
        (self.obj.getImageBufferData(), self.obj.getImageBufferSize())
    }

    pub fn grab(&mut self) -> &mut [u8] {
        let (data, len) = self.grab_raw();
        unsafe { slice::from_raw_parts_mut(data, len) }
    }

    #[cfg(feature = "opencv")]
    /// # Errors
    /// `grab_image_mat` can return any error that [`Mat::new_nd_with_data`](https://docs.rs/opencv/latest/opencv/core/struct.Mat.html#method.new_nd_with_data) could return
    pub fn grab_image_mat(&mut self) -> opencv::Result<Mat> {
        use opencv::core::CV_8UC3;

        unsafe {
            Mat::new_nd_with_data(
                &[self.obj.getWidth().0 as i32, self.obj.getHeight().0 as i32],
                CV_8UC3,
                self.grab().as_mut_ptr().cast::<std::ffi::c_void>(),
                None,
            )
        }
    }
}

impl Default for RaspiCam {
    fn default() -> Self {
        Self::new()
    }
}
