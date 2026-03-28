import sqlite3
import json
import urllib.request
import re

DB_NAME = "workmaite_core.db"
LLAMA_URL = "http://127.0.0.1:8081/completion"
PRUNE_THRESHOLD = 10

def get_recent_chat(limit=PRUNE_THRESHOLD):
    conn = sqlite3.connect(DB_NAME)
    conn.row_factory = sqlite3.Row
    c = conn.cursor()
    c.execute("SELECT * FROM chat_history ORDER BY id DESC LIMIT ?", (limit,))
    rows = c.fetchall()
    conn.close()
    return [dict(row) for row in rows]

def summarize_history(messages):
    """Call the local LLM to summarize previous chat history."""
    if not messages: return ""
    
    chat_text = "\n".join([f"{m['role']}: {m['message']}" for m in reversed(messages)])
    prompt = f"Summarize the following chat history into 1 concise paragraph for long-term memory:\n\n{chat_text}\n\nSummary:"
    
    data = json.dumps({
        "prompt": prompt,
        "n_predict": 128,
        "temperature": 0.1,
        "stop": ["\n"]
    }).encode('utf-8')
    
    try:
        req = urllib.request.Request(LLAMA_URL, data=data, 
                                    headers={'Content-Type': 'application/json'})
        with urllib.request.urlopen(req, timeout=30) as response:
            res_body = response.read().decode('utf-8')
            res_json = json.loads(res_body)
            return res_json.get('content', '').strip()
    except Exception as e:
        print(f"Summarization failed: {e}")
    return ""

def prune():
    """Main pruning logic: Summarize and Archive."""
    conn = sqlite3.connect(DB_NAME)
    c = conn.cursor()
    
    # Check count
    c.execute("SELECT COUNT(*) FROM chat_history")
    count = c.fetchone()[0]
    
    if count >= PRUNE_THRESHOLD:
        print(f"Chat history ({count}) exceeds threshold. Pruning...")
        
        recent = get_recent_chat(PRUNE_THRESHOLD)
        summary = summarize_history(recent)
        
        if summary:
            # 1. Archive the summary as a system message
            c.execute("INSERT INTO chat_history (role, message) VALUES (?, ?)", 
                      ("system", f"[MEMORY_ARCHIVE]: {summary}"))
            
            # 2. Delete the old messages that were summarized
            # (In a real app, you might flag them instead of deleting)
            min_id = min([m['id'] for m in recent])
            max_id = max([m['id'] for m in recent])
            c.execute("DELETE FROM chat_history WHERE id >= ? AND id <= ?", (min_id, max_id))
            
            conn.commit()
            print("Successfully pruned chat history and updated memory.")
            
    conn.close()

if __name__ == "__main__":
    prune()
