import os
import subprocess
import platform

# Common Windows paths for Resolve and Studio One
APP_PATHS = {
    "Resolve": [
        r"C:\Program Files\Blackmagic Design\DaVinci Resolve\Resolve.exe"
    ],
    "Studio One": [
        r"C:\Program Files\PreSonus\Studio One 6\Studio One.exe",
        r"C:\Program Files\PreSonus\Studio One 5\Studio One.exe"
    ]
}

def find_app_path(app_name):
    """Try to find the app executable if not in the default path."""
    for path in APP_PATHS.get(app_name, []):
        if os.path.exists(path):
            return path
    return None

def launch_app(app_name):
    """Launch the requested application."""
    if platform.system() != 'Windows':
        return {"error": "App control is currently Windows-only."}
        
    path = find_app_path(app_name)
    if not path:
        return {"error": f"Executable for {app_name} not found in default locations."}
        
    try:
        # Launch using start command to detach properly
        print(f"Launching {app_name} from {path}...")
        subprocess.Popen([path], start_new_session=True, creationflags=subprocess.DETACHED_PROCESS)
        return {"status": "success", "message": f"{app_name} initiated."}
    except Exception as e:
        return {"error": str(e)}

def is_running(app_name):
    """Check if the app process is running."""
    try:
        output = subprocess.check_output('tasklist /FI "IMAGENAME eq Resolve.exe" /FO CSV /NH', shell=True).decode()
        if "Resolve.exe" in output and app_name == "Resolve":
            return True
        output_s1 = subprocess.check_output('tasklist /FI "IMAGENAME eq Studio One.exe" /FO CSV /NH', shell=True).decode()
        if "Studio One.exe" in output_s1 and app_name == "Studio One":
            return True
    except:
        pass
    return False

if __name__ == "__main__":
    # Test
    # print(launch_app("Resolve"))
    pass
