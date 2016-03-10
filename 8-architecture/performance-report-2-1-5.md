---
layout: documentation
title: 'RethinkDB Performance & Scaling Report'
docs_active: performance-scaling-report
permalink: docs/performance-scaling-report/
---

# Performance & Scaling of RethinkDB

What sort of performance can I expect from a RethinkDB cluster? Does RethinkDB scale? Can I use the outdated read mode to improve throughput in read-heavy workloads?
We’ll answer these questions by using a workloads from the YCSB benchmark suite. You can learn more about YCSB here, and review the source code here. An additional test investigates scalability for analytical workloads.
In the results, we’ll see how RethinkDB 2.1.5 can be scaled to perform 1.3 million individual reads per second. We will also demonstrate well above 100 thousand operations per second in a mixed 50:50 read/write workload - while at the full level of durability and data integrity guarantees. All benchmarks are performed across a range of cluster sizes, scaling up to 16 servers.
A quick overview of the results

Getting right down to the details, we found that in a mixed read/write workload RethinkDB with two servers was able to perform nearly 16K queries per second (QPS) and scaled to almost 120K QPS while in a 16 server cluster. Under a “read only” workload and synchronous read settings, RethinkDB was able to scale from about 150,000 QPS up to over 550K QPS. Under the same workload, in an asynchronous “outdated read” setting, RethinkDB went from 150K QPS on one server to 1.3M in a 16 server cluster.
Finally, we used a map-reduce query to compute word counts across the whole data set. This test evaluates RethinkDB's scalability for analytical workloads.
Here we we show how RethinkDB scales up to 16 servers with these various workloads.

![](/assets/images/docs/performance-report/w-a.png)
![](/assets/images/docs/performance-report/w-c-sync.png)
![](/assets/images/docs/performance-report/w-c-async.png)
![](/assets/images/docs/performance-report/analytical.png)

## Workloads, Clusters, and Hardware. Oh My.

YCSB comes with a variety of workloads in which to perform, but for the purposes of our testing we chose two of them to run against RethinkDB. Workload A is a 50% read and 50% update workload and workload C, which is read-only. All documents stored by the YCSB tests are composed of documents with 10 fields with randomized 100 byte strings as values totaling about 1 KB in size.
We used a port of YCSB based on our official Java driver and intend to submit a pull request for it in the near future. Our fork is available for review here.
As for clustering, we wanted to show what happens in terms of performance when you move from a single RethinkDB instance to a larger cluster. So we tested all the workloads on a single instance, up to a sixteen server cluster in varying increments of cluster size.
Hardware

In terms of hardware, we used the OnMetal offerings from Rackspace to run both RethinkDB server and RethinkDB client nodes. We used different hardware configurations for the server and client nodes as shown below:

1-16 RethinkDB servers | 8 RethinkDB clients
-----------------------|--------------------
Rackspace OnMetal I/O                             | Rackspace OnMetal Compute
2x Intel Xeon E5-2680 v2 CPU 2.8 GHz (2x10 cores) | Intel Xeon E5-2680 v2 CPU 2.8 GHz (10 cores)
128 GB RAM                                        | 32 GB RAM
10Gbps Ethernet                                   | 10Gbps Ethernet
Seagate Nytro WarpDrive BLP4-1600 storage         |
http://www.rackspace.com/cloud/servers/onmetal/


## Configuration

At the time of the test, we used RethinkDB 2.1.5 for the test which was compiled on Ubuntu 14.04 LTS. We used the at the time, unreleased RethinkDB Java driver with Oracle Java 1.8.0. A full list of configuration settings are listed here:
RethinkDB version 2.1.5

```
Ubuntu 14.04
RethinkDB cache size set to 64,000 MB per server, otherwise default parameters
Oracle Java 1.8.0 on the client nodes
RethinkDB port of YCSB used:
https://github.com/rethinkdb/YCSB at commit a15e249d6b10147e615ddfaf03672bad35e85e7f
```

## Detailed Results

### Workload A

* Simulates a mixed read/write workload with equally many writes as reads
* Query types:  50% single-document gets, 50% single-document updates
* Zipfian key access distribution
* 128 client connections per server
* Data is replicated to two servers, and sharded across all available servers
* Writes are performed with “hard” durability (wait for data to be on disk on both replicas)
* Performed 50 million operations in total

Our first workload from YCSB is workload “A”. It performs an equal number of get
and update operations. The data set consists of 25 million documents sized at 1 KB
each. All data fits into cache in this scenario. Reads and writes are performed by
eight client servers, with 128 concurrent connections per database server. This
means we have 128 connections with just a single RethinkDB server, and 2048
concurrent connections with a 16 server cluster. We used a replication factor of
two per table meaning each document was replicated to two separate servers.


RethinkDB achieves a throughput of 14,600 queries per second with two servers, and
scales near-linearly as servers are added to the cluster. Latency is also an
important metric to measure when doing performance testing. We’ve included a graph
showing the latency of writes in Workload A given a 16 server RethinkDB cluster. We
can see here that a majority of writes take between 1ms and 5ms.

![](/assets/images/docs/performance-report/w-a-reads-latency.png)

### Workload C

* Simulates a read-only workload
* Query types: Single-document gets
* Zipfian key access distribution
* 128 client connections per server
* Data is replicated and sharded across all available servers
* In the “synchronous” test, we use the default `{readMode: ”single”}` setting
* In the “asynchronous” test, reads use the `{readMode: ”outdated”}` setting
* Performed 200 million operations in total

This workload performs exclusively get operations to retrieve individual documents from the database (YCSB workload “C”). We use the same setup and data set as for the read/write workload above.
Reads are performed from 8 client servers with 128 concurrent connections per database server.

RethinkDB is able to perform 134,700 queries per second on a single server. While the overhead of network communication between the servers becomes visible when increasing the cluster size from one to two servers, adding further servers to the cluster demonstrates the near-linear scalability of RethinkDB, up to over half a million queries per second on 16 servers.

As a variation of this workload, we also tested scalability in the “outdated” read mode. In this mode, the chance of slightly outdated read results can be traded for additional performance, as read operations can be handled directly by secondary replicas.

A typical application for this mode would be for an asynchronous cache.
RethinkDB demonstrates extremely high scalability in this configuration, reaching throughputs of well over a million queries per second. The slightly sub-linear scalability when going from 12 to 16 database servers is caused by the client servers' CPUs getting saturated at these throughputs. In an actual deployment setting, we recommend setting up a RethinkDB proxy on the same machine as your application to reduce load on data-storing RethinkDB instances.

![](/assets/images/docs/performance-report/w-c-reads-latency.png)

### Analytical queries

* Tests the response time for analytical map/reduce queries involving string operations
* Query types:  Count the total number of sentences over a single field:
table.map(r.row("field0").split(".").count()).sum()
* We run one query at a time. Results show the average over five runs

Finally, we demonstrate RethinkDB's automatic query parallelization. Analytical queries are transparently distributed over servers in the cluster in a map/reduce style pattern. In this example, we count the number of sentences over the whole data set of 25 million documents based on one of the fields. We use the following map/reduce style query

```
table.map( r.row("field0").split(".").count() ).sum()
```

We run one query at a time and measure its completion time. The numbers below show the average over five runs.
With a single server, our query takes 59 seconds to complete. The automatic query parallelization in RethinkDB results in practically linear scalability, with the same query being executed in just above 4 seconds on 16 servers. The graph shown in the results overview section, shows the inverse execution time (queries per second) of the query.
Conclusion

In conclusion, we wanted to provide a reasonably comprehensive RethinkDB test that covers a variety of different work loads. Given limited time we chose using the YCSB database testing framework as a reliable and crowd-approved means of giving our database a rigorous testing. Ultimately, we saw that most of the tests resulted in near-linear scalability as we moved from a single RethinkDB instance, to a 16 sized cluster. While we did see that most of the tests resulted in performance metrics that suggest linear horizontal scalability we know that there are plenty of improvements to make as our database evolves. Namely, we have a few performance improvements coming down the line in RethinkDB 2.3 where we will again publish our next set of performance metrics during the lifetime of  the 2.3 release and furthermore measure performance while scaling up to 16 servers and beyond.

### Notes

* We were fortunate enough to receive free credits from Rackspace to perform the majority of these tests and are very grateful for their contributions to open source software. All of Rackspace’s OnMetal offerings can be found here.
* We’d love to answer any questions you have about these tests. Come join us at http://slack.rethinkdb.com and feel free to ask more specific questions we don’t answer here by pinging @danielmewes or @dalanmiller.
