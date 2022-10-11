if [[ $UID != 0 ]]; then
  echo "Please run this script with sudo:"
  echo "sudo $0 $*"
  exit 1
fi

rm -rf ./raspicam-build
git clone --depth 1 https://github.com/cedricve/raspicam
cd raspicam-build
mkdir build
cd build
cmake ..
make
make install
ldconfig
rm -rf ./raspicam-build
