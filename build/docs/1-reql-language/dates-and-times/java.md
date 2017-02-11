---
layout: documentation
title: Dates and times in RethinkDB
docs_active: dates-and-times
permalink: docs/dates-and-times/java/
alias: docs/dates-and-times/
switcher: true
language: Java
---


RethinkDB has native support for millisecond-precision times with time zones. Some highlights:

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

First, let's create a table and insert some events.  We'll insert the first event using a native OffsetDateTime object, and the second with the `epochTime` constructor:

```java
import java.time.OffsetDateTime;
import java.util.List;

r.tableCreate("events").run(conn);

OffsetDateTime nowDateTime = OffsetDateTime.now();

r.table("events").insert(r.array(
    r.hashMap("id", 0).with("timestamp", nowDateTime),
    r.hashMap("id", 1).with("timestamp", r.epochTime(1376436769.923))
)).run(conn);
```

Now, let's get those back:

```java
Cursor cursor = r.table("events").run(conn);
List events = cursor.toList();
System.out.println(events);
```

Result:

```
[{id=0, timestamp=2016-01-05T10:41:45.100-08:00}, {id=1, timestamp=2013-08-13T23:32:49.923Z}]
```

Both times are returned as native Java 8 `OffsetDateTime` objects.

We can now filter based on these times:

```java
cursor = r.table("events").filter(
    row -> row.g("timestamp").hours().gt(20)
).run(conn);
events = cursor.toList();
System.out.println(events);
```

```
[{id=1, timestamp=2013-08-13T23:32:49.923Z}]
```

Or create a secondary index on them:

```java
r.table("events").indexCreate("timestamp").run(conn);

cursor = r.table("events").between(
    r.epochTime(1376436769.913), r.epochTime(1376436769.933)
).optArg("index", "timestamp").run(conn);
events = cursor.toList();
System.out.println(events);
```

```
[{id=1, timestamp=2013-08-13T23:32:49.923Z}]
```

# Technical details #

Times are stored on the server as seconds since epoch (UTC) with millisecond
precision plus a time zone.  Currently the only available time zones are
minute-precision time offsets from UTC, but we may add support for DST-aware
time zones in the future.  Time zones are strings as specified by ISO
8601.

Times are considered equal when their epoch (UTC) time values are equal, **regardless of what time zone they're in**. This is true for both comparisons and indexed operations. Times are compared in floating point with millisecond precision.

Most date operations are only defined on years in the range `[1400, 10000]` (but
note that times in the year `10000` cannot be printed as ISO 8601 dates).

Leap-seconds aren't well-supported right now: `2012-06-30T23:59:60` and
`2012-07-01T00:00:00` parse to the same time.

# Inserting times #

You can insert times by simply passing a native `OffsetDateTime` object. 

```java
OffsetDateTime myDateTime = OffsetDateTime.now();

r.table("events").insert(
    r.hashMap("id", 2).with("timestamp", myDateTime),
).run(conn);
```

```
{unchanged=0, skipped=0, replaced=0, inserted=1, errors=0, deleted=0}
```

You can also use `r.now` (which the server interprets as the time the
query was received in UTC), or construct a time using `r.time`,
`r.epochTime`, or `r.ISO8601`.

```java
r.now().toISO8601().run(conn, callback);
// returns "2013-08-09T18:53:15.012+00:00"

r.time(2013, r.august(), 9, 18, 53, 15.012, "-07:00").toIso8601().run(conn);
// returns "2013-08-09T18:53:15.012-07:00"

r.epochTime(1376074395.012).toIso8601().run(conn);
// returns "2013-08-09T18:53:15.012+00:00"

r.iso8601("2013-08-09T18:53:15.012-07:00").toIso8601().run(conn);
// returns "2013-08-09T18:53:15.012-07:00"
```

Times may be used as the primary key for a table.  Two times are considered
equal if they have the same number of milliseconds since epoch (UTC), regardless
of time zone.

```js
r.table("t").insert(
    r.hashMap("id", r.iso8601("2013-08-09T11:58:00.1111-07:00"))
).run(conn);

// returns:
// {deleted=0, errors=0, inserted=1, replaced=0, skipped=0, unchanged=0}

r.table("t").insert(
    r.hashMap("id", r.iso8601("2013-08-09T10:58:00.1112-08:00"))
).run(conn);

// returns: 
// {deleted=0, errors=1, inserted=0, replaced: 0, skipped=0, unchanged=0, first_error="Duplicate primary key `id`=..."}
```

You may also insert a time by inserting a literal pseudotype object.  This is
useful if, for instance, you exported a row using `{timeFormat: 'raw'}` (see
<strong>Retrieving Times</strong> below).

{% infobox %}
__Note:__ Avoid using keys matching the regular expression
`^\$reql_.+\$$` in your objects.  RethinkDB considers those to be reserved
keywords.
{% endinfobox %}

```java
r.expr(
    r.hashMap("$reql_type$", "TIME")
     .with("epoch_time", 1376075362.662)
     .with("timezone", "+00:00")
).toIso8601().run(conn);
```

# Retrieving times #

By default, times are converted into native objects when they are retrieved from the server.  This may be overridden by passing the [optArg](/api/java/optarg) `timeFormat` to `run`.  The only options right now are `native`, the default, and `raw`.

```java
r.now().run(conn);
// returns "2016-01-06T00:34:13.623Z"

r.now().inTimezone("-07:00").run(conn);
// returns "2016-01-05T17:34:13.623Z-07:00"

import com.rethinkdb.model.OptArgs;
r.now().run(conn, OptArgs.of("time_format", "raw"));
// returns:
// {"timezone":"+00:00","$reql_type$":"TIME","epoch_time":1.452040701881E9}

r.now().inTimezone("-07:00").run(conn, OptArgs.of("time_format", "raw"));
// returns:
// {"timezone":"-07:00","$reql_type$":"TIME","epoch_time":1.452040701881E9}
```

You can also transform a time object on the server using either `toEpochTime` or `toIso8601`.

```java
r.now().toEpochTime().run(conn);
// returns 1376075986.574

r.now().toISO8601().run(conn);
// returns "2013-08-09T19:19:46.574+00:00"
```

# Working with times #

There are only three useful things you can do with a time: modify it, compare it
to another time, or retrieve a portion of it.

## Modifying times ##

You can add or subtract a duration (in seconds):

```java
r.time(2015, 1, 1, "Z").add(86400).run(conn);
// returns "2015-01-02T00:00Z"
```

If you subtract two times, you get a duration:

```java
r.time(2015, 1, 2, "Z").sub(r.time(2015, 1, 1, "Z")).run(conn);
// returns 86400
```

## Comparing times ##

All of the normal comparison operators are defined on times:

```java
r.epochTime(1376081287.982).lt(new Date()).run(conn, callback);
// true
```

Times are only compared with millisecond precision:

```java
r.epochTime(1376081287.9821).eq(r.epochTime(1376081287.9822)).run(conn);
// true
```

There's also the [during](/api/java/during) command, which can check whether a time is in a particular range of times.

## Retrieving portions of times ##

If you have a time, you can retrieve a particular portion (like the month, or
the hours) relative to the current time zone.  (See the full list at the
[API reference](/api).)

```java
OffsetDateTime nowDateTime = OffsetDateTime.now();

r.expr(nowDateTime).run(conn);
// returns "2013-08-13T23:32:49.923Z"

r.expr(nowDateTime).month().run(conn);
// returns 8

r.expr(nowDateTime).hours().run(conn);
// returns 23

r.expr(nowDateTime).inTimezone("-06:00").hours()run(conn);
// returns 17
```

We use the ISO 8601 definition of a week, which starts with Monday, represented
as `1`.

```java
r.expr(nowDateTime).dayOfWeek().run(conn);
// returns 2 for Tuesday
```

We define `r.monday...r.sunday` and `r.january...r.december` for convenience:

```js
r.expr(nowDateTime).dayOfWeek().eq(r.tuesday).run(conn);
// returns true
```

We also let you slice the time into the date and the current time of day (a time
and a duration, respectively):

```java
r.now().toEpochTime().run(conn);
// returns 1376351312.744

r.now().date().toEpochTime().run(conn);
// returns 1376265600

r.now().timeOfDay().run(conn);
// returns 85712.744
```

# Putting it all together #

By combining these operations, you can write surprisingly useful queries in pure
ReQL.  For example, let's say you have a table of sales your company has made,
and you want to figure out how much of the gross comes from people who were
working overtime:

```java
r.table("sales").filter(sale ->
    // Weekends are overtime
    sale.g("time").dayOfWeek().eq(r.saturday())
    .or(sale.g("time").dayOfWeek().eq(r.sunday()))
    // Weekdays outside 9-5 are overtime
    .or(sale.g("time").hours().lt(9))
    .or(sale.g("time").hours().ge(17))
).sum("dollars").run(conn);
```

If your timestamps are stored with time zones, this query will work even if you
have sales from different offices in different countries (assuming they all work
9-5 local time).

Since this query is pure ReQL, the entire query will be distributed efficiently
over the cluster without any computation being done on the client.

Further, because it's ReQL, the query's individual pieces are easily
composable.  If you decide you want those numbers on a per-month
basis, you can just throw a `group` in there:

```java
r.table("sales").filter(sale ->
    // Weekends are overtime
    sale.g("time").dayOfWeek().eq(r.saturday())
    .or(sale.g("time").dayOfWeek().eq(r.sunday()))
    // Weekdays outside 9-5 are overtime
    .or(sale.g("time").hours().lt(9))
    .or(sale.g("time").hours().ge(17))
).group(sale -> sale.g("time").month()).sum("dollars").run(conn);
```
