import re
import markdown
import os
import sys
import yaml
import json

# We don't read the index
IGNORED_FILES = "index.md"



# Builds docs
def build_docs(script_path, result):
    """
    Read all .md files
    """

    browse_files(script_path+"/../api/python/", result)

# Add docs in result for one file
def add_doc(file_name, result):
    details_file = open(file_name, "r")

    ignore_pattern = re.compile("#.*#.*")

    start_body_pattern = re.compile("{%\s*apibody\s*%}\s*")
    end_body_pattern = re.compile("{%\s*endapibody\s*%}\s*")
    parsing_body = False
    current_body = ""


    limiter_yaml = re.compile('---\s*')
    is_yaml = False
    yaml_header = ""

    current_description = ""

    example_pattern = re.compile("(__Example:__.*)|(__Example__:.*)")
    example_code_start_pattern = re.compile("```py")
    example_code_end_pattern = re.compile("```")
    parsing_example = False
    parsing_example_code = False
    examples = []
    current_example = {
        "description": "",
        "code": ""
    }


    for line in details_file:
        # Ignore titles (h1 tags)
        if ignore_pattern.match(line) != None:
            continue

        if limiter_yaml.match(line) != None:
            # We ignore the yaml header
            if is_yaml == False:
                is_yaml = True
            else:
                is_yaml = False
        elif is_yaml == True:
            yaml_header += line
        elif is_yaml == False:
            # We are not parsing yaml
            if start_body_pattern.match(line) != None:
                parsing_body = True
            elif end_body_pattern.match(line) != None:
                parsing_body = False
            else:
                if parsing_body == True:
                    current_body += line.strip()
                else:
                    # We are not parsing the yaml or the body
                    if parsing_example == True:
                        if example_code_start_pattern.match(line) != None:
                            current_example["code"] += line
                            parsing_example_code = True
                        elif example_code_end_pattern.match(line) != None:
                            parsing_example = False
                            parsing_example_code = False
                            current_example["code"] += line
                        else:
                            if parsing_example_code == True:
                                current_example["code"] += line
                            else:
                                current_example["description"] += line
                    else:
                        # We are not parsing the yaml or the body or an example
                        if example_pattern.match(line) != None:
                            parsing_example = True
                            if current_example["description"] != "":
                                current_example["description"] = markdown.markdown(current_example["description"])
                                current_example["code"] = current_example["code"].replace('```py', '').replace('```', '').strip()
                                examples.append(current_example)
                                current_example = {
                                    "description": "",
                                    "code": ""
                                }
                            current_example["description"] += line
                        else:
                            # So we are parsing the description of the method
                            current_description += line

    current_example["description"] = markdown.markdown(current_example["description"])
    current_example["code"] = current_example["code"].replace('```py', '').replace('```', '').strip()

    examples.append(current_example)

    yaml_data = yaml.load(yaml_header)
    details_file.close()

    # Reading the JS file to extract the io data
    file_name = file_name.replace('python', 'javascript')
    try:
        details_file_js = open(file_name, "r")

        yaml_header_js = ""
        for line in details_file_js:
            if limiter_yaml.match(line) != None:
                # We ignore the yaml header
                if is_yaml == False:
                    is_yaml = True
                else:
                    break
            elif is_yaml == True:
                yaml_header_js += line

        yaml_data_js = yaml.load(yaml_header_js)
        result[yaml_data["command"]] = {
            "name": yaml_data["command"],
            "description": current_description,
            "examples": examples,
            "io": yaml_data_js["io"]
        }
        details_file_js.close()
    except:
        # The file may not exist (for repl for example)
        result[yaml_data["command"]] = {
            "name": yaml_data["command"],
            "description": markdown.markdown(current_description),
            "examples": examples,
            "io": [[None, None]]
        }


# Browse all the docs
def browse_files(base, result):
    subdirlist = []
    for item in os.listdir(base):
        if item[0] != '.' and item not in IGNORED_FILES:
            full_path = os.path.join(base, item)
            if os.path.isfile(full_path):
                add_doc(full_path, result)
            else:
                #print os.path.join(base, item)
                subdirlist.append(full_path)

    for subdir in subdirlist:
        browse_files(subdir, result)

if __name__ == "__main__":
    script_path = os.path.dirname(os.path.realpath(__file__))

    result = {}

    build_docs(script_path, result)

    result_file = open(script_path+"/py_docs.json", "w")

    result_file.write(json.dumps(result, indent=2))
    result_file.close()
