use std::borrow::{Cow};
use std::path::{PathBuf, Path};
use sdl2::audio::{AudioCallback, AudioSpecDesired, AudioSpecWAV, AudioCVT};


use super::sdl2::audio::AudioDevice;
use super::sdl2::Sdl;
use std::collections::HashMap;

struct Sound {
    data: Vec<u8>,
    volume: f32,
    pos: usize,
    end: usize,
    counter: i32,
    id: usize,
}

pub enum Playing {
    Playing,
    Ended,
    Finished,
}

impl AudioCallback for Sound {
    type Channel = u8;

    fn callback(&mut self, out: &mut [u8]) {
        for dst in out.iter_mut() {
            let pre_scale = *self.data.get(self.pos).unwrap_or(&128);
            let scaled_signed_float = (pre_scale as f32 - 128.0) * self.volume;
            let scaled = (scaled_signed_float + 128.0) as u8;
            *dst = scaled;

            unsafe {
                let currently = PLAYING_MAP.as_mut().unwrap().get(&self.id);
                match currently {
                    Some(Playing::Playing) => {
                        self.pos += 1;
                        self.counter = self.counter + 1;
                        if self.pos >= self.end {
                            PLAYING_MAP.as_mut().unwrap().insert(self.id, Playing::Ended);
                            self.pos = 0;
                        }
                    },
                    _ => {

                    },
                }
            }
        }
    }
}


static mut PLAYING_MAP: Option<HashMap<usize, Playing>> = None;
static mut SOUNDS_MAP: Option<HashMap<usize, AudioDevice<Sound>>> = None;

pub static CHOMP: usize = 0;
pub static DEAD: usize = 1;
pub static GOTGHOST: usize = 4;


pub fn load_sound(sdl_context: &Sdl) {
    unsafe {
        PLAYING_MAP = Some(HashMap::new());
        SOUNDS_MAP = Some(HashMap::new());
        SOUNDS_MAP.as_mut().unwrap().insert(CHOMP, load_in_file(sdl_context, "artifacts/Chomping.wav", CHOMP, 2600));
        SOUNDS_MAP.as_mut().unwrap().insert(GOTGHOST, load_in_file(sdl_context, "artifacts/gotghost.wav", GOTGHOST, 2500));
        SOUNDS_MAP.as_mut().unwrap().insert(DEAD, load_in_file(sdl_context, "artifacts/dead.wav", DEAD, 3000));
    }
}

pub fn play(id: usize) {
    unsafe {
        PLAYING_MAP.as_mut().unwrap().insert(id, Playing::Playing);
        SOUNDS_MAP.as_mut().unwrap().get(&id).as_ref().unwrap().resume();

    }
}

pub fn pause_any_finished_sounds() {
    unsafe {
        for (k, v) in PLAYING_MAP.as_ref().unwrap().iter() {
            match v {
                Playing::Ended => {
                    SOUNDS_MAP.as_mut().unwrap().get(k).as_ref().unwrap().pause();
                    PLAYING_MAP.as_mut().unwrap().insert(*k, Playing::Finished);
                }
                _ => {}
            }
        }
    }
}
/*
pub fn pause(id: usize) {
    unsafe {
        match PLAYING_MAP.as_mut().unwrap().get(&id).unwrap() {
            Playing::Ended => SOUNDS_MAP.as_mut().unwrap().get(&id).as_ref().unwrap().pause(),
            _ => {}
        }
    }
}
 */

fn load_in_file(sdl_context: &Sdl, file_name: &'static str, id: usize, offend: usize) -> AudioDevice<Sound> {
    unsafe {
        match &mut PLAYING_MAP {
            Some(p) => {
                p.insert(id, Playing::Ended);
            }
            None => println!("WHAT!!!"),
        }
    }

    let wav_file: Cow<'static, Path> = match std::env::args().nth(1) {
        None => Cow::from(Path::new(file_name)),
        Some(s) => Cow::from(PathBuf::from(s))
    };
    let audio_subsystem = sdl_context.audio().unwrap();

    let desired_spec = AudioSpecDesired {
        freq: Some(44_100),
        channels: Some(1), // mono
        samples: None,      // default
    };

    let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        let wav = AudioSpecWAV::load_wav(wav_file)
            .expect("Could not load test WAV file");

        let cvt = AudioCVT::new(
            wav.format, wav.channels, wav.freq,
            spec.format, spec.channels, spec.freq)
            .expect("Could not convert WAV file");

        let data = cvt.convert(wav.buffer().to_vec());

        let size = data.len() - offend;
        // initialize the audio callback
        Sound {
            data,
            volume: 0.3,
            pos: 0,
            end: size,
            counter: 0,
            id:id,
        }
    }).unwrap();
    device
}


