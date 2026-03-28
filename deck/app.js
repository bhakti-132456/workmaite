// Vanilla JS for Kanban & Task Management
// Syncs with the Python server which talk to workmaite_core.db

async function fetchTasks() {
    try {
        const response = await fetch('/api/tasks');
        const tasks = await response.json();
        renderTasks(tasks);
    } catch (err) {
        console.error('Failed to fetch tasks:', err);
    }
}

function renderTasks(tasks) {
    const containers = {
        'Backlog': document.getElementById('tasks-backlog'),
        'Active': document.getElementById('tasks-active'),
        'Done': document.getElementById('tasks-done')
    };

    // Clean existing
    Object.values(containers).forEach(c => c.innerHTML = '');

    tasks.forEach(task => {
        const card = createTaskCard(task);
        const container = containers[task.status] || containers['Backlog'];
        container.appendChild(card);
    });
}

function createTaskCard(task) {
    const card = document.createElement('div');
    card.className = 'task-card';
    card.draggable = true;
    card.id = `task-${task.id}`;
    card.innerHTML = `
        <div style="font-size: 13px; margin-bottom: 5px;">#${task.id}</div>
        <div style="font-weight: bold; margin-bottom: 10px;">${task.title}</div>
        <div class="priority" style="font-size: 10px; opacity: 0.6;">PRIO: ${task.priority}</div>
        <div style="margin-top: 10px; display: flex; align-items: center; gap: 5px; opacity: 0.8; font-size: 11px;">
            <input type="checkbox" ${task.status === 'Done' ? 'checked' : ''} onchange="toggleDone(${task.id}, this.checked)">
            <span>COMPLETE</span>
        </div>
    `;
    card.ondragstart = (e) => drag(e);
    return card;
}

async function addTask() {
    const input = document.getElementById('new-task-title');
    const title = input.value.trim();
    if (!title) return;

    try {
        const response = await fetch('/api/tasks', {
            method: 'POST',
            body: JSON.stringify({ title, status: 'Backlog', priority: 'Medium' }),
            headers: { 'Content-Type': 'application/json' }
        });
        if (response.ok) {
            input.value = '';
            fetchTasks();
        }
    } catch (err) {
        console.error('Add task failed:', err);
    }
}

async function updateTaskStatus(taskId, status) {
    try {
        await fetch(`/api/tasks/${taskId}`, {
            method: 'PATCH',
            body: JSON.stringify({ status }),
            headers: { 'Content-Type': 'application/json' }
        });
        fetchTasks();
    } catch (err) {
        console.error('Status update failed:', err);
    }
}

async function toggleDone(taskId, isChecked) {
    const status = isChecked ? 'Done' : 'Backlog';
    updateTaskStatus(taskId, status);
}

// Drag & Drop
function allowDrop(ev) {
    ev.preventDefault();
}

function drag(ev) {
    ev.dataTransfer.setData("text", ev.target.id);
}

function drop(ev) {
    ev.preventDefault();
    const data = ev.dataTransfer.getData("text");
    const taskIdString = data.split('-')[1];
    const taskId = parseInt(taskIdString);
    
    // Find target column status based on the drop target ID
    let target = ev.target;
    while (target && !target.id.startsWith('tasks-')) {
        target = target.parentElement;
    }
    
    if (target && target.id) {
        const statusMap = {
            'tasks-backlog': 'Backlog',
            'tasks-active': 'Active',
            'tasks-done': 'Done'
        };
        const status = statusMap[target.id];
        if (status) {
            updateTaskStatus(taskId, status);
        }
    }
}

async function launchApp(appName) {
    try {
        const response = await fetch('/api/launch', {
            method: 'POST',
            body: JSON.stringify({ app: appName }),
            headers: { 'Content-Type': 'application/json' }
        });
        const data = await response.json();
        if (data.status === 'success') {
            alert(`[SYS_INFO]: ${appName} launched.`);
        } else {
            alert(`[SYS_ERR]: ${data.error}`);
        }
    } catch (err) {
        console.error('Launch failed:', err);
    }
}

document.getElementById('image-input').onchange = function(e) {
    const reader = new FileReader();
    reader.onload = function(re) {
        document.getElementById('preview-img').src = re.target.result;
        document.getElementById('image-preview').style.display = 'block';
    }
    reader.readAsDataURL(e.target.files[0]);
};

async function analyzeImage() {
    const input = document.getElementById('image-input');
    const prompt = document.getElementById('vision-prompt').value.trim() || "What is in this image?";
    const results = document.getElementById('vision-results');
    
    if (!input.files[0]) return alert("Please select an image first.");
    
    results.innerHTML = "ANALYZING...";
    
    // Read image as base64
    const reader = new FileReader();
    reader.onload = async function() {
        const base64Image = reader.result.split(',')[1];
        
        try {
            const response = await fetch('/api/vision', {
                method: 'POST',
                body: JSON.stringify({ 
                    image: base64Image,
                    prompt: prompt
                }),
                headers: { 'Content-Type': 'application/json' }
            });
            const data = await response.json();
            results.innerHTML = `<div style="color: #0F0; margin-bottom: 5px;">> ${prompt}</div><div>${data.answer || "No response."}</div>`;
        } catch (err) {
            results.innerHTML = "SCAN_FAILED: " + err;
        }
    };
    reader.readAsDataURL(input.files[0]);
}

async function updateStatus() {
    try {
        const response = await fetch('/api/status');
        const data = await response.json();
        const pulse = document.getElementById('pulse');
        const statusText = document.querySelector('header div:last-child');
        
        if (data.status === 'idle') {
            pulse.style.backgroundColor = '#0F0';
            pulse.style.boxShadow = '0 0 8px #0F0';
            statusText.childNodes[2].textContent = ' SYS_READY';
        } else {
            pulse.style.backgroundColor = '#FA0'; // Amber
            pulse.style.boxShadow = '0 0 8px #FA0';
            statusText.childNodes[2].textContent = ' SYS_LOADING';
        }
    } catch (err) {}
}

// Initial fetch
fetchTasks();
updateStatus();
// Polling for updates from state_manager
setInterval(() => {
    fetchTasks();
    updateStatus();
}, 5000);
