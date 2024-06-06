import os


# Construct the path to the directory
dir = "../src/instructions"

# Get a list of all files in the directory
entries = os.listdir(dir)

# Print the full path of each file
for entry in entries:
    full_path = dir + "/" + entry
    print(full_path)
