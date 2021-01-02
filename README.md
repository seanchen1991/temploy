# Temploy

Temploy can be treated as two separate tools: a project generation / scaffolding tool,
and an automated deployment tool that handles deploying to your infrastructure platform
of choice. 

## Generating a Project

Temploy supports scaffolding a new project from a local directory or a specified GitHub
repository.

```
# Generating from a local directory
temploy generate [PATH_TO_TEMPLATE_DIRECTORY]

# Generating from a github repository
temploy generate https://github.com/[USERNAME]/[REPO_NAME].git
```

Here are the available commands for `temploy generate`:
```
USAGE:
    temploy generate [OPTIONS] <template>

OPTIONS:
    -n, --name <name>
        Specify the name of your generated project

    -d, --target-directory <target-directory>    
        Specify the target directory

ARGS:
    <template>    
        Specify your template location
```

If a `<name>` for the generated project is not specified, then the project is generated
and named `<original-prject-name>-clone`. If this project name already exists at the 
target directory, then the generated project is not created to avoid overwriting the 
original directory.

If a `<target-directory>` is not specified, then the cloned project will be placed in the
current working directory.

## Deploying a Project

