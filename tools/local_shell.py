import subprocess
import os

def run_command(command, shell=True):
    """Securely run a terminal command after approval."""
    try:
        # Use subprocess.run for simple command execution
        result = subprocess.run(
            command,
            shell=shell,
            capture_output=True,
            text=True,
            timeout=30
        )
        return {
            "stdout": result.stdout,
            "stderr": result.stderr,
            "exit_code": result.returncode
        }
    except Exception as e:
        return {"error": str(e)}

if __name__ == "__main__":
    # Test
    # print(run_command("dir"))
    pass
