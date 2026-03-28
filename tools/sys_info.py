import platform
import subprocess
import os
import json

def get_stats():
    """Get system stats without external libraries."""
    stats = {
        "os": platform.system(),
        "release": platform.release(),
        "arch": platform.machine(),
        "cpu_count": os.cpu_count(),
        "ram": "Unknown",
        "gpu": "Unknown"
    }
    
    # Windows-specific wmic commands for RAM and GPU
    if platform.system() == 'Windows':
        try:
            # Get Total RAM
            ram_out = subprocess.check_output("wmic OS get FreePhysicalMemory,TotalVisibleMemorySize /Value", shell=True).decode()
            stats['ram'] = dict(filter(None, (line.split('=') for line in ram_out.splitlines())))
            
            # Get GPU info
            gpu_out = subprocess.check_output("wmic path win32_VideoController get name /Value", shell=True).decode()
            stats['gpu'] = gpu_out.strip().split('=')[-1]
            
        except Exception as e:
            print(f"Stats check failed: {e}")
            
    return stats

if __name__ == "__main__":
    print(json.dumps(get_stats(), indent=2))
