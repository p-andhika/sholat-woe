<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import {
    isPermissionGranted,
    requestPermission,
    sendNotification,
  } from "@tauri-apps/plugin-notification";
  import {
    prayerData,
    config,
    loading,
    error,
    showSettings,
  } from "./lib/stores";
  import type { PrayerTimesResult, PrayerConfig } from "./lib/stores";
  import PrayerCard from "./lib/PrayerCard.svelte";
  import Settings from "./lib/Settings.svelte";

  let now = $state(new Date());
  let notifiedPrayers: Set<string> = new Set();

  // ─── Fetch prayer times from Rust backend ──────────────────
  async function loadPrayerTimes() {
    try {
      $loading = true;
      const result = await invoke<PrayerTimesResult>("get_prayer_times");
      $prayerData = result;
      $error = null;
    } catch (e: any) {
      $error = e.toString();
    }
    $loading = false;
  }

  async function loadConfig() {
    try {
      const cfg = await invoke<PrayerConfig>("get_config");
      $config = cfg;
    } catch (_) {}
  }

  // ─── Notifications ─────────────────────────────────────────
  async function setupNotifications() {
    let granted = await isPermissionGranted();
    if (!granted) {
      const permission = await requestPermission();
      granted = permission === "granted";
    }
    return granted;
  }

  function checkAndNotify() {
    if (!$prayerData) return;

    const nowTime = new Date();
    const currentMinutes = nowTime.getHours() * 60 + nowTime.getMinutes();

    for (const prayer of $prayerData.prayers) {
      const [h, m] = prayer.time.split(":").map(Number);
      const prayerMinutes = h * 60 + m;
      const diff = prayerMinutes - currentMinutes;

      // Advance notification (trigger within 1-minute window before target)
      const advKey = `${prayer.name}_adv_${nowTime.toDateString()}`;
      if (diff <= $config.notify_before_mins && diff > $config.notify_before_mins - 1 && !notifiedPrayers.has(advKey)) {
        sendNotification({
          title: `🕌 ${prayer.name} in ${$config.notify_before_mins} min`,
          body: `${prayer.name} prayer at ${prayer.time}. Prepare for salah.`,
        });
        notifiedPrayers.add(advKey);
      }

      // Exact time notification (trigger within 1-minute window after prayer time)
      const exactKey = `${prayer.name}_exact_${nowTime.toDateString()}`;
      if (diff <= 0 && diff > -1 && !notifiedPrayers.has(exactKey)) {
        sendNotification({
          title: `🕌 ${prayer.name} Time`,
          body: `It's time for ${prayer.name} prayer. Allahu Akbar!`,
        });
        notifiedPrayers.add(exactKey);
      }
    }
  }

  // ─── Lifecycle ─────────────────────────────────────────────
  onMount(() => {
    loadConfig().then(() => {
      loadPrayerTimes();
    });
    setupNotifications();

    // Update clock every second
    const clockInterval = setInterval(() => {
      now = new Date();
    }, 1000);

    // Check notifications every 10 seconds
    const notifInterval = setInterval(checkAndNotify, 10_000);

    // Refresh prayer times every hour
    const refreshInterval = setInterval(loadPrayerTimes, 3_600_000);

    return () => {
      clearInterval(clockInterval);
      clearInterval(notifInterval);
      clearInterval(refreshInterval);
    };
  });

  function formatTime(date: Date): string {
    return date.toLocaleTimeString("en-US", {
      hour: "2-digit",
      minute: "2-digit",
      second: "2-digit",
      hour12: false,
    });
  }

  function formatDate(date: Date): string {
    return date.toLocaleDateString("en-US", {
      weekday: "long",
      month: "long",
      day: "numeric",
    });
  }
</script>

<main>
  <!-- Header -->
  <header>
    <div class="header-top">
      <div>
        <div class="clock">{formatTime(now)}</div>
        <div class="date">{formatDate(now)}</div>
      </div>
      <button
        class="settings-btn"
        onclick={() => ($showSettings = true)}
        title="Settings"
      >
        ⚙️
      </button>
    </div>

    {#if $prayerData}
      <div class="hijri">{$prayerData.hijri_date}</div>

      <div class="next-prayer-banner">
        <div class="next-label">Next Prayer</div>
        <div class="next-name">{$prayerData.next_prayer}</div>
        <div class="next-details">
          <span class="next-time">{$prayerData.next_prayer_time}</span>
          <span class="next-remaining">· {$prayerData.time_remaining}</span>
        </div>
      </div>
    {/if}
  </header>

  <!-- Prayer Times List -->
  <section class="prayer-list">
    {#if $loading}
      <div class="loading">
        <div class="spinner"></div>
        <span>Loading prayer times...</span>
      </div>
    {:else if $error}
      <div class="error-msg">
        <span>⚠️ {$error}</span>
        <button onclick={loadPrayerTimes}>Retry</button>
      </div>
    {:else if $prayerData}
      {#each $prayerData.prayers as prayer (prayer.name)}
        <PrayerCard {prayer} />
      {/each}

      <!-- Sunrise (not a prayer, just info) -->
      <div class="sunrise-info">
        ☀️ Sunrise: {$prayerData.sunrise}
      </div>
    {/if}
  </section>

  <!-- Footer -->
  <footer>
    <span class="method-label">📍 {$config.city}, {$config.country}</span>
    {#if $prayerData}
      <span class="method-label">· {$prayerData.method_name}</span>
    {/if}
  </footer>

  <!-- Settings Modal -->
  {#if $showSettings}
    <Settings />
  {/if}
</main>

<style>
  main {
    max-width: 420px;
    margin: 0 auto;
    padding: 20px;
    min-height: 100vh;
    display: flex;
    flex-direction: column;
  }

  /* ─── Header ─────────────────────────────────── */
  header {
    margin-bottom: 24px;
  }

  .header-top {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 4px;
  }

  .clock {
    font-size: 2.2rem;
    font-weight: 700;
    font-variant-numeric: tabular-nums;
    letter-spacing: -0.02em;
    color: var(--text-primary);
  }

  .date {
    font-size: 0.85rem;
    color: var(--text-secondary);
    margin-top: 2px;
  }

  .hijri {
    font-size: 0.8rem;
    color: var(--text-muted);
    margin-top: 2px;
    margin-bottom: 16px;
  }

  .settings-btn {
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    border-radius: 10px;
    padding: 8px 12px;
    font-size: 1.1rem;
    cursor: pointer;
    transition: background 0.2s;
  }

  .settings-btn:hover {
    background: var(--bg-card-active);
  }

  /* ─── Next Prayer Banner ─────────────────────── */
  .next-prayer-banner {
    background: linear-gradient(135deg, #1e3a5f 0%, #0f2744 100%);
    border: 1px solid rgba(56, 189, 248, 0.2);
    border-radius: 16px;
    padding: 20px;
    text-align: center;
  }

  .next-label {
    font-size: 0.7rem;
    text-transform: uppercase;
    letter-spacing: 0.12em;
    color: var(--text-muted);
    margin-bottom: 4px;
  }

  .next-name {
    font-size: 1.6rem;
    font-weight: 700;
    color: var(--accent);
    margin-bottom: 6px;
  }

  .next-details {
    font-size: 0.95rem;
    color: var(--text-secondary);
  }

  .next-time {
    font-weight: 600;
    color: var(--text-primary);
  }

  .next-remaining {
    color: var(--text-muted);
  }

  /* ─── Prayer List ────────────────────────────── */
  .prayer-list {
    flex: 1;
  }

  .sunrise-info {
    text-align: center;
    font-size: 0.8rem;
    color: var(--text-muted);
    margin-top: 8px;
    padding: 8px;
  }

  /* ─── Loading & Error ────────────────────────── */
  .loading {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    padding: 40px;
    color: var(--text-muted);
  }

  .spinner {
    width: 32px;
    height: 32px;
    border: 3px solid var(--border);
    border-top-color: var(--accent);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .error-msg {
    text-align: center;
    padding: 24px;
    color: #f87171;
    display: flex;
    flex-direction: column;
    gap: 12px;
    align-items: center;
  }

  .error-msg button {
    padding: 8px 20px;
    border-radius: 8px;
    background: var(--accent);
    color: var(--bg-primary);
    border: none;
    font-weight: 600;
    cursor: pointer;
  }

  /* ─── Footer ─────────────────────────────────── */
  footer {
    margin-top: 20px;
    padding-top: 16px;
    border-top: 1px solid var(--border);
    text-align: center;
  }

  .method-label {
    font-size: 0.75rem;
    color: var(--text-muted);
  }
</style>
