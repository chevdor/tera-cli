# A basic example

Given some context data:

**`data.json`**
```json
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
```

and a template:

**`template.tera`**
```
<title>{% block title %} {{title}} {% endblock title %}</title>

<ul>
{% for user in users -%}
    <li><a href="{{ user.url }}">{{ user.username }}
    {{ user.username }} likes {% for color in user.fav_colors -%}{{ color }} {% endfor %}
    </a></li>
{% endfor %}
</ul>
```

Running:

```bash
tera --template template.tera data.json
```

produces:

```html
<title> Demo </title>

<ul>
<li><a href="http://example.org/alice">Alice
    Alice likes red green yellow
    </a></li>
<li><a href="http://example.org/bob">Bob
    Bob likes orange
    </a></li>

</ul>
```

See [ENV variables](/guide/env-variables.md) and
[Includes &amp; inheritance](/guide/includes-inheritance.md) for the rest of
`tera-cli`'s features, and the [CLI reference](/reference/cli.md) for every
flag.
