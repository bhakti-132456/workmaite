<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount, onDestroy } from 'svelte';
  import { fade } from 'svelte/transition';

  export let show = false;

  interface SystemStats {
    cpu_usage: number;
    mem_usage: number;
    gpu_usage: number | null;
    cpu_temp: number | null;
    gpu_temp: number | null;
  }

  let stats: SystemStats = {
    cpu_usage: 0,
    mem_usage: 0,
    gpu_usage: null,
    cpu_temp: null,
    gpu_temp: null
  };

  let interval: any;

  async function updateStats() {
    try {
      stats = await invoke('get_system_stats');
    } catch (e) {
      console.error('Failed to get system stats', e);
    }
  }

  onMount(() => {
    updateStats();
    interval = setInterval(updateStats, 3000);
  });

  onDestroy(() => {
    if (interval) clearInterval(interval);
  });
</script>

{#if show}
<div class="system-monitor" transition:fade>
  <div class="stat">
    <span class="label">CPU</span>
    <span class="value">{stats.cpu_usage.toFixed(1)}%</span>
  </div>
  {#if stats.gpu_usage !== null}
  <div class="stat">
    <span class="label">GPU</span>
    <span class="value">{stats.gpu_usage.toFixed(1)}%</span>
  </div>
  {/if}
  <div class="stat">
    <span class="label">MEM</span>
    <span class="value">{stats.mem_usage.toFixed(1)}%</span>
  </div>
  {#if stats.cpu_temp !== null}
  <div class="stat">
    <span class="label">TEMP</span>
    <span class="value">{stats.cpu_temp.toFixed(0)}°</span>
  </div>
  {/if}
</div>
{/if}

<style>
  .system-monitor {
    display: flex;
    gap: 12px;
    padding: 2px 8px;
    background: rgba(255, 255, 255, 0.05);
    border-radius: 6px;
    font-family: 'JetBrains Mono', 'Courier New', monospace;
    font-size: 10px;
    color: var(--text-muted);
    border: 1px solid rgba(255, 255, 255, 0.1);
    backdrop-filter: blur(4px);
  }

  .stat {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .label {
    opacity: 0.5;
    text-transform: uppercase;
  }

  .value {
    color: var(--accent);
    font-weight: bold;
  }
</style>
