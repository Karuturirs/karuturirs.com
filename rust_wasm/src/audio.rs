use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = playWebAudioTone)]
    fn js_play_tone(freq: f32, delay: f64, dur: f64, wave: &str, gain: f32);

    #[wasm_bindgen(js_name = speakWebSpeech)]
    fn js_speak_text(text: &str);

    #[wasm_bindgen(js_name = stopWebSpeech)]
    fn js_stop_speech();
}

pub struct AudioManager {
    pub enabled: bool,
}

impl AudioManager {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    pub fn play_tone(&self, freq: f32, delay: f64, dur: f64, wave: &str, gain: f32) {
        if self.enabled {
            js_play_tone(freq, delay, dur, wave, gain);
        }
    }

    pub fn sound_click(&self) {
        self.play_tone(600.0, 0.0, 0.05, "sine", 0.12);
    }

    pub fn sound_good(&self) {
        self.play_tone(523.25, 0.0, 0.14, "sine", 0.22); // C5
        self.play_tone(659.25, 0.10, 0.14, "sine", 0.22); // E5
        self.play_tone(783.99, 0.20, 0.26, "sine", 0.24); // G5
    }

    pub fn sound_try(&self) {
        self.play_tone(220.0, 0.0, 0.18, "triangle", 0.18);
        self.play_tone(185.0, 0.14, 0.24, "triangle", 0.18);
    }

    pub fn sound_win(&self) {
        let notes = [523.25, 659.25, 783.99, 1046.50, 783.99, 1046.50];
        for (i, &f) in notes.iter().enumerate() {
            self.play_tone(f, i as f64 * 0.12, 0.22, "sine", 0.24);
        }
    }

    pub fn sound_level_up(&self) {
        self.play_tone(440.0, 0.0, 0.10, "sine", 0.20);
        self.play_tone(554.37, 0.08, 0.10, "sine", 0.20);
        self.play_tone(659.25, 0.16, 0.10, "sine", 0.20);
        self.play_tone(880.0, 0.24, 0.25, "sine", 0.25);
    }

    pub fn sound_tile_tap(&self, letter_idx: usize) {
        let f = 500.0 + (letter_idx as f32 * 45.0);
        self.play_tone(f, 0.0, 0.08, "sine", 0.18);
    }
}

pub fn speak(text: &str) {
    js_speak_text(text);
}

pub fn stop_speech() {
    js_stop_speech();
}
