mod app_state;
mod audio;
mod home;
mod kids;
mod storage;

use app_state::{render_current_view, STATE};
use audio::{speak, stop_speech};
use kids::data::GAMES_LIST;
use kids::generators::generate_question;
use kids::models::{Difficulty, GameSession, PlayStyle};
use wasm_bindgen::prelude::*;

// JS Extern functions
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = triggerConfetti)]
    fn js_trigger_confetti(n: u32);

    #[wasm_bindgen(js_name = scheduleNextQuestion)]
    fn js_schedule_next_question(delay_ms: i32);

    #[wasm_bindgen(js_name = scheduleFinishGame)]
    fn js_schedule_finish_game(delay_ms: i32);

    #[wasm_bindgen(js_name = startKidsClock)]
    fn js_start_kids_clock(seconds: u32);

    #[wasm_bindgen(js_name = stopKidsClock)]
    fn js_stop_kids_clock();
}

#[wasm_bindgen]
pub fn navigate_to(path: &str) {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.current_path = path.to_string();
        state.session = None;
        state.showing_result = false;
    });

    if let Some(win) = web_sys::window() {
        if let Ok(hist) = win.history() {
            let _ = hist.push_state_with_url(&JsValue::NULL, "", Some(path));
        }
        win.scroll_to_with_x_and_y(0.0, 0.0);
    }

    render_current_view();
}

#[wasm_bindgen]
pub fn kids_set_diff(diff: &str) {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.diff = Difficulty::from_str(diff);
        state.audio.play_tone(660.0, 0.0, 0.1, "sine", 0.15);
        state.save_stats();
    });
    render_current_view();
}

#[wasm_bindgen]
pub fn kids_set_style(style: &str) {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.style = PlayStyle::from_str(style);
        state.audio.play_tone(600.0, 0.0, 0.1, "sine", 0.15);
        state.save_stats();
    });
    render_current_view();
}

#[wasm_bindgen]
pub fn kids_set_round_len(len: u32) {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.round_len = len;
        state.audio.play_tone(600.0, 0.0, 0.1, "sine", 0.15);
        state.save_stats();
    });
    render_current_view();
}

#[wasm_bindgen]
pub fn kids_toggle_sound() {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.audio.enabled = !state.audio.enabled;
        if state.audio.enabled {
            state.audio.play_tone(660.0, 0.0, 0.12, "sine", 0.2);
        }
        state.save_stats();
    });
    render_current_view();
}

#[wasm_bindgen]
pub fn kids_start_game(game_id: &str) {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        let (title, total) = if game_id == "adventure" {
            (
                "🗺️ The Big Adventure".to_string(),
                if state.round_len == 0 { 0 } else { state.round_len },
            )
        } else {
            let found = GAMES_LIST.iter().find(|g| g.id == game_id);
            let t = match found {
                Some(g) => format!("{} {}", g.icon, g.title),
                None => "Game".to_string(),
            };
            (
                t,
                if state.round_len == 0 { 0 } else { state.round_len },
            )
        };

        state.streak = 0;
        state.showing_result = false;
        state.feedback_text = String::new();
        state.feedback_cls = "feedback".to_string();
        state.level_up_msg = None;

        let q = generate_question(game_id, state.diff, 0);

        let time_total = match state.style {
            PlayStyle::Clock => match state.diff {
                Difficulty::PreK => 20,
                Difficulty::Little => 18,
                Difficulty::Growing => 15,
                Difficulty::Super => 12,
            },
            _ => 0,
        };

        state.session = Some(GameSession {
            game_id: game_id.to_string(),
            title,
            current: q,
            idx: 0,
            total,
            score: 0,
            lives: 3,
            boost: 0,
            locked: false,
            spell_built: String::new(),
            spell_tiles_used: Vec::new(),
            tech_open: false,
            time_left: time_total,
            time_total,
            history: Vec::new(),
            queue_filter: "all".to_string(),
            queue_open_mobile: false,
        });

        if state.style == PlayStyle::Clock {
            js_start_kids_clock(time_total);
        } else {
            js_stop_kids_clock();
        }
    });

    if let Some(win) = web_sys::window() {
        win.scroll_to_with_x_and_y(0.0, 0.0);
    }
    render_current_view();
}

#[wasm_bindgen]
pub fn kids_answer_choice(_choice_idx: usize, val: &str) {
    stop_speech();
    let (is_correct, should_finish) = STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.handle_answer_choice(val)
    });

    js_stop_kids_clock();
    render_current_view();

    if is_correct {
        js_trigger_confetti(22);
        js_schedule_next_question(1400);
    } else if should_finish {
        js_schedule_finish_game(2500);
    } else {
        js_schedule_next_question(2400);
    }
}

#[wasm_bindgen]
pub fn kids_tap_spell_tile(tile_idx: usize, ch_str: &str) {
    let (is_completed, is_correct) = STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.handle_tap_spell_tile(tile_idx, ch_str)
    });

    render_current_view();

    if is_completed && is_correct {
        js_stop_kids_clock();
        js_trigger_confetti(25);
        js_schedule_next_question(1500);
    }
}

#[wasm_bindgen]
pub fn kids_undo_spell() {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.handle_undo_spell();
    });
    render_current_view();
}

#[wasm_bindgen]
pub fn kids_toggle_tech() {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        if let Some(sess) = state.session.as_mut() {
            sess.tech_open = !sess.tech_open;
            if sess.tech_open {
                state.audio.play_tone(740.0, 0.0, 0.1, "sine", 0.18);
            }
        }
    });
    render_current_view();
}

#[wasm_bindgen]
pub fn kids_speak_question() {
    STATE.with(|s| {
        let state = s.borrow();
        if let Some(sess) = &state.session {
            let mut text = sess.current.speak.trim().to_string();
            if text.is_empty() {
                text = format!("{}. {}", sess.current.prompt, sess.current.sub);
            }
            if sess.tech_open {
                if let Some(t) = &sess.current.technique {
                    text.push_str(". Here is the trick: ");
                    text.push_str(&t.name);
                    text.push_str(". ");
                    text.push_str(&t.steps.join(" "));
                }
            }
            speak(&text);
        }
    });
}

#[wasm_bindgen]
pub fn kids_tick_clock() {
    let timed_out = STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.handle_tick_clock()
    });

    if timed_out {
        js_stop_kids_clock();
        render_current_view();
        js_schedule_next_question(2600);
    } else {
        if let Some(win) = web_sys::window() {
            if let Some(doc) = win.document() {
                STATE.with(|s| {
                    let state = s.borrow();
                    if let Some(sess) = &state.session {
                        if sess.time_total > 0 {
                            let pct = ((sess.time_left as f32 / sess.time_total as f32) * 100.0) as u32;
                            if let Some(bar_i) = doc.query_selector(".timerbar i").ok().flatten() {
                                let _ = bar_i.set_attribute("style", &format!("width:{pct}%;"));
                            }
                            if let Some(timerbar) = doc.query_selector(".timerbar").ok().flatten() {
                                if pct <= 30 {
                                    let _ = timerbar.class_list().add_1("low");
                                }
                            }
                        }
                    }
                });
            }
        }
    }
}

#[wasm_bindgen]
pub fn kids_next_question() {
    stop_speech();
    let should_finish = STATE.with(|s| {
        let mut state = s.borrow_mut();
        let finish = state.handle_next_question();
        if !finish && state.style == PlayStyle::Clock {
            let tt = state.session.as_ref().map(|sess| sess.time_total).unwrap_or(15);
            js_start_kids_clock(tt);
        }
        finish
    });

    if should_finish {
        kids_finish_game();
    } else {
        render_current_view();
    }
}

#[wasm_bindgen]
pub fn kids_finish_game() {
    stop_speech();
    js_stop_kids_clock();
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.handle_finish_game();
    });
    js_trigger_confetti(60);
    render_current_view();
}

#[wasm_bindgen]
pub fn kids_replay_game() {
    let gid = STATE.with(|s| s.borrow().last_game_id.clone());
    kids_start_game(&gid);
}

#[wasm_bindgen]
pub fn kids_keep_going() {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        let prev_score = state.last_result.as_ref().map(|r| r.0).unwrap_or(0);
        let prev_total = state.last_result.as_ref().map(|r| r.1).unwrap_or(10);
        let last_hist = state.last_history.clone();
        state.showing_result = false;

        let q = generate_question("adventure", state.diff, 0);

        let time_total = match state.style {
            PlayStyle::Clock => match state.diff {
                Difficulty::PreK => 20,
                Difficulty::Little => 18,
                Difficulty::Growing => 15,
                Difficulty::Super => 12,
            },
            _ => 0,
        };

        state.session = Some(GameSession {
            game_id: "adventure".to_string(),
            title: "🗺️ The Big Adventure (Extended)".to_string(),
            current: q,
            idx: prev_total,
            total: prev_total + 10,
            score: prev_score,
            lives: 3,
            boost: 1,
            locked: false,
            spell_built: String::new(),
            spell_tiles_used: Vec::new(),
            tech_open: false,
            time_left: time_total,
            time_total,
            history: last_hist,
            queue_filter: "all".to_string(),
            queue_open_mobile: false,
        });

        if state.style == PlayStyle::Clock {
            js_start_kids_clock(time_total);
        } else {
            js_stop_kids_clock();
        }
    });

    render_current_view();
}

#[wasm_bindgen]
pub fn kids_set_queue_filter(filter: &str) {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        if let Some(sess) = state.session.as_mut() {
            sess.queue_filter = filter.to_string();
        } else {
            state.result_queue_filter = filter.to_string();
        }
    });
    render_current_view();
}

#[wasm_bindgen]
pub fn kids_toggle_queue_mobile() {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        if let Some(sess) = state.session.as_mut() {
            sess.queue_open_mobile = !sess.queue_open_mobile;
        }
    });
    render_current_view();
}

#[wasm_bindgen]
pub fn kids_read_queue_item(idx: usize) {
    STATE.with(|s| {
        let state = s.borrow();
        let history = if let Some(sess) = &state.session {
            &sess.history
        } else {
            &state.last_history
        };
        if let Some(item) = history.get(idx) {
            let mut text = format!("Question {}: {}. ", item.idx, item.prompt);
            if !item.sub.is_empty() {
                text.push_str(&item.sub);
                text.push_str(". ");
            }
            if item.is_correct {
                text.push_str(&format!(
                    "You got this right! Your answer was {}. Excellent job!",
                    item.user_answer
                ));
            } else {
                text.push_str(&format!(
                    "You chose {}, but the correct answer is {}. ",
                    item.user_answer, item.correct_answer
                ));
                if let Some(name) = &item.technique_name {
                    text.push_str(&format!("Here is the trick: {}. ", name));
                }
                if !item.technique_steps.is_empty() {
                    text.push_str(&item.technique_steps.join(". "));
                }
                if let Some(w) = &item.technique_worked {
                    if !w.is_empty() {
                        text.push_str(&format!(". Worked answer: {w}."));
                    }
                }
            }
            speak(&text);
        }
    });
}

#[wasm_bindgen]
pub fn kids_quit_game() {
    stop_speech();
    js_stop_kids_clock();
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.session = None;
        state.showing_result = false;
    });
    render_current_view();
}

#[wasm_bindgen]
pub fn kids_show_home() {
    stop_speech();
    js_stop_kids_clock();
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.session = None;
        state.showing_result = false;
    });
    render_current_view();
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    render_current_view();
    Ok(())
}
