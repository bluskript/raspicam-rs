#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
pub mod ffi {
    pub trait ToCppString {
        fn into_cpp(self) -> cxx::UniquePtr<cxx::CxxString>;
    }
    impl ToCppString for &str {
        fn into_cpp(self) -> cxx::UniquePtr<cxx::CxxString> {
            make_string(self)
        }
    }
    impl ToCppString for String {
        fn into_cpp(self) -> cxx::UniquePtr<cxx::CxxString> {
            make_string(&self)
        }
    }
    impl ToCppString for &String {
        fn into_cpp(self) -> cxx::UniquePtr<cxx::CxxString> {
            make_string(self)
        }
    }
    impl ToCppString for cxx::UniquePtr<cxx::CxxString> {
        fn into_cpp(self) -> cxx::UniquePtr<cxx::CxxString> {
            self
        }
    }
    unsafe impl cxx::ExternType for bindgen::root::raspicam::RaspiCam {
        type Id = cxx::type_id!("raspicam::RaspiCam");
        type Kind = cxx::kind::Opaque;
    }
    unsafe impl cxx::ExternType for bindgen::root::raspicam::RASPICAM_FORMAT {
        type Id = cxx::type_id!("raspicam::RASPICAM_FORMAT");
        type Kind = cxx::kind::Trivial;
    }
    unsafe impl cxx::ExternType for bindgen::root::raspicam::RASPICAM_EXPOSURE {
        type Id = cxx::type_id!("raspicam::RASPICAM_EXPOSURE");
        type Kind = cxx::kind::Trivial;
    }
    unsafe impl cxx::ExternType for bindgen::root::raspicam::RASPICAM_AWB {
        type Id = cxx::type_id!("raspicam::RASPICAM_AWB");
        type Kind = cxx::kind::Trivial;
    }
    unsafe impl cxx::ExternType for bindgen::root::raspicam::RASPICAM_IMAGE_EFFECT {
        type Id = cxx::type_id!("raspicam::RASPICAM_IMAGE_EFFECT");
        type Kind = cxx::kind::Trivial;
    }
    unsafe impl cxx::ExternType for bindgen::root::raspicam::RASPICAM_METERING {
        type Id = cxx::type_id!("raspicam::RASPICAM_METERING");
        type Kind = cxx::kind::Trivial;
    }
    mod bindgen {
        pub(super) mod root {
            pub mod raspicam {
                #[doc = "Base class that do all the hard work"]
                #[repr(C, align(8))]
                pub struct RaspiCam {
                    _pinned: core::marker::PhantomData<core::marker::PhantomPinned>,
                    _non_send_sync: core::marker::PhantomData<[*const u8; 0]>,
                    _data: [u8; 8],
                }
                #[repr(u32)]
                #[doc = "Image formats"]
                #[derive(Clone, Hash, PartialEq, Eq)]
                pub enum RASPICAM_FORMAT {
                    RASPICAM_FORMAT_YUV420 = 0,
                    RASPICAM_FORMAT_GRAY = 1,
                    RASPICAM_FORMAT_BGR = 2,
                    RASPICAM_FORMAT_RGB = 3,
                    RASPICAM_FORMAT_IGNORE = 4,
                }
                #[repr(u32)]
                #[doc = "Exposure types"]
                #[derive(Clone, Hash, PartialEq, Eq)]
                pub enum RASPICAM_EXPOSURE {
                    RASPICAM_EXPOSURE_OFF = 0,
                    RASPICAM_EXPOSURE_AUTO = 1,
                    RASPICAM_EXPOSURE_NIGHT = 2,
                    RASPICAM_EXPOSURE_NIGHTPREVIEW = 3,
                    RASPICAM_EXPOSURE_BACKLIGHT = 4,
                    RASPICAM_EXPOSURE_SPOTLIGHT = 5,
                    RASPICAM_EXPOSURE_SPORTS = 6,
                    RASPICAM_EXPOSURE_SNOW = 7,
                    RASPICAM_EXPOSURE_BEACH = 8,
                    RASPICAM_EXPOSURE_VERYLONG = 9,
                    RASPICAM_EXPOSURE_FIXEDFPS = 10,
                    RASPICAM_EXPOSURE_ANTISHAKE = 11,
                    RASPICAM_EXPOSURE_FIREWORKS = 12,
                }
                #[repr(u32)]
                #[doc = "Auto white balance types"]
                #[derive(Clone, Hash, PartialEq, Eq)]
                pub enum RASPICAM_AWB {
                    RASPICAM_AWB_OFF = 0,
                    RASPICAM_AWB_AUTO = 1,
                    RASPICAM_AWB_SUNLIGHT = 2,
                    RASPICAM_AWB_CLOUDY = 3,
                    RASPICAM_AWB_SHADE = 4,
                    RASPICAM_AWB_TUNGSTEN = 5,
                    RASPICAM_AWB_FLUORESCENT = 6,
                    RASPICAM_AWB_INCANDESCENT = 7,
                    RASPICAM_AWB_FLASH = 8,
                    RASPICAM_AWB_HORIZON = 9,
                }
                #[repr(u32)]
                #[doc = "Image effects"]
                #[derive(Clone, Hash, PartialEq, Eq)]
                pub enum RASPICAM_IMAGE_EFFECT {
                    RASPICAM_IMAGE_EFFECT_NONE = 0,
                    RASPICAM_IMAGE_EFFECT_NEGATIVE = 1,
                    RASPICAM_IMAGE_EFFECT_SOLARIZE = 2,
                    RASPICAM_IMAGE_EFFECT_SKETCH = 3,
                    RASPICAM_IMAGE_EFFECT_DENOISE = 4,
                    RASPICAM_IMAGE_EFFECT_EMBOSS = 5,
                    RASPICAM_IMAGE_EFFECT_OILPAINT = 6,
                    RASPICAM_IMAGE_EFFECT_HATCH = 7,
                    RASPICAM_IMAGE_EFFECT_GPEN = 8,
                    RASPICAM_IMAGE_EFFECT_PASTEL = 9,
                    RASPICAM_IMAGE_EFFECT_WATERCOLOR = 10,
                    RASPICAM_IMAGE_EFFECT_FILM = 11,
                    RASPICAM_IMAGE_EFFECT_BLUR = 12,
                    RASPICAM_IMAGE_EFFECT_SATURATION = 13,
                    RASPICAM_IMAGE_EFFECT_COLORSWAP = 14,
                    RASPICAM_IMAGE_EFFECT_WASHEDOUT = 15,
                    RASPICAM_IMAGE_EFFECT_POSTERISE = 16,
                    RASPICAM_IMAGE_EFFECT_COLORPOINT = 17,
                    RASPICAM_IMAGE_EFFECT_COLORBALANCE = 18,
                    RASPICAM_IMAGE_EFFECT_CARTOON = 19,
                }
                #[repr(u32)]
                #[doc = "Metering types"]
                #[derive(Clone, Hash, PartialEq, Eq)]
                pub enum RASPICAM_METERING {
                    RASPICAM_METERING_AVERAGE = 0,
                    RASPICAM_METERING_SPOT = 1,
                    RASPICAM_METERING_BACKLIT = 2,
                    RASPICAM_METERING_MATRIX = 3,
                }
                impl RaspiCam {
                    #[doc = "autocxx bindings couldn't be generated: autocxx does not yet know how to support the built-in C++ type std::option::Option - please raise an issue on github"]
                    fn setUserCallback(_uhoh: autocxx::BindingGenerationFailure) {}
                    #[doc = " Returns an id of the camera. We assume the camera id is the one of the raspberry"]
                    #[doc = "the id is obtained using raspberry serial number obtained in /proc/cpuinfo"]
                    pub fn getId(
                        self: &root::raspicam::RaspiCam,
                    ) -> cxx::UniquePtr<cxx::CxxString> {
                        cxxbridge::getId_autocxx_wrapper(self)
                    }
                    #[doc = "Constructor"]
                    pub fn new() -> impl autocxx::moveit::new::New<Output = Self> {
                        unsafe {
                            autocxx::moveit::new::by_raw(move |this| {
                                let this = this.get_unchecked_mut().as_mut_ptr();
                                cxxbridge::new_autocxx_autocxx_wrapper(this)
                            })
                        }
                    }
                }
                unsafe impl autocxx::moveit::MakeCppStorage for root::raspicam::RaspiCam {
                    unsafe fn allocate_uninitialized_cpp_storage() -> *mut root::raspicam::RaspiCam
                    {
                        cxxbridge::RaspiCam_alloc_autocxx_wrapper()
                    }
                    unsafe fn free_uninitialized_cpp_storage(arg0: *mut root::raspicam::RaspiCam) {
                        cxxbridge::RaspiCam_free_autocxx_wrapper(arg0)
                    }
                }
                impl Drop for root::raspicam::RaspiCam {
                    #[doc = "Destructor"]
                    fn drop(self: &mut root::raspicam::RaspiCam) {
                        unsafe { cxxbridge::RaspiCam_destructor_autocxx_wrapper(self) }
                    }
                }
                unsafe impl autocxx::moveit::new::CopyNew for root::raspicam::RaspiCam {
                    #[doc = "Synthesized copy constructor."]
                    unsafe fn copy_new(
                        other: &root::raspicam::RaspiCam,
                        this: ::std::pin::Pin<
                            &mut ::std::mem::MaybeUninit<root::raspicam::RaspiCam>,
                        >,
                    ) {
                        cxxbridge::new_synthetic_const_copy_ctor_0x84f8a11596003003_autocxx_wrapper(
                            this.get_unchecked_mut().as_mut_ptr(),
                            other,
                        )
                    }
                }
                #[allow(unused_imports)]
                use self::super::super::super::{cxxbridge, ToCppString};
                #[allow(unused_imports)]
                use self::super::super::root;
            }
            #[allow(unused_imports)]
            use self::super::super::{cxxbridge, ToCppString};
            #[allow(unused_imports)]
            use self::super::root;
        }
    }
    #[cxx::bridge]
    mod cxxbridge {
        impl UniquePtr<RaspiCam> {}
        impl SharedPtr<RaspiCam> {}
        impl WeakPtr<RaspiCam> {}
        impl UniquePtr<RASPICAM_FORMAT> {}
        impl SharedPtr<RASPICAM_FORMAT> {}
        impl WeakPtr<RASPICAM_FORMAT> {}
        impl CxxVector<RASPICAM_FORMAT> {}
        impl UniquePtr<RASPICAM_EXPOSURE> {}
        impl SharedPtr<RASPICAM_EXPOSURE> {}
        impl WeakPtr<RASPICAM_EXPOSURE> {}
        impl CxxVector<RASPICAM_EXPOSURE> {}
        impl UniquePtr<RASPICAM_AWB> {}
        impl SharedPtr<RASPICAM_AWB> {}
        impl WeakPtr<RASPICAM_AWB> {}
        impl CxxVector<RASPICAM_AWB> {}
        impl UniquePtr<RASPICAM_IMAGE_EFFECT> {}
        impl SharedPtr<RASPICAM_IMAGE_EFFECT> {}
        impl WeakPtr<RASPICAM_IMAGE_EFFECT> {}
        impl CxxVector<RASPICAM_IMAGE_EFFECT> {}
        impl UniquePtr<RASPICAM_METERING> {}
        impl SharedPtr<RASPICAM_METERING> {}
        impl WeakPtr<RASPICAM_METERING> {}
        impl CxxVector<RASPICAM_METERING> {}
        unsafe extern "C++" {
            fn autocxx_make_string_0x84f8a11596003003(str_: &str) -> UniquePtr<CxxString>;
            pub unsafe fn RaspiCam_alloc_autocxx_wrapper() -> *mut RaspiCam;
            pub unsafe fn RaspiCam_free_autocxx_wrapper(arg0: *mut RaspiCam);
            #[namespace = "raspicam"]
            #[doc = "Base class that do all the hard work"]
            type RaspiCam = super::bindgen::root::raspicam::RaspiCam;
            #[namespace = "raspicam"]
            #[doc = "Opens the camera"]
            #[doc = " @param StartCapture determines if camera must start capture or not."]
            pub fn open(self: Pin<&mut RaspiCam>, StartCapture: bool) -> bool;
            #[namespace = "raspicam"]
            #[doc = "Makes camera start capturing"]
            pub fn startCapture(self: Pin<&mut RaspiCam>) -> bool;
            #[namespace = "raspicam"]
            #[doc = "indicates if camera is open"]
            pub fn isOpened(self: &RaspiCam) -> bool;
            #[namespace = "raspicam"]
            #[doc = "Grabs the next frame and keeps it in internal buffer. Blocks until next frame arrives"]
            pub fn grab(self: Pin<&mut RaspiCam>) -> bool;
            #[namespace = "raspicam"]
            #[doc = "Retrieves the buffer previously grabbed."]
            #[doc = " You can decide how image is given by setting type. The input buffer provided must have the appropriate"]
            #[doc = " size accordingly. You can use getImageTypeSize() to determine the size"]
            #[doc = " NOTE: Change in version 0.0.5. Format is stablished in setFormat function"]
            #[doc = " So type param is ignored. Do not use this parameter."]
            #[doc = " You can use getFormat() to know the current format"]
            pub unsafe fn retrieve(self: Pin<&mut RaspiCam>, data: *mut u8, type_: RASPICAM_FORMAT);
            #[namespace = "raspicam"]
            #[doc = "Alternative to retrieve. Returns a pointer to the original image data buffer (which is in getFormat() format)."]
            #[doc = ""]
            #[doc = " Be careful, if you call grab(), this will be rewritten with the new data"]
            pub fn getImageBufferData(self: &RaspiCam) -> *mut u8;
            #[namespace = "raspicam"]
            #[doc = " Returns the size of the images captured."]
            pub fn getImageBufferSize(self: &RaspiCam) -> usize;
            #[namespace = "raspicam"]
            #[doc = " Stops camera and free resources"]
            pub fn release(self: Pin<&mut RaspiCam>);
            #[namespace = "raspicam"]
            #[doc = "Sets capture format"]
            pub fn setFormat(self: Pin<&mut RaspiCam>, fmt: RASPICAM_FORMAT);
            #[namespace = "raspicam"]
            #[doc = "Sets camera width. Use a multiple of 320 (640, 1280)"]
            pub fn setWidth(self: Pin<&mut RaspiCam>, width: c_uint);
            #[namespace = "raspicam"]
            #[doc = "Sets camera Height. Use a multiple of 240 (480, 960)"]
            pub fn setHeight(self: Pin<&mut RaspiCam>, height: c_uint);
            #[namespace = "raspicam"]
            pub fn setCaptureSize(self: Pin<&mut RaspiCam>, width: c_uint, height: c_uint);
            #[namespace = "raspicam"]
            #[doc = " Set image brightness [0,100]"]
            pub fn setBrightness(self: Pin<&mut RaspiCam>, brightness: c_uint);
            #[namespace = "raspicam"]
            #[doc = " Set image sharpness (-100 to 100)"]
            pub fn setSharpness(self: Pin<&mut RaspiCam>, sharpness: c_int);
            #[namespace = "raspicam"]
            #[doc = "  Set image contrast (-100 to 100)"]
            pub fn setContrast(self: Pin<&mut RaspiCam>, contrast: c_int);
            #[namespace = "raspicam"]
            #[doc = " Set capture ISO (100 to 800)"]
            pub fn setISO(self: Pin<&mut RaspiCam>, iso: c_int);
            #[namespace = "raspicam"]
            #[doc = " Set image saturation (-100 to 100)"]
            pub fn setSaturation(self: Pin<&mut RaspiCam>, saturation: c_int);
            #[namespace = "raspicam"]
            #[doc = "Sets on/off video stabilisation"]
            pub fn setVideoStabilization(self: Pin<&mut RaspiCam>, v: bool);
            #[namespace = "raspicam"]
            #[doc = "  Set EV compensation (-10,10)"]
            pub fn setExposureCompensation(self: Pin<&mut RaspiCam>, val: c_int);
            #[namespace = "raspicam"]
            pub fn setRotation(self: Pin<&mut RaspiCam>, rotation: c_int);
            #[namespace = "raspicam"]
            pub fn setExposure(self: Pin<&mut RaspiCam>, exposure: RASPICAM_EXPOSURE);
            #[namespace = "raspicam"]
            pub fn setShutterSpeed(self: Pin<&mut RaspiCam>, ss: c_uint);
            #[namespace = "raspicam"]
            pub fn setAWB(self: Pin<&mut RaspiCam>, awb: RASPICAM_AWB);
            #[namespace = "raspicam"]
            pub fn setAWB_RB(self: Pin<&mut RaspiCam>, r: f32, b: f32);
            #[namespace = "raspicam"]
            pub fn setImageEffect(self: Pin<&mut RaspiCam>, imageEffect: RASPICAM_IMAGE_EFFECT);
            #[namespace = "raspicam"]
            pub fn setMetering(self: Pin<&mut RaspiCam>, metering: RASPICAM_METERING);
            #[namespace = "raspicam"]
            pub fn setHorizontalFlip(self: Pin<&mut RaspiCam>, hFlip: bool);
            #[namespace = "raspicam"]
            pub fn setVerticalFlip(self: Pin<&mut RaspiCam>, vFlip: bool);
            #[namespace = "raspicam"]
            pub fn setFrameRate(self: Pin<&mut RaspiCam>, frames_per_second: c_int);
            #[namespace = "raspicam"]
            pub fn setSensorMode(self: Pin<&mut RaspiCam>, mode: c_int);
            #[namespace = "raspicam"]
            pub fn getFormat(self: &RaspiCam) -> RASPICAM_FORMAT;
            #[namespace = "raspicam"]
            pub fn getWidth(self: &RaspiCam) -> c_uint;
            #[namespace = "raspicam"]
            pub fn getHeight(self: &RaspiCam) -> c_uint;
            #[namespace = "raspicam"]
            pub fn getBrightness(self: &RaspiCam) -> c_uint;
            #[namespace = "raspicam"]
            pub fn getRotation(self: &RaspiCam) -> c_uint;
            #[namespace = "raspicam"]
            pub fn getISO(self: &RaspiCam) -> c_int;
            #[namespace = "raspicam"]
            pub fn getSharpness(self: &RaspiCam) -> c_int;
            #[namespace = "raspicam"]
            pub fn getContrast(self: &RaspiCam) -> c_int;
            #[namespace = "raspicam"]
            pub fn getSaturation(self: &RaspiCam) -> c_int;
            #[namespace = "raspicam"]
            pub fn getShutterSpeed(self: &RaspiCam) -> c_uint;
            #[namespace = "raspicam"]
            pub fn getExposure(self: &RaspiCam) -> RASPICAM_EXPOSURE;
            #[namespace = "raspicam"]
            pub fn getAWB(self: &RaspiCam) -> RASPICAM_AWB;
            #[namespace = "raspicam"]
            pub fn getAWBG_red(self: &RaspiCam) -> f32;
            #[namespace = "raspicam"]
            pub fn getAWBG_blue(self: &RaspiCam) -> f32;
            #[namespace = "raspicam"]
            pub fn getImageEffect(self: &RaspiCam) -> RASPICAM_IMAGE_EFFECT;
            #[namespace = "raspicam"]
            pub fn getMetering(self: &RaspiCam) -> RASPICAM_METERING;
            #[namespace = "raspicam"]
            pub fn getFrameRate(self: &RaspiCam) -> c_int;
            #[namespace = "raspicam"]
            pub fn isHorizontallyFlipped(self: &RaspiCam) -> bool;
            #[namespace = "raspicam"]
            pub fn isVerticallyFlipped(self: &RaspiCam) -> bool;
            #[doc = " Returns an id of the camera. We assume the camera id is the one of the raspberry"]
            #[doc = "the id is obtained using raspberry serial number obtained in /proc/cpuinfo"]
            pub fn getId_autocxx_wrapper(autocxx_gen_this: &RaspiCam) -> UniquePtr<CxxString>;
            #[namespace = "raspicam"]
            #[doc = "Returns the size of the required buffer for the different image types in retrieve"]
            pub fn getImageTypeSize(self: &RaspiCam, type_: RASPICAM_FORMAT) -> usize;
            #[doc = "Constructor"]
            pub unsafe fn new_autocxx_autocxx_wrapper(autocxx_gen_this: *mut RaspiCam);
            #[doc = "Destructor"]
            pub unsafe fn RaspiCam_destructor_autocxx_wrapper(autocxx_gen_this: *mut RaspiCam);
            #[doc = "Synthesized copy constructor."]
            pub unsafe fn new_synthetic_const_copy_ctor_0x84f8a11596003003_autocxx_wrapper(
                autocxx_gen_this: *mut RaspiCam,
                other: &RaspiCam,
            );
            #[namespace = "raspicam"]
            #[doc = "Image formats"]
            type RASPICAM_FORMAT = super::bindgen::root::raspicam::RASPICAM_FORMAT;
            #[namespace = "raspicam"]
            #[doc = "Exposure types"]
            type RASPICAM_EXPOSURE = super::bindgen::root::raspicam::RASPICAM_EXPOSURE;
            #[namespace = "raspicam"]
            #[doc = "Auto white balance types"]
            type RASPICAM_AWB = super::bindgen::root::raspicam::RASPICAM_AWB;
            #[namespace = "raspicam"]
            #[doc = "Image effects"]
            type RASPICAM_IMAGE_EFFECT = super::bindgen::root::raspicam::RASPICAM_IMAGE_EFFECT;
            #[namespace = "raspicam"]
            #[doc = "Metering types"]
            type RASPICAM_METERING = super::bindgen::root::raspicam::RASPICAM_METERING;
            type c_uint = autocxx::c_uint;
            type c_int = autocxx::c_int;
            include!("raspicam.h");
            include!("autocxxgen_ffi.h");
        }
        extern "Rust" {}
    }
    #[allow(unused_imports)]
    use bindgen::root;
    pub use cxxbridge::autocxx_make_string_0x84f8a11596003003 as make_string;
    pub mod raspicam {
        pub use super::bindgen::root::raspicam::RaspiCam;
        pub use super::bindgen::root::raspicam::RASPICAM_AWB;
        pub use super::bindgen::root::raspicam::RASPICAM_EXPOSURE;
        pub use super::bindgen::root::raspicam::RASPICAM_FORMAT;
        pub use super::bindgen::root::raspicam::RASPICAM_IMAGE_EFFECT;
        pub use super::bindgen::root::raspicam::RASPICAM_METERING;
    }
}
