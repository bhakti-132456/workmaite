<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onDestroy } from 'svelte';
  
  export let activePersonality = 'guide';
  
  let isActive = false;
  let durationMins = 25;
  let timeLeft = 25 * 60; // in seconds
  let timerId: number | null = null;
  let sessionId: number | null = null;
  
  $: progress = 1 - (timeLeft / (durationMins * 60));
  $: strokeDashoffset = 113 - (113 * progress); // 113 is approx circumference of r=18
  
  function formatTime(secs: number) {
    const m = Math.floor(secs / 60);
    const s = secs % 60;
    return `${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`;
  }
  
  async function toggleFocus() {
    if (isActive) {
      // Stop
      isActive = false;
      if (timerId) clearInterval(timerId);
      if (sessionId) {
        await invoke('end_focus_session', { sessionId, notes: 'Ended early' });
      }
      timeLeft = durationMins * 60;
    } else {
      // Start
      isActive = true;
      timeLeft = durationMins * 60;
      
      try {
        sessionId = await invoke<number>('start_focus_session', { 
          durationMins, 
          personality: activePersonality 
        });
        
        timerId = window.setInterval(async () => {
          timeLeft -= 1;
          
          if (timeLeft <= 0) {
            isActive = false;
            clearInterval(timerId!);
            await invoke('end_focus_session', { sessionId, notes: 'Completed full session' });
            timeLeft = durationMins * 60;
            // Here we would trigger the Voice Sidecar TTS check-in
            console.log("Pomodoro complete! Voice check-in triggered.");
          }
        }, 1000);
      } catch (e) {
        console.error("Failed to start session", e);
        isActive = false;
      }
    }
  }
  
  onDestroy(() => {
    if (timerId) clearInterval(timerId);
  });
</script>

<div class="focus-widget {isActive ? 'active' : ''}">
  <button class="ring-button" on:click={toggleFocus} title="Toggle Focus Mode">
    <svg width="40" height="40" viewBox="0 0 40 40">
      <circle 
        cx="20" cy="20" r="18" 
        fill="transparent" 
        stroke="rgba(255,255,255,0.1)" 
        stroke-width="3" 
      />
      
      {#if isActive}
        <circle 
          cx="20" cy="20" r="18" 
          fill="transparent" 
          stroke="var(--active-color)" 
          stroke-width="3" 
          stroke-linecap="round"
          stroke-dasharray="113"
          stroke-dashoffset={strokeDashoffset}
          transform="rotate(-90 20 20)"
          style="transition: stroke-dashoffset 1s linear;"
        />
      {/if}
      
      <circle 
        cx="20" cy="20" r="14" 
        class="inner"
      />
    </svg>
    <div class="time">{isActive ? formatTime(timeLeft) : '25m'}</div>
  </button>
</div>

<style>
  .focus-widget {
    display: flex;
    align-items: center;
    justify-content: center;
  }
  
  .ring-button {
    position: relative;
    background: transparent;
    border: none;
    padding: 0;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
  }
  
  .inner {
    fill: rgba(0, 0, 0, 0.3);
    transition: fill 0.2s;
  }
  
  .ring-button:hover .inner {
    fill: rgba(255, 255, 255, 0.1);
  }
  
  .active .ring-button .inner {
    fill: rgba(var(--active-color-rgb), 0.1);
  }
  
  .active .ring-button {
    animation: pulseGlow 4s infinite;
  }
  
  .time {
    position: absolute;
    font-size: 10px;
    font-weight: 600;
    color: var(--text-main);
    font-variant-numeric: tabular-nums;
  }
</style>
