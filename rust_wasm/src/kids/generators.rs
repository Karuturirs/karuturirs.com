use super::data::{RHYMES, STORIES, WORDS};
use super::models::{Difficulty, Question, Technique};
use std::cmp::{max, min};
use std::collections::HashSet;

pub fn rnd(low: i32, high: i32) -> i32 {
    if low >= high {
        return low;
    }
    let r = js_sys::Math::random();
    low + (r * (high - low + 1) as f64).floor() as i32
}

pub fn pick<'a, T>(slice: &'a [T]) -> &'a T {
    let idx = (js_sys::Math::random() * slice.len() as f64).floor() as usize;
    &slice[idx.min(slice.len() - 1)]
}

pub fn shuffle<T: Clone>(slice: &[T]) -> Vec<T> {
    let mut vec = slice.to_vec();
    let n = vec.len();
    for i in (1..n).rev() {
        let j = (js_sys::Math::random() * (i + 1) as f64).floor() as usize;
        vec.swap(i, j);
    }
    vec
}

pub fn num_choices(ans: i32, spread: i32) -> Vec<String> {
    let mut set: HashSet<i32> = HashSet::new();
    set.insert(ans);
    let mut tries = 0;
    while set.len() < 4 && tries < 40 {
        let offset = rnd(-spread, spread);
        let val = ans + offset;
        if val >= 0 {
            set.insert(val);
        }
        tries += 1;
    }
    let mut num = 1;
    while set.len() < 4 {
        set.insert(ans + num);
        num += 1;
    }
    let list: Vec<i32> = set.into_iter().collect();
    let shuffled = shuffle(&list);
    shuffled.into_iter().map(|n| n.to_string()).collect()
}

pub fn words_up_to(max_len: usize) -> Vec<(&'static str, &'static str)> {
    WORDS
        .iter()
        .filter(|w| w.word.len() <= max_len)
        .map(|w| (w.word, w.emoji))
        .collect()
}

// Techniques
fn tech_add(a: i32, b: i32) -> Technique {
    let ans = a + b;
    if a == b {
        return Technique {
            name: "✌️ Doubles Trick".to_string(),
            steps: vec![
                "Both numbers are the same — it is a double!".to_string(),
                format!("Double {a} means {a} + {a}."),
                format!("Count in 2s or use a double you know: {a} + {a} = {ans}."),
            ],
            worked: format!("{a} + {b} = {ans}"),
        };
    }
    if ans > 10 && max(a, b) < 10 {
        let big = max(a, b);
        let small = min(a, b);
        let need = 10 - big;
        let left = small - need;
        if need > 0 && left >= 0 {
            return Technique {
                name: "🔟 Make a Ten".to_string(),
                steps: vec![
                    format!("Start with the bigger number: {big}."),
                    format!("It needs {need} more to make 10. Take {need} from the {small}."),
                    format!("Now you have 10 + {left} left over."),
                    format!("10 + {left} = {ans}. Easy!"),
                ],
                worked: format!("{big} + {small} → 10 + {left} = {ans}"),
            };
        }
    }
    if a >= 10 || b >= 10 {
        let at = (a / 10) * 10;
        let ao = a - at;
        let bt = (b / 10) * 10;
        let bo = b - bt;
        return Technique {
            name: "✂️ Split & Add (tens and ones)".to_string(),
            steps: vec![
                format!("Split {a} into {at} and {ao}. Split {b} into {bt} and {bo}."),
                format!("Add the tens: {at} + {bt} = {}.", at + bt),
                format!("Add the ones: {ao} + {bo} = {}.", ao + bo),
                format!("Put them together: {} + {} = {ans}.", at + bt, ao + bo),
            ],
            worked: format!("{a} + {b} = {ans}"),
        };
    }

    let mut seq = Vec::new();
    for i in 1..=b {
        seq.push((a + i).to_string());
    }
    Technique {
        name: "👉 Count On".to_string(),
        steps: vec![
            format!("Start from the bigger number — put {} in your head.", max(a, b)),
            format!("Now count on {} more, touching a finger for each jump.", min(a, b)),
            format!("Say it slowly: {}.", seq.join(", ")),
            format!("You landed on {ans} — that is the answer!"),
        ],
        worked: format!("{a} + {b} = {ans}"),
    }
}

fn tech_sub(a: i32, b: i32) -> Technique {
    let ans = a - b;
    if b <= 5 {
        let mut seq = Vec::new();
        for i in 1..=b {
            seq.push((a - i).to_string());
        }
        return Technique {
            name: "👈 Count Back".to_string(),
            steps: vec![
                format!("Put {a} in your head."),
                format!("Count back {b} jumps, one finger at a time."),
                format!("Say it: {}.", seq.join(", ")),
                format!("You landed on {ans}."),
            ],
            worked: format!("{a} − {b} = {ans}"),
        };
    }
    if ans <= 5 {
        let mut up = Vec::new();
        for i in (b + 1)..=a {
            up.push(i.to_string());
        }
        return Technique {
            name: "⬆️ Count Up (find the gap)".to_string(),
            steps: vec![
                "Take-away can be tricky — so count UP instead!".to_string(),
                format!("Start at {b} and count up to {a}: {}.", up.join(", ")),
                format!("Count your jumps: you jumped {ans} times."),
                format!("So the gap — and the answer — is {ans}."),
            ],
            worked: format!("{a} − {b} = {ans}"),
        };
    }
    let at = (a / 10) * 10;
    let jump1 = a - at;
    let rem = b - jump1;
    Technique {
        name: "🔟 Jump Back to Ten".to_string(),
        steps: vec![
            format!("Start at {a}. First jump back to the friendly ten: {at} (that is {jump1} back)."),
            format!("You still need to take away {rem} more."),
            format!("{at} − {rem} = {ans}."),
            "Done — two easy jumps beat one big jump!".to_string(),
        ],
        worked: format!("{a} − {b} = {ans}"),
    }
}

fn tech_mul(t: i32, f: i32) -> Technique {
    let ans = t * f;
    let mut seq = Vec::new();
    for i in 1..=f {
        seq.push((t * i).to_string());
    }
    Technique {
        name: "👥 Groups & Skip-Counting".to_string(),
        steps: vec![
            format!("{t} × {f} means {f} groups with {t} in each group."),
            format!("Skip-count in {t}s, holding up a finger for each group."),
            format!("Count: {} — that is {f} jumps.", seq.join(", ")),
            format!("The last number you say is the answer: {ans}."),
        ],
        worked: format!("{t} × {f} = {ans}"),
    }
}

// 1. Counting Fun
pub fn gen_counting(diff: Difficulty, boost: u32) -> Question {
    let emoji = *pick(&["🍎", "⭐", "🦋", "🎈", "🍓", "🐥", "🚗", "🌸"]);
    let cap = match diff {
        Difficulty::Little => 10 + (boost as i32 * 2),
        Difficulty::Growing => 20 + (boost as i32 * 2),
        Difficulty::Super => 30 + (boost as i32 * 2),
    };
    let n = if diff == Difficulty::Little {
        rnd(2, cap)
    } else {
        rnd(4, cap)
    };

    let mut visual = String::new();
    for _ in 0..n {
        visual.push_str(emoji);
        visual.push(' ');
    }

    let mut groups = Vec::new();
    let mut rem = n;
    while rem > 0 {
        let g = min(5, rem);
        groups.push(g.to_string());
        rem -= g;
    }

    let group_str = if groups.len() > 1 {
        format!("{} — add the groups together.", groups.join(" + "))
    } else {
        groups.join(" + ")
    };

    Question {
        visual,
        visual_small: n > 12,
        prompt: "How many can you count?".to_string(),
        sub: "Tap the number that matches.".to_string(),
        choices: num_choices(n, 4),
        answer: n.to_string(),
        reveal: None,
        speak: "How many can you count? Count carefully!".to_string(),
        technique: Some(Technique {
            name: "👆 Touch & Count in 5s".to_string(),
            steps: vec![
                "Touch (or point at) each one as you count — never count the same one twice!".to_string(),
                format!("Counting in groups of 5 helps: {group_str}"),
                "The last number you say is how many there are.".to_string(),
                format!("There are {n} altogether."),
            ],
            worked: format!("Answer = {n}"),
        }),
        is_spell: false,
        word: None,
    }
}

// 2. Addition Adventure
pub fn gen_add(diff: Difficulty, boost: u32) -> Question {
    let max_val = match diff {
        Difficulty::Little => 10 + (boost as i32 * 4),
        Difficulty::Growing => 30 + (boost as i32 * 5),
        Difficulty::Super => 100 + (boost as i32 * 10),
    };

    let a = rnd(1, max_val);
    let mut b = rnd(1, max(2, if max_val - a > 1 { max_val - a } else { max_val }));
    if a + b > max_val + (max_val / 2) {
        b = max(1, max_val - a);
    }
    let ans = a + b;

    let style = rnd(1, 3);
    let (visual, small, prompt, sub_text, speak) = if diff != Difficulty::Super && style == 1 {
        let mut vis = String::new();
        for _ in 0..a {
            vis.push_str("🍊");
        }
        vis.push_str(" ＋ ");
        for _ in 0..b {
            vis.push_str("🍊");
        }
        (
            vis,
            (a + b) > 12,
            format!("{a} + {b} = ?"),
            "Add them together. The pictures can help — or tap “Show me how”!".to_string(),
            format!("What is {a} plus {b}?"),
        )
    } else if style == 2 {
        let s = pick(STORIES);
        (
            s.emoji.to_string(),
            false,
            format!("{} has {a} {}. {} finds {b} more. How many now?", s.who, s.thing, s.who),
            format!("A story sum — it is still {a} + {b}. Tap “Show me how” for the trick!"),
            format!("{} has {a} {}. {} finds {b} more. How many now?", s.who, s.thing, s.who),
        )
    } else {
        (
            String::new(),
            false,
            format!("{a} + {b} = ?"),
            if diff == Difficulty::Super {
                "Add them together — tap “Show me how” to split into tens and ones!".to_string()
            } else {
                "Add them together — tap “Show me how” for a trick!".to_string()
            },
            format!("What is {a} plus {b}?"),
        )
    };

    Question {
        visual,
        visual_small: small,
        prompt,
        sub: sub_text,
        choices: num_choices(ans, max(3, ans / 3)),
        answer: ans.to_string(),
        reveal: None,
        speak,
        technique: Some(tech_add(a, b)),
        is_spell: false,
        word: None,
    }
}

// 3. Take-Away Safari
pub fn gen_subtract(diff: Difficulty, boost: u32) -> Question {
    let max_val = match diff {
        Difficulty::Little => 10 + (boost as i32 * 4),
        Difficulty::Growing => 30 + (boost as i32 * 5),
        Difficulty::Super => 100 + (boost as i32 * 10),
    };

    let a = rnd(3, max_val);
    let b = rnd(1, a - 1);
    let ans = a - b;

    let style = rnd(1, 3);
    let (visual, small, prompt, sub_text, speak) = if diff != Difficulty::Super && style == 1 {
        let mut vis = String::new();
        for _ in 0..ans {
            vis.push_str("🍌 ");
        }
        vis.push_str("✂️ ");
        for _ in 0..b {
            vis.push_str("💨 ");
        }
        (
            vis,
            a > 12,
            format!("{a} − {b} = ?"),
            format!("Start with {a}, cross out {b}. What is left?"),
            format!("What is {a} take away {b}?"),
        )
    } else if style == 2 {
        let s = pick(STORIES);
        (
            s.emoji.to_string(),
            false,
            format!("{} had {a} {}. {} gave {b} to a friend. How many left?", s.who, s.thing, s.who),
            format!("Take-away story sum — {a} − {b}. Tap “Show me how” for help!"),
            format!("{} had {a} {}. {} gave {b} to a friend. How many left?", s.who, s.thing, s.who),
        )
    } else {
        (
            String::new(),
            false,
            format!("{a} − {b} = ?"),
            "Take away the second number. Tap “Show me how” for jumping tricks!".to_string(),
            format!("What is {a} minus {b}?"),
        )
    };

    Question {
        visual,
        visual_small: small,
        prompt,
        sub: sub_text,
        choices: num_choices(ans, max(3, ans / 3)),
        answer: ans.to_string(),
        reveal: None,
        speak,
        technique: Some(tech_sub(a, b)),
        is_spell: false,
        word: None,
    }
}

// 4. Times Table Challenge
pub fn gen_multiply(diff: Difficulty, boost: u32) -> Question {
    let (tables, max_factor) = match diff {
        Difficulty::Little => (vec![2, 5, 10], 5 + (boost as i32)),
        Difficulty::Growing => (vec![2, 3, 4, 5, 10], 10 + (boost as i32)),
        Difficulty::Super => (vec![3, 4, 6, 7, 8, 9, 11, 12], 12),
    };

    let t = *pick(&tables);
    let f = rnd(1, max_factor);
    let ans = t * f;

    let visual = if f <= 5 {
        let group_dot = "⭐".repeat(t as usize);
        let mut v = String::new();
        for _ in 0..f {
            v.push('[');
            v.push_str(&group_dot);
            v.push_str("] ");
        }
        v
    } else {
        String::new()
    };

    Question {
        visual,
        visual_small: t * f > 20,
        prompt: format!("{t} × {f} = ?"),
        sub: format!("{f} groups of {t}. Tap “Show me how” for skip-counting!"),
        choices: num_choices(ans, max(4, ans / 4)),
        answer: ans.to_string(),
        reveal: None,
        speak: format!("What is {t} times {f}?"),
        technique: Some(tech_mul(t, f)),
        is_spell: false,
        word: None,
    }
}

// 5. Missing Number
pub fn gen_missing(diff: Difficulty, _boost: u32) -> Question {
    let mode = rnd(1, 3);
    if mode == 1 {
        // Sequence pattern
        let step = match diff {
            Difficulty::Little => *pick(&[1, 2]),
            Difficulty::Growing => *pick(&[1, 2, 5, 10]),
            Difficulty::Super => *pick(&[2, 3, 5, 10]),
        };
        let start = rnd(1, 20);
        let pos = rnd(1, 3); // 1, 2, 3
        let ans = start + (pos * step);
        let mut display = Vec::new();
        for i in 0..4 {
            if i == pos {
                display.push("___".to_string());
            } else {
                display.push((start + i * step).to_string());
            }
        }
        Question {
            visual: "🔢".to_string(),
            visual_small: false,
            prompt: display.join(" , "),
            sub: format!("What number is hiding? Look at how the numbers jump! (Jumping by {step})"),
            choices: num_choices(ans, max(3, step * 2)),
            answer: ans.to_string(),
            reveal: None,
            speak: "What number fills the gap in the pattern?".to_string(),
            technique: Some(Technique {
                name: "🔎 Find the Jump Pattern".to_string(),
                steps: vec![
                    "Look at two numbers next to each other.".to_string(),
                    format!("How much is added each time? The jump is +{step}."),
                    format!("Take the number before the gap and add {step} to it: {} + {step} = {ans}.", ans - step),
                    "Check the next number to be sure it matches!".to_string(),
                ],
                worked: format!("Pattern +{step} → missing number is {ans}"),
            }),
            is_spell: false,
            word: None,
        }
    } else if mode == 2 {
        // a + ? = total
        let a = match diff {
            Difficulty::Little => rnd(1, 7),
            Difficulty::Growing => rnd(3, 15),
            Difficulty::Super => rnd(10, 45),
        };
        let ans = match diff {
            Difficulty::Little => rnd(1, 5),
            Difficulty::Growing => rnd(2, 12),
            Difficulty::Super => rnd(5, 30),
        };
        let total = a + ans;
        Question {
            visual: "🧩".to_string(),
            visual_small: false,
            prompt: format!("{a} + ? = {total}"),
            sub: "What number is hiding in the puzzle?".to_string(),
            choices: num_choices(ans, 4),
            answer: ans.to_string(),
            reveal: None,
            speak: format!("{a} plus what equals {total}?"),
            technique: Some(Technique {
                name: "🔄 Work Backwards".to_string(),
                steps: vec![
                    format!("You start with {a} and need to reach {total}."),
                    "Undo addition with take-away!".to_string(),
                    format!("{total} − {a} = {ans}."),
                    format!("Check: {a} + {ans} = {total}. ✓"),
                ],
                worked: format!("? = {ans}"),
            }),
            is_spell: false,
            word: None,
        }
    } else {
        // ? - ans = left
        let left = match diff {
            Difficulty::Little => rnd(2, 6),
            Difficulty::Growing => rnd(4, 15),
            Difficulty::Super => rnd(10, 40),
        };
        let ans = match diff {
            Difficulty::Little => rnd(1, 4),
            Difficulty::Growing => rnd(2, 10),
            Difficulty::Super => rnd(5, 25),
        };
        let total = left + ans;
        Question {
            visual: "🧩".to_string(),
            visual_small: false,
            prompt: format!("? − {ans} = {left}"),
            sub: "What did we start with before taking away?".to_string(),
            choices: num_choices(total, 4),
            answer: total.to_string(),
            reveal: None,
            speak: format!("What minus {ans} equals {left}?"),
            technique: Some(Technique {
                name: "🔄 Work Backwards".to_string(),
                steps: vec![
                    format!("Something was taken away ({ans}) and {left} was left."),
                    "Undo it with the opposite job: add them back together!".to_string(),
                    format!("{left} + {ans} = {total}."),
                    format!("Check it: {total} − {ans} = {left}. ✓"),
                ],
                worked: format!("? = {total}"),
            }),
            is_spell: false,
            word: None,
        }
    }
}

// 6. Spell It!
pub fn gen_spell(diff: Difficulty, _boost: u32) -> Question {
    let max_len = match diff {
        Difficulty::Little => 3,
        Difficulty::Growing => 5,
        Difficulty::Super => 7,
    };
    let pool = words_up_to(max_len);
    let item = *pick(&pool);
    let word = item.0.to_string();
    let first_ch = word.chars().next().unwrap_or('A');

    Question {
        visual: item.1.to_string(),
        visual_small: false,
        prompt: "Spell this word!".to_string(),
        sub: format!("It starts with “{first_ch}”. Tap the letters in order."),
        choices: Vec::new(),
        answer: word.clone(),
        reveal: Some(word.clone()),
        speak: format!("Can you spell the word in the picture? It starts with {first_ch}."),
        technique: Some(Technique {
            name: "🔊 Sound It Out".to_string(),
            steps: vec![
                "Say the word slowly and stretch the sounds out.".to_string(),
                "Listen for the first sound, then the middle, then the end.".to_string(),
                "Tap one letter for each sound you hear, in order.".to_string(),
                "Read your letters back — do they say the word?".to_string(),
            ],
            worked: format!("First letter: {first_ch} · {} letters long", word.len()),
        }),
        is_spell: true,
        word: Some(word),
    }
}

// 7. Missing Letter
pub fn gen_missing_letter(diff: Difficulty, _boost: u32) -> Question {
    let max_len = match diff {
        Difficulty::Little => 3,
        Difficulty::Growing => 5,
        Difficulty::Super => 7,
    };
    let pool = words_up_to(max_len);
    let item = *pick(&pool);
    let chars: Vec<char> = item.0.chars().collect();
    let idx = if chars.len() > 3 {
        rnd(1, (chars.len() - 2) as i32) as usize
    } else {
        rnd(0, (chars.len() - 1) as i32) as usize
    };
    let letter = chars[idx].to_string();

    let mut display = Vec::new();
    for (i, &c) in chars.iter().enumerate() {
        if i == idx {
            display.push("_".to_string());
        } else {
            display.push(c.to_string());
        }
    }

    let all_letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let all_chars: Vec<char> = all_letters.chars().collect();
    let mut opts: HashSet<String> = HashSet::new();
    opts.insert(letter.clone());
    while opts.len() < 4 {
        let ch = *pick(&all_chars);
        opts.insert(ch.to_string());
    }
    let choices = shuffle(&opts.into_iter().collect::<Vec<String>>());

    Question {
        visual: item.1.to_string(),
        visual_small: false,
        prompt: display.join(" "),
        sub: "Which letter is missing from this word?".to_string(),
        choices,
        answer: letter,
        reveal: Some(item.0.to_string()),
        speak: "Which letter is missing from this word?".to_string(),
        technique: Some(Technique {
            name: "🔊 Sound It Out".to_string(),
            steps: vec![
                "Look at the picture and say the word slowly.".to_string(),
                "Say the sounds one by one and find the gap.".to_string(),
                "Which sound has no letter yet? That is your missing letter!".to_string(),
            ],
            worked: format!("The word has {} letters", chars.len()),
        }),
        is_spell: false,
        word: None,
    }
}

// 8. Picture Words (Vocab)
pub fn gen_vocab(diff: Difficulty, _boost: u32) -> Question {
    let max_len = match diff {
        Difficulty::Little => 4,
        Difficulty::Growing => 6,
        Difficulty::Super => 8,
    };
    let pool = words_up_to(max_len);
    let shuffled_pool = shuffle(&pool);
    let target = shuffled_pool[0];
    let others = &shuffled_pool[1..min(4, shuffled_pool.len())];

    let mut choices_list = vec![target.0.to_string()];
    for o in others {
        choices_list.push(o.0.to_string());
    }
    let choices = shuffle(&choices_list);

    let first_ch = target.0.chars().next().unwrap_or('A');

    Question {
        visual: target.1.to_string(),
        visual_small: false,
        prompt: "What is this?".to_string(),
        sub: "Tap the right word for the picture.".to_string(),
        choices,
        answer: target.0.to_string(),
        reveal: None,
        speak: "What is this? Tap the right word.".to_string(),
        technique: Some(Technique {
            name: "👀 First-Letter Clue".to_string(),
            steps: vec![
                "Name the picture out loud.".to_string(),
                "What sound does it start with?".to_string(),
                "Find the word that starts with that same sound — and check its first letter!".to_string(),
            ],
            worked: format!("It starts with “{first_ch}”"),
        }),
        is_spell: false,
        word: None,
    }
}

// 9. Rhyme Time
pub fn gen_rhyme(_diff: Difficulty, _boost: u32) -> Question {
    let r = pick(RHYMES);
    let mut choices_list = vec![r.rhyme.to_string()];
    for o in r.others {
        choices_list.push(o.to_string());
    }
    let choices = shuffle(&choices_list);

    Question {
        visual: "🎶".to_string(),
        visual_small: false,
        prompt: format!("Which word rhymes with {}?", r.base),
        sub: format!("{} sounds like… think of the ending sound!", r.base),
        choices,
        answer: r.rhyme.to_string(),
        reveal: None,
        speak: format!("Which word rhymes with {}?", r.base),
        technique: Some(Technique {
            name: "👂 Listen to the Ending".to_string(),
            steps: vec![
                format!("Say {} slowly and listen to its ending sound.", r.base),
                "Say each choice out loud, one at a time.".to_string(),
                "Rhyming words end with the SAME sound — pick the one that matches!".to_string(),
            ],
            worked: format!("{} ends with the same sound as its rhyme", r.base),
        }),
        is_spell: false,
        word: None,
    }
}

pub fn generate_question(game_id: &str, diff: Difficulty, boost: u32) -> Question {
    match game_id {
        "counting" => gen_counting(diff, boost),
        "add" => gen_add(diff, boost),
        "subtract" => gen_subtract(diff, boost),
        "multiply" => gen_multiply(diff, boost),
        "missing" => gen_missing(diff, boost),
        "spell" => gen_spell(diff, boost),
        "missingLetter" => gen_missing_letter(diff, boost),
        "vocab" => gen_vocab(diff, boost),
        "rhyme" => gen_rhyme(diff, boost),
        _ => {
            // "adventure" or unknown -> pick random game
            let all_ids = [
                "counting",
                "add",
                "subtract",
                "multiply",
                "missing",
                "spell",
                "missingLetter",
                "vocab",
                "rhyme",
            ];
            let picked_id = *pick(&all_ids);
            generate_question(picked_id, diff, boost)
        }
    }
}
