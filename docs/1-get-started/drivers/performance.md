---
layout: documentation
title: "Improving RethinkDB client driver performance"
active: docs
docs_active: install-drivers
permalink: docs/driver-performance/
---

{% infobox info %}
__Haven't installed the driver yet?__ [Go do that](/docs/install-drivers/) first!
{% endinfobox %}

# JavaScript #

For faster performance, the JavaScript driver requires a protobuf library that
uses a C++ backend. This library should build automatically as part of the
driver install process.

You can verify that you are running the C++ backend by checking the following:

```javascript
$ node
r = require('rethinkdb')
r.protobuf_implementation
```
If the output is `'cpp'` then you are running the optimized C++ backend. Well done!

If the output is `'js'` the driver will still work, but may have performance
penalties. To build the required libraries, check out the `node-protobuf`
[installation instructions](https://github.com/fuwaneko/node-protobuf) on
Github for details.

# Python #

For faster performance, the Python driver requires a protobuf library that
uses a C++ backend. Out of the box, the Python driver will not use the
optimized backend. 

To build the optimized backend, [follow these
instructions](https://code.google.com/p/protobuf/source/browse/trunk/python/README.txt?r=388#78)
from Google Protobuf's project page.

You can verify that you are running the C++ backend by checking the following:

```py
$ python
import rethinkdb as r
r.protobuf_implementation
```
If the output is `'cpp'` then you are running the optimized C++ backend. Bravo! 

If the output is `'python'` the driver will still work, but may have performance
penalties.
