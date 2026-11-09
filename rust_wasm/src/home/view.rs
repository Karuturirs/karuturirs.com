pub fn render_home_html() -> String {
    r##"
<div class="portfolio-page">
  <!-- TOP NAVIGATION BAR -->
  <header class="navbar">
    <div class="nav-container">
      <a href="/" class="brand-link" onclick="event.preventDefault(); window.navigateTo('/');">
        <span class="brand-logo">RK</span>
        <span class="brand-name">Ravi Sankar Karuturi</span>
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
      <div class="hero-status-pill">
        <span class="status-dot"></span>
        <span>Lead Software Engineer &bull; Cloud &amp; Distributed Systems</span>
      </div>
      <h1 class="hero-title">
        Building Resilient Systems &amp; <span class="highlight-text">Joyful Software</span>
      </h1>
      <p class="hero-subtitle">
        Hi, I'm <strong>Ravi Sankar Karuturi</strong>. Specializing in high-throughput enterprise architectures, event-driven distributed platforms, and delightfully fast interactive web applications.
      </p>
      <div class="hero-cta">
        <a href="/kids-school-learning" class="btn-primary" onclick="event.preventDefault(); window.navigateTo('/kids-school-learning');">
          <span>🎮 Launch Kids School Learning</span>
          <span class="btn-arrow">&rarr;</span>
        </a>
        <a href="#projects" class="btn-secondary">
          <span>🚀 View Projects</span>
        </a>
        <a href="#contact" class="btn-secondary">
          <span>📫 Get in Touch</span>
        </a>
      </div>
      <div class="hero-badges">
        <span class="hero-badge">☁️ Distributed Systems</span>
        <span class="hero-badge">☕ Java / Spring Boot</span>
        <span class="hero-badge">🗄️ Apache Cassandra</span>
        <span class="hero-badge">🦀 Rust &amp; Wasm</span>
        <span class="hero-badge">🎓 EduTech</span>
      </div>
    </div>

    <!-- HERO HIGHLIGHT CARD -->
    <div class="hero-card">
      <div class="hero-card-header">
        <div class="window-dots">
          <span class="wdot wdot-red"></span>
          <span class="wdot wdot-yellow"></span>
          <span class="wdot wdot-green"></span>
        </div>
        <span class="hero-card-badge">System Profile</span>
      </div>
      <div class="hero-stat-box">
        <div class="hero-profile-row">
          <div class="hero-profile-avatar">RK</div>
          <div>
            <h3 class="hero-profile-name">Ravi Sankar Karuturi</h3>
            <p class="hero-profile-title">Lead Software Engineer &amp; Architect</p>
          </div>
        </div>
        <div class="hero-metrics-grid">
          <div class="metric-item">
            <span class="metric-val">10+</span>
            <span class="metric-label">Years Exp</span>
          </div>
          <div class="metric-item">
            <span class="metric-val">Enterprise</span>
            <span class="metric-label">Scale &amp; Reliability</span>
          </div>
          <div class="metric-item">
            <span class="metric-val">14+</span>
            <span class="metric-label">Edu Games Built</span>
          </div>
        </div>
      </div>
      <div class="hero-skills-wrapper">
        <span class="skills-label">Core Capabilities &amp; Stack</span>
        <div class="hero-skills-row">
          <span class="skill-pill">Java / Spring Boot</span>
          <span class="skill-pill">Distributed Architecture</span>
          <span class="skill-pill">Apache Cassandra</span>
          <span class="skill-pill">Docker &amp; K8s</span>
          <span class="skill-pill">Machine Learning</span>
          <span class="skill-pill">Kafka &amp; Event-Driven</span>
          <span class="skill-pill">WebAssembly</span>
        </div>
      </div>
    </div>
  </section>

  <!-- SPOTLIGHT SECTION: KIDS SCHOOL LEARNING -->
  <section class="spotlight-section">
    <div class="spotlight-card">
      <div class="spotlight-content">
        <div class="spotlight-badge">
          <span class="badge-star">🌟</span>
          <span>Interactive Showcase Application</span>
        </div>
        <h2 class="spotlight-title">Kids School Learning</h2>
        <p class="spotlight-desc">
          Full-featured educational suite crafted for Pre-K through Grade 4 explorers. Packed with step-by-step mental math tricks, ten-frame counting visuals, spelling tiles, WebAudio real-time sounds, and built-in speech reader!
        </p>
        <div class="spotlight-features">
          <div class="feature-pill"><span class="feat-ico">👶</span> Pre-K to Grade 4 Modes</div>
          <div class="feature-pill"><span class="feat-ico">🔟</span> Ten-Frames &amp; Mental Math</div>
          <div class="feature-pill"><span class="feat-ico">🗣️</span> Voice Question Reader</div>
          <div class="feature-pill"><span class="feat-ico">🎵</span> WebAudio Chimes &amp; Tones</div>
          <div class="feature-pill"><span class="feat-ico">📝</span> Live Review Queue &amp; Tricks</div>
          <div class="feature-pill"><span class="feat-ico">🔒</span> 100% Free &amp; Ad-Free</div>
        </div>
        <div class="spotlight-actions">
          <a href="/kids-school-learning" class="btn-primary" onclick="event.preventDefault(); window.navigateTo('/kids-school-learning');">
            <span>Play Educational Games</span>
            <span class="btn-arrow">&rarr;</span>
          </a>
        </div>
      </div>
      <div class="spotlight-preview">
        <div class="preview-mascot">🦉</div>
        <h3 class="preview-title">14 Fun Learning Games</h3>
        <p class="preview-subtitle">
          Colors, Animals, Shapes, Phonics, Counting, Addition, Take-Away, Times Tables, Spelling, Rhymes &amp; The Big Adventure!
        </p>
        <div class="preview-badges">
          <span class="pbadge">Pre-K</span>
          <span class="pbadge">Kindergarten</span>
          <span class="pbadge">Grade 1-4</span>
        </div>
        <a href="/kids-school-learning" class="btn-preview-play" onclick="event.preventDefault(); window.navigateTo('/kids-school-learning');">
          Launch Live App &rarr;
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
            <div class="project-icon-box">🎮</div>
            <span class="badge-live">Live App</span>
          </div>
          <h3 class="project-title">Kids School Learning App</h3>
          <p class="project-desc">
            Full-featured educational suite featuring 14 games across Maths, English, and Pre-K fundamentals with real-time sound synthesis, question speech reading, ten-frame visuals, and step-by-step mental tricks.
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
            <span>Open Application</span>
            <span class="link-arrow">&rarr;</span>
          </a>
        </div>
      </div>

      <!-- Project 2: TimePropose.com -->
      <div class="project-card">
        <div>
          <div class="project-top-row">
            <div class="project-icon-box">⏰</div>
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
            <span>GitHub Repository</span>
            <span class="link-arrow">&rarr;</span>
          </a>
        </div>
      </div>

      <!-- Project 3: FoodTrucksInSF -->
      <div class="project-card">
        <div>
          <div class="project-top-row">
            <div class="project-icon-box">🚚</div>
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
            <span>GitHub Repository</span>
            <span class="link-arrow">&rarr;</span>
          </a>
        </div>
      </div>

      <!-- Project 4: mocktail.bird -->
      <div class="project-card">
        <div>
          <div class="project-top-row">
            <div class="project-icon-box">🍸</div>
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
            <span>GitHub Repository</span>
            <span class="link-arrow">&rarr;</span>
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
        <div class="achieve-icon-box">🎓</div>
        <div class="achieve-body">
          <div class="achieve-meta">
            <span class="achieve-issuer">eCornell</span>
            <span class="achieve-date">June 2022</span>
          </div>
          <h4 class="achieve-title">Machine Learning Certification</h4>
          <p class="achieve-desc">Applied statistical models, classification, deep neural networks, and predictive modeling algorithms.</p>
        </div>
      </div>
      <div class="achieve-card">
        <div class="achieve-icon-box">📈</div>
        <div class="achieve-body">
          <div class="achieve-meta">
            <span class="achieve-issuer">Thinkful</span>
            <span class="achieve-date">Dec 2022</span>
          </div>
          <h4 class="achieve-title">Digital Marketing Certification</h4>
          <p class="achieve-desc">Customer acquisition analytics, brand strategy, user funnel optimization, and conversion engineering.</p>
        </div>
      </div>
      <div class="achieve-card">
        <div class="achieve-icon-box">🗄️</div>
        <div class="achieve-body">
          <div class="achieve-meta">
            <span class="achieve-issuer">DataStax</span>
            <span class="achieve-date">July 2020</span>
          </div>
          <h4 class="achieve-title">Apache Cassandra 3.x Developer Associate</h4>
          <p class="achieve-desc">Distributed data modeling, replication strategies, clustering, high availability, and NoSQL architecture.</p>
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
        <div class="article-meta">
          <span class="article-tag">Focus &amp; Productivity</span>
          <span class="article-date">June 09, 2020 &bull; 1 Min Read</span>
        </div>
        <h3 class="article-title">Concentration &amp; Hobbies related?</h3>
        <p class="article-desc">
          Hobbies are important aspects that make us better. In the age of constant mobile notifications, cultivating deep focus through singular, intentional activities strengthens our cognitive discipline.
        </p>
      </div>
      <div class="article-card">
        <div class="article-meta">
          <span class="article-tag">Personal Development</span>
          <span class="article-date">June 08, 2020 &bull; 1 Min Read</span>
        </div>
        <h3 class="article-title">Like to talk on anything..!</h3>
        <p class="article-desc">
          Have you ever thought, "I have something to share and discuss"? Technology, new ideas, distributed systems, or small innovations. Great things always start with a small conversation.
        </p>
      </div>
    </div>
  </section>

  <!-- CONTACT HUB -->
  <section class="section-wrapper" id="contact">
    <div class="contact-box">
      <h2 class="contact-title">📬 Let's Connect &amp; Collaborate</h2>
      <p class="contact-desc">
        Have questions about software architecture, distributed systems, or educational tools? Feel free to reach out.
      </p>
      <div class="contact-links">
        <a href="mailto:hello@karuturirs.com" class="cbtn cbtn-email">
          <span class="cbtn-ico">📧</span>
          <span>hello@karuturirs.com</span>
        </a>
        <a href="https://www.linkedin.com/in/ravisankarkaruturi/" target="_blank" rel="noreferrer" class="cbtn cbtn-linkedin">
          <span class="cbtn-ico">💼</span>
          <span>LinkedIn</span>
        </a>
        <a href="https://github.com/Karuturirs" target="_blank" rel="noreferrer" class="cbtn cbtn-github">
          <span class="cbtn-ico">🐙</span>
          <span>GitHub</span>
        </a>
        <a href="https://twitter.com/karuturirs" target="_blank" rel="noreferrer" class="cbtn cbtn-twitter">
          <span class="cbtn-ico">🐦</span>
          <span>Twitter / X</span>
        </a>
      </div>
    </div>
  </section>

  <!-- FOOTER -->
  <footer class="footer">
    <div class="footer-content">
      <div class="footer-brand">
        <span class="brand-logo footer-logo">RK</span>
        <h4 class="footer-name">Ravi Sankar Karuturi</h4>
        <p class="footer-role">Lead Software Engineer &bull; karuturirs.com</p>
      </div>
      <div class="footer-links">
        <a href="#projects">Projects</a>
        <a href="#certifications">Certifications</a>
        <a href="#articles">Articles</a>
        <a href="/kids-school-learning" onclick="event.preventDefault(); window.navigateTo('/kids-school-learning');">Kids Learning Games</a>
        <a href="mailto:hello@karuturirs.com">Contact</a>
      </div>
      <p class="footer-copy">&copy; karuturirs.com &bull; All rights reserved</p>
    </div>
  </footer>
</div>
"##
    .to_string()
}
