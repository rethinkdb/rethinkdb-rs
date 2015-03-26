---
layout: documentation
title: Installing the Ruby driver
title_image: /assets/images/docs/driver-languages/ruby.png
docs_active: install-drivers
permalink: docs/install-drivers/ruby/
---
{% include docs/install-driver-docs-header.md %}

# Installation #

Install the driver with gem:

```bash
sudo gem install rethinkdb
```

# Usage #

You can use the drivers from Ruby like this:

```bash
$ irb
require 'rubygems'
require 'rethinkdb'
include RethinkDB::Shortcuts
r.connect(:host => 'localhost', :port => 28015).repl
r.db('test').table_create('tv_shows').run
r.table('tv_shows').insert({ 'name'=>'Star Trek TNG' }).run
```

# Next steps #

{% infobox %}
Move on to the [ten-minute guide](/docs/guide/ruby/) and learn how to use RethinkDB.
{% endinfobox %}
