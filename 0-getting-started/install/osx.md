---
layout: documentation
title: Install RethinkDB on OS X
title_image: /assets/images/docs/install-platforms/osx.png
docs_active: install
permalink: docs/install/osx/
---
{% include docs/install-docs-header.md %}

# Using the installer #

__Prerequisites:__ We provide native binaries for OS X 10.7 (Lion) and above. _OS X 10.9 (Mavericks) or higher is required for HTTPS support. See [issue #5681][i5681] for more information._

[i5681]: https://github.com/rethinkdb/rethinkdb/issues/5681

[Download](https://download.rethinkdb.com/osx/rethinkdb-{{site.version.full}}.dmg) the disk
image, run `rethinkdb.pkg`, and follow the installation instructions.

# Using Homebrew #

_Prerequisites:_ Make sure you're on OS X 10.9 (Mavericks) or above, and
have [Homebrew](http://mxcl.github.com/homebrew/) installed.

Run the following in your terminal:

```bash
brew update && brew install rethinkdb
```

# Compile from source #

Building RethinkDB from source requires OS X 10.9 (Mavericks) or greater. [Xcode](https://developer.apple.com/xcode/) is required to
build from source.

## Get the source code ##

Download and extract the archive:

```bash
wget https://download.rethinkdb.com/dist/rethinkdb-{{site.version.full}}.tgz
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

{% include docs/install-next-step.md %}
