---
layout: documentation
title: Install RethinkDB on OS X
title_image: /assets/images/docs/install-platforms/osx.png
active: docs
docs_active: install
permalink: docs/install/osx/
---
{% include install-docs-header.md %}

# Using the installer #

_Prerequisites:_ We provide native binaries for OS X Lion and above (>= 10.7).

[Download](http://download.rethinkdb.com/osx/rethinkdb-{{site.version.full}}.dmg) the disk
image, run `rethinkdb.pkg`, and follow the installation instructions.

# Using Homebrew #

_Prerequisites:_ Make sure you're on OS X Lion or above (>= 10.7) and
have [Homebrew](http://mxcl.github.com/homebrew/) installed.

Run the following in your terminal:

```bash
brew update && brew install rethinkdb
```
# Compile from source #

## Get the build dependencies ##

On OS X, [Xcode](https://developer.apple.com/xcode/) is required to
build from source.

## Get the source code ##

Download and extract the archive:

```bash
wget http://download.rethinkdb.com/dist/rethinkdb-{{site.version.full}}.tgz
tar xf rethinkdb-{{site.version.full}}.tgz
```

## Build RethinkDB ##

Kick off the build process:

```bash
cd rethinkdb-{{site.version.full}}
./configure --allow-fetch
make
```

You will find the `rethinkdb` binary in the `build/release/` subfolder.

{% include install-next-step.md %}
