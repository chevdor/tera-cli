# tera-cli

## Intro

![tera cli logo 256](resources/logo/tera-cli-logo_256.png)

[tera](https://github.com/Keats/tera) is a template engine written in Rust and inspired by Jinja2. It allows merging some data called `context data` into a template and produces a new output. This project, `tera-cli`, is a command line for the [tera template engine](https://github.com/Keats/tera).

This project is called `tera-cli` but the command installed on your system is simply `tera`.

`tera-ci` offers powerful features related to your environment variables, allowing you to control the output **both** from the context data you pass but also from the ENV variables set on your system.

## Example

Here is a basic example. For instance, you will pass data such as:

    .data.json:
    {
        "title": "Demo",
        "users": [
            {
                "username": "Alice",
                "url": "http://example.org/alice",
                "fav_colors": ["red", "green", "yellow"]
            },
            {
                "username": "Bob",
                "url": "http://example.org/bob",
                "fav_colors": ["orange"]
            }
        ]
    }

as well as a template such as:

    .template.tmpl
    <title>{% block title %} {{title}} {% endblock title %}</title>

    <ul>
    {% for user in users -%}
        <li><a href="{{ user.url }}">{{ user.username }}
        {{ user.username }} likes {% for color in user.fav_colors -%}{{ color }} {% endfor %}
        </a></li>
    {% endfor %}
    </ul>

and a call such as `tera --template template.tera data.json` will produce:

    .result
    <title> Demo </title>

    <ul>
      <li><a href="http://example.org/alice">Alice</a></li>
      <li><a href="http://example.org/bob">Bob</a></li>

    </ul>

The **tera** engine allows way more than the simple replacements shown above. You may check out the [doc](https://tera.netlify.app/docs) for more information. To name only a few, **tera** offers the following:

-   variables & expressions (you can do math…​)

-   comments

-   control structure & loops (if, for, …​)

-   filters

-   formatting functions (show a file size, format a date, etc…​)

-   inheritance, include, etc…​

-   built-ins: capitalize strings, replace, trim, etc…​

## Install

    cargo install --git https://github.com/chevdor/tera-cli

## Hot reload

You may find it useful to watch a folder with your templates and run `tera` if a template changes. For this to work, it is recommended to
name you template as `foobar.md.tera` if your template expands into a markdown file for instance.
You may then use [fswatch](https://github.com/emcrisostomo/fswatch) and watch a `templates` folder using:

    fswatch templates -e ".*\.md$" | \
        xargs -n1 -I{} \
        tera --include-path templates \
            --template templates/template.md.tera context.json

## Execute as Docker container

You can find a `tera` Docker image at `chevdor/tera`. The image is very small and should be less than 8MB.

You can test it with:

    docker run --rm -it chevdor/tera --version

The Docker image mentioned above is not yet built by the CI so you may not find the very latest version from time to time.

### Build container image

    docker build --tag tera-cli .

### Execute `tera` from the Docker container

**Check the tera help**

    docker run -it --rm tera-cli --help

**Parse a template**

    docker run -it --rm \
        --volume="$(pwd)/templates:/templates" \
        --read-only \
        --env=FOO=BAR \
        tera-cli --template /templates/env-debug.txt --env-only --env-key env

## What can I do with that anyway ?

Well…​ if you have **data** and you want to format them, this tool will likely be a great companion.

-   You may generate beautiful changelogs in markdown, asciidoc, restructured text, etc…​

-   You may generate some more human views of your data

-   You may…​ make a blog with that…​

-   You may generate k8s config files…​.

## Features

### Supported formats

You may pass the `context` data either as file of into **stdin**.

Current **stdin** supports only json.

### ENV support

There are several options related to the environment variables.

#### Enable ENV variables injection

By default, the environments variables are not merged in. You can turn this feature on with `--env`.

#### Collisions

Now that you enabled the merging of the ENV variables, it is important to understand that, in some cases, your ENV may collide with your context data. This can be convenient if you want your ENV to override the context data.

#### ENV injection priority

If you prefer the context data to overwrite the ENV, you may use `--env-first`. As a result, the ENV will be applied first to the context and your context data will be loaded afterward.

#### Collisions handling

You may perfer to consider collisions as failures. This is what `--fail-on-collision` is for. If a collision is detected, the program will exit with a status code of `1` and an appropriate message.

#### ENV only

You may also want to ONLY load ENV variables as context data. This is what `--env-only` does.

#### ENV sub key

By default, your ENV variables will be loaded at the root of the context data. For instance, the `HOME` ENV variable will be then available in your tera template as `{{ HOME }}`. As we just mentioned, collisions may be an issue. There is an easy to prevent them entirely: you may move the ENV into a sub key in the context data. This is allowed thanks to the `--env-key <name>` option. For instance, using `--env-key env` will make your `HOME` ENV variable available in the tera template as `{{ env.HOME }}`.

While the syntax is a little more verbose, paired with `--fail-on-collision`, this option allows ensuring that nothing happens in your back.

### External files

Using the `--include` flag, the command will scan recursively for files that could be [included](https://tera.netlify.app/docs/#include), used as [macros](https://tera.netlify.app/docs/#macros) or for [inheritance](https://tera.netlify.app/docs/#inheritance). By default, it will scan the folder where the main template is located, unless the `--include-path` option is given.

From this repository, you can test the **include** feature with the command:

    USER="[YOURNAME]" tera --template data/include/hello.txt --include --env-only

and test the **inheritance** feature with:

    USER="[YOURNAME]" tera --template data/inheritance/child.txt --inherit --env-only

### Content escaping

Passing the `-a | --escape` flag allows escaping the content.

## Usage

    teracli 0.2.2
    chevdor <chevdor@gmail.com>
    Command line utility for the tera templating engine. You need to provide a template using the tera
    syntax as well as some data (various format are supported)

    USAGE:
        tera [OPTIONS] --template <TEMPLATE> [CONTEXT]

    ARGS:
        <CONTEXT>    Location of the context data. This file can be of the following type: json |
                     toml | yaml. If you prefer to pass the data as stdin, use `--stdin`

    OPTIONS:
        -a, --escape
                Auto-escape rendered content. This is useful for HTML output

        -e, --env
                If true, the current ENV will be appended to the data under the --env-key key

            --env-first
                By default, the context is made of the data you pass and the ENV is applied afterwards.
                Setting this option will apply the ENV first. This is interesting if you prefer your
                data to override the ENV

            --env-key <ENV_KEY>
                By default, if --env is set, the environment variables will be attached at the root of
                the context. This is convenient but may end up conflicting with your data. To prevent
                collisions, you can provide a custom key with this option

            --env-only
                If you want to solely use the ENV as context, you may pass this option. This will
                prevent an error about no context being passed to be raised

            --fail-on-collision
                if you prefer your data to override the ENV

        -h, --help
                Print help information

        -i, --include
                This flag tells the command to parse all templates found in the same path where the
                given template is located [aliases: inherit]

            --include-path <INCLUDE_PATH>
                Option to define a different path from which search and parse templates [aliases:
                inherit-path]

        -o, --out <OUT>
                Optional output file. If not passed, using stdout

        -s, --stdin
                The context data can be passed using stdin

        -t, --template <TEMPLATE>
                Location of the template

        -V, --version
                Print version information
