---
layout: documentation
title: Install RethinkDB on Gentoo
title_image: /assets/images/docs/install-platforms/gentoo.png
docs_active: install
permalink: docs/install/gentoo/
---
{% include docs/install-docs-header.md %}
{% include docs/install-community-platform-warning.md %}

# Install RethinkDB #

```bash
emerge -av rethinkdb
```

Note that RethinkDB will be built using the **jemalloc** memory allocator by default.
See the USE flags of the dev-db/rethinkdb package for more options.

# Configure your RethinkDB instance #

The dev-db/rethinkdb OpenRC init script supports multiple instances.

Create your instance(s) by using the configuration helper:

```bash
emerge --config dev-db/rethinkdb
```

Example usage and result:

```
 * This will prepare a new RethinkDB instance. Press Control-C to abort.
 * Enter the name for the new instance:
staging_a

 * Successfully created the instance at /var/lib/rethinkdb/instances.d/staging_a.
 * To change the default settings edit the configuration file:
 * /etc/rethinkdb/instances.d/staging_a.conf
 *
 * To start your instance, run:
 * /etc/init.d/rethinkdb.staging_a start
```

That's all, enjoy RethinkDB !

{% include docs/install-next-step.md %}
