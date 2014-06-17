---
layout: documentation
title: Installing the Python driver
title_image: /assets/images/docs/driver-languages/python.png
active: docs
docs_active: install-drivers
permalink: docs/install-drivers/python/
---
{% include install-driver-docs-header.md %}

# Installation #

The Python driver for RethinkDB is compatible with Python 2.

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

{% include install-driver-docs-footer.md %}
