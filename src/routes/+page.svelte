<script lang="ts">
  import GhostOverlay from '$lib/components/GhostOverlay.svelte';
  import PersonalityToggle from '$lib/components/PersonalityToggle.svelte';
  import ChatPanel from '$lib/components/ChatPanel.svelte';
  import FocusTimer from '$lib/components/FocusTimer.svelte';
  import SystemMonitor from '$lib/components/SystemMonitor.svelte';
  import type { Personality } from '$lib/components/PersonalityToggle.svelte';

  let currentPersonality: Personality = 'guide';
  let showMonitor = true;

  function handlePersonalityChange(e: CustomEvent<Personality>) {
    currentPersonality = e.detail;
  }
</script>

<GhostOverlay>
  <div class="top-bar">
    <div class="left-controls">
      <PersonalityToggle on:change={handlePersonalityChange} active={currentPersonality} />
      <SystemMonitor show={showMonitor} />
    </div>
    
    <div class="right-controls">
      <button 
        class="monitor-toggle {showMonitor ? 'active' : ''}" 
        on:click={() => showMonitor = !showMonitor}
        title="Toggle System Monitor"
      >
        <div class="monitor-icon">
          <span></span>
          <span></span>
          <span></span>
        </div>
      </button>
      <FocusTimer activePersonality={currentPersonality} />
    </div>
  </div>
  
  <div class="chat-container">
    <ChatPanel personality={currentPersonality} />
  </div>
</GhostOverlay>

<style>
  :global(body) {
    background-color: transparent !important;
  }

  .top-bar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 8px 16px;
    border-bottom: 1px solid var(--border-light);
    background: rgba(0, 0, 0, 0.2);
    gap: 12px;
  }

  .left-controls, .right-controls {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .monitor-toggle {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 8px;
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.2s;
    color: var(--text-muted);
  }

  .monitor-toggle:hover {
    background: rgba(255, 255, 255, 0.1);
  }

  .monitor-toggle.active {
    background: var(--active-color, var(--accent));
    border-color: transparent;
    color: white;
  }

  .monitor-icon {
    display: flex;
    align-items: flex-end;
    gap: 2px;
    height: 12px;
  }

  .monitor-icon span {
    width: 3px;
    background: currentColor;
    border-radius: 1px;
    opacity: 0.6;
  }

  .monitor-toggle.active .monitor-icon span {
    opacity: 1;
  }

  .monitor-icon span:nth-child(1) { height: 60%; }
  .monitor-icon span:nth-child(2) { height: 90%; }
  .monitor-icon span:nth-child(3) { height: 40%; }

  .chat-container {
    flex: 1;
    overflow: hidden;
  }
</style>
