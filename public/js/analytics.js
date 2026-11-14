/**
 * KaruturiRS Analytics & Quiz Engagement Telemetry
 * Integrates with Google Tag Manager (dataLayer) and Google Analytics 4 (gtag).
 */
(function() {
  'use strict';

  // Base telemetry event dispatcher
  function trackEvent(eventName, params) {
    try {
      params = params || {};
      params.page_path = window.location.pathname;
      params.timestamp = new Date().toISOString();

      if (typeof window.gtag === 'function') {
        window.gtag('event', eventName, params);
      }
      if (window.dataLayer && Array.isArray(window.dataLayer)) {
        window.dataLayer.push({
          event: eventName,
          ...params
        });
      }
    } catch (err) {
      console.warn('Telemetry error:', err);
    }
  }

  // Expose trackEvent globally
  window.trackAnalyticsEvent = trackEvent;

  // Active quiz session tracking
  let activeQuiz = null;

  function getStoredSettings() {
    return {
      difficulty: localStorage.getItem('kids_diff') || 'growing',
      style: localStorage.getItem('kids_style') || 'chill',
      round_length: localStorage.getItem('kids_round_len') || '10',
      sound: localStorage.getItem('kids_sound') !== 'false'
    };
  }

  // 1. Track runtime errors & issues
  window.addEventListener('error', function(event) {
    trackEvent('app_error', {
      error_type: 'javascript_error',
      message: event.message || 'Unknown JS error',
      filename: event.filename || '',
      lineno: event.lineno || 0,
      colno: event.colno || 0
    });
  });

  window.addEventListener('unhandledrejection', function(event) {
    trackEvent('app_error', {
      error_type: 'unhandled_promise_rejection',
      message: (event.reason && (event.reason.message || event.reason.stack))
        ? String(event.reason.message || event.reason)
        : String(event.reason)
    });
  });

  // 2. Track Portfolio Interest & Outbound Links
  document.addEventListener('click', function(e) {
    const target = e.target.closest('a, button');
    if (!target) return;

    // Outbound Link (e.g. GitHub repos, certifications, external articles)
    if (target.tagName === 'A' && target.href) {
      const url = target.href;
      if (url.startsWith('http') && !url.includes(window.location.hostname)) {
        trackEvent('outbound_click', {
          destination_url: url,
          link_text: (target.textContent || '').trim().slice(0, 80)
        });
      } else if (url.startsWith('mailto:')) {
        trackEvent('contact_click', {
          email: url.replace('mailto:', '')
        });
      }
    }

    // Portfolio Section or Feature Clicks
    const sectionLink = target.closest('.nav-item, .btn-primary, .btn-secondary, .project-link, .brand-link');
    if (sectionLink && sectionLink.textContent) {
      const label = sectionLink.textContent.trim().replace(/\s+/g, ' ');
      if (label && !label.includes('Kids')) {
        trackEvent('portfolio_interest', {
          element_text: label.slice(0, 60),
          target_id: sectionLink.getAttribute('href') || ''
        });
      }
    }
  });

  // 3. WebAssembly Quiz Telemetry Hook
  window.initAnalyticsTracking = function(wasm) {
    if (!wasm) return;

    // Helper to evaluate and emit quiz completion data
    function checkAndTrackCompletion() {
      setTimeout(() => {
        const resultCard = document.querySelector('.result-card');
        if (resultCard && activeQuiz && !activeQuiz.completed) {
          activeQuiz.completed = true;
          const title = document.querySelector('.result-title')?.textContent?.trim() || '';
          const stats = document.querySelectorAll('.res-stat .num');
          const score = stats[0] ? parseInt(stats[0].textContent.trim(), 10) : activeQuiz.right;
          const total = stats[1] ? parseInt(stats[1].textContent.trim(), 10) : (activeQuiz.right + activeQuiz.wrong);
          const starsText = stats[2] ? stats[2].textContent.trim().replace('+', '') : '0';
          const stars = parseInt(starsText, 10) || 0;
          const accuracy = total > 0 ? Math.round((score / total) * 100) : 0;
          const durationSec = Math.round((Date.now() - activeQuiz.startTime) / 1000);

          trackEvent('quiz_complete', {
            game_id: activeQuiz.gameId,
            score: score,
            total: total,
            accuracy_pct: accuracy,
            stars_earned: stars,
            outcome_title: title,
            duration_seconds: durationSec,
            difficulty: activeQuiz.difficulty,
            play_style: activeQuiz.style
          });
        }
      }, 60);
    }

    // Quiz Start & Topic Interest
    const origStartGame = wasm.kids_start_game;
    window.kidsStartGame = window.startGame = function(gameId) {
      const settings = getStoredSettings();
      activeQuiz = {
        gameId: gameId,
        startTime: Date.now(),
        right: 0,
        wrong: 0,
        difficulty: settings.difficulty,
        style: settings.style,
        roundLength: settings.round_length,
        completed: false
      };

      trackEvent('quiz_start', {
        game_id: gameId,
        difficulty: settings.difficulty,
        play_style: settings.style,
        round_length: settings.round_length
      });

      return origStartGame(gameId);
    };

    // Quiz Multiple Choice Answer
    const origAnswerChoice = wasm.kids_answer_choice;
    window.kidsAnswerChoice = window.answerChoice = function(choiceIdx, val) {
      const prompt = document.querySelector('.qprompt')?.textContent?.trim() || '';
      const result = origAnswerChoice(choiceIdx, val);

      setTimeout(() => {
        const feedback = document.querySelector('.feedback');
        if (feedback) {
          const feedbackText = feedback.textContent.trim();
          if (feedback.classList.contains('good')) {
            if (activeQuiz) activeQuiz.right++;
            trackEvent('quiz_answer_correct', {
              game_id: activeQuiz ? activeQuiz.gameId : 'unknown',
              prompt: prompt.slice(0, 100),
              answer: String(val)
            });
          } else if (feedback.classList.contains('try')) {
            if (activeQuiz) activeQuiz.wrong++;
            trackEvent('quiz_mistake', {
              game_id: activeQuiz ? activeQuiz.gameId : 'unknown',
              prompt: prompt.slice(0, 100),
              user_answer: String(val),
              feedback: feedbackText.slice(0, 120),
              issue_type: feedbackText.includes('Time up') ? 'timeout' : 'wrong_choice'
            });
          }
        }
        checkAndTrackCompletion();
      }, 40);

      return result;
    };

    // Spelling Tile Tap
    const origTapSpellTile = wasm.kids_tap_spell_tile;
    window.kidsTapSpellTile = window.tapTile = function(tileIdx, chStr) {
      const result = origTapSpellTile(tileIdx, chStr);
      setTimeout(() => {
        const feedback = document.querySelector('.feedback');
        if (feedback) {
          const feedbackText = feedback.textContent.trim();
          if (feedback.classList.contains('try')) {
            trackEvent('quiz_mistake', {
              game_id: activeQuiz ? activeQuiz.gameId : 'spelling',
              feedback: feedbackText.slice(0, 120),
              issue_type: 'spelling_mistake'
            });
          } else if (feedback.classList.contains('good')) {
            if (activeQuiz) activeQuiz.right++;
            trackEvent('quiz_answer_correct', {
              game_id: activeQuiz ? activeQuiz.gameId : 'spelling',
              feedback: feedbackText.slice(0, 120)
            });
          }
        }
        checkAndTrackCompletion();
      }, 40);
      return result;
    };

    // Early Finish
    const origFinishGame = wasm.kids_finish_game;
    window.kidsFinishGame = window.finishGame = function() {
      const res = origFinishGame();
      checkAndTrackCompletion();
      return res;
    };

    // Quit / Abandon Quiz
    const origQuitGame = wasm.kids_quit_game;
    window.kidsQuitGame = window.quitGame = function() {
      if (activeQuiz && !activeQuiz.completed) {
        trackEvent('quiz_abandon', {
          game_id: activeQuiz.gameId,
          questions_answered: activeQuiz.right + activeQuiz.wrong,
          correct_answers: activeQuiz.right,
          duration_seconds: Math.round((Date.now() - activeQuiz.startTime) / 1000)
        });
      }
      return origQuitGame();
    };

    // Replay Quiz
    const origReplayGame = wasm.kids_replay_game;
    window.kidsReplayGame = window.replayGame = function() {
      trackEvent('quiz_replay', {
        game_id: activeQuiz ? activeQuiz.gameId : 'unknown'
      });
      return origReplayGame();
    };

    // Keep Going
    const origKeepGoing = wasm.kids_keep_going;
    window.kidsKeepGoing = window.keepGoing = function() {
      trackEvent('quiz_keep_going', {
        game_id: activeQuiz ? activeQuiz.gameId : 'unknown'
      });
      return origKeepGoing();
    };

    // Show Me How / Technique Viewed (User interest in learning trick)
    const origToggleTech = wasm.kids_toggle_tech;
    window.kidsToggleTech = window.toggleTechnique = function() {
      const prompt = document.querySelector('.qprompt')?.textContent?.trim() || '';
      const techTitle = document.querySelector('.tech-title')?.textContent?.trim() || '';
      trackEvent('learning_trick_viewed', {
        game_id: activeQuiz ? activeQuiz.gameId : 'unknown',
        prompt: prompt.slice(0, 100),
        technique_name: techTitle.slice(0, 80)
      });
      return origToggleTech();
    };

    // Speech / Audio Assist
    const origSpeakQuestion = wasm.kids_speak_question;
    window.kidsSpeakQuestion = window.speakQuestion = function() {
      const prompt = document.querySelector('.qprompt')?.textContent?.trim() || '';
      trackEvent('speech_assist_used', {
        game_id: activeQuiz ? activeQuiz.gameId : 'unknown',
        prompt: prompt.slice(0, 100),
        type: 'question'
      });
      return origSpeakQuestion();
    };

    const origReadQueueItem = wasm.kids_read_queue_item;
    window.kidsReadQueueItem = window.readQueueItem = function(idx) {
      trackEvent('speech_assist_used', {
        queue_idx: idx,
        type: 'review_queue'
      });
      return origReadQueueItem(idx);
    };

    // Settings Changes
    const origSetDiff = wasm.kids_set_diff;
    window.kidsSetDiff = window.setDiff = function(diff) {
      trackEvent('quiz_difficulty_changed', { difficulty: diff });
      return origSetDiff(diff);
    };

    const origSetStyle = wasm.kids_set_style;
    window.kidsSetStyle = window.setStyle = function(style) {
      trackEvent('quiz_style_changed', { play_style: style });
      return origSetStyle(style);
    };

    const origSetRoundLen = wasm.kids_set_round_len;
    window.kidsSetRoundLen = window.setLength = function(len) {
      trackEvent('quiz_round_length_changed', { round_length: len });
      return origSetRoundLen(len);
    };

    const origToggleSound = wasm.kids_toggle_sound;
    window.kidsToggleSound = window.toggleSound = function() {
      trackEvent('sound_toggled');
      return origToggleSound();
    };

    // Review Queue Filter Changed
    const origSetQueueFilter = wasm.kids_set_queue_filter;
    window.kidsSetQueueFilter = window.setQueueFilter = function(filter) {
      trackEvent('quiz_review_filter_changed', { filter: filter });
      return origSetQueueFilter(filter);
    };
  };
})();
