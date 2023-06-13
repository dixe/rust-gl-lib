use sdl2::audio::AudioSpecDesired;
use sdl2::audio::{AudioCallback, AudioDevice};



struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
    master_volume: f32
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume * self.master_volume
            } else {
                -self.volume * self.master_volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

struct Channel {
    spec: AudioSpecDesired,
    device: AudioDevice<SquareWave>,
    vol: f32,
    freq: f32,
    play_seconds: f32
}


pub struct AudioPlayer {
    channels: Vec::<Channel>,
    audio_subsystem: sdl2::AudioSubsystem,
    desired_spec: AudioSpecDesired,
    master_volume: f32,
}


impl AudioPlayer {

    pub fn new(audio_subsystem: sdl2::AudioSubsystem) -> Self {
        Self {
            audio_subsystem,
            channels: vec![],
            master_volume: 0.25,
            desired_spec: AudioSpecDesired {
                freq: Some(44100),
                channels: Some(1),  // mono
                samples: None       // default sample size
            }
        }
    }

    pub fn update(&mut self, dt: f32) {
        for channel in &mut self.channels {
            channel.play_seconds -= dt;
            if channel.play_seconds < 0.0 {
                channel.device.pause();
            }
        }
    }


    pub fn play_sound(&mut self) {
        println!("{:?}", self.channels.len());
        let device = self.audio_subsystem.open_playback(None, &self.desired_spec, |spec| {
            // initialize the audio callback
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 1.0,
                master_volume: self.master_volume,
            }
        }).unwrap();


        let mut channel =  Channel {
            spec: self.desired_spec.clone(),
            device,
            vol:0.70,
            freq: 440.0,
            play_seconds: 0.5
        };
        channel.device.resume();

        self.channels.push(channel);
    }
}
