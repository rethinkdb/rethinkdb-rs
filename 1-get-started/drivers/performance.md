---
layout: documentation
title: "Improving RethinkDB client driver performance"
active: docs
docs_active: install-drivers
permalink: docs/driver-performance/
---

# JavaScript #

The JavaScript driver can take advantage of the C++ protocol buffer
backend for faster performance.

First, install the protobuf library and development files. On Ubuntu,
they can be installed by running this command:

```
sudo apt-get install libprotobuf-dev
```

On platforms other than Ubuntu, if the package manager does not have
the protobuf development files, they can be downloaded from
http://code.google.com/p/protobuf/downloads/list.

Second, the `node-protobuf` npm package must be installed alongside
the `rethinkdb` npm package:

```
npm install rethinkdb
npm install node-protobuf
```

The Javascript driver will automatically use the `node-protobuf`
package if it is installed. You can verify that it is being used
by checking `r.protobuf_implementation`:

```javascript
$ node
r = require('rethinkdb')
r.protobuf_implementation
```

If the output is `'cpp'` then you are running the optimized C++
backend. Well done!

If the output is `'js'` then the `node-protobuf` package was not
installed correctly.

# Python #

For faster performance, the Python driver requires a protobuf library that
uses a C++ backend. Out of the box, the Python driver will not use the
optimized backend.

To build the optimized backend, first install the protobuf library and
development files.  On Ubuntu, they can be installed by running this
command:

```
sudo apt-get install libprotobuf-dev protobuf-compiler
```

On platforms other than Ubuntu, if the package manager does not have
the protobuf development files, they can be downloaded from
http://code.google.com/p/protobuf/downloads/list.

The second step for building the C++ implementation is to set the
following variable to `cpp`:

```
export PROTOCOL_BUFFERS_PYTHON_IMPLEMENTATION=cpp
```

The third step is to install the C++ implementation of the protobuf
python library. On most versions of Ubuntu, this command will install
the library:

```
sudo apt-get install python-protobuf
```

Some versions of this package (notably, the one installed by pip) do
not contain the C++ implementation. The protobuf python library needs
to be installed from source. Run following command. It will download and
build the protobuf library if the protobuf library is not installed or
if it is installed without the C++ implementation.

```
python -c 'from google.protobuf.internal import cpp_message' || \
  protoc --version && pbver=$(protoc --version | \
  awk '{ print $2 }') && wget http://protobuf.googlecode.com/files/protobuf-$pbver.tar.gz && \
  tar xf protobuf-$pbver.tar.gz && cd protobuf-$pbver/python && \
  PROTOCOL_BUFFERS_PYTHON_IMPLEMENTATION=cpp python setup.py build
```

Once built, the protobuf library can be installed by using:

```
sudo PROTOCOL_BUFFERS_PYTHON_IMPLEMENTATION=cpp python setup.py install
```

The fourth and last step is to reinstall the `rethinkdb` python
package. This step will fail if it cannot build the RethinkDB-specific
C++ code.

```
sudo pip uninstall rethinkdb
sudo PROTOCOL_BUFFERS_PYTHON_IMPLEMENTATION=cpp pip install rethinkdb
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
