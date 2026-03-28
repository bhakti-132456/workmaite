<script lang="ts">
  import { onMount } from 'svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  
  const appWindow = getCurrentWindow();
</script>

<!-- Custom titlebar for dragging -->
<div data-tauri-drag-region class="titlebar draggable">
  <div class="brand">workmAIte</div>
  <div class="controls">
    <button on:click={() => appWindow.minimize()} class="titlebar-button">_</button>
    <button on:click={() => appWindow.close()} class="titlebar-button close">×</button>
  </div>
</div>

<div class="ghost-overlay glass-panel">
  <slot></slot>
</div>

<style>
  .titlebar {
    height: 32px;
    background: rgba(0,0,0,0.1);
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0 12px;
    user-select: none;
    -webkit-user-select: none;
    cursor: move;
  }
  
  .brand {
    font-size: 12px;
    font-weight: 600;
    letter-spacing: 0.5px;
    color: var(--text-muted);
    font-variant: small-caps;
    pointer-events: none;
  }
  
  .controls {
    display: flex;
    gap: 8px;
  }
  
  .titlebar-button {
    background: transparent;
    border: none;
    color: var(--text-muted);
    font-size: 14px;
    cursor: pointer;
    width: 24px;
    height: 24px;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background 0.2s;
  }
  
  .titlebar-button:hover {
    background: rgba(255, 255, 255, 0.1);
    color: var(--text-main);
  }
  
  .titlebar-button.close:hover {
    background: rgba(235, 87, 87, 0.8);
  }

  .ghost-overlay {
    height: calc(100vh - 48px);
    margin: 0 8px 8px 8px;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }
</style>
