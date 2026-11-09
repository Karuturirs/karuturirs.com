pub fn render_home_html() -> String {
    r##"
<div class="portfolio-page">
  <!-- TOP NAVIGATION BAR -->
  <header class="navbar">
    <div class="nav-container">
      <a href="/" class="brand-link" onclick="event.preventDefault(); window.navigateTo('/');">
        <span class="brand-logo">RK</span>
        <span>Ravi Sankar Karuturi</span>
      </a>
      <nav class="nav-links">
        <a href="#projects" class="nav-item">Projects</a>
        <a href="#certifications" class="nav-item">Certifications</a>
        <a href="#articles" class="nav-item">Articles</a>
        <a href="#contact" class="nav-item">Contact</a>
        <a href="/kids-school-learning" class="nav-btn-highlight" onclick="event.preventDefault(); window.navigateTo('/kids-school-learning');">
          <span>🎮 Kids Learning</span>
        </a>
      </nav>
    </div>
  </header>

  <!-- HERO SECTION -->
  <section class="hero-section">
    <div class="hero-content">
      <div class="hero-badges">
        <span class="hero-badge">☁️ Distributed Systems</span>
        <span class="hero-badge">☕ Java / Spring</span>
        <span class="hero-badge">🦀 Systems Engineering</span>
        <span class="hero-badge">🎓 EduTech</span>
      </div>
      <h1>
        Building Resilient Systems &amp; <span class="highlight-text">Interactive Experiences</span>
      </h1>
      <p class="hero-subtitle">
        Hi, I'm <strong>Ravi Sankar Karuturi</strong>. Lead Software Engineer specializing in scalable enterprise architectures, event-driven distributed systems, and joyful educational software for future thinkers.
      </p>
      <div class="hero-cta">
        <a href="/kids-school-learning" class="btn-primary" onclick="event.preventDefault(); window.navigateTo('/kids-school-learning');">
          🎮 Launch Kids School Learning
        </a>
        <a href="#projects" class="btn-secondary">
          🚀 View Projects
        </a>
        <a href="#contact" class="btn-secondary">
          📫 Get in Touch
        </a>
      </div>
    </div>

    <!-- HERO HIGHLIGHT CARD -->
    <div class="hero-card">
      <span class="card-tag">Lead Software Engineer</span>
      <div class="hero-stat-box">
        <h3>Distributed &amp; Cloud Systems</h3>
        <p>Architecting scalable enterprise platforms, event-driven pipelines, NoSQL persistence, and modern web applications.</p>
      </div>
      <div class="hero-skills-row">
        <span class="skill-pill">Java</span>
        <span class="skill-pill">Spring Boot</span>
        <span class="skill-pill">Apache Cassandra</span>
        <span class="skill-pill">Docker &amp; K8s</span>
        <span class="skill-pill">Machine Learning</span>
        <span class="skill-pill">Microservices</span>
        <span class="skill-pill">Modern Web</span>
      </div>
    </div>
  </section>

  <!-- SPOTLIGHT SECTION: KIDS SCHOOL LEARNING -->
  <section class="spotlight-section">
    <div class="spotlight-card">
      <div class="spotlight-content">
        <span class="spotlight-badge">🌟 Featured Application</span>
        <h2 class="spotlight-title">Kids School Learning</h2>
        <p class="spotlight-desc">
          Interactive Maths and English games designed for Pre-K through Grade 4 explorers. Features step-by-step mental math tricks, ten-frame counting visuals, spelling tiles, real-time sound synthesis, and speech question reader!
        </p>
        <div class="spotlight-features">
          <div class="feature-pill"><span>👶</span> Pre-K to Grade 4 Modes</div>
          <div class="feature-pill"><span>🔟</span> Ten-Frames &amp; Mental Tricks</div>
          <div class="feature-pill"><span>🗣️</span> Speech Synthesis Reader</div>
          <div class="feature-pill"><span>🎵</span> WebAudio Tones &amp; Chimes</div>
          <div class="feature-pill"><span>⏱️</span> Chill, Clock &amp; Streak Styles</div>
          <div class="feature-pill"><span>🔒</span> 100% Free &amp; Ad-Free</div>
        </div>
        <a href="/kids-school-learning" class="btn-primary" onclick="event.preventDefault(); window.navigateTo('/kids-school-learning');">
          Play Educational Games &rarr;
        </a>
      </div>
      <div class="spotlight-preview">
        <div class="preview-mascot">🦉</div>
        <h3 style="font-family: var(--font-display); font-size: 1.5rem; margin-top: 0.5rem; color: #fff;">14 Fun Learning Games</h3>
        <p style="color: var(--text-dark-secondary); font-size: 0.95rem; margin: 0.5rem 0 1.25rem;">
          Colors, Animals, Big/Small, Phonics, Counting, Addition, Take-Away, Times Tables, Spelling, Rhymes &amp; The Big Adventure!
        </p>
        <a href="/kids-school-learning" class="btn-secondary" onclick="event.preventDefault(); window.navigateTo('/kids-school-learning');" style="width: 100%; justify-content: center;">
          Launch Live App
        </a>
      </div>
    </div>
  </section>

  <!-- PROJECTS SECTION -->
  <section class="section-wrapper" id="projects">
    <div class="section-header">
      <h2 class="section-title">🚀 Featured Projects</h2>
      <p class="section-subtitle">Open-source utilities, distributed services, and interactive web software</p>
    </div>
    <div class="projects-grid">
      <!-- Project 1: Kids Learning -->
      <div class="project-card">
        <div>
          <div class="project-top-row">
            <span class="project-icon">🎮</span>
            <span class="badge-live">Live</span>
          </div>
          <h3 class="project-title">Kids School Learning App</h3>
          <p class="project-desc">
            Full-featured educational suite featuring 14 games across Maths, English, and Pre-K fundamentals with real-time sound synthesis, question speech reading, and adaptive difficulty.
          </p>
          <div class="project-tags">
            <span class="project-tag">EduTech</span>
            <span class="project-tag">WebAudio</span>
            <span class="project-tag">Speech API</span>
            <span class="project-tag">Netlify</span>
          </div>
        </div>
        <div class="project-footer">
          <a href="/kids-school-learning" class="project-link" onclick="event.preventDefault(); window.navigateTo('/kids-school-learning');">
            Open App &rarr;
          </a>
        </div>
      </div>

      <!-- Project 2: TimePropose.com -->
      <div class="project-card">
        <div>
          <div class="project-top-row">
            <span class="project-icon">⏰</span>
            <span class="badge-oss">Open Source</span>
          </div>
          <h3 class="project-title">TimePropose.com</h3>
          <p class="project-desc">
            Clean web tool engineered to share meeting availability across global timezones with zero scheduling friction. Designed using purely functional frontend architecture.
          </p>
          <div class="project-tags">
            <span class="project-tag">Elm</span>
            <span class="project-tag">Functional UI</span>
            <span class="project-tag">Timezone Math</span>
          </div>
        </div>
        <div class="project-footer">
          <a href="https://github.com/Karuturirs/timepropose.git" target="_blank" rel="noreferrer" class="project-link">
            GitHub Repository &rarr;
          </a>
        </div>
      </div>

      <!-- Project 3: FoodTrucksInSF -->
      <div class="project-card">
        <div>
          <div class="project-top-row">
            <span class="project-icon">🚚</span>
            <span class="badge-oss">Open Source</span>
          </div>
          <h3 class="project-title">FoodTrucksInSF</h3>
          <p class="project-desc">
            Fast command-line tool to discover open food trucks across San Francisco streets using municipal open datasets and spatial coordinates queries.
          </p>
          <div class="project-tags">
            <span class="project-tag">Java</span>
            <span class="project-tag">CLI</span>
            <span class="project-tag">Open Data API</span>
          </div>
        </div>
        <div class="project-footer">
          <a href="https://github.com/Karuturirs/FoodTruckInSF" target="_blank" rel="noreferrer" class="project-link">
            GitHub Repository &rarr;
          </a>
        </div>
      </div>

      <!-- Project 4: mocktail.bird -->
      <div class="project-card">
        <div>
          <div class="project-top-row">
            <span class="project-icon">🍸</span>
            <span class="badge-oss">Open Source</span>
          </div>
          <h3 class="project-title">mocktail.bird</h3>
          <p class="project-desc">
            Mocking and API contract simulation application designed for day-to-day microservice development, sandbox environments, and consumer integration testing.
          </p>
          <div class="project-tags">
            <span class="project-tag">Java</span>
            <span class="project-tag">MongoDB</span>
            <span class="project-tag">Docker</span>
          </div>
        </div>
        <div class="project-footer">
          <a href="https://github.com/mocktaillbird/mocktail.bird" target="_blank" rel="noreferrer" class="project-link">
            GitHub Repository &rarr;
          </a>
        </div>
      </div>
    </div>
  </section>

  <!-- CERTIFICATIONS & CREDENTIALS -->
  <section class="section-wrapper" id="certifications">
    <div class="section-header">
      <h2 class="section-title">🏆 Certifications &amp; Credentials</h2>
      <p class="section-subtitle">Continuous learning and verified professional milestones</p>
    </div>
    <div class="achieve-list">
      <div class="achieve-card">
        <div class="achieve-icon">🎓</div>
        <div>
          <h4>Machine Learning Certification</h4>
          <p style="color: var(--primary-light); font-weight: 600; margin-bottom: 0.25rem;">Issued by eCornell &bull; June 2022</p>
          <p>Applied statistical models, classification, deep neural networks, and predictive modeling algorithms.</p>
        </div>
      </div>
      <div class="achieve-card">
        <div class="achieve-icon">📈</div>
        <div>
          <h4>Digital Marketing Certification</h4>
          <p style="color: var(--primary-light); font-weight: 600; margin-bottom: 0.25rem;">Issued by Thinkful &bull; Dec 2022</p>
          <p>Customer acquisition analytics, brand strategy, user funnel optimization, and conversion engineering.</p>
        </div>
      </div>
      <div class="achieve-card">
        <div class="achieve-icon">🗄️</div>
        <div>
          <h4>Apache Cassandra 3.x Developer Associate</h4>
          <p style="color: var(--primary-light); font-weight: 600; margin-bottom: 0.25rem;">Issued by DataStax &bull; July 2020</p>
          <p>Distributed data modeling, replication strategies, clustering, high availability, and NoSQL architecture.</p>
        </div>
      </div>
    </div>
  </section>

  <!-- ARTICLES & IDEAS -->
  <section class="section-wrapper" id="articles">
    <div class="section-header">
      <h2 class="section-title">📝 Articles &amp; Ideas</h2>
      <p class="section-subtitle">Reflections on engineering, intentional focus, and innovation</p>
    </div>
    <div class="articles-grid">
      <div class="article-card">
        <span class="article-date">June 09, 2020 &bull; 1 Min Read &bull; Focus &amp; Productivity</span>
        <h3>Concentration &amp; Hobbies related?</h3>
        <p>
          Hobbies are important aspects that make us better. In the age of constant mobile notifications, cultivating deep focus through singular, intentional activities strengthens our cognitive discipline.
        </p>
      </div>
      <div class="article-card">
        <span class="article-date">June 08, 2020 &bull; 1 Min Read &bull; Personal Development</span>
        <h3>Like to talk on anything..!</h3>
        <p>
          Have you ever thought, "I have something to share and discuss"? Technology, new ideas, distributed systems, or small innovations. Great things always start with a small conversation.
        </p>
      </div>
    </div>
  </section>

  <!-- CONTACT HUB -->
  <section class="section-wrapper" id="contact">
    <div class="contact-box">
      <h2>📬 Let's Connect &amp; Collaborate</h2>
      <p>Have questions about software architecture, distributed systems, or educational tools?</p>
      <div class="contact-links">
        <a href="mailto:hello@karuturirs.com" class="cbtn cbtn-email">📧 hello@karuturirs.com</a>
        <a href="https://www.linkedin.com/in/ravisankarkaruturi/" target="_blank" rel="noreferrer" class="cbtn cbtn-linkedin">💼 LinkedIn</a>
        <a href="https://github.com/Karuturirs" target="_blank" rel="noreferrer" class="cbtn cbtn-github">🐙 GitHub</a>
        <a href="https://twitter.com/karuturirs" target="_blank" rel="noreferrer" class="cbtn cbtn-twitter">🐦 Twitter / X</a>
      </div>
    </div>
  </section>

  <!-- FOOTER -->
  <footer class="footer">
    <div class="footer-content">
      <div class="footer-brand">
        <span class="brand-logo" style="margin: 0 auto 0.5rem;">RK</span>
        <h4 style="color: #fff; font-size: 1.15rem; font-family: var(--font-display);">Ravi Sankar Karuturi</h4>
        <p style="font-size: 0.92rem; color: var(--text-dark-muted);">Lead Software Engineer &bull; karuturirs.com</p>
      </div>
      <div class="footer-links">
        <a href="#projects">Projects</a>
        <a href="#certifications">Certifications</a>
        <a href="#articles">Articles</a>
        <a href="/kids-school-learning" onclick="event.preventDefault(); window.navigateTo('/kids-school-learning');">Kids Learning Games</a>
        <a href="mailto:hello@karuturirs.com">Contact</a>
      </div>
      <p style="font-size: 0.85rem; color: var(--text-dark-muted);">&copy; karuturirs.com &bull; All rights reserved</p>
    </div>
  </footer>
</div>
"##
    .to_string()
}
