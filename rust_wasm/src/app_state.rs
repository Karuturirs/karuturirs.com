use super::audio::AudioManager;
use super::home::view::render_home_html;
use super::kids::data::{GAMES_LIST, PRAISES};
use super::kids::generators::{generate_question, pick};
use super::kids::models::{Difficulty, GameSession, PlayStyle};
use super::kids::view::{render_game_session, render_kids_home, render_result_view};
use super::storage::{
    get_storage_bool, get_storage_item, get_storage_u32, set_storage_bool, set_storage_item,
    set_storage_u32,
};
use std::cell::RefCell;
use std::collections::HashMap;

pub struct AppState {
    pub current_path: String,
    pub audio: AudioManager,
    pub diff: Difficulty,
    pub style: PlayStyle,
    pub round_len: u32,
    pub stars: u32,
    pub right: u32,
    pub streak: u32,
    pub best_streak: u32,
    pub games_done: u32,
    pub best_scores: HashMap<String, (u32, u32, u32)>, // game_id -> (score, total, stars)
    pub session: Option<GameSession>,
    pub showing_result: bool,
    pub feedback_text: String,
    pub feedback_cls: String,
    pub level_up_msg: Option<String>,
    pub last_result: Option<(u32, u32, u32, String, String, String)>,
}

thread_local! {
    pub static STATE: RefCell<AppState> = RefCell::new(AppState::init());
}

impl AppState {
    pub fn init() -> Self {
        let sound = get_storage_bool("kids_sound", true);
        let diff_str = get_storage_item("kids_diff").unwrap_or_else(|| "growing".to_string());
        let style_str = get_storage_item("kids_style").unwrap_or_else(|| "chill".to_string());
        let round_len = get_storage_u32("kids_round_len", 10);
        let stars = get_storage_u32("kids_stars", 0);
        let right = get_storage_u32("kids_right", 0);
        let best_streak = get_storage_u32("kids_best_streak", 0);
        let games_done = get_storage_u32("kids_games_done", 0);

        let mut best_scores = HashMap::new();
        for g in GAMES_LIST {
            let key = format!("kids_best_{}", g.id);
            if let Some(val) = get_storage_item(&key) {
                let parts: Vec<&str> = val.split(',').collect();
                if parts.len() == 3 {
                    if let (Ok(s), Ok(t), Ok(st)) = (
                        parts[0].parse::<u32>(),
                        parts[1].parse::<u32>(),
                        parts[2].parse::<u32>(),
                    ) {
                        best_scores.insert(g.id.to_string(), (s, t, st));
                    }
                }
            }
        }

        let pathname = if let Some(win) = web_sys::window() {
            win.location().pathname().unwrap_or_else(|_| "/".to_string())
        } else {
            "/".to_string()
        };

        Self {
            current_path: pathname,
            audio: AudioManager::new(sound),
            diff: Difficulty::from_str(&diff_str),
            style: PlayStyle::from_str(&style_str),
            round_len,
            stars,
            right,
            streak: 0,
            best_streak,
            games_done,
            best_scores,
            session: None,
            showing_result: false,
            feedback_text: String::new(),
            feedback_cls: "feedback".to_string(),
            level_up_msg: None,
            last_result: None,
        }
    }

    pub fn save_stats(&self) {
        set_storage_u32("kids_stars", self.stars);
        set_storage_u32("kids_right", self.right);
        set_storage_u32("kids_best_streak", self.best_streak);
        set_storage_u32("kids_games_done", self.games_done);
        set_storage_bool("kids_sound", self.audio.enabled);
        set_storage_item("kids_diff", self.diff.as_str());
        set_storage_item("kids_style", self.style.as_str());
        set_storage_u32("kids_round_len", self.round_len);

        for (k, (s, t, st)) in &self.best_scores {
            let key = format!("kids_best_{k}");
            set_storage_item(&key, &format!("{s},{t},{st}"));
        }
    }

    pub fn handle_answer_choice(&mut self, val: &str) -> (bool, bool) {
        let (is_correct, answer_str, reveal_str, has_technique, should_finish) = {
            let sess = match self.session.as_mut() {
                Some(s) => s,
                None => return (false, false),
            };

            if sess.locked {
                return (false, false);
            }

            sess.locked = true;
            let correct = val == sess.current.answer;
            let ans = sess.current.answer.clone();
            let rev = sess.current.reveal.clone();
            let has_tech = sess.current.technique.is_some();

            if correct {
                sess.score += 1;
            } else if sess.boost > 0 {
                sess.boost -= 1;
            }

            let mut finish = false;
            if !correct && self.style == PlayStyle::Streak {
                sess.lives -= 1;
                if sess.lives <= 0 {
                    finish = true;
                }
            }

            (correct, ans, rev, has_tech, finish)
        };

        if is_correct {
            self.right += 1;
            self.streak += 1;
            if self.streak > self.best_streak {
                self.best_streak = self.streak;
            }

            if self.streak > 0 && self.streak % 3 == 0 {
                if let Some(sess) = self.session.as_mut() {
                    sess.boost = std::cmp::min(3, sess.boost + 1);
                }
                self.level_up_msg = Some(format!(
                    "🔥 {} in a row! Level up — the questions just adapted!",
                    self.streak
                ));
                self.audio.sound_level_up();
            }

            let praise = *pick(PRAISES);
            let extra = match reveal_str {
                Some(r) => format!(" — the word is {r}!"),
                None => String::new(),
            };
            self.feedback_text = format!("{praise}{extra}");
            self.feedback_cls = "feedback good".to_string();
            self.audio.sound_good();
        } else {
            self.streak = 0;
            if has_technique {
                if let Some(sess) = self.session.as_mut() {
                    sess.tech_open = true;
                }
            }

            let lives_left = self.session.as_ref().map(|s| s.lives).unwrap_or(0);
            let mut msg = format!("Not quite — the answer was {answer_str}. Look at the trick above and you'll nail the next one! 💪");
            if self.style == PlayStyle::Streak {
                if lives_left > 0 {
                    msg.push_str(&format!(" Hearts left: {lives_left}."));
                } else {
                    msg.push_str(" That was your last heart — fantastic run!");
                }
            }

            self.feedback_text = msg;
            self.feedback_cls = "feedback try".to_string();
            self.audio.sound_try();
        }

        self.save_stats();
        (is_correct, should_finish)
    }

    pub fn handle_tap_spell_tile(&mut self, tile_idx: usize, ch_str: &str) -> (bool, bool) {
        let mut is_completed = false;
        let mut is_correct = false;
        let target_word;
        let spell_built_str;

        {
            let sess = match self.session.as_mut() {
                Some(s) => s,
                None => return (false, false),
            };

            if sess.locked {
                return (false, false);
            }

            let idx_str = tile_idx.to_string();
            if sess.spell_tiles_used.contains(&idx_str) {
                return (false, false);
            }

            sess.spell_tiles_used.push(idx_str);
            sess.spell_built.push_str(ch_str);
            spell_built_str = sess.spell_built.clone();

            target_word = match &sess.current.word {
                Some(w) => w.clone(),
                None => sess.current.answer.clone(),
            };

            if sess.spell_built.len() == target_word.len() {
                is_completed = true;
                if sess.spell_built == target_word {
                    is_correct = true;
                    sess.locked = true;
                    sess.score += 1;
                }
            }
        }

        self.audio.sound_tile_tap(spell_built_str.len());

        if is_completed {
            if is_correct {
                self.right += 1;
                self.streak += 1;
                if self.streak > self.best_streak {
                    self.best_streak = self.streak;
                }
                self.feedback_text = format!("🎉 Brilliant! You spelled {target_word}! ✏️");
                self.feedback_cls = "feedback good".to_string();
                self.audio.sound_good();
            } else {
                self.audio.sound_try();
                self.feedback_text = format!(
                    "Nearly! That spells “{spell_built_str}”. The word is {target_word} — tap Undo and try again."
                );
                self.feedback_cls = "feedback try".to_string();
                self.streak = 0;
            }
            self.save_stats();
        }

        (is_completed, is_correct)
    }

    pub fn handle_undo_spell(&mut self) {
        if let Some(sess) = self.session.as_mut() {
            if !sess.locked && !sess.spell_built.is_empty() {
                sess.spell_built.pop();
                sess.spell_tiles_used.pop();
                self.feedback_text = String::new();
                self.audio.sound_click();
            }
        }
    }

    pub fn handle_tick_clock(&mut self) -> bool {
        let mut timed_out = false;
        let mut ans_str = String::new();

        if let Some(sess) = self.session.as_mut() {
            if !sess.locked {
                if sess.time_left > 0 {
                    sess.time_left -= 1;
                }
                if sess.time_left == 0 {
                    timed_out = true;
                    sess.locked = true;
                    sess.tech_open = true;
                    ans_str = sess.current.answer.clone();
                }
            }
        }

        if timed_out {
            self.streak = 0;
            self.feedback_text = format!(
                "⏰ Time up! The answer was {ans_str}. Check out the trick above — you'll beat the clock next time! 💪"
            );
            self.feedback_cls = "feedback try".to_string();
            self.audio.sound_try();
        }

        timed_out
    }

    pub fn handle_next_question(&mut self) -> bool {
        let diff = self.diff;
        let style = self.style;

        {
            let sess = match self.session.as_mut() {
                Some(s) => s,
                None => return false,
            };

            sess.idx += 1;
            if sess.total > 0 && sess.idx >= sess.total {
                return true;
            }

            sess.locked = false;
            sess.spell_built.clear();
            sess.spell_tiles_used.clear();
            sess.tech_open = false;

            let q = generate_question(&sess.game_id, diff, sess.boost);
            sess.current = q;

            let tt = match style {
                PlayStyle::Clock => match diff {
                    Difficulty::PreK => 20,
                    Difficulty::Little => 18,
                    Difficulty::Growing => 15,
                    Difficulty::Super => 12,
                },
                _ => 0,
            };
            sess.time_total = tt;
            sess.time_left = tt;
        }

        self.feedback_text.clear();
        self.feedback_cls = "feedback".to_string();
        self.level_up_msg = None;

        false
    }

    pub fn handle_finish_game(&mut self) {
        let sess = match self.session.take() {
            Some(s) => s,
            None => return,
        };

        let score = sess.score;
        let total = if sess.total == 0 { sess.idx } else { sess.total };
        let ratio = if total > 0 { score as f32 / total as f32 } else { 0.0 };

        let mut stars = if ratio >= 0.9 {
            3
        } else if ratio >= 0.7 {
            2
        } else if ratio >= 0.45 {
            1
        } else {
            0
        };
        if score == total && total > 0 {
            stars = 3;
        }
        if stars == 0 && score > 0 {
            stars = 1;
        }

        self.stars += stars;
        self.games_done += 1;

        if sess.game_id != "adventure" {
            let prev = self.best_scores.get(&sess.game_id).copied();
            let should_update = match prev {
                Some((prev_score, _, _)) => score > prev_score,
                None => true,
            };
            if should_update {
                self.best_scores.insert(sess.game_id.clone(), (score, total, stars));
            }
        }

        self.showing_result = true;

        let (title, text) = if ratio >= 0.9 {
            (
                "🏆 Champion!".to_string(),
                format!("You scored {score} out of {total} — absolutely amazing! Try the next level up, or conquer another game."),
            )
        } else if ratio >= 0.7 {
            (
                "🌟 Super work!".to_string(),
                format!("You scored {score} out of {total}. Just a little more practice and you will get all three stars!"),
            )
        } else if ratio >= 0.45 {
            (
                "💪 Good trying!".to_string(),
                format!("You scored {score} out of {total}. Every game makes your brain stronger — play again and beat your score!"),
            )
        } else {
            (
                "🌱 Keep going!".to_string(),
                format!("You scored {score} out of {total}. That was a tricky one! Playing again is the best way to master it."),
            )
        };

        let tip = match self.style {
            PlayStyle::Streak => "❤️ That was Streak mode! Tap “Keep going” for 10 fresh questions with 3 new hearts, or choose another game on the home screen.",
            _ => "Want more? Tap “Keep going — 10 more!” for fresh questions that carry your score forward, or explore the tricks on the home screen!",
        };

        self.last_result = Some((score, total, stars, title, text, tip.to_string()));
        self.audio.sound_win();
        self.save_stats();
    }
}

pub fn render_current_view() {
    STATE.with(|s| {
        let state = s.borrow();
        let html = if state.current_path.starts_with("/kids-school-learning") {
            if state.showing_result {
                if let Some((score, total, stars, ref title, ref text, ref tip)) = state.last_result {
                    render_result_view(score, total, stars, title, text, tip)
                } else {
                    render_kids_home(
                        state.diff,
                        state.style,
                        state.round_len,
                        state.audio.enabled,
                        state.stars,
                        state.right,
                        state.best_streak,
                        state.games_done,
                        |id| state.best_scores.get(id).copied(),
                    )
                }
            } else if let Some(ref sess) = state.session {
                render_game_session(
                    sess,
                    state.style,
                    &state.feedback_text,
                    &state.feedback_cls,
                    state.level_up_msg.as_deref(),
                )
            } else {
                render_kids_home(
                    state.diff,
                    state.style,
                    state.round_len,
                    state.audio.enabled,
                    state.stars,
                    state.right,
                    state.best_streak,
                    state.games_done,
                    |id| state.best_scores.get(id).copied(),
                )
            }
        } else {
            render_home_html()
        };

        if let Some(win) = web_sys::window() {
            if let Some(doc) = win.document() {
                if let Some(app_el) = doc.get_element_by_id("app") {
                    app_el.set_inner_html(&html);
                }
            }
        }
    });
}
