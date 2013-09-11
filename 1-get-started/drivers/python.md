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

Install the driver with pip:

```bash
$ sudo pip install rethinkdb
```

Make sure the following line appears at the end of the output:

```
Successfully installed rethinkdb
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

During the installation, you can ignore errors similar to this one:

```
./rethinkdb/ql2.pb.h:9:10: fatal error: 'google/protobuf/stubs/common.h' file not found
#include <google/protobuf/stubs/common.h>
         ^
1 error generated.
*** WARNING: Unable to compile the C++ extension
command 'clang' failed with exit status 1
*** WARNING: Defaulting to the python implementation
```

The C++ errors above indicate the the faster C++ backend could not
be compiled. The driver will use the slower Python backend.

For faster Python driver performance, read about [using an
optimized C++ protobuf backend](/docs/driver-performance/).

# Optional: Python 3 #

The RethinkDB Python driver itself is compatible with Python 3.3+ in
addition to Python 2.6 and 2.7. However, the protocol buffers library
that the driver relies on is not. While we wait for Google to update
the protocol buffers library, it is possible to build and install a
Python 3 compatible version of the library from source using the
following instructions.

These steps assume that Python 3.3 is already installed as the default
Python environment on your system.

```bash
$ # Install dev dependencies for protobuf
$ apt-get install git curl autoconf libtool
$
$ # Download the Python 3 compatible fork of Google protobuf
$ git clone https://github.com/malthe/google-protobuf.git
$ cd google-protobuf
$ 
$ # Build and install protoc to generate Python 3 sources
$ ./autogen.sh
$ ./configure --prefix=/usr
$ make
$ make install
$ 
$ # Build and install the Python 3 compatible Python library
$ cd python
$ python setup.py build
$ python setup.py install
$ cd ../../
$
$ # Download and install the RethinkDB Python driver
$ git clone https://github.com/rethinkdb/rethinkdb.git
$ cd rethinkdb drivers/python
$ make
$ make install
```

{% infobox info padded %}
_Note_: The optimized protobuf serializer backend is not available
for Python 3.  During compilation of the RethinkDB Python driver you
will see a warning indicating a failure to build the C++ backend.
{% endinfobox %}

# Next steps #

{% include install-driver-docs-footer.md %}
