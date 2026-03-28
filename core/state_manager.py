import sqlite3
import json
import os
import re
import time
from datetime import datetime

# Configuration
DB_NAME = "workmaite_core.db"
WATCH_FILE = "current_context.md"
POLL_INTERVAL = 2  # seconds

def init_db():
    conn = sqlite3.connect(DB_NAME)
    c = conn.cursor()
    
    # Create chat_history table
    c.execute('''
        CREATE TABLE IF NOT EXISTS chat_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
            role TEXT NOT NULL,
            message TEXT NOT NULL,
            tokens INTEGER DEFAULT 0
        )
    ''')
    
    # Create tasks table
    c.execute('''
        CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            status TEXT DEFAULT 'Backlog',
            priority TEXT DEFAULT 'Medium',
            subtasks_json TEXT DEFAULT '[]',
            context_link TEXT
        )
    ''')
    
    conn.commit()
    conn.close()
    print(f"Initialized {DB_NAME}")

def cleanup_tasks_with_llm(content):
    """Call the local Llama model to clean up the task list into JSON format."""
    import urllib.request
    
    prompt = f"Extract all actionable tasks from this text into a clean JSON list of strings. Text: {content}\nJSON Output:"
    data = json.dumps({
        "prompt": prompt,
        "n_predict": 256,
        "temperature": 0.1,
    }).encode('utf-8')
    
    try:
        req = urllib.request.Request("http://127.0.0.1:8081/completion", data=data, 
                                    headers={'Content-Type': 'application/json'})
        with urllib.request.urlopen(req, timeout=10) as response:
            res_body = response.read().decode('utf-8')
            res_json = json.loads(res_body)
            # Try to find JSON list in response
            response_text = res_json.get('content', '')
            # Very basic extraction if LLM outputs markdown
            json_match = re.search(r'\[.*\]', response_text, re.DOTALL)
            if json_match:
                return json.loads(json_match.group(0))
    except Exception as e:
        print(f"LLM cleanup failed: {e}")
    return []

def index_context(text):
    """Indices raw text from context into the database as records if needed."""
    if not text.strip():
        return
        
    conn = sqlite3.connect(DB_NAME)
    c = conn.cursor()
    
    # 1. Regex-based Task extraction
    tasks_found = []
    
    # Pattern 1: Markdown checkboxes
    checkbox_matches = re.findall(r'- \[ \] (.*)', text)
    for title in checkbox_matches:
        tasks_found.append(title.strip())
        
    # Pattern 2: TODO lines
    todo_matches = re.findall(r'TODO: (.*)', text, re.IGNORECASE)
    for title in todo_matches:
        tasks_found.append(title.strip())
    
    # 2. LLM Cleanup
    llm_tasks = cleanup_tasks_with_llm(text)
    tasks_found.extend([t for t in llm_tasks if t not in tasks_found])
    
    # Insert new tasks into DB if they don't exist
    for task_title in tasks_found:
        if not task_title or len(task_title) < 3: continue
        c.execute("SELECT id FROM tasks WHERE title = ?", (task_title,))
        if not c.fetchone():
            c.execute("INSERT INTO tasks (title, status) VALUES (?, ?)", (task_title, 'Backlog'))
            print(f"New task indexed: {task_title}")
            
    conn.commit()
    conn.close()

def watch_context_file():
    if not os.path.exists(WATCH_FILE):
        with open(WATCH_FILE, "w") as f:
            f.write("# WorkmAIte Context\nPaste context or tasks here...")
            
    last_mtime = os.path.getmtime(WATCH_FILE)
    print(f"Watching {WATCH_FILE}...")
    
    try:
        while True:
            time.sleep(POLL_INTERVAL)
            current_mtime = os.path.getmtime(WATCH_FILE)
            if current_mtime > last_mtime:
                print(f"Context updated! Re-indexing...")
                with open(WATCH_FILE, "r") as f:
                    content = f.read()
                    index_context(content)
                last_mtime = current_mtime
    except KeyboardInterrupt:
        print("Stopping watcher...")

if __name__ == "__main__":
    init_db()
    watch_context_file()
