<h1 align="center">
  karuturirs.com
</h1>

<p align="center">
  <b>Personal Engineering Portfolio & Kids School Learning Games</b><br>
  Built with <b>Rust WebAssembly (WASM)</b> and deployed on <b>Netlify</b> via <b>GitHub</b>.
</p>

<p align="center">
  <a href="https://app.netlify.com/sites/youthful-ride-5d6fe5/deploys">
    <img src="https://api.netlify.com/api/v1/badges/62952849-ed6f-45d6-98a9-1ab7de06d242/deploy-status" alt="Netlify Status">
  </a>
</p>

---

## 🚀 Quick Start (Run & Test Locally)

The compiled WebAssembly binary and static web assets are ready inside the `public/` folder. You can launch and test immediately using any of the following methods:

### Option 1: Using npm (Recommended)

```bash
# Start the local server
npm start
```
> Or run: `npm run serve`

Then open your browser at:
- **Homepage (Portfolio)**: [http://localhost:3000](http://localhost:3000)
- **Kids School Learning**: [http://localhost:3000/kids-school-learning](http://localhost:3000/kids-school-learning)

---

### Option 2: Using Python

If you prefer Python's built-in web server:

```bash
# From the project root directory:
python3 -m http.server 8080 --directory public
```

Then visit:
- **Homepage**: [http://localhost:8080](http://localhost:8080)
- **Kids School Learning**: [http://localhost:8080/kids-school-learning](http://localhost:8080/kids-school-learning)

---

### Option 3: Using `npx serve`

```bash
npx serve public -l 5000
```

Then open [http://localhost:5000](http://localhost:5000).

---

## 🦀 Rebuilding the Rust WASM Package

If you make modifications to any Rust code in `rust_wasm/src/`:

### Prerequisites

Ensure you have Rust and `wasm-pack` installed:
```bash
# Install wasm-pack if not already installed
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
# Or via Homebrew on macOS:
brew install wasm-pack
```

### Build Command

```bash
# Rebuild the WASM bundle into public/pkg/
npm run build:wasm
```

Or run directly with `wasm-pack`:
```bash
cd rust_wasm
wasm-pack build --target web --release --out-dir ../public/pkg
cd ..
```

---

## 🎮 Kids School Learning Features (`/kids-school-learning`)

- **10 Interactive Games**:
  - 🔢 **Counting Garden**: Count cute items with visual grids
  - ➕ **Addition Magic**: Add numbers with sum tricks
  - ➖ **Take-Away Time**: Subtraction with step-by-step jump back
  - ✖️ **Times Tables Hero**: Fast multiplication strategies & skip counting
  - ❓ **Missing Number**: Number line detective patterns
  - 🔤 **Spell It Out**: Interactive letter tiles with slot filling & undo
  - 🧩 **Missing Letter**: Word completion phonics
  - 🖼️ **Picture Words**: Vocabulary builder with emojis
  - 🎶 **Rhyme Time**: Rhyming word families
  - 🗺️ **The Big Adventure**: 10 mixed quest questions across all worlds
- **Interactive Audio**: Real-time WebAudio synthesized tones and text-to-speech read-aloud via the Web Speech API.
- **Secret Tricks & Techniques**: Step-by-step mental math explanations (*Make a Ten*, *Doubles*, *Count On*, *Count Back*, *Jump Back*, etc.).
- **Age Tiers**: Little Explorers (Ages 4-5), Growing Minds (Ages 6-7), Super Stars (Ages 8-9).
- **Play Styles**: Practice (untimed), 10 Questions, and Timed Clock modes.
- **Trophies & Stats**: Local storage tracking for stars, streaks, and high scores.

---

## ☁️ Deployment (GitHub & Netlify)

1. Commit and push changes to GitHub:
   ```bash
   git push origin game-kids
   ```
2. Netlify will detect the configuration in `netlify.toml` and execute `build.sh`.
3. Pre-compiled WASM artifacts in `public/pkg/` allow instant, reliable deployments without requiring toolchain installations in CI.
