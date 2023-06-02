use gl_lib::{gl, helpers};
use gl_lib::imode_gui::drawer2d::*;
use gl_lib::imode_gui::ui::*;

use gl_lib::color::Color;
use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired, AudioStatus};


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

fn main() -> Result<(), failure::Error> {
    let sdl_setup = helpers::setup_sdl()?;
    let window = sdl_setup.window;
    let sdl = sdl_setup.sdl;
    let viewport = sdl_setup.viewport;
    let gl = &sdl_setup.gl;
    let audio_subsystem = sdl.audio().unwrap();

    let drawer_2d = Drawer2D::new(&gl, viewport).unwrap();
    let mut ui = Ui::new(drawer_2d);

    let mut event_pump = sdl.event_pump().unwrap();
    let _onoff = false;
    let _color = Color::Rgb(0,0,0);

     // Set background color to white
    unsafe {
        gl.ClearColor(0.9, 0.9, 0.9, 1.0);
    }



    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),  // mono
        samples: None       // default sample size
    };

    let mut master_vol = 0.3;

    let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        // initialize the audio callback
        SquareWave {
            phase_inc: 440.0 / spec.freq as f32,
            phase: 0.0,
            volume: 1.0,
            master_volume: master_vol
        }
    }).unwrap();



    let mut channels = vec![ Channel {
        spec: desired_spec.clone(),
        device,
        vol: 1.0,
        freq: 440.0
    }];


    loop {

        // Basic clear gl stuff and get events to UI
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        if ui.button("Add") {
            let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
                // initialize the audio callback
                SquareWave {
                    phase_inc: 440.0 / spec.freq as f32,
                    phase: 0.0,
                    volume: 1.0,
                    master_volume: master_vol,

                }
            }).unwrap();


            channels.push( Channel {
                spec: desired_spec.clone(),
                device,
                vol: 1.0,
                freq: 440.0,
            });
        }

        ui.consume_events(&mut event_pump);

        for i in 0..channels.len() {
            channel_ui(&mut ui, &mut channels[i], i);
        }



        // master volumne, only update devices when there are changes
        ui.label("Master");
        let mut new_vol = master_vol;
        ui.slider(&mut new_vol, 0.0, 0.3);


        if new_vol != master_vol {
            master_vol = new_vol;
            for channel in &mut channels {
                let mut sw = channel.device.lock();
                sw.volume = master_vol;
            }
        }

        window.gl_swap_window();
    }
}


fn channel_ui(ui: &mut Ui, channel: &mut Channel, i: usize) {
    ui.window_begin(&format!("{i}"));

    match channel.device.status() {
        AudioStatus::Stopped => {
            if ui.button("Play") {
                channel.device.resume();
            };
        },
        AudioStatus::Paused => {
            if ui.button("Play") {
                channel.device.resume();
            };
        },
        AudioStatus::Playing => {
            if ui.button("Pause") {
                channel.device.pause();
            };
        }
    };

    ui.newline();
    ui.label("Vol");
    if ui.slider(&mut channel.vol, 0.0, 1.0) {
        let mut cb = channel.device.lock();
        cb.volume = channel.vol;
    }

    ui.newline();
    ui.label("Freq");
    if ui.slider(&mut channel.freq, 10.0, 1000.0) {
        let mut cb = channel.device.lock();
        cb.phase_inc = channel.freq / channel.spec.freq.unwrap() as f32;
    }

    ui.window_end(&format!("{i}"));
}

struct Channel {
    spec: AudioSpecDesired,
    device: AudioDevice<SquareWave>,
    vol: f32,
    freq: f32
}
