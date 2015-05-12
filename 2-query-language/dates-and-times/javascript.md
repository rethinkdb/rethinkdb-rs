---
layout: documentation
title: Dates and times in RethinkDB
docs_active: dates-and-times
permalink: docs/dates-and-times/javascript/
alias: docs/dates-and-times/
switcher: true
language: JavaScript
---

RethinkDB has native support for millisecond-precision times with time zones.
Some highlights:

* **Times are integrated with the official drivers**, which will automatically
  convert to and from your language's native time type.
* **Queries are timezone-aware**, so you can ask questions like "Did this event
    happen on a Monday in the time zone where it was recorded?"
* **Times work as indexes**, so you can efficiently retrieve events based on when
   they occurred.
* **Time operations are pure ReQL**, which means that even complicated date-time
  queries can be distributed efficiently across the cluster.

{% toctag %}

<img alt="Dates and Times Illustration" class="api_command_illustration"
     src="/assets/images/docs/api_illustrations/dates-and-times.png" />

# A quick example #

First, let's create a table and insert some events.  We'll insert the first
event using a native Date object, and the second with the `epochTime`
constructor:

```js
r.tableCreate('events').run(conn, callback);

r.table('events').insert([
    {id: 0, timestamp: new Date()},
    {id: 1, timestamp: r.epochTime(1376436769.923)}
]).run(conn, callback);
```

Now, let's get those back:

```js
> r.table('events');
// Result passed to callback
[
    { "id": 0, "timestamp": Date("2013-08-13T23:32:49.923Z") },
    { "id": 1, "timestamp": Date("2013-08-13T23:32:49.923Z") }
]
```

You'll notice that both times we inserted are returned as native
JavaScript `Date` objects. (`Date` objects don't store time zone
information, so both times are UTC, regardless of your server's local time
zone.)

We can now filter based on these times:

```js
> r.table('events').filter(r.row('timestamp').hours().gt(20)).run(conn, callback);
// Result passed to callback
[ { "id": 1, "timestamp": Date("2013-08-13T23:32:49.923Z") } ]
```

Or create a secondary index on them:

```js
> r.table('events').indexCreate('timestamp').run(conn, callback);

> r.table('events').between(r.epochTime(1376436769.913),
      r.epochTime(1376436769.933), {index: 'timestamp'}
  ).run(conn, callback);
// Result passed to callback
[ { "id": 1, "timestamp": Date("2013-08-13T23:32:49.923Z") } ]
```

# Technical details #

Times are stored on the server as seconds since epoch (UTC) with millisecond
precision plus a time zone.  Currently the only available time zones are
minute-precision time offsets from UTC, but we may add support for DST-aware
time zones in the future.  Time zones are strings as specified by ISO
8601. Note that the JavaScript driver strips time zone information due to limitations with the `Date` object, although you can retrieve time zone data via the raw ReQL time object. (See below.)

Times are considered equal if their seconds since epoch (UTC) are equal,
<strong>regardless of what time zone they're in</strong>.  This is true for both
comparisons and indexed operations.

Most date operations are only defined on years in the range `[1400, 10000]` (but
note that times in the year `10000` cannot be printed as ISO 8601 dates).

Leap-seconds aren't well-supported right now: `2012-06-30T23:59:60` and
`2012-07-01T00:00:00` parse to the same time.

# Inserting times #

You can insert times by simply passing a native `Date` object. 

```js
> r.table('events').insert({id: 2, timestamp: new Date()}).run(conn, callback);
// Result passed to callback
{"unchanged"=>0, "skipped"=>0, "replaced"=>0, "inserted"=>1, "errors"=>0, "deleted"=>0}
```

You can also use `r.now` (which the server interprets as the time the
query was received in UTC), or construct a time using `r.time`,
`r.epochTime`, or `r.ISO8601`.

```js
> r.now().toISO8601().run(conn, callback);
// Result passed to callback
"2013-08-09T18:53:15.012+00:00"

> r.time(2013, r.august, 9, 18, 53, 15.012, '-07:00').toISO8601().run(conn, callback);
// Result passed to callback
"2013-08-09T18:53:15.012-07:00"

> r.epochTime(1376074395.012).toISO8601().run(conn, callback);
// Result passed to callback
"2013-08-09T18:53:15.012+00:00"

> r.ISO8601("2013-08-09T18:53:15.012-07:00").toISO8601().run(conn, callback);
// Result passed to callback
"2013-08-09T18:53:15.012-07:00"
```

Times may be used as the primary key for a table.  Two times are considered
equal if they have the same number of milliseconds since epoch (UTC), regardless
of time zone.

```js
> r.table('t').insert(
      {id: r.ISO8601("2013-08-09T11:58:00.1111-07:00")}
  ).run(conn, callback);
// Result passed to callback
{"unchanged"=>0, "skipped"=>0, "replaced"=>0, "inserted"=>1, "errors"=>0, "deleted"=>0}

> r.table('t').insert(
      {id: r.ISO8601("2013-08-09T10:58:00.1111-08:00")}
  ).run(conn, callback);
// Result passed to callback
{"unchanged"=>0, "skipped"=>0, "replaced"=>0, "inserted"=>0,
 "first_error"=>"Duplicate primary key `id`: ...", "errors"=>1, "deleted"=>0}
```

You may also insert a time by inserting a literal pseudotype object.  This is
useful if, for instance, you exported a row using `{timeFormat: 'raw'}` (see
<strong>Retrieving Times</strong> below).

{% infobox %}
__Note:__ Avoid using keys matching the regular expression
`^\$reql_.+\$$` in your objects.  RethinkDB considers those to be reserved
keywords.
{% endinfobox %}

```js
> r.expr(
      {'$reql_type$': 'TIME', epoch_time: 1376075362.662, timezone: '+00:00'}
  ).run(conn, callback);
// Result passed to callback
Date("2013-08-09T19:09:22.662Z")
```

# Retrieving times #

By default, times are converted into native time objects when they are retrieved
from the server.  This may be overridden by passing the optarg `timeFormat` to
`run`.  The only options right now are `native`, the default, and `raw`.  See
the [API reference](/api) if you are uncertain how to pass an optional argument in JavaScript.

```js
> r.now().run(conn, callback);
// Result passed to callback
Date("2013-08-13T23:32:49.923Z")

> r.now().inTimezone('-07:00').run(conn, callback);
// Result passed to callback: same as above, no TZ info retrieved
Date("2013-08-13T23:32:49.923Z")

> r.now().run(conn, {timeFormat: 'raw'}, callback);
// Result passed to callback
{
  "$reql_type$": "TIME",
  "epoch_time": 1423077622.659,
  "timezone": "+00:00"
}

> r.now().inTimezone('-07:00').run(conn, {timeFormat: 'raw'}, callback);
// Result passed to callback, now with TZ info
{
  "$reql_type$": "TIME",
  "epoch_time": 1423077646.772,
  "timezone": "-07:00"
}
```

You can also transform a time object on the server using either `toEpochTime`
or `toISO8601`.

```js
> r.now().toEpochTime().run(conn, callback);
// Result passed to callback
1376075986.574

> r.now().toISO8601().run(conn, callback);
// Result passed to callback
"2013-08-09T19:19:46.574+00:00"
```

# Working with times #

There are only three useful things you can do with a time: modify it, compare it
to another time, or retrieve a portion of it.

## Modifying times ##

You can add or subtract a duration (in seconds):

```js
> r.time(2015, 1, 1, 'Z').add(86400).run(conn, callback);
// Result passed to callback
Fri Jan 02 2015 00:00:00 GMT+00:00
```

If you subtract two times, you get a duration:

```js
> r.time(2015, 1, 2, 'Z').sub(r.time(2015, 1, 1, 'Z')).run(conn, callback);
// Result passed to callback
86400
```

## Comparing times ##

All of the normal comparison operators are defined on times:

```js
> r.epochTime(1376081287.982).lt(new Date()).run(conn, callback);
true
```

Times are only compared with millisecond precision:

```js
> r.epochTime(1376081287.9821).eq(r.epochTime(1376081287.9822)).run(conn, callback);
true
```

There's also the [during](/api/javascript/during) command, which can check whether a time is in a particular range of times.

## Retrieving portions of times ##

If you have a time, you can retrieve a particular portion (like the month, or
the hours) relative to the current time zone.  (See the full list at the
[API reference](/api).)

```js
> r.expr(new Date()).run(conn, callback);
// Result passed to callback
"2013-08-13T23:32:49.923Z"

> r.expr(new Date()).month().run(conn, callback);
// Result passed to callback
8

> r.expr(new Date()).hours().run(conn, callback);
// Result passed to callback
23

> r.expr(new Date()).inTimezone('-06:00').hours().run(conn, callback);
// Result passed to callback
17
```

We use the ISO 8601 definition of a week, which starts with Monday, represented
as `1`.

```js
> r.expr(new Date()).dayOfWeek().run(conn, callback);
5 # Friday
```

We define `r.monday...r.sunday` and `r.january...r.december` for convenience:

```js
> r.expr(new Date()).dayOfWeek().eq(r.friday).run(conn, callback);
true
```

We also let you slice the time into the date and the current time of day (a time
and a duration, respectively):

```js
> r.now().toEpochTime().run(conn, callback);
// Result passed to callback
1376351312.744

> r.now().date().toEpochTime().run(conn, callback);
// Result passed to callback
1376265600

> r.now().timeOfDay().run(conn, callback);
// Result passed to callback
85712.744
```

# Putting it all together #

By combining these operations, you can write surprisingly useful queries in pure
ReQL.  For example, let's say you have a table of sales your company has made,
and you want to figure out how much of the gross comes from people who were
working overtime:

```js
r.table('sales').filter(function (sale) {
    // Weekends are overtime
    return sale('time').dayOfWeek().eq(r.saturday).or(
        sale('time').dayOfWeek().eq(r.sunday)).or(
        // Weekdays outside 9-5 are overtime
        sale('time').hours().lt(9)).or(
        sale('time').hours().ge(17));
}).sum('dollars').run(conn, callback);
```

If your timestamps are stored with time zones, this query will work even if you
have sales from different offices in different countries (assuming they all work
9-5 local time).

Since this query is pure ReQL, the entire query will be distributed efficiently
over the cluster without any computation being done on the client.

Further, because it's ReQL, the query's individual pieces are easily
composable.  If you decide you want those numbers on a per-month
basis, you can just throw a `group` in there:

```js
r.table('sales').filter(function (sale) {
    // Weekends are overtime
    return sale('time').dayOfWeek().eq(r.saturday).or(
        sale('time').dayOfWeek().eq(r.sunday)).or(
        // Weekdays outside 9-5 are overtime
        sale('time').hours().lt(9)).or(
        sale('time').hours().ge(17));
}).group(function (sale) {
    return sale('time').month();
}).sum('dollars').run(conn, callback);
```
