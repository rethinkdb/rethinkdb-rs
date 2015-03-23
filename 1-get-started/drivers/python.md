---
layout: documentation
title: Installing the Python driver
title_image: /assets/images/docs/driver-languages/python.png
docs_active: install-drivers
permalink: docs/install-drivers/python/
---
{% include install-driver-docs-header.md %}

# Installation #

{% infobox info %}
From version 1.14, the Python driver for RethinkDB supports Python 2 and 3. Version 1.13 and older support Python 2 only.
{% endinfobox %}

Install the driver with pip:

```bash
$ sudo pip install rethinkdb
```

# Usage #

You can use the drivers from Python like this:

```bash
$ python
import rethinkdb as r
r.connect('localhost', 28015).repl()
r.db('test').table_create('tv_shows').run()
r.table('tv_shows').insert({ 'name': 'Star Trek TNG' }).run()
```

# Next steps #

{% infobox %}
Move on to the [ten-minute guide](/docs/guide/python/) and learn how to use RethinkDB.
{% endinfobox %}
