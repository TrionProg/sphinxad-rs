extern crate sphinxad;
extern crate hound;

use sphinxad::AudioDevice;
use std::io;
use std::thread;
use std::sync::{Arc,Mutex};

const SPHINX_FREQ:u32=16000; //sphinx want 16kSPS

fn record_and_write_to_file() -> io::Result<()> {
    println!("input name of outer wav file");

    let mut wavFileName = String::new();
    io::stdin().read_line(&mut wavFileName)?;

    if wavFileName.trim().len()==0 {
        return Err(io::Error::new(io::ErrorKind::Other, "no name of outer wav file "));
    }

    //specify wav file
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SPHINX_FREQ,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(wavFileName.as_str().trim(), spec).unwrap();

    //open Microphone
    let mut device=AudioDevice::default()?;

    loop{
        println!("input `start` to record or `exit`");

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.as_str().trim(){
            "exit" => break,
            "start" => {
                println!("recording...");

                let shouldStop=Arc::new(Mutex::new(false));
                let shouldStopWait=shouldStop.clone();

                let handle=thread::spawn(move||{
                    loop{
                        println!("input `stop` to stop recording");

                        let mut input = String::new();
                        io::stdin().read_line(&mut input);

                        if input.as_str().trim()=="stop" {
                            *shouldStopWait.lock().unwrap()=true;
                            break;
                        }
                    }
                });

                let mut buffer = [0i16; 2048];

                //start recording
                device.start_recording()?;

                while !{*shouldStop.lock().unwrap()} {//TODO:strange behaviour, last part of sound do not record
                    //record to buffer
                    match device.read(&mut buffer[..])? {
                        Some( buffer ) => {
                            for i in 0..buffer.len() {
                                writer.write_sample(buffer[i]).unwrap();
                            }
                        },
                        None => break,
                    }

                    thread::sleep_ms(100);
                }

                //stop recording
                device.stop_recording()?;

                handle.join();

                println!("stoped");
            },
            _=>println!("unknown command"),
        }
    }

    writer.finalize();

    Ok(())
    //device will be closed here
}

fn main(){
    match record_and_write_to_file(){
        Ok ( _ ) => println!("ok"),
        Err( e ) => println!("Error:{:?}",e),
    }
}
