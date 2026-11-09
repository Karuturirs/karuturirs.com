pub fn render_home_html() -> String {
    r##"
<div class="home-container">
  <!-- HERO SECTION -->
  <section class="hero-home">
    <div class="hero-left">
      <div class="hero-badge">
        <span class="pulse-dot"></span> Powered by Rust &amp; WebAssembly
      </div>
      <h1 class="hero-title">
        Building Resilient Systems &amp; <span class="highlight">Interactive Experiences</span>
      </h1>
      <p class="hero-desc">
        Hi, I'm <strong>Ravi Sankar Karuturi</strong>. Lead Software Engineer exploring high-performance distributed systems, modern WebAssembly, and joyful educational tools for future thinkers.
      </p>
      <div class="hero-actions">
        <a href="/kids-school-learning" class="btn btn-coral" onclick="event.preventDefault(); window.navigateTo('/kids-school-learning');">
          🎮 Launch Kids School Learning
        </a>
        <a href="#projects" class="btn btn-ghost">
          🚀 View Engineering Projects
        </a>
      </div>
      <div class="hero-chips">
        <span class="chip">🦀 Rust WASM</span>
        <span class="chip">☁️ Distributed Systems</span>
        <span class="chip">🎓 EduTech</span>
        <span class="chip">⚡ Netlify CI/CD</span>
      </div>
    </div>

    <!-- FEATURED CALLOUT CARD -->
    <div class="hero-right">
      <div class="featured-card">
        <div class="card-glow"></div>
        <div class="featured-badge">🌟 Spotlight App</div>
        <div class="featured-header">
          <span class="featured-icon">🦉</span>
          <div>
            <h3>Kids School Learning</h3>
            <p>karuturirs.com/kids-school-learning</p>
          </div>
        </div>
        <p class="featured-text">
          Interactive Maths &amp; English games engineered in Rust WASM with real-time WebAudio synthesis, adaptive difficulty, step-by-step mental math tricks, and speech synthesis!
        </p>
        <div class="game-tag-row">
          <span class="gtag">➕ Addition</span>
          <span class="gtag">➖ Take-Away</span>
          <span class="gtag">✖️ Times Tables</span>
          <span class="gtag">🔤 Spelling</span>
          <span class="gtag">🎵 Rhymes</span>
          <span class="gtag">🧩 Patterns</span>
        </div>
        <a href="/kids-school-learning" class="featured-cta" onclick="event.preventDefault(); window.navigateTo('/kids-school-learning');">
          Play Educational Games &rarr;
        </a>
      </div>
    </div>
  </section>

  <!-- PROJECTS SECTION -->
  <section class="section" id="projects">
    <div class="sec-title-row">
      <h2>🚀 Featured Projects</h2>
      <span>Open source &amp; engineering builds</span>
    </div>

    <div class="projects-grid">
      <div class="project-card">
        <div class="project-top">
          <span class="picon">🎮</span>
          <div>
            <h3>Kids School Learning App</h3>
            <span class="plink">Live on karuturirs.com</span>
          </div>
        </div>
        <p>
          Educational web app featuring 10 maths and english games for kids ages 4–9. Includes WebAudio synthesis, speech synthesis, adaptive learning dynamics, and step-by-step mental math tricks.
        </p>
        <div class="tech-tags">
          <span class="tag tag-rust">Rust</span>
          <span class="tag tag-wasm">WebAssembly</span>
          <span class="tag tag-audio">WebAudio</span>
          <span class="tag tag-net">Netlify</span>
        </div>
        <div class="project-footer">
          <a href="/kids-school-learning" class="card-btn" onclick="event.preventDefault(); window.navigateTo('/kids-school-learning');">Open App &rarr;</a>
        </div>
      </div>

      <div class="project-card">
        <div class="project-top">
          <span class="picon">⏰</span>
          <div>
            <h3>TimePropose.com</h3>
            <span class="plink">github.com/Karuturirs/timepropose</span>
          </div>
        </div>
        <p>
          Web utility engineered to easily share availability across global timezones without scheduling friction. Designed with purely functional frontend architecture.
        </p>
        <div class="tech-tags">
          <span class="tag">ELM</span>
          <span class="tag">Timezones</span>
          <span class="tag">Functional UI</span>
        </div>
        <div class="project-footer">
          <a href="https://github.com/Karuturirs/timepropose.git" target="_blank" rel="noreferrer" class="card-btn">GitHub Repo &rarr;</a>
        </div>
      </div>

      <div class="project-card">
        <div class="project-top">
          <span class="picon">🚚</span>
          <div>
            <h3>FoodTrucksInSF</h3>
            <span class="plink">github.com/Karuturirs/FoodTruckInSF</span>
          </div>
        </div>
        <p>
          High-performance command-line application to locate open food trucks on San Francisco streets using municipal open datasets and geo-spatial queries.
        </p>
        <div class="tech-tags">
          <span class="tag">Java</span>
          <span class="tag">CLI</span>
          <span class="tag">Open Data</span>
        </div>
        <div class="project-footer">
          <a href="https://github.com/Karuturirs/FoodTruckInSF" target="_blank" rel="noreferrer" class="card-btn">GitHub Repo &rarr;</a>
        </div>
      </div>

      <div class="project-card">
        <div class="project-top">
          <span class="picon">🍸</span>
          <div>
            <h3>mocktail.bird</h3>
            <span class="plink">github.com/mocktaillbird/mocktail.bird</span>
          </div>
        </div>
        <p>
          Flexible API mocking and simulation service designed for day-to-day development, contract validation, and integration testing across microservice clusters.
        </p>
        <div class="tech-tags">
          <span class="tag">Java</span>
          <span class="tag">MongoDB</span>
          <span class="tag">Docker</span>
        </div>
        <div class="project-footer">
          <a href="https://github.com/mocktaillbird/mocktail.bird" target="_blank" rel="noreferrer" class="card-btn">GitHub Repo &rarr;</a>
        </div>
      </div>
    </div>
  </section>

  <!-- ACHIEVEMENTS & CERTIFICATIONS -->
  <section class="section" id="achievements">
    <div class="sec-title-row">
      <h2>🏆 Certifications &amp; Milestones</h2>
      <span>Professional and continuous learning</span>
    </div>

    <div class="achieve-grid">
      <div class="achieve-card">
        <div class="achieve-badge">🎓 eCornell</div>
        <h3>Machine Learning Certification</h3>
        <p class="achieve-sub">Issued: June 2022</p>
        <p class="achieve-more">Applied statistical models, classification, deep networks, and predictive modeling.</p>
      </div>

      <div class="achieve-card">
        <div class="achieve-badge">📈 Thinkful</div>
        <h3>Digital Marketing Certification</h3>
        <p class="achieve-sub">Issued: December 2022</p>
        <p class="achieve-more">Customer acquisition analytics, brand strategy, user funnel optimization, and conversion engineering.</p>
      </div>

      <div class="achieve-card">
        <div class="achieve-badge">🗄️ DataStax</div>
        <h3>Apache Cassandra 3.x Associate</h3>
        <p class="achieve-sub">Issued: July 2020 · Credential: 7212832b-6284-4c52-8d3f-b807d829fd14</p>
        <p class="achieve-more">Distributed data modeling, replication strategies, fault tolerance, and NoSQL architecture.</p>
      </div>
    </div>
  </section>

  <!-- ARTICLES / THOUGHTS -->
  <section class="section" id="articles">
    <div class="sec-title-row">
      <h2>📝 Articles &amp; Ideas</h2>
      <span>Reflections on tech, focus, and innovation</span>
    </div>

    <div class="articles-list">
      <div class="article-item">
        <h3>Concentration &amp; Hobbies related?</h3>
        <span class="article-meta">June 09, 2020 · 1 Min Read · #General</span>
        <p>
          Hobbies are important aspects that make us better. In the age of constant mobile notifications, cultivating deep focus through singular, intentional activities strengthens our cognitive discipline.
        </p>
      </div>

      <div class="article-item">
        <h3>Like to talk on anything..!</h3>
        <span class="article-meta">June 08, 2020 · 1 Min Read · Personal Development</span>
        <p>
          Have you ever thought, "I have something to share and discuss"? Tech, new ideas, distributed systems, or small innovations. Great things always start small.
        </p>
      </div>
    </div>
  </section>

  <!-- CONTACT HUB -->
  <section class="section" id="contact">
    <div class="contact-box">
      <h2>📬 Let's Connect &amp; Collaborate</h2>
      <p>Have questions about Rust WASM, distributed systems, or interested in collaborating on educational tools?</p>
      <div class="contact-links">
        <a href="mailto:hello@karuturirs.com" class="cbtn cbtn-email">📧 hello@karuturirs.com</a>
        <a href="https://www.linkedin.com/in/ravisankarkaruturi/" target="_blank" rel="noreferrer" class="cbtn cbtn-linkedin">💼 LinkedIn</a>
        <a href="https://github.com/Karuturirs" target="_blank" rel="noreferrer" class="cbtn cbtn-github">🐙 GitHub</a>
        <a href="https://twitter.com/karuturirs" target="_blank" rel="noreferrer" class="cbtn cbtn-twitter">🐦 Twitter / X</a>
      </div>
    </div>
  </section>
</div>
"##
    .to_string()
}
