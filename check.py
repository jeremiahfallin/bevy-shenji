import subprocess
import json
import sys

# Using a list for the command arguments
cmd = ["cargo", "check", "--message-format=json"]

# Run the command
process = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, encoding='utf-8', errors='replace')

# Process stdout line by line
for line in process.stdout:
    try:
        msg = json.loads(line)
        if msg.get("reason") == "compiler-message":
            # Print the rendered message
            print(msg["message"]["rendered"])
    except json.JSONDecodeError:
        pass
    except Exception as e:
        print(f"Error processing line: {e}")
