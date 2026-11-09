use super::models::GameMeta;

pub struct WordDef {
    pub word: &'static str,
    pub emoji: &'static str,
}

pub const WORDS: &[WordDef] = &[
    WordDef { word: "CAT", emoji: "🐱" },
    WordDef { word: "DOG", emoji: "🐶" },
    WordDef { word: "SUN", emoji: "☀️" },
    WordDef { word: "EGG", emoji: "🥚" },
    WordDef { word: "ANT", emoji: "🐜" },
    WordDef { word: "BEE", emoji: "🐝" },
    WordDef { word: "PIG", emoji: "🐷" },
    WordDef { word: "COW", emoji: "🐮" },
    WordDef { word: "BUS", emoji: "🚌" },
    WordDef { word: "CUP", emoji: "☕" },
    WordDef { word: "HAT", emoji: "🎩" },
    WordDef { word: "MAP", emoji: "🗺️" },
    WordDef { word: "FISH", emoji: "🐟" },
    WordDef { word: "BIRD", emoji: "🐦" },
    WordDef { word: "FROG", emoji: "🐸" },
    WordDef { word: "CAKE", emoji: "🍰" },
    WordDef { word: "MILK", emoji: "🥛" },
    WordDef { word: "BOOK", emoji: "📖" },
    WordDef { word: "STAR", emoji: "⭐" },
    WordDef { word: "MOON", emoji: "🌙" },
    WordDef { word: "TREE", emoji: "🌳" },
    WordDef { word: "BOAT", emoji: "⛵" },
    WordDef { word: "DUCK", emoji: "🦆" },
    WordDef { word: "LION", emoji: "🦁" },
    WordDef { word: "BEAR", emoji: "🐻" },
    WordDef { word: "APPLE", emoji: "🍎" },
    WordDef { word: "HORSE", emoji: "🐴" },
    WordDef { word: "TIGER", emoji: "🐯" },
    WordDef { word: "ZEBRA", emoji: "🦓" },
    WordDef { word: "PLANE", emoji: "✈️" },
    WordDef { word: "TRAIN", emoji: "🚂" },
    WordDef { word: "HOUSE", emoji: "🏠" },
    WordDef { word: "WATER", emoji: "💧" },
    WordDef { word: "BREAD", emoji: "🍞" },
    WordDef { word: "BANANA", emoji: "🍌" },
    WordDef { word: "ORANGE", emoji: "🍊" },
    WordDef { word: "PENCIL", emoji: "✏️" },
    WordDef { word: "SCHOOL", emoji: "🏫" },
    WordDef { word: "FLOWER", emoji: "🌸" },
    WordDef { word: "TURTLE", emoji: "🐢" },
    WordDef { word: "MONKEY", emoji: "🐵" },
    WordDef { word: "ROCKET", emoji: "🚀" },
    WordDef { word: "GIRAFFE", emoji: "🦒" },
    WordDef { word: "DOLPHIN", emoji: "🐬" },
    WordDef { word: "PENGUIN", emoji: "🐧" },
    WordDef { word: "RAINBOW", emoji: "🌈" },
    WordDef { word: "OCTOPUS", emoji: "🐙" },
    WordDef { word: "ELEPHANT", emoji: "🐘" },
];

pub struct RhymeDef {
    pub base: &'static str,
    pub rhyme: &'static str,
    pub others: &'static [&'static str],
}

pub const RHYMES: &[RhymeDef] = &[
    RhymeDef { base: "CAT", rhyme: "HAT", others: &["DOG", "SUN", "EGG"] },
    RhymeDef { base: "DOG", rhyme: "FROG", others: &["CAT", "BEE", "BUS"] },
    RhymeDef { base: "SUN", rhyme: "RUN", others: &["MOON", "PIG", "MAP"] },
    RhymeDef { base: "CAKE", rhyme: "SNAKE", others: &["MILK", "BOAT", "STAR"] },
    RhymeDef { base: "BOAT", rhyme: "GOAT", others: &["DUCK", "TREE", "FISH"] },
    RhymeDef { base: "BEE", rhyme: "TREE", others: &["ANT", "COW", "CUP"] },
    RhymeDef { base: "PIG", rhyme: "WIG", others: &["HEN", "BUS", "EGG"] },
    RhymeDef { base: "MOON", rhyme: "SPOON", others: &["CAKE", "LION", "BOAT"] },
    RhymeDef { base: "BALL", rhyme: "FALL", others: &["BOOK", "DUCK", "SUN"] },
    RhymeDef { base: "KING", rhyme: "RING", others: &["BEAR", "CAKE", "FROG"] },
    RhymeDef { base: "FOX", rhyme: "BOX", others: &["BEE", "MOON", "PIG"] },
    RhymeDef { base: "HEN", rhyme: "TEN", others: &["CAT", "BOAT", "MILK"] },
];

pub struct StoryDef {
    pub who: &'static str,
    pub thing: &'static str,
    pub emoji: &'static str,
}

pub const STORIES: &[StoryDef] = &[
    StoryDef { who: "Mia", thing: "apples", emoji: "🍎" },
    StoryDef { who: "Leo", thing: "marbles", emoji: "🔮" },
    StoryDef { who: "Ava", thing: "stickers", emoji: "⭐" },
    StoryDef { who: "Sam", thing: "crayons", emoji: "🖍️" },
    StoryDef { who: "Zoe", thing: "shells", emoji: "🐚" },
    StoryDef { who: "Ben", thing: "toy cars", emoji: "🚗" },
];

pub struct ColorDef {
    pub name: &'static str,
    pub emoji: &'static str,
}

pub const COLORS: &[ColorDef] = &[
    ColorDef { name: "Red", emoji: "🔴" },
    ColorDef { name: "Blue", emoji: "🔵" },
    ColorDef { name: "Yellow", emoji: "🟡" },
    ColorDef { name: "Green", emoji: "🟢" },
    ColorDef { name: "Purple", emoji: "🟣" },
    ColorDef { name: "Orange", emoji: "🟠" },
    ColorDef { name: "Pink", emoji: "🌸" },
    ColorDef { name: "Brown", emoji: "🟤" },
    ColorDef { name: "Black", emoji: "⚫" },
    ColorDef { name: "White", emoji: "⚪" },
];

pub struct ShapeDef {
    pub name: &'static str,
    pub emoji: &'static str,
    pub desc: &'static str,
}

pub const SHAPES: &[ShapeDef] = &[
    ShapeDef { name: "Circle", emoji: "⭕", desc: "round with no corners" },
    ShapeDef { name: "Square", emoji: "🟦", desc: "4 equal sides and 4 corners" },
    ShapeDef { name: "Triangle", emoji: "🔺", desc: "3 sides and 3 sharp points" },
    ShapeDef { name: "Star", emoji: "⭐", desc: "5 shining points" },
    ShapeDef { name: "Heart", emoji: "💖", desc: "gentle curves meeting at a point" },
    ShapeDef { name: "Diamond", emoji: "🔶", desc: "tilted square like a kite" },
];

pub struct AnimalSoundDef {
    pub animal: &'static str,
    pub sound: &'static str,
    pub emoji: &'static str,
    pub prompt: &'static str,
}

pub const ANIMAL_SOUNDS: &[AnimalSoundDef] = &[
    AnimalSoundDef { animal: "Dog", sound: "Woof woof!", emoji: "🐶", prompt: "Who says Woof woof?" },
    AnimalSoundDef { animal: "Cat", sound: "Meow meow!", emoji: "🐱", prompt: "Who says Meow meow?" },
    AnimalSoundDef { animal: "Cow", sound: "Moo moo!", emoji: "🐮", prompt: "Who says Moo moo?" },
    AnimalSoundDef { animal: "Duck", sound: "Quack quack!", emoji: "🦆", prompt: "Who says Quack quack?" },
    AnimalSoundDef { animal: "Lion", sound: "Roar!", emoji: "🦁", prompt: "Who says Roar?" },
    AnimalSoundDef { animal: "Sheep", sound: "Baa baa!", emoji: "🐑", prompt: "Who says Baa baa?" },
    AnimalSoundDef { animal: "Frog", sound: "Ribbit ribbit!", emoji: "🐸", prompt: "Who says Ribbit ribbit?" },
    AnimalSoundDef { animal: "Pig", sound: "Oink oink!", emoji: "🐷", prompt: "Who says Oink oink?" },
    AnimalSoundDef { animal: "Rooster", sound: "Cock-a-doodle-doo!", emoji: "🐓", prompt: "Who says Cock-a-doodle-doo?" },
    AnimalSoundDef { animal: "Bee", sound: "Bzzzz!", emoji: "🐝", prompt: "Who says Bzzzz?" },
    AnimalSoundDef { animal: "Owl", sound: "Hoot hoot!", emoji: "🦉", prompt: "Who says Hoot hoot?" },
    AnimalSoundDef { animal: "Horse", sound: "Neigh!", emoji: "🐴", prompt: "Who says Neigh?" },
];

pub struct ComparisonDef {
    pub big: &'static str,
    pub big_emoji: &'static str,
    pub small: &'static str,
    pub small_emoji: &'static str,
}

pub const COMPARISONS: &[ComparisonDef] = &[
    ComparisonDef { big: "Elephant", big_emoji: "🐘", small: "Mouse", small_emoji: "🐭" },
    ComparisonDef { big: "Big Tree", big_emoji: "🌳", small: "Small Flower", small_emoji: "🌸" },
    ComparisonDef { big: "Airplane", big_emoji: "✈️", small: "Bicycle", small_emoji: "🚲" },
    ComparisonDef { big: "Bus", big_emoji: "🚌", small: "Skateboard", small_emoji: "🛹" },
    ComparisonDef { big: "Whale", big_emoji: "🐋", small: "Goldfish", small_emoji: "🐟" },
    ComparisonDef { big: "House", big_emoji: "🏠", small: "Apple", small_emoji: "🍎" },
    ComparisonDef { big: "Mountain", big_emoji: "⛰️", small: "Pebble", small_emoji: "🪨" },
    ComparisonDef { big: "Watermelon", big_emoji: "🍉", small: "Strawberry", small_emoji: "🍓" },
];

pub struct AbcDef {
    pub letter: &'static str,
    pub word: &'static str,
    pub emoji: &'static str,
}

pub const ABC_LIST: &[AbcDef] = &[
    AbcDef { letter: "A", word: "Apple", emoji: "🍎" },
    AbcDef { letter: "B", word: "Bear", emoji: "🐻" },
    AbcDef { letter: "C", word: "Cat", emoji: "🐱" },
    AbcDef { letter: "D", word: "Dog", emoji: "🐶" },
    AbcDef { letter: "E", word: "Elephant", emoji: "🐘" },
    AbcDef { letter: "F", word: "Fish", emoji: "🐟" },
    AbcDef { letter: "G", word: "Giraffe", emoji: "🦒" },
    AbcDef { letter: "H", word: "House", emoji: "🏠" },
    AbcDef { letter: "I", word: "Ice cream", emoji: "🍦" },
    AbcDef { letter: "J", word: "Juice", emoji: "🧃" },
    AbcDef { letter: "K", word: "Kite", emoji: "🪁" },
    AbcDef { letter: "L", word: "Lion", emoji: "🦁" },
    AbcDef { letter: "M", word: "Monkey", emoji: "🐵" },
    AbcDef { letter: "N", word: "Nest", emoji: "🪹" },
    AbcDef { letter: "O", word: "Orange", emoji: "🍊" },
    AbcDef { letter: "P", word: "Penguin", emoji: "🐧" },
    AbcDef { letter: "Q", word: "Queen", emoji: "👑" },
    AbcDef { letter: "R", word: "Rainbow", emoji: "🌈" },
    AbcDef { letter: "S", word: "Sun", emoji: "☀️" },
    AbcDef { letter: "T", word: "Tiger", emoji: "🐯" },
    AbcDef { letter: "U", word: "Umbrella", emoji: "☂️" },
    AbcDef { letter: "V", word: "Van", emoji: "🚐" },
    AbcDef { letter: "W", word: "Watermelon", emoji: "🍉" },
    AbcDef { letter: "X", word: "Xylophone", emoji: "🎵" },
    AbcDef { letter: "Y", word: "Yacht", emoji: "⛵" },
    AbcDef { letter: "Z", word: "Zebra", emoji: "🦓" },
];

pub const PRAISES: &[&str] = &[
    "Brilliant! 🌟",
    "Super star! ⭐",
    "You did it! 🎉",
    "Amazing work! 🏆",
    "Fantastic! 🥳",
    "Clever you! 🧠",
    "Hooray! 🎈",
    "Wonderful! 🌈",
];

pub const GAMES_LIST: &[GameMeta] = &[
    // Pre-K games
    GameMeta {
        id: "colorsShapes",
        subject: "prek",
        icon: "🎨",
        cls: "sun",
        title: "Colors & Shapes",
        desc: "Match friendly shapes and bright rainbow colors with Professor Owl!",
        qs: "10 questions",
    },
    GameMeta {
        id: "animalSounds",
        subject: "prek",
        icon: "🦁",
        cls: "berry",
        title: "Animal Sounds Safari",
        desc: "Who says woof? Who says moo? Listen and tap the matching animal!",
        qs: "10 questions",
    },
    GameMeta {
        id: "bigSmall",
        subject: "prek",
        icon: "🐘",
        cls: "leaf",
        title: "Big & Small",
        desc: "Compare gentle giants and tiny friends — which one is bigger or smaller?",
        qs: "10 questions",
    },
    GameMeta {
        id: "abcPhonics",
        subject: "prek",
        icon: "🔤",
        cls: "sky",
        title: "ABC Phonics Safari",
        desc: "Explore letters from A to Z! A is for Apple, B is for Bear, C is for Cat.",
        qs: "10 letters",
    },
    // Maths games
    GameMeta {
        id: "counting",
        subject: "maths",
        icon: "🍎",
        cls: "coral",
        title: "Counting Fun",
        desc: "Count the objects and tap the right number. How high can you count?",
        qs: "10 questions",
    },
    GameMeta {
        id: "add",
        subject: "maths",
        icon: "➕",
        cls: "teal",
        title: "Addition Adventure",
        desc: "Add the numbers together. Use the pictures to help you count!",
        qs: "10 questions",
    },
    GameMeta {
        id: "subtract",
        subject: "maths",
        icon: "➖",
        cls: "sky",
        title: "Take-Away Safari",
        desc: "Take numbers away and find what is left. Watch out, they get bigger!",
        qs: "10 questions",
    },
    GameMeta {
        id: "multiply",
        subject: "maths",
        icon: "✖️",
        cls: "berry",
        title: "Times Table Challenge",
        desc: "Practise your times tables — 2s, 5s, 10s and lots more as you level up.",
        qs: "10 questions",
    },
    GameMeta {
        id: "missing",
        subject: "maths",
        icon: "🧩",
        cls: "grape",
        title: "Missing Number",
        desc: "A number is hiding! Solve number puzzles and finish the sequences.",
        qs: "10 questions",
    },
    // English games
    GameMeta {
        id: "spell",
        subject: "english",
        icon: "🔤",
        cls: "sun",
        title: "Spell It!",
        desc: "Look at the picture, tap the letters in order and spell the word.",
        qs: "10 words",
    },
    GameMeta {
        id: "missingLetter",
        subject: "english",
        icon: "✏️",
        cls: "leaf",
        title: "Missing Letter",
        desc: "One letter has run away from each word. Can you find it?",
        qs: "10 words",
    },
    GameMeta {
        id: "vocab",
        subject: "english",
        icon: "🖼️",
        cls: "coral",
        title: "Picture Words",
        desc: "What is in the picture? Tap the matching word to build your vocabulary.",
        qs: "10 words",
    },
    GameMeta {
        id: "rhyme",
        subject: "english",
        icon: "🎵",
        cls: "teal",
        title: "Rhyme Time",
        desc: "Words that sound the same at the end rhyme — like CAT and HAT. Find the rhyme!",
        qs: "10 rhymes",
    },
];
