#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Difficulty {
    PreK,    // Ages 3-4
    Little,  // Ages 4-5
    Growing, // Ages 6-7
    Super,   // Ages 8-9
}

impl Difficulty {
    pub fn as_str(&self) -> &'static str {
        match self {
            Difficulty::PreK => "prek",
            Difficulty::Little => "little",
            Difficulty::Growing => "growing",
            Difficulty::Super => "super",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "prek" => Difficulty::PreK,
            "little" => Difficulty::Little,
            "super" => Difficulty::Super,
            _ => Difficulty::Growing,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Difficulty::PreK => "🎈 Pre-K Explorers",
            Difficulty::Little => "🌱 Little Learners",
            Difficulty::Growing => "🚀 Growing Minds",
            Difficulty::Super => "⚡ Super Challengers",
        }
    }

    pub fn age_sub(&self) -> &'static str {
        match self {
            Difficulty::PreK => "Ages 3–4 · Colors, shapes, animals & ABCs",
            Difficulty::Little => "Ages 4–5 · Numbers to 10 · 3-letter words",
            Difficulty::Growing => "Ages 6–7 · Numbers to 30 · 2s, 5s, 10s",
            Difficulty::Super => "Ages 8–9 · Big numbers · Full tables & patterns",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PlayStyle {
    Chill,
    Clock,
    Streak,
}

impl PlayStyle {
    pub fn as_str(&self) -> &'static str {
        match self {
            PlayStyle::Chill => "chill",
            PlayStyle::Clock => "clock",
            PlayStyle::Streak => "streak",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "clock" => PlayStyle::Clock,
            "streak" => PlayStyle::Streak,
            _ => PlayStyle::Chill,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Technique {
    pub name: String,
    pub steps: Vec<String>,
    pub worked: String,
}

#[derive(Clone, Debug)]
pub struct Question {
    pub visual: String,
    pub visual_small: bool,
    pub prompt: String,
    pub sub: String,
    pub choices: Vec<String>,
    pub answer: String,
    pub reveal: Option<String>,
    pub speak: String,
    pub technique: Option<Technique>,
    pub is_spell: bool,
    pub word: Option<String>,
}

#[derive(Clone, Debug)]
pub struct GameMeta {
    pub id: &'static str,
    pub subject: &'static str,
    pub icon: &'static str,
    pub cls: &'static str,
    pub title: &'static str,
    pub desc: &'static str,
    pub qs: &'static str,
}

#[derive(Clone, Debug)]
pub struct ReviewItem {
    pub idx: u32,
    pub prompt: String,
    pub sub: String,
    pub visual: String,
    pub user_answer: String,
    pub correct_answer: String,
    pub is_correct: bool,
    pub technique_name: Option<String>,
    pub technique_worked: Option<String>,
    pub technique_steps: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct GameSession {
    pub game_id: String,
    pub title: String,
    pub current: Question,
    pub idx: u32,
    pub total: u32, // 0 for endless
    pub score: u32,
    pub lives: i32,
    pub boost: u32,
    pub locked: bool,
    pub spell_built: String,
    pub spell_tiles_used: Vec<String>, // indices of buttons or characters used
    pub tech_open: bool,
    pub time_left: u32,
    pub time_total: u32,
    pub history: Vec<ReviewItem>,
    pub queue_filter: String,
    pub queue_open_mobile: bool,
}
