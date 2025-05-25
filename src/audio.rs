use std::time::Duration;
use rodio::{
    cpal, source::{
        Function, SignalGenerator, SineWave
    }, OutputStream, OutputStreamHandle, Sample, Sink, Source
};

pub struct AudioManager {
    // Immutable, used to determine if audio is able to be played or not.
    audio_available: bool,

    _stream: Option<OutputStream>,
    handle: Option<OutputStreamHandle>,
    sinks: Vec<Sink>,
}

impl Default for AudioManager {
    fn default() -> Self {
        let stream_data = OutputStream::try_default().ok();
        let audio_available = stream_data.is_some();
        let (stream, handle) = stream_data.map_or((None, None), |(s, h)| (Some(s), Some(h)));

        Self {
            audio_available,

            _stream: stream,
            handle,
            sinks: vec![],
        }
    }
}

impl AudioManager {
    fn play_audio<S>(&mut self, source: S)
    where
        S: Source + Send + 'static,
        S::Item: Sample + Send,
        f32: cpal::FromSample<S::Item>,
    {
        dbg!(self.sinks.len());
        // Find an available sink to play the sound on.
        for sink in &self.sinks {
            if sink.empty() {
                sink.append(source);
                return;
            }
        }

        // No sinks are free, so make a new one.
        let new_sink = Sink::try_new(self.handle.as_ref().unwrap()).unwrap();
        new_sink.append(source);
        self.sinks.push(new_sink);
    }

    pub fn play_sine(&mut self, freq: f32) {
        if !self.audio_available {
            return;
        }

        let sound_length = Duration::from_millis(250);
        let fade = Duration::from_millis(50);
        let sine = SineWave::new(freq)
            .take_duration(Duration::from_millis(250))
            .fade_in(fade)
            .fade_out(sound_length - fade / 2);

        self.play_audio(sine);
    }

    pub fn button_press(&mut self) {
        let sound = SignalGenerator::new(cpal::SampleRate(48_000), 500.0, Function::Sine)
            .take_duration(Duration::from_millis(4))
            .fade_in(Duration::from_millis(2))
            .fade_out(Duration::from_millis(2));

        self.play_audio(sound);
    }

    pub fn play_sines(&mut self, freqs: &[f32]) {
        if !self.audio_available {
            return;
        }

        for freq in freqs {
            self.play_sine(*freq);
        }
    }
}
