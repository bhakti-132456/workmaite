<script lang="ts" module>
  export type Personality = 'captain' | 'guide' | 'scholar' | 'mirror';
</script>

<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  
  export let active: Personality = 'guide';
  
  const dispatch = createEventDispatcher();
  
  const personalities = [
    { id: 'captain', name: 'CAPTAIN', color: 'var(--color-captain)' },
    { id: 'guide', name: 'GUIDE', color: 'var(--color-guide)' },
    { id: 'scholar', name: 'SCHOLAR', color: 'var(--color-scholar)' },
    { id: 'mirror', name: 'MIRROR', color: 'var(--color-mirror)' },
  ] as const;
  
  function select(p: Personality) {
    active = p;
    document.documentElement.style.setProperty('--active-color', `var(--color-${p})`);
    dispatch('change', p);
  }
</script>

<div class="toggles">
  {#each personalities as p}
    <button 
      class="toggle {active === p.id ? 'active' : ''}"
      style="--p-color: {p.color}"
      on:click={() => select(p.id)}
      title={p.name}
    >
      <div class="dot"></div>
      {#if active === p.id}
        <span class="label animate-fade-in">{p.name}</span>
      {/if}
    </button>
  {/each}
</div>

<style>
  .toggles {
    display: flex;
    gap: 8px;
    background: rgba(0, 0, 0, 0.2);
    padding: 8px;
    border-radius: 20px;
    border: 1px solid var(--border-light);
  }
  
  .toggle {
    display: flex;
    align-items: center;
    gap: 6px;
    background: transparent;
    border: none;
    border-radius: 16px;
    padding: 6px;
    cursor: pointer;
    transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
  }
  
  .toggle:hover {
    background: rgba(255, 255, 255, 0.05);
  }
  
  .toggle.active {
    background: rgba(255, 255, 255, 0.1);
    padding: 6px 12px 6px 6px;
  }
  
  .dot {
    width: 12px;
    height: 12px;
    border-radius: 50%;
    background: var(--p-color);
    opacity: 0.5;
    transition: all 0.3s;
  }
  
  .toggle.active .dot {
    opacity: 1;
    box-shadow: 0 0 10px var(--p-color);
  }
  
  .label {
    font-size: 10px;
    font-weight: 700;
    letter-spacing: 0.5px;
    color: var(--text-main);
  }
</style>
