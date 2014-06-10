---
layout: documentation
title: Changefeeds in RethinkDB
active: docs
docs_active: changefeeds
permalink: docs/changefeeds/
---

Changefeeds are delightful.

{% infobox info %}
<strong>Want to write useful queries right away?</strong> Check out the [ten-minute guide](/docs/guide/javascript/).
{% endinfobox %}

You can use changefeeds for the following reasons:

1. Integrate with things.
2. Write cool apps.

Let's do that now!

# Basic usage #

You start using ReQL in your program similarly to how you'd use other
databases:

```python
feed = r.table('users').changes().run()
for change in feed:
    print change
```

# Read More #

Browse the following resources to learn more about ReQL:

- [Lambda functions in RethinkDB](/blog/lambda-functions/)
- [Introduction to map/reduce](/docs/map-reduce/)
- [Introduction to Joins](/docs/table-joins/)
- [API Reference](/api/)
