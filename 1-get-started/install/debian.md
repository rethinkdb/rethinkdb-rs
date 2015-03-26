---
layout: documentation
title: Install RethinkDB on Debian
title_image: /assets/images/docs/install-platforms/debian.png
docs_active: install
permalink: docs/install/debian/
---
{% include docs/install-docs-header.md %}

# With binaries #

We provide binaries for Wheezy and Jessie, 32-bit and 64-bit.

To install the server, you have to add the [RethinkDB
repository](http://download.rethinkdb.com/apt) to your list of
repositories and install via `apt-get`.
To do this, paste the
following lines into your terminal:

```bash
echo "deb http://download.rethinkdb.com/apt `lsb_release -cs` main" | sudo tee /etc/apt/sources.list.d/rethinkdb.list
wget -qO- http://download.rethinkdb.com/apt/pubkey.gpg | sudo apt-key add -
sudo apt-get update
sudo apt-get install rethinkdb
```

{% include docs/debian-based-install-from-source.md %}

{% include docs/install-next-step.md %}
