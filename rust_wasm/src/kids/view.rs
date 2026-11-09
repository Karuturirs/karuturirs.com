use super::data::GAMES_LIST;
use super::models::{Difficulty, GameSession, PlayStyle};

pub fn render_kids_home(
    diff: Difficulty,
    style: PlayStyle,
    round_len: u32,
    sound: bool,
    stars: u32,
    right: u32,
    best_streak: u32,
    games_done: u32,
    get_best: impl Fn(&str) -> Option<(u32, u32, u32)>,
) -> String {
    let mode_summary_len = if round_len == 0 {
        "Endless ♾️ (play as long as you like)".to_string()
    } else {
        format!("{round_len} questions")
    };
    let mode_summary_style = match style {
        PlayStyle::Chill => "Chill (no timer, take your time)",
        PlayStyle::Clock => "Beat the Clock ⏱️ (answer before the bar runs out)",
        PlayStyle::Streak => "Streak Challenge ❤️ (3 hearts, lose one for mistakes)",
    };

    let mut prek_cards = String::new();
    let mut maths_cards = String::new();
    let mut english_cards = String::new();

    for g in GAMES_LIST {
        let best_str = match get_best(g.id) {
            Some((score, tot, st)) => {
                let star_emojis = "⭐".repeat(st as usize);
                format!("<span class=\"best\">Best: {score}/{tot} {star_emojis}</span>")
            }
            None => "<span class=\"best\"></span>".to_string(),
        };

        let q_label = if round_len == 0 {
            "Endless ♾️".to_string()
        } else if g.subject == "english" || g.id == "abcPhonics" {
            format!("{round_len} words")
        } else {
            format!("{round_len} questions")
        };

        let card_html = format!(
            r#"<button class="gcard" onclick="window.kidsStartGame('{id}')" aria-label="Play {title}">
  <span class="gicon bg-{cls}">{icon}</span>
  <h3>{title}</h3>
  <p>{desc}</p>
  <span class="gmeta">
    <span class="q">{q_label}</span>
    {best_str}
  </span>
  <span class="play c-{cls}">▶ Play</span>
</button>"#,
            id = g.id,
            title = g.title,
            cls = g.cls,
            icon = g.icon,
            desc = g.desc,
            q_label = q_label,
            best_str = best_str
        );

        if g.subject == "prek" {
            prek_cards.push_str(&card_html);
        } else if g.subject == "maths" {
            maths_cards.push_str(&card_html);
        } else {
            english_cards.push_str(&card_html);
        }
    }

    format!(
        r#"
<div class="wrap kids-container">
  <!-- TOP STATS & NAV -->
  <header class="kids-top-nav">
    <a href="/" class="nav-back-btn" onclick="event.preventDefault(); window.navigateTo('/');">
      &larr; Back to Karuturi.com
    </a>
    <div class="top-stats-row">
      <span class="stat-pill"><span class="pill-icon">⭐</span> <b id="statStars">{stars}</b> stars</span>
      <span class="stat-pill"><span class="pill-icon">🎯</span> <b id="statRight">{right}</b> correct</span>
      <span class="stat-pill"><span class="pill-icon">🔥</span> <b id="statStreak">{best_streak}</b> best streak</span>
      <span class="stat-pill"><span class="pill-icon">🏆</span> <b id="statGames">{games_done}</b> games</span>
      <button class="sound-toggle-btn" onclick="window.kidsToggleSound()" id="soundBtn">
        {sound_icon} Sound: {sound_state}
      </button>
    </div>
  </header>

  <!-- HERO BANNER -->
  <section class="kids-hero">
    <div class="hero-text-col">
      <span class="subhead-pill">✨ Powered by Rust WebAssembly</span>
      <h1>Kids <span class="m">Maths</span> &amp; <span class="e">English</span> Games</h1>
      <p>
        Fun, bite-sized learning games for little explorers! Practise numbers, counting, addition, times tables, spelling, phonics and rhymes — with step-by-step tricks that teach you <em>how</em> to think.
      </p>
      <div class="chips">
        <span class="chip">🎨 Colors &amp; Shapes</span>
        <span class="chip">🦁 Animal Sounds</span>
        <span class="chip">🐘 Big &amp; Small</span>
        <span class="chip">🍎 Counting</span>
        <span class="chip">➕ Addition</span>
        <span class="chip">🔤 ABC Phonics</span>
      </div>
      <div class="hero-cta-row">
        <button class="btn btn-coral" onclick="window.kidsStartGame('adventure')">
          🚀 Start The Big Adventure
        </button>
        <button class="btn btn-sun" onclick="document.getElementById('prekSec').scrollIntoView({{behavior:'smooth'}})">
          🎈 Pre-K Explorers
        </button>
        <button class="btn btn-teal" onclick="document.getElementById('mathsSec').scrollIntoView({{behavior:'smooth'}})">
          📐 Maths Games
        </button>
        <button class="btn btn-grape" onclick="document.getElementById('englishSec').scrollIntoView({{behavior:'smooth'}})">
          📚 English Games
        </button>
      </div>
    </div>
    <div class="hero-img-col">
      <div class="mascot-card">
        <div class="mascot-avatar">🦉</div>
        <h3>Professor Owl</h3>
        <p>"Learning is an adventure — have a go, try the tricks, and collect your stars!"</p>
      </div>
    </div>
  </section>

  <!-- 1. LEVEL PICKER -->
  <section class="level-box" aria-label="Choose your level">
    <h2>1. Pick your level 🎯</h2>
    <p>Questions adapt to you — little learners get smaller numbers and short words, super challengers get bigger ones.</p>
    <div class="levels">
      <button class="level-btn {diff_prek}" onclick="window.kidsSetDiff('prek')">
        <strong>🎈 Pre-K Explorers</strong>
        <span>Ages 3–4 · Colors, shapes, animals &amp; ABCs</span>
      </button>
      <button class="level-btn {diff_little}" onclick="window.kidsSetDiff('little')">
        <strong>🌱 Little Learners</strong>
        <span>Ages 4–5 · Numbers to 10 · 3-letter words</span>
      </button>
      <button class="level-btn {diff_growing}" onclick="window.kidsSetDiff('growing')">
        <strong>🚀 Growing Minds</strong>
        <span>Ages 6–7 · Numbers to 30 · 2s, 5s, 10s</span>
      </button>
      <button class="level-btn {diff_super}" onclick="window.kidsSetDiff('super')">
        <strong>⚡ Super Challengers</strong>
        <span>Ages 8–9 · Numbers to 100 · Full tables &amp; patterns</span>
      </button>
    </div>
  </section>

  <!-- 2. MODE & LENGTH PICKER -->
  <section class="mode-box" aria-label="Game style and length">
    <h2>2. Pick how you play 🎮</h2>
    <p>Customise your round to fit homework time, quick practice, or relaxed fun.</p>
    <div class="mode-row">
      <span class="lbl">Questions:</span>
      <button class="pill {len_5}" onclick="window.kidsSetRoundLen(5)">⚡ Quick 5</button>
      <button class="pill {len_10}" onclick="window.kidsSetRoundLen(10)">⭐ Classic 10</button>
      <button class="pill {len_20}" onclick="window.kidsSetRoundLen(20)">🏃 Marathon 20</button>
      <button class="pill {len_0}" onclick="window.kidsSetRoundLen(0)">♾️ Endless</button>
    </div>
    <div class="mode-row">
      <span class="lbl">Game Style:</span>
      <button class="pill {style_chill}" onclick="window.kidsSetStyle('chill')">🍃 Chill (Untimed)</button>
      <button class="pill {style_clock}" onclick="window.kidsSetStyle('clock')">⏱️ Beat the Clock</button>
      <button class="pill {style_streak}" onclick="window.kidsSetStyle('streak')">❤️ Streak Challenge (3 Hearts)</button>
    </div>
    <p class="mode-summary" id="modeSummary">
      Playing: <strong>{mode_summary_len}</strong> &middot; <strong>{mode_summary_style}</strong>.
    </p>
  </section>

  <!-- ADVENTURE BANNER -->
  <section class="adventure">
    <div>
      <h2>🗺️ The Big Adventure</h2>
      <p>A fun mystery quest mixing maths, spelling, and rhyme questions together! Questions adapt as your streak builds.</p>
    </div>
    <button class="btn btn-adventure" onclick="window.kidsStartGame('adventure')">
      Play Adventure &rarr;
    </button>
  </section>

  <!-- PRE-K GAMES -->
  <section id="prekSec">
    <div class="sec-head">
      <h2>🎈 Pre-K Early Explorers (Ages 3–4)</h2>
      <span>Colors &middot; Shapes &middot; Animal Sounds &middot; Big vs Small &middot; ABC Phonics</span>
    </div>
    <div class="grid" id="prekGrid">
      {prek_cards}
    </div>
  </section>

  <!-- MATHS GAMES -->
  <section id="mathsSec">
    <div class="sec-head">
      <h2>📐 Maths Games</h2>
      <span>Numbers &middot; Adding &middot; Take-away &middot; Tables &middot; Sequences</span>
    </div>
    <div class="grid" id="mathsGrid">
      {maths_cards}
    </div>
  </section>

  <!-- ENGLISH GAMES -->
  <section id="englishSec">
    <div class="sec-head">
      <h2>📚 English Games</h2>
      <span>Spelling &middot; Phonics &middot; Missing Letters &middot; Rhymes</span>
    </div>
    <div class="grid" id="englishGrid">
      {english_cards}
    </div>
  </section>

  <!-- QUICK TRICKS ACCORDION -->
  <section class="mode-box">
    <h2>💡 Mental Maths &amp; Phonics Tricks</h2>
    <p>Techniques taught step-by-step inside the games:</p>
    <div class="tricks">
      <div class="trick">
        <h3>🔟 Ten-Frames &amp; Place Value</h3>
        <p>Don't count 20+ things one by one! Group them into full tens and add the loose ones.</p>
        <div class="ex">2 boxes of 10 + 4 singles &rarr; 20 + 4 = 24 🍎</div>
      </div>
      <div class="trick">
        <h3>🔟 Make a Ten</h3>
        <p>When adding numbers like 8 + 5, borrow 2 from the 5 to turn 8 into a friendly 10!</p>
        <div class="ex">8 + 5 &rarr; (8 + 2) + 3 = 10 + 3 = 13</div>
      </div>
      <div class="trick">
        <h3>✌️ Doubles Trick</h3>
        <p>Know your doubles? Use them to add near doubles easily in your head!</p>
        <div class="ex">6 + 7 &rarr; Double 6 is 12, plus 1 more = 13</div>
      </div>
      <div class="trick">
        <h3>⬆️ Count Up to Subtract</h3>
        <p>If numbers are close together, don't count backwards — count forward to find the gap!</p>
        <div class="ex">23 − 19 &rarr; 19 to 20 is 1, 20 to 23 is 3 &rarr; Gap is 4</div>
      </div>
      <div class="trick">
        <h3>👥 Skip Counting</h3>
        <p>Multiplication is repeated addition. Count by 2s, 5s, or 10s on your fingers!</p>
        <div class="ex">4 &times; 5 &rarr; 5, 10, 15, 20 (4 jumps of 5) = 20</div>
      </div>
      <div class="trick">
        <h3>🔊 Sound It Out (Phonics)</h3>
        <p>Stretch words out slowly to hear the beginning sound, middle vowel, and ending sound.</p>
        <div class="ex">F - R - O - G &rarr; FROG 🐸</div>
      </div>
      <div class="trick">
        <h3>👂 Rhyme Clue</h3>
        <p>Rhyming words have the exact same sound at the end, even if they look slightly different!</p>
        <div class="ex">CAT &amp; HAT &middot; MOON &amp; SPOON 🌙</div>
      </div>
      <div class="trick">
        <h3>🦁 Sound Association</h3>
        <p>Animals and objects make distinctive sounds. Listen to the vowels and syllables!</p>
        <div class="ex">"Moo" &rarr; Cow 🐮 &middot; "Quack" &rarr; Duck 🦆</div>
      </div>
      <div class="trick">
        <h3>🔍 Real-World Size Comparison</h3>
        <p>Think about how big things are in nature before tapping your answer!</p>
        <div class="ex">Elephant 🐘 &gt; Mouse 🐭 &middot; Whale 🐋 &gt; Goldfish 🐟</div>
      </div>
    </div>
  </section>

  <!-- GROWN-UPS SECTION -->
  <section class="grown">
    <h2>👨‍🏫 Guide for Parents &amp; Teachers</h2>
    <p>
      This app is engineered to encourage deep conceptual understanding rather than rote stress:
    </p>
    <ul>
      <li><strong>Step-by-Step Tricks:</strong> Every question includes a "Show me how" breakdown showing mental models.</li>
      <li><strong>Audio &amp; Speech:</strong> Tones provide auditory satisfaction, while Speech Synthesis reads questions aloud for emerging readers.</li>
      <li><strong>Adaptive Feedback:</strong> Scoring 3 in a row triggers an adaptive challenge boost with positive encouragement.</li>
      <li><strong>Privacy &amp; Offline:</strong> Completely client-side WebAssembly. No cookies, no tracking, works smoothly offline!</li>
    </ul>
  </section>

  <footer>
    <p>&copy; karuturirs.com &middot; Built with Rust &amp; WebAssembly &middot; Deployed on Netlify</p>
  </footer>
</div>
"#,
        sound_icon = if sound { "🔊" } else { "🔇" },
        sound_state = if sound { "On" } else { "Off" },
        stars = stars,
        right = right,
        best_streak = best_streak,
        games_done = games_done,
        diff_prek = if diff == Difficulty::PreK { "active" } else { "" },
        diff_little = if diff == Difficulty::Little { "active" } else { "" },
        diff_growing = if diff == Difficulty::Growing { "active" } else { "" },
        diff_super = if diff == Difficulty::Super { "active" } else { "" },
        len_5 = if round_len == 5 { "active" } else { "" },
        len_10 = if round_len == 10 { "active" } else { "" },
        len_20 = if round_len == 20 { "active" } else { "" },
        len_0 = if round_len == 0 { "active" } else { "" },
        style_chill = if style == PlayStyle::Chill { "active" } else { "" },
        style_clock = if style == PlayStyle::Clock { "active" } else { "" },
        style_streak = if style == PlayStyle::Streak { "active" } else { "" },
        mode_summary_len = mode_summary_len,
        mode_summary_style = mode_summary_style,
        prek_cards = prek_cards,
        maths_cards = maths_cards,
        english_cards = english_cards
    )
}

pub fn render_game_session(
    session: &GameSession,
    style: PlayStyle,
    feedback_text: &str,
    feedback_class: &str,
    level_up_msg: Option<&str>,
) -> String {
    let q = &session.current;
    let total_str = if session.total == 0 {
        format!("Question {} &middot; Endless ♾️", session.idx + 1)
    } else {
        format!("Question {} of {}", session.idx + 1, session.total)
    };

    let progress_pct = if session.total == 0 {
        ((session.idx % 10) as f32 / 10.0 * 100.0) as u32
    } else {
        ((session.idx as f32 / session.total as f32) * 100.0) as u32
    };

    let hearts_html = if style == PlayStyle::Streak {
        let full_hearts = session.lives.max(0) as usize;
        let empty_hearts = (3 - session.lives.max(0)) as usize;
        format!(
            "<div class=\"hearts\">{}{}</div>",
            "❤️".repeat(full_hearts),
            "🖤".repeat(empty_hearts)
        )
    } else {
        String::new()
    };

    let timer_bar_html = if style == PlayStyle::Clock {
        let pct = if session.time_total > 0 {
            ((session.time_left as f32 / session.time_total as f32) * 100.0) as u32
        } else {
            100
        };
        let low_cls = if pct <= 30 { " low" } else { "" };
        format!(
            r#"<div class="timerbar{low_cls}" style="display:block;">
  <i style="width:{pct}%;"></i>
</div>"#,
            low_cls = low_cls,
            pct = pct
        )
    } else {
        String::new()
    };

    let level_up_html = match level_up_msg {
        Some(msg) => format!(r#"<div class="levelup show">{msg}</div>"#),
        None => String::new(),
    };

    let technique_content = match &q.technique {
        Some(t) => {
            let lis: Vec<String> = t.steps.iter().map(|s| format!("<li>{s}</li>")).collect();
            let worked_line = if !t.worked.is_empty() {
                format!(r#"<div class="tryline">✏️ Worked answer: {} — now try tapping your answer!</div>"#, t.worked)
            } else {
                String::new()
            };
            format!(
                r#"<h3>{}</h3><ol>{}</ol>{}"#,
                t.name,
                lis.join(""),
                worked_line
            )
        }
        None => {
            r#"<h3>💡 Think it through</h3><ol><li>Read the question slowly, twice.</li><li>Say what you are trying to find.</li><li>Have a go — trying is how we learn!</li></ol>"#.to_string()
        }
    };

    let tech_panel_cls = if session.tech_open { "show" } else { "" };
    let tech_btn_label = if session.tech_open {
        "🙈 Hide the trick"
    } else {
        "💡 Show me how — step by step"
    };

    // Render Answer Area
    let answer_area_html = if q.is_spell {
        // Spelling blanks + tiles
        let target_len = match &q.word {
            Some(w) => w.len(),
            None => q.answer.len(),
        };

        let mut blanks_html = String::new();
        let chars_built: Vec<char> = session.spell_built.chars().collect();
        for i in 0..target_len {
            if i < chars_built.len() {
                blanks_html.push_str(&format!(
                    r#"<span class="blank filled">{}</span>"#,
                    chars_built[i]
                ));
            } else {
                blanks_html.push_str(r#"<span class="blank"></span>"#);
            }
        }

        // Shuffled tiles
        let word_chars = match &q.word {
            Some(w) => w.clone(),
            None => q.answer.clone(),
        };
        // Let's deterministically show letters of the word
        let mut tiles_html = String::new();
        let letters: Vec<char> = word_chars.chars().collect();
        for (idx, &ch) in letters.iter().enumerate() {
            let used = session.spell_tiles_used.contains(&idx.to_string());
            let used_cls = if used { " used" } else { "" };
            tiles_html.push_str(&format!(
                r#"<button class="tile{used_cls}" onclick="window.kidsTapSpellTile({idx}, '{ch}')">{ch}</button>"#,
                used_cls = used_cls,
                idx = idx,
                ch = ch
            ));
        }

        format!(
            r#"<div class="blanks">{blanks}</div>
<div class="tiles">{tiles}</div>
<div class="spell-actions">
  <button class="btn btn-ghost" onclick="window.kidsUndoSpell()">⌫ Undo letter</button>
</div>"#,
            blanks = blanks_html,
            tiles = tiles_html
        )
    } else {
        // Multiple choices
        let mut choices_html = String::new();
        for (i, c) in q.choices.iter().enumerate() {
            let onclick = if session.locked {
                String::new()
            } else {
                format!(r#"onclick="window.kidsAnswerChoice({i}, '{c}')""#)
            };
            let mut choice_cls = "choice".to_string();
            if session.locked {
                if c == &q.answer {
                    choice_cls.push_str(" correct");
                }
            }
            choices_html.push_str(&format!(
                r#"<button class="{choice_cls}" {onclick}>{c}</button>"#,
                choice_cls = choice_cls,
                onclick = onclick,
                c = c
            ));
        }
        format!(r#"<div class="choices">{choices_html}</div>"#)
    };

    let visual_small_cls = if q.visual_small { " small" } else { "" };
    let finish_early_btn = if session.total == 0 || style == PlayStyle::Streak {
        r#"<button class="btn btn-ghost finish-btn" onclick="window.kidsFinishGame()">🏁 Finish</button>"#
    } else {
        ""
    };

    format!(
        r#"
<div class="wrap game-shell">
  <div class="game-top">
    <button class="btn btn-ghost back-btn" onclick="window.kidsQuitGame()">🏠 Quit</button>
    <h2>{title}</h2>
    <span class="qcount">{total_str}</span>
    <span class="qcount">Score: {score}</span>
    {hearts}
    {finish_early}
  </div>

  <div class="progress">
    <i style="width:{progress_pct}%;"></i>
  </div>

  {timer_bar}
  {level_up}

  <div class="play-card">
    <div class="visual{visual_small_cls}">{visual}</div>
    <div class="qprompt">{prompt}</div>
    <div class="qsub">{sub}</div>

    <div class="listen-wrap">
      <button type="button" class="btn btn-ghost listen-btn" onclick="window.kidsSpeakQuestion()" aria-label="Read question out loud">
        🗣️ Read question to me
      </button>
    </div>

    <div class="tech-wrap">
      <button class="tech-btn" onclick="window.kidsToggleTech()">{tech_btn_label}</button>
      <div class="tech-panel {tech_panel_cls}">
        {technique_content}
      </div>
    </div>

    <div id="answerArea">
      {answer_area}
    </div>

    <div class="feedback {feedback_class}">
      {feedback_text}
    </div>
  </div>
</div>
"#,
        title = session.title,
        total_str = total_str,
        score = session.score,
        hearts = hearts_html,
        finish_early = finish_early_btn,
        progress_pct = progress_pct,
        timer_bar = timer_bar_html,
        level_up = level_up_html,
        visual_small_cls = visual_small_cls,
        visual = q.visual,
        prompt = q.prompt,
        sub = q.sub,
        tech_btn_label = tech_btn_label,
        tech_panel_cls = tech_panel_cls,
        technique_content = technique_content,
        answer_area = answer_area_html,
        feedback_class = feedback_class,
        feedback_text = feedback_text
    )
}

pub fn render_result_view(
    score: u32,
    total: u32,
    stars: u32,
    title: &str,
    text: &str,
    tip: &str,
) -> String {
    let mut star_spans = String::new();
    for i in 0..3 {
        let cls = if i < stars { "" } else { "off" };
        star_spans.push_str(&format!(r#"<span class="{cls}">⭐</span>"#));
    }

    format!(
        r#"
<div class="wrap game-shell">
  <div class="result-card">
    <div class="result-stars">
      {star_spans}
    </div>
    <h2 class="result-title">{title}</h2>
    <p class="result-text">{text}</p>

    <div class="result-box">
      <div class="res-stat">
        <span class="num">{score}</span>
        <span class="lbl">Correct</span>
      </div>
      <div class="res-stat">
        <span class="num">{total}</span>
        <span class="lbl">Questions</span>
      </div>
      <div class="res-stat">
        <span class="num">+{stars}</span>
        <span class="lbl">Stars Earned</span>
      </div>
    </div>

    <p class="result-tip">{tip}</p>

    <div class="result-actions">
      <button class="btn btn-coral" onclick="window.kidsKeepGoing()">
        🚀 Keep going — 10 more!
      </button>
      <button class="btn btn-teal" onclick="window.kidsReplayGame()">
        🔄 Play Again
      </button>
      <button class="btn btn-ghost" onclick="window.kidsShowHome()">
        🏠 Choose Another Game
      </button>
    </div>
  </div>
</div>
"#
    )
}
