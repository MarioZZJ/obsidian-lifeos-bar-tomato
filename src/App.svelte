<script lang="ts">
  import { onMount } from 'svelte';
  import Timer from './components/Timer.svelte';
  import TaskSelector from './components/TaskSelector.svelte';
  import DailySummary from './components/DailySummary.svelte';
  import Settings from './components/Settings.svelte';
  import { initializeStores, setupEventListeners, vaultPath } from './lib/stores';

  let showSettings = $state(false);
  let initialized = $state(false);
  let needsSetup = $derived(!$vaultPath && initialized);

  onMount(async () => {
    await initializeStores();
    setupEventListeners();
    initialized = true;

    // If no vault configured, show settings
    if (!$vaultPath) {
      showSettings = true;
    }
  });
</script>

<main>
  {#if showSettings || needsSetup}
    <Settings onClose={() => (showSettings = false)} />
  {:else}
    <div class="app-content">
      <Timer />
      <TaskSelector />
      <DailySummary />
      <div class="footer">
        <button class="settings-btn" onclick={() => (showSettings = true)}>
          ⚙️ 设置
        </button>
      </div>
    </div>
  {/if}
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    background: transparent;
    overflow-x: hidden;
  }

  main {
    width: 100%;
    min-height: 480px;
    display: flex;
    flex-direction: column;
    overflow-x: hidden;
    background: transparent;
    border-radius: 12px;
    overflow: hidden;
  }

  .app-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow-x: hidden;
  }

  .footer {
    padding: 8px 16px;
    border-top: 1px solid var(--glass-border);
    display: flex;
    justify-content: center;
  }

  .settings-btn {
    background: none;
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    font-size: 13px;
    padding: 4px 8px;
    transition: color 0.2s;
  }

  .settings-btn:hover {
    color: var(--text-primary);
  }
</style>
