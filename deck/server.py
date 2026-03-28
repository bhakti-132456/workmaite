import http.server
import socketserver
import json
import sqlite3
import os
import urllib.parse
import urllib.request
import time
import socket

PORT = 8080
DB_NAME = "../workmaite_core.db"

class WorkmaiteHandler(http.server.SimpleHTTPRequestHandler):
    def do_GET(self):
        if self.path == '/api/tasks':
            self.send_response(200)
            self.send_header('Content-type', 'application/json')
            self.end_headers()
            
            conn = sqlite3.connect(DB_NAME)
            conn.row_factory = sqlite3.Row
            c = conn.cursor()
            c.execute("SELECT * FROM tasks ORDER BY id DESC")
            rows = c.fetchall()
            tasks = [dict(row) for row in rows]
            conn.close()
            
            self.wfile.write(json.dumps(tasks).encode())
        elif self.path == '/api/status':
            self.send_response(200)
            self.send_header('Content-type', 'application/json')
            self.end_headers()
            
            # Simple check if llama is responding
            import socket
            sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            sock.settimeout(0.5)
            result = sock.connect_ex(('127.0.0.1', 8081))
            status = "idle" if result == 0 else "off"
            sock.close()
            
            self.wfile.write(json.dumps({"status": status}).encode())
        else:
            return super().do_GET()

    def do_POST(self):
        if self.path == '/api/tasks':
            content_length = int(self.headers['Content-Length'])
            post_data = self.rfile.read(content_length)
            data = json.loads(post_data.decode('utf-8'))
            
            title = data.get('title')
            status = data.get('status', 'Backlog')
            priority = data.get('priority', 'Medium')
            
            conn = sqlite3.connect(DB_NAME)
            c = conn.cursor()
            c.execute("INSERT INTO tasks (title, status, priority) VALUES (?, ?, ?)", 
                      (title, status, priority))
            conn.commit()
            task_id = c.last_insert_rowid()
            conn.close()
            
            self.send_response(201)
            self.end_headers()
            self.wfile.write(json.dumps({"id": task_id}).encode())
            
        elif self.path == '/api/vision':
            import sys
            sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), '..')))
            import core.resource_manager as rm
            import base64
            
            content_length = int(self.headers['Content-Length'])
            post_data = self.rfile.read(content_length)
            request_data = json.loads(post_data.decode('utf-8'))
            
            prompt = request_data.get('prompt', 'Describe this image.')
            image_base64 = request_data.get('image')
            
            # 1. Swap to Vision model
            rm.start_llama('vision')
            time.sleep(5) # Give it some time to load
            
            # 2. Call Llama Server Vision API
            try:
                vision_data = json.dumps({
                    "prompt": f"USER:[image]\n{prompt}\nASSISTANT:",
                    "n_predict": 256,
                    "image_data": [{"data": image_base64, "id": 10}]
                }).encode('utf-8')
                
                req = urllib.request.Request("http://127.0.0.1:8081/completion", data=vision_data, 
                                            headers={'Content-Type': 'application/json'})
                with urllib.request.urlopen(req, timeout=60) as response:
                    res_body = response.read().decode('utf-8')
                    res_json = json.loads(res_body)
                    answer = res_json.get('content', '')
            except Exception as e:
                answer = f"Vision Error: {str(e)}"
                
            self.send_response(200)
            self.send_header('Content-type', 'application/json')
            self.end_headers()
            self.wfile.write(json.dumps({"answer": answer}).encode())

        elif self.path == '/api/launch':
            import sys
            sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), '..')))
            import tools.app_control as ac
            
            content_length = int(self.headers['Content-Length'])
            post_data = self.rfile.read(content_length)
            data = json.loads(post_data.decode('utf-8'))
            app_name = data.get('app')
            
            result = ac.launch_app(app_name)
            
            self.send_response(200)
            self.send_header('Content-type', 'application/json')
            self.end_headers()
            self.wfile.write(json.dumps(result).encode())

    def do_PATCH(self):
        if self.path.startswith('/api/tasks/'):
            task_id = self.path.split('/')[-1]
            content_length = int(self.headers['Content-Length'])
            patch_data = self.rfile.read(content_length)
            data = json.loads(patch_data.decode('utf-8'))
            
            status = data.get('status')
            
            conn = sqlite3.connect(DB_NAME)
            c = conn.cursor()
            c.execute("UPDATE tasks SET status = ? WHERE id = ?", (status, task_id))
            conn.commit()
            conn.close()
            
            self.send_response(200)
            self.end_headers()
            self.wfile.write(b'{"status": "ok"}')

if __name__ == "__main__":
    os.chdir(os.path.dirname(os.path.abspath(__file__)))
    with socketserver.TCPServer(("", PORT), WorkmaiteHandler) as httpd:
        print(f"Deck serving at http://localhost:{PORT}")
        httpd.serve_forever()
