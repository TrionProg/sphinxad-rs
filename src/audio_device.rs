use sphinxad_sys;

use std::ffi::CString;
use std::io;

use error::Error;

///Audio device like microphone
///It will automatically stop recording and will be destroyed
pub struct AudioDevice{
    device: *const sphinxad_sys::ad_rec_t,
    is_recording:bool,
}

impl AudioDevice{
    ///Open default audio device(microphone) with SPS(samples per second) = 16000
    pub fn default() -> io::Result<AudioDevice>{
        let device=unsafe{
            let device=sphinxad_sys::ad_open();

            if device.is_null() {
                return Err(io::Error::new(io::ErrorKind::Other, "Can not open default audio device"));
            }

            device
        };

        Ok(
            AudioDevice{
                device:device,
                is_recording:false,
            }
        )
    }

    ///Open default audio device(microphone) with given SPS
    ///
    ///# Example
    ///
    ///```
    ///use std::thread;
    ///
    ///let mut device=AudioDevice::default_with_sps(44100)?;
    ///device.start_recording()?;
    ///
    ///loop{ //recording loop
    ///    match device.read(&mut buffer[..])? {
    ///        Some( buffer ) => {
    ///            for i in 0..buffer.len() {
    ///                ...
    ///            }
    ///        },
    ///        None => break,
    ///    }
    ///
    ///    thread::sleep_ms(100);
    ///}
    ///```
    pub fn default_with_sps(samples_per_second:usize) -> io::Result<AudioDevice>{
        let device=unsafe{
            let device=sphinxad_sys::ad_open_sps(samples_per_second as u32);

            if device.is_null() {
                return Err(io::Error::new(io::ErrorKind::Other, "Can not open default audio device"));
            }

            device
        };

        Ok(
            AudioDevice{
                device:device,
                is_recording:false,
            }
        )
    }

    ///Open specified audio device with given SPS
    pub fn with_name_and_sps(name:&str, samples_per_second:usize) -> io::Result<AudioDevice>{
        let device=unsafe{
            let device_name=CString::new(name)?;
            let device=sphinxad_sys::ad_open_dev(device_name.as_ptr(), samples_per_second as u32);

            if device.is_null() {
                return Err(io::Error::new(io::ErrorKind::Other, format!("Can not open audio device \"{}\"", name)));
            }

            device
        };

        Ok(
            AudioDevice{
                device:device,
                is_recording:false,
            }
        )
    }

    ///Start recording
    pub fn start_recording(&mut self) -> io::Result<()> {
        if self.is_recording {
            return Ok(())
        }

        unsafe{
            let result=sphinxad_sys::ad_start_rec(self.device);

            if result!=sphinxad_sys::AD_OK {
                return Err(io::Error::new(io::ErrorKind::Other, Error::from(result)));
            }

            self.is_recording=true;
        }

        Ok(())
    }

    ///Stop recording
    ///
    ///# Example
    ///
    ///```
    ///let mut device=AudioDevice::default_with_sps(44100)?;
    ///device.start_recording()?;
    ///
    ///loop{ //recording loop
    ///    ...
    ///}
    ///
    ///device.stop_recording()?;
    ///
    ///thread::sleep_ms(1000);
    ///
    ///device.start_recording()?;
    ///
    ///```
    pub fn stop_recording(&mut self) -> io::Result<()> {
        if !self.is_recording {
            return Ok(())
        }

        unsafe{
            let result=sphinxad_sys::ad_stop_rec(self.device);

            if result!=sphinxad_sys::AD_OK {
                return Err(io::Error::new(io::ErrorKind::Other, Error::from(result)));
            }

            self.is_recording=false;
        }

        Ok(())
    }

    ///Read sound to buffer, and return slide of read samples(length of slice may be not equal to buffer length). Return None if nothing to read
    pub fn read<'a>(&self, buffer:&'a mut [i16]) -> io::Result<Option<&'a [i16]>>{
        if !self.is_recording {
            return Err(io::Error::new(io::ErrorKind::Other, "Device is not recording"));
        }

        let raw=&mut buffer[0] as *mut i16;
        let n=unsafe{ sphinxad_sys::ad_read(self.device, raw, buffer.len() as u32) };

        if n==0 {
            return Ok(None);
        }

        if n<0 {
            return Err(io::Error::new(io::ErrorKind::Other, Error::from(n)));
        }

        return Ok(Some(&buffer[0..n as usize]))
    }
}

impl Drop for AudioDevice {
    fn drop(&mut self) {
        if !self.device.is_null() {
            unsafe{
                if self.is_recording {
                    sphinxad_sys::ad_stop_rec(self.device);
                }

                sphinxad_sys::ad_close(self.device);
            }
        }
    }
}
