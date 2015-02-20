---
layout: documentation
title: Install RethinkDB on Linux Mint
title_image: /assets/images/docs/install-platforms/mint.png
active: docs
docs_active: install
permalink: docs/install/mint/
---
{% include install-docs-header.md %}
{% include install-community-platform-warning.md %}
The following instructions were tested on Linux Mint 16.

# With binaries #
Add the [RethinkDB repository](http://download.rethinkdb.com/apt) to your list of
repositories and then install via `apt-get`:

## Linux Mint 16 ##
```bash
echo "deb http://download.rethinkdb.com/apt saucy main" | sudo tee /etc/apt/sources.list.d/rethinkdb.list
```

## Linux Mint 17 ##
```bash
echo "deb http://download.rethinkdb.com/apt trusty main" | sudo tee /etc/apt/sources.list.d/rethinkdb.list
```

## Install RethinkDB ##
```bash
wget -qO- http://download.rethinkdb.com/apt/pubkey.gpg | sudo apt-key add -
sudo apt-get update
sudo apt-get install rethinkdb
```

{% include debian-based-install-from-source.md %}

{% include install-next-step.md %}
