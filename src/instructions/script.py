import os
import re

# Directory containing the files
dir = os.getcwd() + "/nes_emulator/src/instructions"

# Regular expression to match the opcode lines
op_regex = r"^(?P<OpCode>\w+): (?P<Description>.+)"

# Dictionary to store opcodes and their descriptions
op_dict = {}


def write_to_file(function_dict, op_dict, file_path):
    file_name = file_path.split("/")[-1]

    with open(file_path, "a") as f:
        file_ops = op_dict.get(file_name)
        if file_ops:
            for op in file_ops:
                if op in function_dict:
                    f.write(function_dict[op])
            f.write("\n")


def extract_functions(file_content, function_names):
    function_dict = {}
    fn_regex = r"(pub\s+)?fn\s+(?P<FunctionName>\w+)\s*\([^)]*\)\s*{"
    matches = re.finditer(fn_regex, file_content)

    for match in matches:
        function_name = match.group("FunctionName")
        if function_name in function_names:
            start_index = match.start()
            open_braces = 0
            total_braces = 0

            for i in range(start_index, len(file_content)):
                if file_content[i] == "{":
                    open_braces += 1
                    total_braces += 1
                elif file_content[i] == "}":
                    open_braces -= 1
                    total_braces += 1
                if open_braces == 0 and total_braces > 0:
                    end_index = i + 1
                    break

            function_body = file_content[start_index:end_index]
            function_dict[function_name] = function_body
    return function_dict


# Iterate over the files in the directory
with os.scandir(dir) as entries:
    for entry in entries:
        if entry.is_file():
            with open(entry.path, "r") as f:
                file_name = entry.name.split("/")[-1]
                for line in f:
                    # Search for the opcode pattern in the line
                    match = re.search(op_regex, line)
                    if match:
                        op_code = match.group("OpCode")
                        if file_name not in op_dict:
                            op_dict[file_name] = [op_code.lower()]
                        else:
                            op_dict[file_name].append(op_code.lower())

function_names = [value for values in op_dict.values() for value in values]

function_dict = extract_functions(
    open(os.getcwd() + "/nes_emulator/src/cpu/cpu.rs").read(), function_names
)

with os.scandir(dir) as entries:
    for entry in entries:
        if entry.is_file():
            write_to_file(function_dict, op_dict, entry.path)
