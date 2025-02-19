# scripts/update_source.py
import subprocess
import os
import sys

def run_command(command, repo_path):
    """Runs a command in the terminal and returns the output."""
    try:
        result = subprocess.run(command, cwd=repo_path, text=True, capture_output=True, check=True)
        return result.stdout.strip()
    except subprocess.CalledProcessError as e:
        print(f"Error executing '{' '.join(command)}': {e.stderr.strip()}")
        sys.exit(1)

def check_local_changes(repo_path):
    """Checks if there are uncommitted local changes."""
    status = run_command(["git", "status", "--porcelain"], repo_path)
    if status:
        print("==> There are uncommitted local changes. Commit or stash them before updating.")
        sys.exit(1)

def update_repository(repo_path):
    """Updates the Git repository by pulling the latest changes."""
    print("==> Updating repository...")
    run_command(["git", "pull"], repo_path)
    print("==> Repository updated successfully!")

def show_recent_logs(repo_path):
    """Displays the last 5 commits for reference."""
    print("\n==> Last 5 changes:")
    logs = run_command(["git", "log", "--oneline", "-5"], repo_path)
    print(logs)

def main():
    repo_path = os.path.dirname(os.path.abspath(__file__))  # Assumes script is inside the repo

    print("==> Starting Renux OS update...")

    check_local_changes(repo_path)  # Ensures there are no local changes before updating
    update_repository(repo_path)    # Pulls the latest updates
    show_recent_logs(repo_path)     # Shows a summary of the last commits

    print("\n==> Update complete!")

if __name__ == "__main__":
    main()
