---
layout: documentation
title: "Improving RethinkDB client driver performance"
active: docs
docs_active: install-drivers
permalink: docs/driver-performance/
---

# JavaScript #

For faster performance, the JavaScript driver uses the `node-protobuf` library,
which is a node module implemented in C++.

The protobuf library and development files are required to install `node-protobuf`. These are
available on Ubuntu in the `libprotobuf-dev` package. They can be installed from source
at http://code.google.com/p/protobuf/downloads/list. Consult the `node-protobuf`
[installation instructions](https://github.com/fuwaneko/node-protobuf) for more information.

The Javascript driver will automatically use `node-protobuf` if it is installed:

```
npm install rethinkdb
npm install node-protobuf
```

You can verify that you are running the C++ backend by checking the following:

```javascript
$ node
r = require('rethinkdb')
r.protobuf_implementation
```
If the output is `'cpp'` then you are running the optimized C++ backend. Well done!

If the output is `'js'` the driver will still work, but may have performance
penalties. To build the required libraries,  on
Github for details.

# Python #

For faster performance, the Python driver requires a protobuf library that
uses a C++ backend. Out of the box, the Python driver will not use the
optimized backend.

To build the optimized backend, protobuf library and development files
need to be installed.  On Ubuntu, they are available in these three
packages: `libprotobuf-dev`, `protobuf-compiler` and
`python-protobuf`. For more information, consult the [install
instructions](https://code.google.com/p/protobuf/source/browse/trunk/python/README.txt?r=388#78)
from Google Protobuf's project page.

The C++ implementation will not be built unless the following variable is set:

```
export PROTOCOL_BUFFERS_PYTHON_IMPLEMENTATION=cpp
```

Run this command to download and build the C++ backend for the protobuf library, if needed:

```
python -c 'from google.protobuf.internal import cpp_message' || \
  protoc --version && pbver=$(protoc --version | \
  awk '{ print $2 }') && wget http://protobuf.googlecode.com/files/protobuf-$pbver.tar.gz && \
  tar xf protobuf-$pbver.tar.gz && cd protobuf-$pbver/python && \
  export PROTOCOL_BUFFERS_PYTHON_IMPLEMENTATION=cpp && python setup.py build
```

Then install it with this command:

```
python setup.py install --user
```

Now reinstall the rethinkdb package:

```
pip uninstall rethinkdb; pip install --user rethinkdb
```

You can verify that you are running the C++ backend by checking the following:

```py
$ python
import rethinkdb as r
r.protobuf_implementation
```

If the output is `'cpp'` then you are running the optimized C++ backend. Bravo!

If the output is `'python'` the driver will still work, but may have performance
penalties.
