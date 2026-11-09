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
        <a href="#initiatives" class="nav-item">Initiatives</a>
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
        <span>Lead Software Engineer &bull; E-Commerce AI Search, AdTech &amp; Distributed Systems</span>
      </div>
      <h1 class="hero-title">
        Architecting Enterprise AI Search &amp; <span class="highlight-text">High-Scale Systems</span>
      </h1>
      <p class="hero-subtitle">
        Hi, I'm <strong>Ravi Sankar Karuturi</strong>. 15+ years of engineering leadership across Ecommerce AI Search, Agentic Workflows, AdTech &amp; Site Monetization, Cloud Security, and Distributed Systems. Driving core platform innovations at Macy's, Gymboree, and Sony International.
      </p>
      <div class="hero-cta">
        <a href="#initiatives" class="btn-primary">
          <span>💼 Enterprise Initiatives</span>
          <span class="btn-arrow">&rarr;</span>
        </a>
        <a href="/kids-school-learning" class="btn-secondary" onclick="event.preventDefault(); window.navigateTo('/kids-school-learning');">
          <span>🎮 Kids Learning App</span>
        </a>
        <a href="#contact" class="btn-secondary">
          <span>📫 Get in Touch</span>
        </a>
      </div>
      <div class="hero-badges">
        <span class="hero-badge">🤖 Agentic AI &amp; "Ask Macy's"</span>
        <span class="hero-badge">🖼️ ColPali Image-RAG</span>
        <span class="hero-badge">⚡ Vector Search &amp; Embeddings</span>
        <span class="hero-badge">☕ Java / Spark / Cassandra</span>
        <span class="hero-badge">🦀 Rust &amp; Microservices</span>
        <span class="hero-badge">💰 Realtime Ads &amp; Monetization</span>
        <span class="hero-badge">🛡️ CyberArk &amp; Fraud Defense</span>
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
        <span class="hero-card-badge">Lead Architect Profile</span>
      </div>
      <div class="hero-stat-box">
        <div class="hero-profile-row">
          <div class="hero-profile-avatar">RK</div>
          <div>
            <h3 class="hero-profile-name">Ravi Sankar Karuturi</h3>
            <p class="hero-profile-title">Lead Software Engineer &amp; System Architect</p>
          </div>
        </div>
        <div class="hero-metrics-grid">
          <div class="metric-item">
            <span class="metric-val">15+</span>
            <span class="metric-label">Years Exp</span>
          </div>
          <div class="metric-item">
            <span class="metric-val">18h &rarr; 12m</span>
            <span class="metric-label">Spark Batch 90x Speedup</span>
          </div>
          <div class="metric-item">
            <span class="metric-val">O(n log n)</span>
            <span class="metric-label">Algorithm Optimization</span>
          </div>
        </div>
      </div>
      <div class="hero-skills-wrapper">
        <span class="skills-label">Core Domains &amp; Technologies</span>
        <div class="hero-skills-row">
          <span class="skill-pill">AI Search &amp; Agentic AI</span>
          <span class="skill-pill">ColPali Multimodal RAG</span>
          <span class="skill-pill">AdTech &amp; Realtime Ads</span>
          <span class="skill-pill">Java / Spark / Cassandra</span>
          <span class="skill-pill">Rust Microservices</span>
          <span class="skill-pill">Checkout Fraud Rekey</span>
          <span class="skill-pill">Unsupervised ML Clustering</span>
          <span class="skill-pill">CyberArk Privileged Access</span>
        </div>
      </div>
    </div>
  </section>

  <!-- KEY INITIATIVES SECTION -->
  <section class="section-wrapper" id="initiatives">
    <div class="section-header">
      <h2 class="section-title">🚀 Core Enterprise Initiatives &amp; Engineering Impact</h2>
      <p class="section-subtitle">Proven technical architecture and high-scale systems delivered across Macy's, Gymboree, and Sony International</p>
    </div>
    <div class="initiatives-grid">
      <!-- Initiative 1: AI Search & Agentic Systems -->
      <div class="initiative-card">
        <div class="init-header">
          <div class="init-icon-box">🤖</div>
          <div class="init-title-wrap">
            <span class="init-company-badge">Search Core &bull; Macys.com</span>
            <h3 class="init-card-title">E-Commerce AI Search &amp; Agentic Systems</h3>
          </div>
        </div>
        <div class="init-impact-callout">
          <span class="init-impact-num">ColPali &amp; Agentic</span>
          <span class="init-impact-text">Transforming retail discovery into intelligent conversational and multimodal agentic commerce.</span>
        </div>
        <ul class="init-bullets">
          <li><strong>"Ask Macy's" Conversational Search:</strong> Designed and developed an AI-powered conversational search &amp; commerce experience supporting shoppers across their entire purchasing lifecycle—from product finding and conversational filtering to order tracking and post-purchase inquiries.</li>
          <li><strong>Multimodal Image-RAG with ColPali:</strong> Built a state-of-the-art visual retrieval engine using the ColPali vision-language model, significantly outperforming traditional text-only RAG approaches on complex retail imagery.</li>
          <li><strong>Semantic Search Enhancement:</strong> Deployed dense vector embeddings on Product Detail Pages (PDPs) to identify and suggest contextually relevant search terms, greatly elevating catalog discoverability.</li>
          <li><strong>AI for Catalog Data Quality:</strong> Engineered automated AI data flow tools that continuously audit and validate product-to-category taxonomy mappings across the catalog.</li>
          <li><strong>Trending Query Intelligence:</strong> Applied embedding models to discover and rank trending consumer search queries in real time against catalog inventory.</li>
          <li><strong>Autonomous Root Cause Agents:</strong> Architected intelligent agents that automate root cause analysis and accelerate stakeholder support during incidents.</li>
        </ul>
        <div class="init-tags">
          <span class="init-tag">Ask Macy's</span>
          <span class="init-tag">ColPali Image-RAG</span>
          <span class="init-tag">Agentic AI</span>
          <span class="init-tag">Vector Embeddings</span>
          <span class="init-tag">Semantic Search</span>
          <span class="init-tag">Conversational Filtering</span>
        </div>
      </div>

      <!-- Initiative 2: Distributed Systems & Performance -->
      <div class="initiative-card">
        <div class="init-header">
          <div class="init-icon-box">⚡</div>
          <div class="init-title-wrap">
            <span class="init-company-badge">High-Scale Architecture &bull; Macys.com</span>
            <h3 class="init-card-title">Distributed Systems &amp; Performance Engineering</h3>
          </div>
        </div>
        <div class="init-impact-callout">
          <span class="init-impact-num">18h &rarr; 12m</span>
          <span class="init-impact-text">90x batch processing acceleration using Java, Apache Spark, and Apache Cassandra.</span>
        </div>
        <ul class="init-bullets">
          <li><strong>Massive Batch Modernization:</strong> Re-engineered legacy multi-stage batch processing with an optimized Java, Apache Spark, and Apache Cassandra pipeline, reducing total runtime from 18 hours down to 12 minutes.</li>
          <li><strong>Rust Microservices Health Check:</strong> Built an ultra-low-overhead health check observability tool in Rust to pinpoint faults across distributed microservice tiers.</li>
          <li><strong>Algorithmic Complexity Reduction:</strong> Optimized codebase method similarity computation, reducing computational complexity from O(n&sup2;) quadratic time to O(n log n).</li>
          <li><strong>Mobile App Dynamic Caching:</strong> Redesigned mobile app homepage content delivery using multi-layer caching, delivering dynamic personalization with high resilience under peak holiday traffic.</li>
        </ul>
        <div class="init-tags">
          <span class="init-tag">Apache Spark</span>
          <span class="init-tag">Apache Cassandra</span>
          <span class="init-tag">Rust</span>
          <span class="init-tag">Java</span>
          <span class="init-tag">O(n log n)</span>
          <span class="init-tag">Distributed Caching</span>
          <span class="init-tag">Microservices</span>
        </div>
      </div>

      <!-- Initiative 3: AdTech & Site Monetization -->
      <div class="initiative-card">
        <div class="init-header">
          <div class="init-icon-box">💰</div>
          <div class="init-title-wrap">
            <span class="init-company-badge">Monetization &bull; Macys.com</span>
            <h3 class="init-card-title">AdTech, Site Monetization &amp; Data Science</h3>
          </div>
        </div>
        <div class="init-impact-callout">
          <span class="init-impact-num">Realtime Ads</span>
          <span class="init-impact-text">High-concurrency ad mediation, programmatic auction engines, and behavioral customer clustering.</span>
        </div>
        <ul class="init-bullets">
          <li><strong>Realtime Ads in Email:</strong> Designed and developed an end-to-end programmatic ad rendering engine serving real-time dynamic targeted ads inside marketing emails at open time.</li>
          <li><strong>Sponsored Products Mediation Engine:</strong> Built high-concurrency mediation engine balancing organic search relevance and sponsored product yield on Macys.com.</li>
          <li><strong>Unsupervised Customer Clustering:</strong> Engineered customer clustering models using K-Means, DBSCAN, and Mean Shift algorithms to uncover distinct behavioral shopping personas.</li>
        </ul>
        <div class="init-tags">
          <span class="init-tag">Realtime Ad Serving</span>
          <span class="init-tag">Sponsored Products</span>
          <span class="init-tag">Ad Mediation</span>
          <span class="init-tag">K-Means / DBSCAN</span>
          <span class="init-tag">Mean Shift</span>
          <span class="init-tag">MarTech</span>
        </div>
      </div>

      <!-- Initiative 4: Security, Fraud & Global Platforms -->
      <div class="initiative-card">
        <div class="init-header">
          <div class="init-icon-box">🛡️</div>
          <div class="init-title-wrap">
            <span class="init-company-badge">Security &amp; Global &bull; Macy's, Gymboree, Sony</span>
            <h3 class="init-card-title">Enterprise Security, Fraud Defense &amp; Global CMS</h3>
          </div>
        </div>
        <div class="init-impact-callout">
          <span class="init-impact-num">$Millions Saved</span>
          <span class="init-impact-text">Holiday checkout fraud defense, enterprise CyberArk PAM, and global content architecture.</span>
        </div>
        <ul class="init-bullets">
          <li><strong>Holiday Fraud Prevention:</strong> Implemented checkout page re-keying protocols that intercepted and stopped millions of dollars in fraudulent transactions during peak holiday volume.</li>
          <li><strong>Password Hashing &amp; CyberArk:</strong> Hardened authentication layers with enterprise password hashing and integrated CyberArk for centralized server and database privileged access management.</li>
          <li><strong>Gymboree Enterprise SEO:</strong> Engineered comprehensive SEO architecture across homepage, category, and PDP pages with high-performance servlet filters managing URL migrations.</li>
          <li><strong>Sony International Multi-Lingual CMS:</strong> Scaled TeamSite enterprise CMS translation workflows, distributing localized content from English business authoring to country portals worldwide.</li>
        </ul>
        <div class="init-tags">
          <span class="init-tag">Fraud Rekey</span>
          <span class="init-tag">CyberArk PAM</span>
          <span class="init-tag">Password Hashing</span>
          <span class="init-tag">Enterprise SEO</span>
          <span class="init-tag">Sony TeamSite CMS</span>
          <span class="init-tag">Servlet Filters</span>
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
            <span class="project-tag">WebAssembly</span>
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
        <a href="#initiatives">Initiatives</a>
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
