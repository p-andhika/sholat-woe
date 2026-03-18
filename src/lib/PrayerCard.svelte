<script lang="ts">
  import type { PrayerTime } from "./stores";

  let { prayer }: { prayer: PrayerTime } = $props();
</script>

<div
  class="prayer-card"
  class:passed={prayer.passed}
  class:next={prayer.is_next}
>
  <div class="prayer-left">
    <span class="prayer-icon">{prayer.icon}</span>
    <span class="prayer-name">{prayer.name}</span>
  </div>
  <div class="prayer-right">
    <span class="prayer-time">{prayer.time}</span>
    {#if prayer.passed}
      <span class="status-badge passed-badge">✓</span>
    {:else if prayer.is_next}
      <span class="status-badge next-badge">NEXT</span>
    {/if}
  </div>
</div>

<style>
  .prayer-card {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 14px 18px;
    border-radius: var(--radius);
    background: var(--bg-card);
    border: 1px solid var(--border);
    transition: all 0.2s ease;
    margin-bottom: 8px;
  }

  .prayer-card:hover {
    background: var(--bg-card-active);
  }

  .prayer-card.passed {
    opacity: 0.5;
  }

  .prayer-card.next {
    background: var(--bg-card-next);
    border-color: var(--accent);
    box-shadow: 0 0 20px var(--accent-glow);
    opacity: 1;
  }

  .prayer-left {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .prayer-icon {
    font-size: 1.3rem;
  }

  .prayer-name {
    font-weight: 600;
    font-size: 0.95rem;
    letter-spacing: 0.02em;
  }

  .prayer-right {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .prayer-time {
    font-size: 1.05rem;
    font-weight: 700;
    font-variant-numeric: tabular-nums;
    color: var(--text-primary);
  }

  .next .prayer-time {
    color: var(--accent);
  }

  .status-badge {
    font-size: 0.65rem;
    font-weight: 700;
    padding: 3px 8px;
    border-radius: 6px;
    letter-spacing: 0.05em;
  }

  .passed-badge {
    background: rgba(74, 222, 128, 0.15);
    color: var(--success);
  }

  .next-badge {
    background: rgba(56, 189, 248, 0.2);
    color: var(--accent);
    animation: pulse 2s infinite;
  }

  @keyframes pulse {
    0%,
    100% {
      opacity: 1;
    }
    50% {
      opacity: 0.6;
    }
  }
</style>
