#![allow(dead_code)]
use std::sync::Arc;
use std::collections::HashMap;
use sdl2::audio::{AudioSpecDesired, AudioCallback, AudioDevice, AudioSpecWAV, AudioCVT, AudioFormat};


#[derive(Clone)]
struct WavFile {
    buffer: Arc<[u8]>, // arc<[T]> Clone is O(1), so cheap do clone, and remove wavFile from mixer vec
    pos: usize,
    volume: f32,
    done: bool
}


struct Mixer {
    files: Vec::<WavFile>,
    master_volume: f32
}

impl Mixer {
    pub fn add(&mut self, sound: &Sound) {
        if self.files.len() == 0 {
            self.files.push(
                WavFile {
                    done: false,
                    buffer: sound.buffer.clone(),
                    pos: 0,
                    volume: 0.25
                });
        }

        self.files[0].buffer = sound.buffer.clone();
        self.files[0].pos = 0;
        self.files[0].done = false;
    }

    pub fn clear(&mut self) {
        self.files.clear();
    }
}

impl AudioCallback for Mixer {
    type Channel = u8;

    fn callback(&mut self, out: &mut [u8]) {

        for x in out.iter_mut() {
            let mut sample_f32 = 0.0;
            for file in &mut self.files {
                let pre_scale = *file.buffer.get(file.pos).unwrap_or(&128);
                let scaled_signed_float = (pre_scale as f32 - 128.0) * file.volume;
                sample_f32 += scaled_signed_float;
                file.pos += 1;

                file.done = file.pos >= file.buffer.len();
            }

            sample_f32 *= self.master_volume;
            *x = (sample_f32 + 128.0) as u8;
        }
    }
}

struct Channel {
    spec: AudioSpecDesired,
    device: AudioDevice<Mixer>,
}

pub struct AudioPlayer {
    channel: Channel,
    audio_subsystem: sdl2::AudioSubsystem,
    desired_spec: AudioSpecDesired,
    master_volume: f32,
    sounds: HashMap::<String, Sound>
}

struct Sound {
    buffer: Arc<[u8]>
}



impl AudioPlayer {

    pub fn clear(&mut self) {
        self.sounds.clear();
        {
            let mut mixer = self.channel.device.lock();
            mixer.clear();
        }
    }

    pub fn add_sound(&mut self, name: &str, path: &str) {

        let wav_raw_file = AudioSpecWAV::load_wav(path).expect("Could not load test WAV file");

        let cvt = AudioCVT::new(
            wav_raw_file.format,
            wav_raw_file.channels,
            wav_raw_file.freq,
            AudioFormat::U8,
            1,
            44_100
        ).expect("Could not convert WAV file");

        let data = cvt.convert(wav_raw_file.buffer().to_vec());

        self.sounds.insert(name.to_string(), Sound {
            buffer: data.into()
        });

    }


    pub fn new(audio_subsystem: sdl2::AudioSubsystem) -> Self {
        let sounds = HashMap::<String, Sound>::default();

        let samples = 128;
        let desired_spec = AudioSpecDesired {
            freq: Some(44_100),
            channels: Some(1),  // mono
            samples: Some(samples)       // default sample size
        };

        let device = audio_subsystem.open_playback(None, &desired_spec, |_spec| {
            Mixer {
                files: vec![],
                master_volume: 0.25
            }
        }).unwrap();

        let channel = Channel {
            spec: desired_spec.clone(),
            device,
        };

        channel.device.resume();

        Self {
            audio_subsystem,
            channel,
            master_volume: 0.25,
            desired_spec,
            sounds
        }
    }

    pub fn play_sound(&mut self) {
        // mutate channel data
        {
            let mut mixer = self.channel.device.lock();
            mixer.add(self.sounds.get("deflect").unwrap())
        }

    }
}
