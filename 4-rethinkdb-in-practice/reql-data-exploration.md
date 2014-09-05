---
layout: documentation
title: Useful RethinkDB commands for exploring data
active: docs
docs_active: reql-data-exploration
permalink: docs/reql-data-exploration/
---

Akshay Chougule, a biologist working with large data sets, wrote a blog post about [Useful Unix commands for exploring data][uu], showing ways to "query" comma-delimited plain text data sets with common shell commands. We thought it'd be interesting to do a similar article using ReQL in the Data Explorer, showing how it can be used for ad hoc queries.

[uu]: http://datavu.blogspot.com/2014/08/useful-unix-commands-for-exploring-data.html

Akshay created a fictitious data set of movies; we'll use [data][] from the [IMDb Top 250][t250]. (Note that we captured this on August 26, 2014, so the data will likely be different if you check it now.) The plain-text data from IMDb isn't in *any* format, but we've turned it into a JSON file available at <http://rethinkdb.com/sample/top-250-ratings.json>. (For the record, we converted it to a tab-delimited file first, used `rethinkdb import` to get it into a database, fixed the column types and re-exported it. See [Importing your data](http://rethinkdb.com/docs/importing/) for details.)

[data]: http://www.imdb.com/interfaces
[t250]: http://www.imdb.com/chart/top

Since it's available online, you can import our sample data set just by creating a table and importing it with [http](/docs/http). Just execute these commands directly in the Data Explorer.

```js
r.tableCreate('movies');
r.table('movies').insert(r.http('http://rethinkdb.com/sample/top-250-ratings.json'))
```

The Data Explorer will return information about the inserted rows.

```js
{
	"deleted": 0 ,
	"errors": 0 ,
	"generated_keys": [
	"bbf81f4d-2a6d-40bb-9b5d-b6e288cc8795" ,
	"0d6054f4-12b0-4c2e-b221-881441c779c4" ,
	...
	] ,
	"inserted": 253 ,
	"replaced": 0 ,
	"skipped": 0 ,
	"unchanged": 0
}
```

The table created has six fields:  an automatically generated primary key (`id`), `rank` (the IMDb rank, 1&ndash;250), `rating` (on a 1&ndash;10 scale), `title`, `votes`, and `year`.

## Get the top movie  ##

So we can see IMDb's number one movie with  `filter({rank: 1})`:

```js
r.table('movies').filter({rank: 1})

[
	{
		"id":  "bbf81f4d-2a6d-40bb-9b5d-b6e288cc8795" ,
		"rank": 1 ,
		"rating": 9.2 ,
		"title":  "The Shawshank Redemption" ,
		"votes": 1262930 ,
		"year": 1994
	}
]
```

("Shawshank" has been the most-loved movie on IMDb for many years. Take that, Orson Welles.)

## Removing duplicate documents ##

You might have caught that there were 253 documents inserted, not 250. Either we have the top 253 movies, or there are a few duplicate records lurking in there. We can use `distinct` to get a count of unique rows, but we need to remove the `id` column from the query, since all ID values are unique.

```js
r.table('movies').without('id').distinct().count()

250
```

To get the list without duplicates, we can simply leave off `count`.

```js
r.table('movies').without('id').distinct()
```

To put these into a new table,  wrap that query with `insert`. We'll get new IDs generated automatically. This is also an example of using subqueries with ReQL: it's easy to pass the results of one query into another. (One of ReQL's other nice properties, which we've already seen, is command chaining: the input of one command is often the output of the command before it, similar to Unix piping.)

```js
r.tableCreate('moviesUnique');
r.table('moviesUnique').insert(
	r.table('movies').without('id').distinct()
)
```

Now with a "clean" data set we can run simple reports right in the Data Explorer. (You might want to switch to Table View for some of these, and you can also add `.without('id')` in the command chain to "prettify" the table display if you wish.)

## Display the top 10 movies...  ##

```js
r.table('moviesUnique').orderBy('rank').limit(10)
```

## ...and the bottom 10 ##

```js
r.table('moviesUnique').orderBy(r.desc('rank')).limit(10)
```

(Those are the bottom 10 of the top 250, so they're still pretty good.)

## Get the 1st, 2nd ,6th and last records ##

```
r.table('moviesUnique').filter(function (doc) {
  return r.expr([1, 2, 6, r.table('moviesUnique').max('rank')('rank')]).
    contains(doc('rank'));
}).orderBy('rank');
```

## Find the average number of votes for the top 25 movies ##

```js
r.table('moviesUnique').orderBy('rank').limit(25).avg('votes')
```

## Find the most recent movie in the top 25 ##

```js
r.table('moviesUnique').orderBy('rank').limit(25).max('year')
```

## Find the highest-ranked movie with under 100,000 votes ##

```js
r.table('moviesUnique').filter(r.row('votes').lt(100000)).min('rank')
```

You can read more about ReQL in the [Introduction to ReQL][intro] article, or go into greater depth with the [API documentation](/api/).

[intro]: /docs/introduction-to-reql/
