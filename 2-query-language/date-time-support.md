---
layout: documentation
title: Dates and times in RethinkDB
active: docs
docs_active: dates-and-times
permalink: docs/dates-and-times/
---

<img alt="Dates and Times Illustration" class="api_command_illustration"
     src="/assets/images/docs/api_illustrations/dates-and-times.png" />

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

# A quick example #

{% infobox info %}
<strong>Note:</strong> Examples below are in Ruby.  Head to the [API
reference](/api) to see the commands in other languages.
{% endinfobox %}

First, let's create a table and insert some events.  We'll insert the first
event using a native time object, and the second with the `epoch_time`
constructor:

```ruby
> r.table_create('ev').run(conn)
{"created"=>1}
> r.table('events').insert(
    [{'id' => 0, 'timestamp' => Time.now},
     {'id' => 1, 'timestamp' => r.epoch_time(1376436769.923)}]
  ).run(conn)
{"unchanged"=>0, "skipped"=>0, "replaced"=>0, "inserted"=>2, "errors"=>0, "deleted"=>0}
```

Now, let's get those back:

```ruby
> r.table('events').run(conn).to_a
[{"timestamp"=>2013-08-13 16:32:48 -0700, "id"=>0},
 {"timestamp"=>2013-08-13 23:32:49 +0000, "id"=>1}]
```

You'll notice that both times we inserted are returned as native Ruby `Time`
objects.  They're in different time zones because `Time.now` creates a time
object in the local time zone, but `r.epoch_time` creates a UTC time (it doesn't
know or care what time zone the client is in).  If we had instead inserted
`Time.now.utc`, they'd both be in UTC when we retrieved them.

We can now filter based on these times:

```ruby
> r.table('events').filter{|row| row['timestamp'].hours() > 20}.run(conn)
[{"timestamp"=>2013-08-13 23:32:49 +0000, "id"=>1}]
> r.table('events').filter{|row|
    row['timestamp'].in_timezone('-02:00').hours() > 20
  }.run(conn)
[{"timestamp"=>2013-08-13 16:32:48 -0700, "id"=>0},
 {"timestamp"=>2013-08-13 23:32:49 +0000, "id"=>1}]
```

Or create a secondary index on them:

```ruby
> r.table('events').index_create('timestamp').run(conn)
{"created"=>1}
> r.table('events').between(r.epoch_time(1376436769.913),
                            r.epoch_time(1376436769.933),
                            :index => 'timestamp').run(conn)
[{"timestamp"=>2013-08-13 23:32:49 +0000, "id"=>1}]
```

# Technical details #

Times are stored on the server as seconds since epoch (UTC) with millisecond
precision plus a time zone.  Currently the only available time zones are
minute-precision time offsets from UTC, but we may add support for DST-aware
time zones in the future.  Time zones are strings as specified by ISO
8601.

Times are considered equal if their seconds since epoch (UTC) are equal,
<strong>regardless of what time zone they're in</strong>.  This is true for both
comparisons and indexed operations.

Most date operations are only defined on years in the range `[1400, 10000]` (but
note that times in the year `10000` cannot be printed as ISO 8601 dates).

Leap-seconds aren't well-supported right now: `2012-06-30T23:59:60` and
`2012-07-01T00:00:00` parse to the same time.

# Inserting times #

You can insert times by simply passing a native time object. (In Ruby, this
will be a `Time` object; in Python, it will be `datetime.datetime`; in
Javascript, it will be `Date`.) If the local time object contains a time zone,
the inserted time will have that time zone; if it doesn't, the inserted time
will be in UTC. (Check this if you're using a third-party driver. The Python
driver requires `datetime` objects to have time zone information.)

```ruby
> r.table('events').insert({'id' => 2, 'timestamp' => Time.now}).run(conn)
{"unchanged"=>0, "skipped"=>0, "replaced"=>0, "inserted"=>1, "errors"=>0, "deleted"=>0}
```

You can also use `r.now` (which the server interprets as the time the
query was received in UTC), or construct a time using `r.time`,
`r.epoch_time`, or `r.iso8601`.

```ruby
> r.now().to_iso8601().run(conn)
"2013-08-09T18:53:15.012+00:00"
> r.time(2013, r.august, 9, 18, 53, 15.012, '-07:00').to_iso8601().run(conn)
"2013-08-09T18:53:15.012-07:00"
> r.epoch_time(1376074395.012).to_iso8601().run(conn)
"2013-08-09T18:53:15.012+00:00"
> r.iso8601("2013-08-09T18:53:15.012-07:00").to_iso8601().run(conn)
"2013-08-09T18:53:15.012-07:00"
```

Times may be used as the primary key for a table.  Two times are considered
equal if they have the same number of milliseconds since epoch (UTC), regardless
of time zone.

```ruby
> r.table('t').insert({'id' => r.iso8601("2013-08-09T11:58:00.1111-07:00")}).run(conn)
{"unchanged"=>0, "skipped"=>0, "replaced"=>0, "inserted"=>1, "errors"=>0, "deleted"=>0}
> r.table('t').insert({'id' => r.iso8601("2013-08-09T10:58:00.1112-08:00")}).run(conn)
{"unchanged"=>0, "skipped"=>0, "replaced"=>0, "inserted"=>0,
 "first_error"=>"Duplicate primary key.", "errors"=>1, "deleted"=>0}
```

You may also insert a time by inserting a literal pseudotype object.  This is
useful if, for instance, you exported a row using `:time_format => 'raw'` (see
<strong>Retrieving Times</strong> below).

{% infobox info %}
<strong>Note:</strong> Avoid using keys matching the regular expression
`^\$reql_.+\$$` in your objects.  RethinkDB considers those to be reserved
keywords.
{% endinfobox %}

```ruby
> r.expr({'$reql_type$' => 'TIME',
          'epoch_time' => 1376075362.662,
          'timezone' => '+00:00'}).to_iso8601().run(conn)
"2013-08-09T19:09:22.662+00:00"
```

# Retrieving times #

By default, times are converted into native time objects when they are retrieved
from the server.  This may be overridden by passing the optarg `time_format` to
`run`.  The only options right now are `native`, the default, and `raw`.  See
the [API reference](/api) if you are uncertain how to pass an optarg in a
non-Ruby language.

{% infobox info %}
<strong>Warning:</strong> Some languages, like Javascript, don't have an easy
way to represent a time in an arbitrary time zone.  In this case, time zone
information will be discarded when converting to a native time object.
{% endinfobox %}

```ruby
> r.now().run(conn)
2013-08-09 19:09:11 UTC
> r.now().in_timezone('-07:00').run(conn)
2013-08-09 12:14:56 -0700
> r.now().run(conn, :time_format => 'raw')
{'timezone'=>'+00:00', "epoch_time"=>1376075362.662, "$reql_type$"=>"TIME"}
> r.now().in_timezone('-07:00').run(conn, :time_format => 'raw')
{"timezone"=>"-07:00", "epoch_time"=>1376075702.485, "$reql_type$"=>"TIME"}
```

You can also transform a time object on the server using either `to_epoch_time`
or `to_iso8601`.

```ruby
> r.now().to_epoch_time().run(conn)
1376075986.574
> r.now().to_iso8601().run(conn)
"2013-08-09T19:19:46.574+00:00"
```

# Working with times #

There are only three useful things you can do with a time: modify it, compare it
to another time, or retrieve a portion of it.

## Modifying times ##

You can put a time into a new time zone:

```ruby
> r.expr(Time.now).to_iso8601().run(conn)
"2013-08-09T12:48:59.103-07:00"
> r.expr(Time.now).in_timezone('-06:00').to_iso8601().run(conn)
"2013-08-09T13:49:15.503-06:00"
```

You can also add or subtract a duration (in seconds):

```ruby
> (r.epoch_time(123.456) + 123.456).to_epoch_time().run(conn)
246.912
```

If you subtract two times, you get a duration:

```ruby
> (r.epoch_time(246.912) - r.epoch_time(123.456)).run(conn)
123.456
```

## Comparing times ##

All of the normal comparison operators are defined on times:

```ruby
> (r.epoch_time(1376081287.982) < Time.now).run(conn)
true
```

Times are only compared with millisecond precision:

```ruby
> r.epoch_time(1376081287.9821).eq(r.epoch_time(1376081287.9822)).run(conn)
true
```

There's also the `during` command which is convenient for checking whether a time is
in a particular range of times.  See more at the [API reference](/api).

## Retrieving portions of times ##

If you have a time, you can retrieve a particular portion (like the month, or
the hours) relative to the current time zone.  (See the full list at the
[API reference](/api).)

```ruby
> r.expr(Time.now).run(conn)
2013-08-09 13:53:00 -0700
> r.expr(Time.now).month().run(conn)
8
> r.expr(Time.now).hours().run(conn)
13
> r.expr(Time.now).in_timezone('-06:00').hours().run(conn)
14
```

We use the ISO 8601 definition of a week, which starts with Monday, represented
as `1`.

```ruby
> r.expr(Time.now).day_of_week().run(conn)
5 # Friday
```

We define `r.monday...r.sunday` and `r.january...r.december` for convenience:

```ruby
> r.expr(Time.now).day_of_week().eq(r.friday).run(conn)
true
```

We also let you slice the time into the date and the current time of day (a time
and a duration, respectively):

```ruby
> r.now().to_epoch_time().run(conn)
1376351312.744
> r.now().date().to_epoch_time().run(conn)
1376265600
> r.now().time_of_day().run(conn)
85712.744
```

# Putting it all together #

By combining these operations, you can write surprisingly useful queries in pure
ReQL.  For example, let's say you have a table of sales your company has made,
and you want to figure out how much of the gross comes from people who were
working overtime:

```ruby
r.table('sales').filter {|sale|
  # Weekends are overtime.
  sale['time'].day_of_week().eq(r.saturday) |
  sale['time'].day_of_week().eq(r.sunday) |
  # Weekdays outside 9-5 are overtime.
  (sale['time'].hours() < 9) |
  (sale['time'].hours() >= 17)
}.sum('dollars').run(conn)
```

If your timestamps are stored with time zones, this query will work even if you
have sales from different offices in different countries (assuming they all work
9-5 local time).

Since this query is pure ReQL, the entire query will be distributed efficiently
over the cluster without any computation being done on the client.

Further, because it's ReQL, the query's individual pieces are easily
composable.  If you decide you want those numbers on a per-month
basis, you can just throw a `group` in there:

```ruby
r.table('sales').filter {|sale|
  # Weekends are overtime.
  sale['time'].day_of_week().eq(r.saturday) |
  sale['time'].day_of_week().eq(r.sunday) |
  # Weekdays outside 9-5 are overtime.
  (sale['time'].hours() < 9) |
  (sale['time'].hours() >= 17)
}.group{|sale| sale['time'].month()}.sum('dollars').run(conn)
```

<a id="native-time-objects"></a>
# Working with native time objects

## Python
RethinkDB accepts Python `datetime` objects:

```py
from datetime import datetime
```

The Python driver will throw an error if you pass it a `datetime`
without a time zone.  (RethinkDB only stores times with time zones.)
If you try to run:

```py
r.expr(datetime.now()).run(conn)
```

You will get the following error:

```
RqlDriverError: Cannot convert datetime to ReQL time object
without timezone information. You can add timezone information with
the third party module "pytz" or by constructing ReQL compatible
timezone values with r.make_timezone("[+-]HH:MM"). Alternatively,
use one of ReQL's builtin time constructors, r.now, r.time, or r.iso8601.
```

To pass a valid time object to the Python driver, you can:

- Use `r.make_timezone`

    ```py
    r.expr(datetime.now(r.make_timezone('-07:00'))).run(conn)
    ```

- Use the `pytz` module

    ```py
    from pytz import timezone
    r.expr(datetime.now(timezone('US/Pacific'))).run(conn)
    ```


## JavaScript
RethinkDB accepts JavaScript `Date` objects:

```js
r.expr(new Date()).run(conn, callback)
```

In JavaScript, `Date` objects store the epoch time, but not the time
zone.  As a result, the JavaScript driver will not send any time zones
to RethinkDB, and will discard the time zones on any time objects it
retrieves from RethinkDB (the epoch time will still be correct).  If
you need to access the time zone of a time stored in RethinkDB, you
can retrieve the raw time object like so:

```js
r.expr(new Date()).run({connection: conn, timeFormat: "raw"}, callback)
```

## Ruby
RethinkDB accepts Ruby `Time` objects.  (Note that we only support Ruby 1.9+.)

```rb
r.expr(Time.now).run(conn)
```
