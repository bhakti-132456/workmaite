import os
import shutil

def edit_file(path, content, mode='w'):
    """Safe editing of local files."""
    try:
        with open(path, mode) as f:
            f.write(content)
        return {"status": "success", "path": os.path.abspath(path)}
    except Exception as e:
        return {"error": str(e)}

def move_file(src, dst):
    """Safe moving of local files."""
    try:
        shutil.move(src, dst)
        return {"status": "success", "src": src, "dst": dst}
    except Exception as e:
        return {"error": str(e)}

def list_files(directory='.'):
    """List directory contents."""
    try:
        return os.listdir(directory)
    except Exception as e:
        return {"error": str(e)}
