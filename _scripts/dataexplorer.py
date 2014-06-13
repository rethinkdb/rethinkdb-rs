import re
import markdown
import os
import sys
import yaml
import json
import codecs

IGNORED_FILES = "index.md"

def read_index(script_path, result):
    """
    Read javascript/index.md and add fields about
        - description
        - body
        - url (used later)
    """
    title_pattern = re.compile('##\s*\[(.*)\]\((.+?)/\)\s*##\s*')
    start_body_pattern = re.compile("{%\s*apibody\s*%}\s*")
    end_body_pattern = re.compile("{%\s*endapibody\s*%}\s*")
    ignore_pattern = re.compile("(.*Read more about this command.*)|(.*apisection.*)")
    open_apisection = re.compile(".* apisection.*")
    example_pattern = re.compile("(__Example:__.*)|(__Example__:.*)")

    index_file = codecs.open(os.path.abspath(script_path+"/../api/javascript/index.md"), "r", "utf-8")

    current_method = None
    current_description = ""
    parsing_body = False
    parsing_example = False
    just_opened_api = False

    for line in index_file:
        # If we just opened an api section, we are going to ignore everything until we hit a new method
        if open_apisection.match(line) != None:
            just_opened_api = True

        # Ignore the "read more about this command" lines
        if ignore_pattern.match(line) != None:
           continue 

        title = title_pattern.match(line)
        if title != None:
            # We just found a new title, let's save the previous one (if defined)
            if current_method != None:
                # The key used is the url of the detailed page about the method
                result["api/javascript/"+current_url+"/"] = {
                    "description": markdown.markdown(current_description),
                    "url":  current_url,
                    "body": current_body,
                    "name": current_method,
                    "example": markdown.markdown(current_example.replace('```js', '```'))
                }
            current_method = title.group(1)
            current_url = title.group(2)
            current_description = ""
            current_body = ""
            current_example = ""
            parsing_example = False
            parsing_body = False
            just_opened_api = False
        elif just_opened_api == False:
            # Check if we hit a body tag
            if start_body_pattern.match(line) != None:
                parsing_body = True
            elif end_body_pattern.match(line) != None:
                parsing_body = False
            else:
                # This line does not contain a body tag
                if parsing_body == True:
                    # If we are parsing the body, let's add the current line
                    current_body += line.strip()
                else:
                    # Here we are whether parsing some code or the description of a method
                    if parsing_example == True:
                        current_example += line
                    else:
                        if example_pattern.match(line) != None:
                            parsing_example = True
                            current_example = line
                        else:
                            current_description += line

    # Save last method
    result["api/javascript/"+current_url+"/"] = {
        "description": markdown.markdown(current_description),
        "url":  current_url,
        "body": current_body,
        "name": current_method,
        "example": markdown.markdown(current_example)
    }

    index_file.close()

def add_io_fields(script_path, result):
    """
    Read all .md files to add the io field in result
    """

    browse_files(os.path.abspath(script_path+"/../api/javascript/"), result)

def add_io_field(file_name, result):
    limiter_yaml = re.compile('---\s*')
    details_file = codecs.open(file_name, "r", "utf-8")

    is_yaml = False
    yaml_raw = ""
    for line in details_file.readlines():
        if limiter_yaml.match(line) != None:
            if is_yaml == False:
                is_yaml = True
            else:
                break
        else:
            if is_yaml == True:
                yaml_raw += line

    yaml_data = yaml.load(yaml_raw)
    if "io" in yaml_data:
        result[yaml_data["permalink"]]["io"] = yaml_data["io"]
    else:
        raise Exception("`io` field not found %s", file_name)


    details_file.close()

def browse_files(base, result):
    subdirlist = []
    for item in os.listdir(base):
        if item[0] != '.' and item not in IGNORED_FILES:
            full_path = os.path.join(base, item)
            if os.path.isfile(full_path):
                add_io_field(full_path, result)
            else:
                #print os.path.join(base, item)
                subdirlist.append(full_path)

    for subdir in subdirlist:
        browse_files(subdir, result)

if __name__ == "__main__":
    script_path = os.path.dirname(os.path.realpath(__file__))

    result = {}
    # Read the index file to retrieve all short description/simple example.
    read_index(script_path, result)

    # Read all the small files to add the `io` field
    add_io_fields(script_path, result)

    # Dump result in a JSON format and write it in a file
    result_file = codecs.open(script_path+"/reql_docs.json", "w", "utf-8")
    result_file.write(json.dumps(result, indent=2, sort_keys=True))
    result_file.close()
