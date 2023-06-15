use std::collections::HashMap;
use sdl2::audio::{AudioSpecDesired, AudioCallback, AudioDevice, AudioSpecWAV, AudioCVT, AudioFormat};


#[derive(Clone)]
struct WavFile {
    buffer: Vec::<u8>,
    pos: usize,
    master_volume: f32,
    done: bool
}

impl AudioCallback for WavFile {
    type Channel = u8;

    fn callback(&mut self, out: &mut [u8]) {
        for x in out.iter_mut() {
            if self.buffer.len() == 0 {
                *x = 128;
                continue;
            }

            let pre_scale = *self.buffer.get(self.pos).unwrap_or(&128);
            let scaled_signed_float = (pre_scale as f32 - 128.0) * self.master_volume;
            let scaled = (scaled_signed_float + 128.0) as u8;
            *x = scaled;
            self.pos += 1;

            self.done = self.pos >= self.buffer.len();
        }
    }
}

struct Channel {
    spec: AudioSpecDesired,
    device: AudioDevice<WavFile>,
}

pub struct AudioPlayer {
    channel: Channel,
    audio_subsystem: sdl2::AudioSubsystem,
    desired_spec: AudioSpecDesired,
    master_volume: f32,
    sounds: HashMap::<String, Sound>
}

struct Sound {
    buffer: Vec::<u8>
}



impl AudioPlayer {

    pub fn new(audio_subsystem: sdl2::AudioSubsystem) -> Self {

        let wav_raw_file = AudioSpecWAV::load_wav(&"examples/pixel_sekiro/assets/audio/test.wav").expect("Could not load test WAV file");

        let cvt = AudioCVT::new(
            wav_raw_file.format,
            wav_raw_file.channels,
            wav_raw_file.freq,
            AudioFormat::U8,
            1,
            44_100
        ).expect("Could not convert WAV file");

        let data = cvt.convert(wav_raw_file.buffer().to_vec());

        let mut sounds = HashMap::<String, Sound>::default();

        sounds.insert("attack".to_string(), Sound {
            buffer: data
        });

        let samples = 128;
        let desired_spec = AudioSpecDesired {
            freq: Some(44_100),
            channels: Some(1),  // mono
            samples: Some(samples)       // default sample size
        };

        let device = audio_subsystem.open_playback(None, &desired_spec, |_spec| {
            WavFile {
                done: false,
                buffer: vec![],
                pos: 0,
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
            let mut cb = self.channel.device.lock();
            cb.buffer = self.sounds.get("attack").unwrap().buffer.clone();
            cb.pos = 0;
            cb.done = false;
            println!("start s");
        }

    }
}
