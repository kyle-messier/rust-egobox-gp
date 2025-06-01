import sys
import subprocess

def check_python_version():
    if sys.version_info < (3, 0):
        print("Error: Python 3.0 or later is required.")
        return False
    else:
        print(f"Python version: {sys.version}")
        return True

def check_snakemake():
    try:
        result = subprocess.run(["snakemake", "--version"], 
                                capture_output=True, text=True, check=True)
        print(f"Snakemake version: {result.stdout.strip()}")
        return True
    except subprocess.CalledProcessError:
        print("Error: Snakemake is not installed or not in the system PATH.")
        return False
    except FileNotFoundError:
        print("Error: Snakemake is not installed or not in the system PATH.")
        return False

if __name__ == "__main__":
    python_ok = check_python_version()
    snakemake_ok = check_snakemake()
    
    if python_ok and snakemake_ok:
        print("Environment check passed. Python 3.0+ and Snakemake are installed.")
    else:
        print("Environment check failed. Please install the required software.")