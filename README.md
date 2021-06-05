# Tera cli

[tera](https://github.com/Keats/tera) is a template engine written in Rust and inspired by Jinja2. It allows merging some data called `context data` into a template and produces a new output.

This project is a command line for the [tera template engine](https://github.com/Keats/tera). This project is called `tera-cli` but the command installed on your system is simply `tera`.

Here is a basic example. For instance, you will pass data such as:

    .data.json:
    {
        "title": "Demo",
        "users": [
            {
                "username": "Alice",
                "url": "http://example.org/alice"
            },
            {
                "username": "Bob",
                "url": "http://example.org/bob"
            }
        ]
    }

as well as a template such as:

    .template.tmpl
    <title>{% block title %} {{title}} {% endblock title %}</title>

    <ul>
    {% for user in users %}  <li><a href="{{ user.url }}">{{ user.username }}</a></li>
    {% endfor %}
    </ul>

and a call such as `tera --template data/template.tmpl data/data.json` will produce:

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

### Output

By default,

### Content escaping

Passing the `-a | --escape` flag allows escaping the content.

## Usage

    tera-cli 0.1.0
    chevdor <chevdor@gmail.com>
    Command line utility for the tera templating engine. You need to provide a template using the tera
    syntax as well as some data (various format are supported)

    USAGE:
        tera [FLAGS] [OPTIONS] --template <template> [context]

    ARGS:
        <context>    Location of the context data. This file can be of the following type: json |
                     toml | yaml. If you prefer to pass the data as stdin, use `--stdin`

    FLAGS:
        -a, --escape               Auto-escape rendered content. This is useful for HTML output
        -e, --env                  If true, the current ENV will be appended to the data under the
                                   --env-key key
            --env-first            By default, the context is made of the data you pass and the ENV is
                                   applied afterwards. Setting this option will apply the ENV first.
                                   This is interesting if you prefer your data to override the ENV
            --env-only             If you want to solely use the ENV as context, you may pass this
                                   option. This will prevent an error about no context being passed to
                                   be raised
            --fail-on-collision    if you prefer your data to override the ENV
        -h, --help                 Prints help information
        -s, --stdin                The context data can be passed using stdin
        -V, --version              Prints version information

    OPTIONS:
            --env-key <env-key>      By default, if --env is set, the environment variables will be
                                     attached at the root of the context. This is convenient but may end
                                     up conflicting with your data. To prevent collisions, you can
                                     provide a custom key with this option
        -o, --out <out>              Optional output file. If not passed, using stdout
        -t, --template <template>    Location of the template
