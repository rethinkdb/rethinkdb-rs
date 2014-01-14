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

{% infobox info padded %}
_Note_: If you have `google-app-engine` installed, you may have a name
collision between `google-app-engine` and
`protobuf`. Renaming/removing the symbolic link `google` in
`/usr/lib/python2.7/site-packages` is a temporary solution. You can
track progress for a better solution on [Github issue #901](https://github.com/rethinkdb/rethinkdb/issues/901).
{% endinfobox %}

# Optional: optimized backend #

For faster Python driver performance, read about [using an
optimized C++ protobuf backend](/docs/driver-performance/).

# Next steps #

{% include install-driver-docs-footer.md %}
