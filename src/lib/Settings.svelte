<script lang="ts">
  import { config, showSettings, loading, prayerData, error } from "./stores";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { indonesianCities } from "./indonesian-cities";

  let localConfig = $derived({ ...$config });
  let saving = $state(false);
  let filteredCities: string[] = $state([]);
  let showSuggestions = $state(false);
  let searchInput = $state("");

  let cityInput: HTMLInputElement;

  onMount(() => {
    searchInput = localConfig.city;
    filteredCities = indonesianCities;
    cityInput?.focus();
  });

  function handleCityInput(e: Event) {
    const value = (e.target as HTMLInputElement).value;
    searchInput = value;
    localConfig.city = value;

    if (value.length > 0) {
      filteredCities = indonesianCities.filter((city) =>
        city.toLowerCase().includes(value.toLowerCase())
      );
      showSuggestions = true;
    } else {
      filteredCities = indonesianCities;
      showSuggestions = false;
    }
  }

  function selectCity(city: string) {
    searchInput = city;
    localConfig.city = city;
    showSuggestions = false;
  }

  function handleFocus() {
    if (searchInput.length > 0) {
      filteredCities = indonesianCities.filter((city) =>
        city.toLowerCase().includes(searchInput.toLowerCase())
      );
      showSuggestions = true;
    }
  }

  function handleBlur() {
    // Delay to allow click on suggestion
    setTimeout(() => {
      showSuggestions = false;
    }, 200);
  }

  async function save() {
    saving = true;
    try {
      const configToSave = { ...localConfig, country: "ID" };
      const result = await invoke<any>("update_config", { config: configToSave });
      $config = { ...configToSave };
      $prayerData = result;
      $showSettings = false;
      $error = null;
      // Update tray title with new prayer data
      const next = result.prayers?.find((p: any) => p.is_next);
      invoke("set_tray_title", { title: next ? `${next.name} ${next.time}` : "" });
    } catch (e: any) {
      $error = e.toString();
    }
    saving = false;
  }

  function cancel() {
    $showSettings = false;
  }
</script>

<div class="overlay" onclick={cancel} role="presentation">
  <div
    class="settings-panel"
    onclick={(e) => e.stopPropagation()}
    onkeydown={(e) => e.key === "Escape" && cancel()}
    role="dialog"
    tabindex="-1"
  >
    <h2>Settings</h2>

    <div class="field">
      <label for="city">City</label>
      <div class="autocomplete-wrapper">
        <input
          bind:this={cityInput}
          id="city"
          type="text"
          value={searchInput}
          oninput={handleCityInput}
          onfocus={handleFocus}
          onblur={handleBlur}
          placeholder="Search city..."
          autocomplete="off"
        />
        {#if showSuggestions && filteredCities.length > 0}
          <div class="suggestions">
            {#each filteredCities.slice(0, 10) as city}
              <button
                type="button"
                class="suggestion-item"
                onclick={() => selectCity(city)}
              >
                {city}
              </button>
            {/each}
          </div>
        {/if}
      </div>
    </div>

    <div class="field">
      <label for="notify">Notify Before (minutes)</label>
      <input
        id="notify"
        type="number"
        bind:value={localConfig.notify_before_mins}
        min="1"
        max="60"
      />
    </div>

    <div class="buttons">
      <button class="btn-cancel" onclick={cancel}>Cancel</button>
      <button class="btn-save" onclick={save} disabled={saving}>
        {saving ? "Saving..." : "Save & Apply"}
      </button>
    </div>
  </div>
</div>

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.7);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
  }

  .settings-panel {
    background: #0a0a0a;
    border: 1px solid #1a1a1a;
    border-radius: 12px;
    padding: 20px;
    width: 340px;
    max-height: 90vh;
    overflow-y: auto;
  }

  h2 {
    font-size: 0.9rem;
    margin-bottom: 16px;
    color: #ffffff;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .field {
    margin-bottom: 14px;
    position: relative;
  }

  .autocomplete-wrapper {
    position: relative;
  }

  label {
    display: block;
    font-size: 0.65rem;
    color: #888888;
    margin-bottom: 6px;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    font-weight: 600;
  }

  input {
    width: 100%;
    padding: 8px 12px;
    border-radius: 6px;
    border: 1px solid #1a1a1a;
    background: #0a0a0a;
    color: #ffffff;
    font-size: 0.75rem;
    outline: none;
    transition: border-color 0.2s;
    box-sizing: border-box;
  }

  input:focus {
    border-color: #7c3aed;
  }

  .suggestions {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    max-height: 180px;
    overflow-y: auto;
    background: #0a0a0a;
    border: 1px solid #1a1a1a;
    border-top: none;
    border-radius: 0 0 6px 6px;
    z-index: 10;
    margin-top: -6px;
    padding-top: 6px;
  }

  .suggestion-item {
    width: 100%;
    padding: 8px 12px;
    text-align: left;
    background: transparent;
    border: none;
    color: #ffffff;
    font-size: 0.75rem;
    cursor: pointer;
    transition: background-color 0.15s;
  }

  .suggestion-item:hover {
    background: #1a1a1a;
  }

  .buttons {
    display: flex;
    gap: 8px;
    margin-top: 20px;
  }

  button {
    flex: 1;
    padding: 8px;
    border-radius: 6px;
    border: none;
    font-weight: 600;
    font-size: 0.7rem;
    cursor: pointer;
    transition: opacity 0.2s;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  button:hover {
    opacity: 0.85;
  }
  button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-cancel {
    background: #1a1a1a;
    color: #888888;
    border: 1px solid #1a1a1a;
  }

  .btn-save {
    background: linear-gradient(135deg, #7c3aed 0%, #a855f7 100%);
    color: #ffffff;
  }
</style>
