<script lang="ts">
  import { onMount, afterUpdate } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  
  export let personality = 'guide';
  
  interface Message {
    role: 'user' | 'assistant';
    content: String;
  }
  
  let messages: Message[] = [];
  let input = '';
  let isLoading = false;
  let isRecording = false;
  let mediaRecorder: MediaRecorder | null = null;
  let audioChunks: Blob[] = [];
  let chatContainer: HTMLElement;
  
  async function sendMessage() {
    if (!input.trim() || isLoading) return;
    
    const userMsg = input.trim();
    messages = [...messages, { role: 'user', content: userMsg }];
    input = '';
    isLoading = true;
    
    try {
      // Save user message to DB
      await invoke('save_message', { 
        role: 'user', 
        content: userMsg, 
        personality 
      });
      
      // Send to LLM sidecar
      const response = await invoke<string>('send_chat', {
        prompt: userMsg,
        personality
      });
      
      messages = [...messages, { role: 'assistant', content: response }];
      
      // Save assistant message to DB
      await invoke('save_message', { 
        role: 'assistant', 
        content: response, 
        personality 
      });
      
    } catch (err) {
      console.error(err);
      messages = [...messages, { 
        role: 'assistant', 
        content: `Error: ${err}` 
      }];
    } finally {
      isLoading = false;
    }
    
    // Auto-TTS for the response
    if (messages[messages.length - 1].role === 'assistant') {
      await speakResponse(messages[messages.length - 1].content);
    }
  }

  async function speakResponse(text: string) {
    try {
      const b64 = await invoke<string>('speak_text', { text });
      const audio = new Audio(`data:audio/wav;base64,${b64}`);
      audio.play();
    } catch (err) {
      console.error("TTS failing:", err);
    }
  }

  async function toggleRecording() {
    if (isRecording) {
      mediaRecorder?.stop();
      isRecording = false;
    } else {
      try {
        const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
        mediaRecorder = new MediaRecorder(stream);
        audioChunks = [];

        mediaRecorder.ondataavailable = (event) => {
          audioChunks.push(event.data);
        };

        mediaRecorder.onstop = async () => {
          const audioBlob = new Blob(audioChunks, { type: 'audio/wav' }); // Note: format depends on browser
          const reader = new FileReader();
          reader.readAsDataURL(audioBlob);
          reader.onloadend = async () => {
            const base64data = (reader.result as string).split(',')[1];
            isLoading = true;
            try {
              const transcribed = await invoke<string>('transcribe_audio', { audioBase64: base64data });
              if (transcribed.trim()) {
                input = transcribed;
                sendMessage();
              }
            } catch (err) {
              console.error("STT Error:", err);
            } finally {
              isLoading = false;
            }
          };
        };

        mediaRecorder.start();
        isRecording = true;
      } catch (err) {
        console.error("Microphone access failed:", err);
      }
    }
  }
  
  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      sendMessage();
    }
  }
  
  afterUpdate(() => {
    if (chatContainer) {
      chatContainer.scrollTop = chatContainer.scrollHeight;
    }
  });
</script>

<div class="chat-panel">
  <div class="messages" bind:this={chatContainer}>
    {#if messages.length === 0}
      <div class="empty-state animate-fade-in">
        <div class="icon">✨</div>
        <p>I am your <strong>WorkmAIte</strong>.</p>
        <p class="sub">100% offline. Privacy-first. Powered by local AI.</p>
      </div>
    {/if}
    
    {#each messages as msg}
      <div class="message {msg.role} animate-fade-in">
        <div class="bubble">
          {msg.content}
        </div>
      </div>
    {/each}
    
    {#if isLoading}
      <div class="message assistant tracking-in">
        <div class="bubble typing">
          <span class="dot"></span>
          <span class="dot"></span>
          <span class="dot"></span>
        </div>
      </div>
    {/if}
  </div>
  
  <div class="input-area">
    <textarea 
      bind:value={input} 
      on:keydown={handleKeydown}
      placeholder="Ask {personality} something..."
      rows="1"
      disabled={isLoading}
    ></textarea>
    <button 
      class="mic-btn {isRecording ? 'recording' : ''}" 
      on:click={toggleRecording} 
      title={isRecording ? 'Stop recording' : 'Start voice chat'}
    >
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M12 1a3 3 0 0 0-3 3v8a3 3 0 0 0 6 0V4a3 3 0 0 0-3-3z"></path>
        <path d="M19 10v2a7 7 0 0 1-14 0v-2"></path>
        <line x1="12" y1="19" x2="12" y2="23"></line>
        <line x1="8" y1="23" x2="16" y2="23"></line>
      </svg>
      {#if isRecording}
        <span class="pulse"></span>
      {/if}
    </button>
    
    <button on:click={sendMessage} disabled={!input.trim() || isLoading}>
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <line x1="22" y1="2" x2="11" y2="13"></line>
        <polygon points="22 2 15 22 11 13 2 9 22 2"></polygon>
      </svg>
    </button>
  </div>
</div>

<style>
  .chat-panel {
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
  }
  
  .messages {
    flex: 1;
    overflow-y: auto;
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }
  
  .empty-state {
    margin: auto;
    text-align: center;
    color: var(--text-muted);
  }
  
  .empty-state .icon {
    font-size: 32px;
    margin-bottom: 8px;
    opacity: 0.8;
  }
  
  .empty-state .sub {
    font-size: 11px;
    opacity: 0.6;
    margin-top: 4px;
  }

  .message {
    display: flex;
    width: 100%;
  }
  
  .message.user {
    justify-content: flex-end;
  }
  
  .message.assistant {
    justify-content: flex-start;
  }
  
  .bubble {
    max-width: 85%;
    padding: 10px 14px;
    border-radius: 16px;
    font-size: 13px;
    line-height: 1.5;
    word-wrap: break-word;
    white-space: pre-wrap;
  }
  
  .user .bubble {
    background: rgba(255, 255, 255, 0.1);
    color: #fff;
    border-bottom-right-radius: 4px;
  }
  
  .assistant .bubble {
    background: rgba(0, 0, 0, 0.3);
    color: var(--text-main);
    border: 1px solid var(--border-light);
    border-bottom-left-radius: 4px;
  }
  
  .input-area {
    padding: 12px;
    background: rgba(0, 0, 0, 0.2);
    border-top: 1px solid var(--border-light);
    display: flex;
    gap: 8px;
    align-items: flex-end;
  }
  
  textarea {
    flex: 1;
    background: transparent;
    border: none;
    color: var(--text-main);
    font-family: inherit;
    font-size: 13px;
    resize: none;
    outline: none;
    padding: 8px 0;
    max-height: 100px;
  }
  
  textarea::placeholder {
    color: var(--text-muted);
  }
  
  button {
    background: var(--active-color);
    color: #121217;
    border: none;
    border-radius: 50%;
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.2s;
    transition-property: transform, opacity;
  }
  
  button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  
  button:not(:disabled):hover {
    transform: scale(1.05);
  }

  .mic-btn {
    background: transparent;
    border: 1px solid var(--border-light);
    color: var(--text-muted);
    position: relative;
    overflow: visible;
  }

  .mic-btn.recording {
    background: #ff4d4d;
    border-color: transparent;
    color: white;
  }

  .pulse {
    position: absolute;
    width: 100%;
    height: 100%;
    border-radius: 50%;
    background: rgba(255, 77, 77, 0.4);
    animation: pulse 1.5s infinite;
  }

  @keyframes pulse {
    0% { transform: scale(1); opacity: 0.8; }
    100% { transform: scale(1.8); opacity: 0; }
  }
  
  /* Typing Indicator */
  .typing {
    display: flex;
    gap: 4px;
    align-items: center;
    padding: 14px 16px;
  }
  
  .dot {
    width: 5px;
    height: 5px;
    background: var(--text-muted);
    border-radius: 50%;
    animation: bounce 1.4s infinite ease-in-out both;
  }
  
  .dot:nth-child(1) { animation-delay: -0.32s; }
  .dot:nth-child(2) { animation-delay: -0.16s; }
  
  @keyframes bounce {
    0%, 80%, 100% { transform: scale(0); }
    40% { transform: scale(1); }
  }
</style>
