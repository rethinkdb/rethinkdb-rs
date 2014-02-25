---
layout: documentation
title: Install RethinkDB on Arch Linux
title_image: /assets/images/docs/install-platforms/arch.png
active: docs
docs_active: install
permalink: docs/install/arch/
---
{% include install-docs-header.md %}
{% include install-community-platform-warning.md %}
# With binaries #
RethinkDB is in the Community Repository. To install the server, run:

```bash
sudo pacman -S rethinkdb
```

See [the Arch wiki article on RethinkDB](https://wiki.archlinux.org/index.php/RethinkDB) for more information.


# Compile from source #

## Get the build dependencies ##
To compile RethinkDB from source, you will need to install the following packages:

```bash
sudo pacman -S make gcc protobuf boost python2 gperftools nodejs
```

You also need to install v8 from AUR (and `base-devel`, which is an implicit dependency
of any AUR package).

```bash
pacman -S base-devel
yaourt -S v8
```

## Tweak the system ##

RethinkDB uses Python 2 to build some documentation. You can use `sed`
to replace `python` with `python2`, or just change the symbolic link in
`/usr/bin/python` to `/usb/bin/python2` using:

{% infobox %}
__Warning:__ This command will break all other applications that require
`/usr/bin/python` to be Python 3. Use at your own risk.
{% endinfobox %}

```bash
sudo ln -s /usr/bin/python2 /usr/bin/python
```

## Get the source code ##
Clone the RethinkDB repository:

```bash
git clone --depth 1 -b v{{site.version.major}}.x https://github.com/rethinkdb/rethinkdb.git
```

## Build RethinkDB ##
Kick off the build process:

```bash
cd rethinkdb
./configure --dynamic tcmalloc_minimal 
make
```

You will find the `rethinkdb` binary in the `build/release/` subfolder.  

{% include install-next-step.md %}
