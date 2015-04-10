---
layout: documentation
title: Full-text search with Elasticsearch
docs_active: full-text-search
permalink: docs/elasticsearch/
---

__Q__: What's the best way to perform [full-text searches][fts] with RethinkDB?

__A__: Use the [Elasticsearch River for RethinkDB][err].

[fts]: http://en.wikipedia.org/wiki/Full_text_search
[err]: https://github.com/rethinkdb/elasticsearch-river-rethinkdb

{% infobox %}

**Before you start**

* Ensure you have [RethinkDB installed][rdb] for your platform.
* Have [Elasticsearch 1.3][e13] installed, running on [Java 8][j8].

[rdb]: /docs/install
[e13]: http://www.elasticsearch.org/overview/elkdownloads/
[j8]: http://www.oracle.com/technetwork/java/javase/downloads/index.html

{% endinfobox %}

# What Elasticsearch does

[Elasticsearch][es] is a database that stores documents in a crafty way that makes it fast to search large fields of pure text. For instance, it indexes words in different ways depending on how frequent they are in your overall data. It doesn't waste time checking [common words][cw] like "is" and "to" when returning results unless they actually make a difference. It also performs [stemming][st], so that a search for "looked" will return results containing the words "looks" and "looking."

It also returns results ordered from most relevant to least, not worrying about small differences. Say you want to ask the question: "What documents best match the phrase 'Holy guacamole, Batman'?" If the hoped-for guacamole reference isn't found, a full-text search should reply with documents containing good matches like "Holy smokes, Batman!" and "Holy armadillo, Batman!" In short, you should be using a full-text search database like Elasticsearch if you find yourself writing convoluted regular expressions to grep through big text fields.

[cw]: http://www.elasticsearch.org/blog/stop-stopping-stop-words-a-look-at-common-terms-query/
[st]: http://www.elasticsearch.org/guide/en/elasticsearch/guide/current/controlling-stemming.html
[es]: http://www.elasticsearch.org

For those applications that need full-text search, we've got you covered. We've written a [plugin for Elasticsearch][err] (called a river) that keeps RethinkDB synced up with Elasticsearch's indexes. It uses [changefeeds][cf] to push new, updated and deleted documents to Elasticsearch in real-time. In addition, it will load existing documents from your RethinkDB tables, so you can get going right away.

[cf]: /docs/changefeeds/

# Venturing into the river

To install the river, we'll use the `plugin` program that comes with Elasticsearch. On most platforms the program is named `plugin`, but it's sometimes called `elasticsearch-plugin`:

```
plugin --install river-rethinkdb --url http://goo.gl/JmMwTf
```

Depending on how you've installed Elasticsearch, you may need to become the elasticsearch user or root to run this command.

Now that we've installed the plugin, the next step is to actually configure it to connect to our RethinkDB instance. We can do that by talking to Elasticsearch's REST API. There are three concepts we need to deal with in the API: indexes, types, and documents. A document is the actual data being stored itself and is just JSON. A type contains documents and is similar to a table in RethinkDB. An index contains types and is similar to a database in RethinkDB.

To configure our river, we need to create a type called `rethinkdb` in the `_river` index. Then we need to insert a document with the id `_meta` into that type. Elasticsearch lets us create the document and the type in one go with a `PUT` request:

```bash
$ curl -XPUT localhost:9200/_river/rethinkdb/_meta -d '
{
  "type": "rethinkdb",
  "rethinkdb": {
    "host": "localhost",
    "port": 28015,
    "databases": {
      "blog": {
        "posts": { "backfill": true },
        "comments": { "backfill": true }
      }
    }
  }
}
```

Here we've told the river to watch two tables in the `blog` database: `posts` and `comments`. The river should also pull in all existing documents from those tables before it starts watching for updates to the tables. By default, the river inserts documents into a type named after its table, and into an index named after its database. So, in the example above, we'd get a new index named "blog" with two types: "posts" and "comments."

You can also specify explicitly which index and type you want synced documents to go to:

```bash
$ curl -XPUT localhost:9200/_river/rethinkdb/_meta -d '
{
  "type": "rethinkdb",
  "rethinkdb": {
    "host": "localhost",
    "port": 28015,
    "databases": {
      "blog": {
        "posts": {
          "backfill": true,
          "index": "fooBlog",
          "type": "barPosts"
        }
      }
    }
  }
}
```

Once you've got the data in your Elasticsearch server, you're ready to go. Here's an example of a simple query using the Elasticsearch REST API:

```
$ curl localhost:9200/blog/posts/_search?q=body:yams
```

The results of which might look something like:

```javascript
{
    "_shards": {
        "failed": 0,
        "successful": 1,
        "total": 1
    },
    "hits": {
        "hits": [
            {
                "_id": "261f4990-627b-4844-96ed-08b182121c5e",
                "_index": "blog",
                "_score": 1.0,
                "_source": {
                    "body": "You won't believe these ten amazing ways to cook yams...",
                    "id": "261f4990-627b-4844-96ed-08b182121c5e",
                    "title": "Thanksgiving dinner blog",
                    "userId": 10.0
                },
                "_type": "posts"
            }
        ],
        "max_score": 1.0,
        "total": 1
    },
    "timed_out": false,
    "took": 6
}
```

For the full details on querying, you'll want to read up on [how to query Elasticsearch][hq].

[hq]: http://www.elasticsearch.org/guide/en/elasticsearch/reference/current/search-search.html
