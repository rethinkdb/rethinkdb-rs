---
layout: documentation
title: Geospatial support
active: docs
docs_active: geo-support
permalink: docs/geo-support/python/
alias: docs/geo-support/
switcher: true
language: Python
---

RethinkDB supports spatial and geographic queries. Geometry objects are implemented through a geographic coordinate system, with points and shapes plotted on the surface of a sphere in three-dimensional space. This is an overview of the system; for more details, consult the API documentation for individual geospatial commands.

# Getting started #

Create a new table:

```py
r.table_create('geo').run(conn)
```

Add a couple points:

```py
r.table('geo').insert([
  {
    'id': 1,
    'name': 'San Francisco',
    'location': r.point(37.779388,-122.423246)
  },
  {
    'id': 2,
    'name': 'San Diego',
    'location': r.point(32.719464,-117.220406)
  }
]).run(conn)
```

Get the distance between the two points in San Francisco and San Diego:

```py
r.table('geo').get(1)['location'].distance(
    r.table('geo').get(2)['location']).run(conn)
```

Add a geospatial index on the table (required for certain operations like `getNearest`):

```py
r.table('geo').index_create('location', geo=True)
```

Get the nearest point in the table to a specified one based on the index:

```py
point = r.point(37.777128,-122.422876)  # San Francisco
r.table('geo').get_nearest(point, index='location')
```

# Coordinate system #

Coordinates of points on the sphere's surface are addressed by a pair of floating point numbers that denote latitude and longitude. The range of latitude is &minus;90 (the south pole) through 90 (the north pole); the range of longitude is &minus;180 through 180, which wraps around the whole of the sphere: &minus;180 and 180 denote the same line.

For a more detailed explanation of this, consult the Wikipedia article on the [geographic coordinate system][gcs].

[gcs]: http://en.wikipedia.org/wiki/Geographic_coordinate_system

# Lines and distances #

Given two endpoints, a line in ReQL is the shortest path between those endpoints on the surface of the sphere, known as a [geodesic]. Lines can be defined with multiple points, in which case each segment of the line will be a geodesic; likewise, sides of a polygon will be geodesics. Geodesics are calculated assuming a perfect sphere.

[geodesic]: http://en.wikipedia.org/wiki/Geodesic

Note that a line between the north pole and south pole (from latitude &minus;90 to latitude 90) cannot be calculated, as *all* possible paths between them are the "shortest"; this may trigger an error in ReQL or it may choose an arbitrary (but technically correct) path.

Distances in ReQL are (by default) calculated assuming not a perfect sphere but an ellipsoid, using a precise and relatively fast algorithm developed by [Charles Karney][ck]. The reference ellipsoid used is [WGS84][], the standard used for GPS. By default distances are specified in meters, but you can pass an optional argument to distance functions to specify kilometers, miles, nautical miles, and feet.

[ck]: http://link.springer.com/article/10.1007%2Fs00190-012-0578-z "Algorithms for geodesics"
[WGS84]: http://en.wikipedia.org/wiki/World_Geodetic_System

# Data types #

The geospatial functions are implemented through a set of new geometric object data types:

* **Points:** a single coordinate pair
* **Lines:** A sequence of two or more coordinate pairs
* **Polygons:** A multipoint line (at least three coordinate pairs) which does not intersect with itself and whose first and last coordinate pairs are equal. The interior of the polygon is considered filled, that is, part of the polygon. Polygons with "holes" in them, where a hole is another polygon contained by the first, can be created with the [polygon_sub][] command.

In addition, there's a "pseudotype" called **geometry** which appears in documentation, to indicate that any of the geometric objects can be used with those commands.

[polygon_sub]: /api/python/polygon_sub/

Lines and polygons can be specified using either point objects or sequences of two-number arrays:

```js
r.line(r.point(0,0), r.point(0,5), r.point(5,5), r.point(5,0), r.point(0,0))
r.line([0,0], [0,5], [5,5], [5,0], [0,0])
```

Both of those define the same square. If `polygon` had been specified instead of `line` they would define a filled square.

While there *is* a [circle] command, it approximates a circle by defining either a line or a polygon. There is no true circular data type.

# Geospatial indexes #

To create indexes on fields containing geometry objects, you simply use the standard [index_create](/api/python/index_create/) command, setting the `geo` optional argument to `Prue`. In JavaScript, this would be:

```js
r.table('sites').indexCreate('locations', {geo: true})
```

Just like other ReQL indexes, you can create an index using an anonymous function rather than a simple field name, as well as create multi indexes by using the `multi` flag with `geo`. Read the [index_create](/api/python/index_create) API documentation for more details.

# Using GeoJSON #

ReQL geometry objects are not [GeoJSON][] objects, but you can convert back and forth between them with the [geojson](/api/python/geojson/) and [to_geojson](/api/python/to_geojson) commands.

[GeoJSON]: http://geojson.org

RethinkDB only allows conversion of GeoJSON objects which have ReQL equivalents: Point, LineString, and Polygon; MultiPoint, MultiLineString, and MultiPolygon are not supported. (You could, however, store multiple points, lines and polygons in an array and use a geospatial multi index with them.)

Only latitude/longitude coordinates are supported. GeoJSON objects that use Cartesian coordinates, specify an altitude, or specify their own coordinate reference system will be rejected.

# Geospatial commands #

* [geojson](/api/python/geojson/): convert a GeoJSON object to a geometry object
* [to_geojson](to_geojson/)/[to_geojson](/api/python/to_geojson/): convert a geometry object to a GeJSON object
* [point](/api/python/point/): create a point object
* [line](/api/python/line/): create a line object
* [polygon](/api/python/polygon/): create a line object
* [circle](/api/python/circle/): create a line or polygon that approximates a circle
* [distance](/api/python/distance/): compute the distance between a point and another geometry object
* [intersects](/api/python/intersects/): determine whether two geometry objects intersect
* [includes](/api/python/includes/): determine whether one geometry object is completely contained by a polygon object
* [get_intersecting](/api/python/get_intersecting/): return documents from a sequence that have a geospatially indexed field whose values intersect with a given geometry object
* [get_nearest](/api/python/get_nearest/): return documents from a sequence that have a geospatially indexed field whose values are within a specified distance of a given point
* [polygon_sub](/api/python/polygon_sub/): use one polygon completely contained within another to cut out a "hole" in the enclosing polygon
