<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { getVersion } from "@tauri-apps/api/app";
  import { listen } from "@tauri-apps/api/event";
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
  } from "../lib/stores";
  import type { PrayerTimesResult, PrayerConfig } from "../lib/stores";
  import Settings from "../lib/Settings.svelte";
  import {
    MoonStars,
    SunHorizon,
    Sun,
    CloudSun,
    Moon,
    Clock,
    Star,
    Sparkle,
    MapPin,
    CheckCircle,
  } from "phosphor-svelte";

  let now = $state(new Date());
  let appVersion = $state("");
  let shouldAnimate = $state(true);

  function loadNotifiedPrayers(): Set<string> {
    try {
      const today = new Date().toDateString();
      const stored = localStorage.getItem("notifiedPrayers");
      if (!stored) return new Set();
      const entries: string[] = JSON.parse(stored);
      // Only keep entries for today to prune stale data
      return new Set(entries.filter((k) => k.endsWith(today)));
    } catch {
      return new Set();
    }
  }

  function saveNotifiedPrayers(set: Set<string>) {
    localStorage.setItem("notifiedPrayers", JSON.stringify([...set]));
  }

  let notifiedPrayers: Set<string> = loadNotifiedPrayers();

  // Map icon names to Phosphor components
  const iconMap: Record<string, any> = {
    "moon-star": MoonStars,
    sunrise: SunHorizon,
    sun: Sun,
    "cloud-sun": CloudSun,
    sunset: SunHorizon,
    moon: Moon,
    clock: Clock,
    star: Star,
    sparkle: Sparkle,
  };

  function isTimePassed(timeStr: string): boolean {
    const [hours, minutes] = timeStr.split(":").map(Number);
    const timeDate = new Date();
    timeDate.setHours(hours, minutes, 0, 0);
    return now > timeDate;
  }

  async function loadPrayerTimes() {
    try {
      $loading = true;
      const result = await invoke<PrayerTimesResult>("get_prayer_times");
      $prayerData = result;
      $error = null;
      updateTrayTitle(result);
    } catch (e: any) {
      $error = e.toString();
    }
    $loading = false;
  }

  function updateTrayTitle(data: PrayerTimesResult) {
    const next = data.prayers.find((p) => p.is_next);
    if (next) {
      invoke("set_tray_title", { title: `${next.name} ${next.time}` });
    } else {
      invoke("set_tray_title", { title: "" });
    }
  }

  async function loadConfig() {
    try {
      const cfg = await invoke<PrayerConfig>("get_config");
      $config = cfg;
    } catch (_) {}
  }

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
    let prayerJustPassed = false;

    for (const prayer of $prayerData.prayers) {
      const [h, m] = prayer.time.split(":").map(Number);
      const prayerMinutes = h * 60 + m;
      const diff = prayerMinutes - currentMinutes;

      // Advance notification (trigger within 1-minute window before target)
      const advKey = `${prayer.name}_adv_${nowTime.toDateString()}`;
      if (
        diff <= $config.notify_before_mins &&
        diff > $config.notify_before_mins - 1 &&
        !notifiedPrayers.has(advKey)
      ) {
        sendNotification({
          title: `${prayer.name} in ${$config.notify_before_mins} min`,
          body: `${prayer.name} prayer at ${prayer.time}. Prepare for salah.`,
        });
        invoke("play_sound", { soundName: "Ping" });
        notifiedPrayers.add(advKey);
        saveNotifiedPrayers(notifiedPrayers);
      }

      // Exact time notification (trigger within 1-minute window after prayer time)
      const exactKey = `${prayer.name}_exact_${nowTime.toDateString()}`;
      if (diff <= 0 && diff > -1 && !notifiedPrayers.has(exactKey)) {
        sendNotification({
          title: `${prayer.name} Time`,
          body: `It's time for ${prayer.name} prayer. Allahu Akbar!`,
        });
        invoke("play_sound", { soundName: "Glass" });
        notifiedPrayers.add(exactKey);
        saveNotifiedPrayers(notifiedPrayers);
        prayerJustPassed = true;
      }
    }

    // When a prayer time passes, refresh data to update next prayer
    if (prayerJustPassed) {
      loadPrayerTimes();
    }
  }

  onMount(() => {
    getVersion().then((v) => (appVersion = v));
    const unlisten = listen("window-shown", () => {
      shouldAnimate = false;
      requestAnimationFrame(() => {
        shouldAnimate = true;
      });
    });

    const unlistenSettings = listen("open-settings", () => {
      $showSettings = true;
    });

    loadConfig().then(() => {
      loadPrayerTimes();
    });
    setupNotifications();

    const clockInterval = setInterval(() => {
      now = new Date();
    }, 1000);

    const notifInterval = setInterval(checkAndNotify, 10_000);
    const refreshInterval = setInterval(loadPrayerTimes, 3_600_000);

    // Handle system wake/sleep - refresh data when page becomes visible
    const handleVisibilityChange = () => {
      if (!document.hidden) {
        loadPrayerTimes();
      }
    };
    document.addEventListener("visibilitychange", handleVisibilityChange);

    return () => {
      clearInterval(clockInterval);
      clearInterval(notifInterval);
      clearInterval(refreshInterval);
      document.removeEventListener("visibilitychange", handleVisibilityChange);
      unlisten.then((fn) => fn());
      unlistenSettings.then((fn) => fn());
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

  // Get all times including prayers and info times
  function getAllTimes() {
    if (!$prayerData) return [];

    const times = [];

    // Add all prayers including Imsak
    for (const prayer of $prayerData.prayers) {
      times.push({
        name: prayer.name,
        time: prayer.time,
        icon: prayer.icon,
        passed: prayer.passed,
        isNext: prayer.is_next,
        type: "prayer",
      });

      // Add sunrise after Fajr
      if (prayer.name === "Fajr") {
        times.push({
          name: "Sunrise",
          time: $prayerData.sunrise,
          icon: "sunrise",
          passed: isTimePassed($prayerData.sunrise),
          isNext: false,
          type: "info",
        });
      }

      // Add sunset after Asr
      if (prayer.name === "Asr") {
        times.push({
          name: "Sunset",
          time: $prayerData.sunset,
          icon: "sunset",
          passed: isTimePassed($prayerData.sunset),
          isNext: false,
          type: "info",
        });
      }

      // Add First Third, Midnight, Last Third after Isha
      if (prayer.name === "Isha") {
        times.push({
          name: "First Third",
          time: $prayerData.firstthird,
          icon: "star",
          passed: isTimePassed($prayerData.firstthird),
          isNext: false,
          type: "info",
        });
        times.push({
          name: "Midnight",
          time: $prayerData.midnight,
          icon: "moon",
          passed: isTimePassed($prayerData.midnight),
          isNext: false,
          type: "info",
        });
        times.push({
          name: "Last Third",
          time: $prayerData.lastthird,
          icon: "sparkle",
          passed: isTimePassed($prayerData.lastthird),
          isNext: false,
          type: "info",
        });
      }
    }

    return times;
  }
</script>

<main class:animate={shouldAnimate}>
  <!-- Top Bar -->
  {#if $prayerData && $prayerData.time_remaining !== "tomorrow"}
    <div class="top-bar">
      <div class="top-bar-left">
        <div class="clock-top">{formatTime(now)}</div>
        <div class="dates-top">
          <span class="date-top">{formatDate(now)}</span>
          <span class="hijri-top">{$prayerData.hijri_date}</span>
        </div>
      </div>
      <div class="top-bar-right">
        <div class="location-pill">
          <MapPin size={11} weight="fill" />
          <span class="location-text"
            >{$config.city.toUpperCase()}, {$config.country}</span
          >
        </div>
      </div>
    </div>
  {/if}

  <!-- Prayer Grid -->
  <section class="prayer-grid">
    {#if $loading}
      <div class="loading">Loading...</div>
    {:else if $error}
      <div class="error-msg">
        <span>⚠️ {$error}</span>
        <button onclick={loadPrayerTimes}>Retry</button>
      </div>
    {:else if $prayerData}
      {#each getAllTimes() as time (time.name)}
        <div
          class="prayer-card"
          class:passed={time.passed}
          class:next={time.isNext}
        >
          <div class="card-top-row">
            {#if iconMap[time.icon]}
              {@const Icon = iconMap[time.icon]}
              <Icon size={22} weight="regular" class="card-icon" />
            {/if}
            {#if time.isNext}
              <div class="upcoming-label"><span>UPCOMING</span></div>
            {:else if time.passed}
              <CheckCircle size={16} weight="bold" class="check-icon" />
            {/if}
          </div>
          <div class="card-bottom-row">
            <div class="card-name">{time.name.toUpperCase()}</div>
            <div class="card-time">{time.time}</div>
          </div>
        </div>
      {/each}
      {#if appVersion}
        <div class="version-cell">v{appVersion}</div>
      {/if}
    {/if}
  </section>

  <!-- Settings Modal -->
  {#if $showSettings}
    <Settings />
  {/if}
</main>

<style>
  main {
    width: 100%;
    height: 100vh;
    max-height: 600px;
    display: flex;
    flex-direction: column;
    background: #0a0a0a;
    overflow: hidden;
    box-sizing: border-box;
    position: relative;
  }

  main.animate {
    animation: slideDown 0.2s ease-out;
  }

  @keyframes slideDown {
    from {
      opacity: 0;
      transform: translateY(-15px) scale(0.98);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }

  /* Top Bar */
  .top-bar {
    background: linear-gradient(135deg, #7c3aed 0%, #a855f7 100%);
    padding: 16px 20px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
  }

  .top-bar-left {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .dates-top {
    display: flex;
    gap: 10px;
    align-items: center;
  }

  .clock-top {
    font-size: 1.2rem;
    font-weight: 700;
    font-variant-numeric: tabular-nums;
    color: #ffffff;
    letter-spacing: -0.02em;
    line-height: 1;
  }

  .date-top {
    font-size: 0.55rem;
    color: #ffffff;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    font-weight: 700;
  }

  .hijri-top {
    font-size: 0.55rem;
    color: rgba(255, 255, 255, 0.7);
    font-weight: 400;
  }

  .top-bar-right {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  /* Prayer Grid */
  .prayer-grid {
    flex: 1;
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 4px;
    background: #1a1a1a;
    overflow-y: auto;
  }

  .prayer-card {
    background: #0a0a0a;
    padding: 18px;
    display: flex;
    flex-direction: column;
    gap: 24px;
    position: relative;
    transition: all 0.2s ease;
    justify-content: center;
  }

  .prayer-card.next {
    border-right: 4px solid #7c3aed;
    padding-right: 16px;
  }

  .prayer-card.next :global(svg) {
    opacity: 1;
    color: #ffffff;
    width: 28px;
    height: 28px;
  }

  .prayer-card.next .card-name {
    color: #ffffff;
    font-size: 1em;
  }

  .prayer-card.next .card-time {
    color: #ffffff;
    font-size: 1em;
  }

  .prayer-card.passed .card-time {
    color: #ffffff;
  }

  .card-top-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .card-top-row :global(svg) {
    opacity: 0.5;
    flex-shrink: 0;
  }

  .prayer-card.passed .card-top-row :global(svg) {
    opacity: 1;
  }

  :global(.check-icon) {
    color: #22c55e;
    flex-shrink: 0;
  }

  .card-bottom-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .card-name {
    font-size: 0.65rem;
    font-weight: 700;
    color: #555555;
    letter-spacing: 0.05em;
  }

  .card-time {
    font-size: 0.65rem;
    font-weight: 700;
    color: rgba(255, 255, 255, 0.6);
    font-variant-numeric: tabular-nums;
    letter-spacing: -0.02em;
  }

  .upcoming-label {
    position: relative;
    overflow: hidden;
    padding: 6px 14px;
    border-radius: 20px;
    font-size: 0.5rem;
    font-weight: 700;
    color: #a855f7;
    letter-spacing: 0.05em;
  }

  .upcoming-label::before {
    content: "";
    position: absolute;
    inset: -100%;
    background: conic-gradient(
      from 0deg,
      transparent 0deg,
      #7c3aed 60deg,
      #a855f7 120deg,
      #c084fc 180deg,
      transparent 240deg
    );
    animation: spinBorder 2s linear infinite;
    z-index: 0;
  }

  .upcoming-label::after {
    content: "";
    position: absolute;
    inset: 1.5px;
    border-radius: 18px;
    background: #0a0a0a;
    z-index: 1;
  }

  .upcoming-label span {
    position: relative;
    z-index: 2;
  }

  @keyframes spinBorder {
    to {
      transform: rotate(360deg);
    }
  }

  .location-pill {
    display: flex;
    align-items: center;
    gap: 4px;
    color: rgba(255, 255, 255, 0.7);
  }

  .location-text {
    font-size: 0.6rem;
    color: #ffffff;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    font-weight: 700;
  }

  /* Loading & Error */
  .loading {
    grid-column: 1 / -1;
    text-align: center;
    padding: 40px;
    color: #888888;
  }

  .error-msg {
    grid-column: 1 / -1;
    text-align: center;
    padding: 24px;
    color: #f87171;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .error-msg button {
    padding: 8px 20px;
    border-radius: 8px;
    background: #7c3aed;
    color: #ffffff;
    border: none;
    font-weight: 600;
    cursor: pointer;
  }

  .version-cell {
    background: #0a0a0a;
    display: flex;
    align-items: flex-end;
    justify-content: flex-end;
    padding: 18px;
    font-size: 0.55rem;
    font-weight: 600;
    color: rgb(168, 85, 247);
    letter-spacing: 0.05em;
  }
</style>
