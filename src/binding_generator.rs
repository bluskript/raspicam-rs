use autocxx::include_cpp;
include_cpp! {
  #include "raspicam.h"
  safety!(unsafe_ffi)
  generate!("raspicam::RaspiCam")
}
