---
layout: documentation
title: RethinkDB 2.1.5 Performance & Scaling Report
docs_active: 2-1-5-performance-report
permalink: docs/performance-reports/2-1-5-performance-report/
---

We are happy to present our first published RethinkDB performance report to the world. After an internal collaborative effort we can reveal what we’ve discovered about the performance of RethinkDB. Some of the questions you might be looking to address may include:

* What sort of performance can I expect from a RethinkDB cluster?
* Does RethinkDB scale?
* Can I use the outdated read mode to improve throughput in read-heavy workloads?

We’ll attempt to answer these questions by using workloads from the YCSB benchmark suite. You can learn more about YCSB here, and review the source code here. We created an additional test which investigates scalability for analytical workloads.
In the results, we’ll see how RethinkDB 2.1.5 scales to perform 1.3 million individual reads per second. We will also demonstrate well above 100 thousand operations per second in a mixed 50:50 read/write workload - while at the full level of durability and data integrity guarantees. We performed all benchmarks across a range of cluster sizes, scaling up to 16 servers.

## A quick overview of the results

We found that in a mixed read/write workload RethinkDB with two servers was able to perform nearly 16K queries per second (QPS) and scaled to almost 120K QPS while in a 16 server cluster. Under a read only workload and synchronous read settings, RethinkDB was able to scale from about 150,000 QPS on a single node up to over 550K QPS on 16 nodes. Under the same workload, in an asynchronous “outdated read” setting, RethinkDB went from 150K QPS on one server to 1.3M in a 16 node cluster.

Finally, we used a MapReduce query to compute word counts across the whole data set. This test evaluates RethinkDB's scalability for analytical workloads in a simplistic but very common fashion. These types of workloads involve doing information processing on the server itself versus typical single or ranged reads and writes of information processed at the application level.

Here we we show how RethinkDB scales up to 16 servers with these various workloads:

![](/assets/images/docs/performance-report/w-a.png)
![](/assets/images/docs/performance-report/w-c-sync.png)
![](/assets/images/docs/performance-report/w-c-async.png)
![](/assets/images/docs/performance-report/analytical.png)

## Workloads, Clusters, and Hardware. Oh My.

YCSB comes with a variety of default workloads, but for the purposes of our testing we chose two of them to run against RethinkDB. Out of the YCSB workload options, we chose to run workload A which comprises 50% reads and 50% update operations and workload C which performs strictly read operations. All documents stored by the YCSB tests contain 10 fields with randomized 100 byte strings as values with each document totaling about 1 KB in size.

We used a port of YCSB based on our official Java driver and intend to submit a pull request for it in the near future. [Our fork is available for review here](https://github.com/rethinkdb/ycsb).

Given the ease of RethinkDB to cluster across multiple instances, we deemed it necessary to observe performance when moving from a single RethinkDB instance to a larger cluster. We tested all of our workloads on a single instance of RethinkDB up to a 16 server cluster in varying increments of cluster size.

## Hardware

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

At the time of the test, we used RethinkDB 2.1.5 which was compiled from source on Ubuntu 14.04 LTS. During the performance test we used the RethinkDB Java driver with Oracle Java 1.8.0. A full list of configuration settings follows below:
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
* Query types:  50% single-document read ops, 50% single-document update ops
* [Zipfian](https://en.wikipedia.org/wiki/Zipf%27s_law) key access distribution
* 128 client connections per server
* Data is replicated to two servers, and sharded across all available servers
* Writes are performed with “hard” durability (wait for data to be on disk on both replicas)
* Performed 50 million operations in total


Our first workload from YCSB is workload A. It performs an equal number of get and update operations.

The data set generated by YCSB consists of 25 million documents sized at 1 KB each. All data fits into the server cache in this scenario.

Reads and writes are performed by eight client servers, with 128 concurrent connections per database server. This means we have 128 connections with just a single RethinkDB server, and 2048 concurrent connections with a 16 server cluster. We used a replication factor of two per table, meaning each document was replicated to two separate servers.

RethinkDB achieves a throughput of 14,600 QPS with two servers, and scales near-linearly as servers are added to the cluster.

Latency is also an important metric to measure when testing performance. We’ve included a graph that shows the latency of writes in Workload A given a 16 server RethinkDB cluster.

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

This workload performs exclusively read operations to retrieve individual documents from the database (YCSB workload C). This workload uses the same setup and data set as workload A above. Also identical, reads are performed from 8 client servers with 128 concurrent connections per database server in the cluster.


We first tested this workload in the [default configuration for RethinkDB which forbids stale reads](https://www.rethinkdb.com/api/javascript/run/). In this configuration, RethinkDB is able to perform 134,700 QPS on a single server. While the overhead of network communication between the servers becomes visible when increasing the cluster size from one to two servers, adding further servers to the cluster demonstrates the near-linear scalability of RethinkDB, up to over 500,000 QPS on 16 servers.

As a variation of this workload, we also tested scalability in the outdated read mode. In this mode, we make the compromise of a higher chance of slightly outdated read results for additional performance, as read operations can be handled directly by secondary replicas. A typical application that exemplifies this kind of access pattern would be an asynchronous cache.

RethinkDB demonstrates extremely high scalability in this configuration, reaching throughputs of well over a million queries per second. The slightly sub-linear scalability when going from 12 to 16 database servers is caused by the client servers' CPUs getting saturated at these throughputs.


![](/assets/images/docs/performance-report/w-c-reads-latency.png)

### Analytical queries

* Tests the response time for analytical MapReduce queries involving string operations
* Query types:  Count the total number of sentences over a single field:
table.map(r.row("field0").split(".").count()).sum()
* We run one query at a time. Results show the average over five runs

Finally, we demonstrate RethinkDB's automatic query parallelization. Analytical queries are transparently distributed over servers in the cluster in a MapReduce style pattern.

In this example, we count the number of sentences over the whole data set of 25 million documents based on one of the fields. We use the following exact query which utilizes some the map and sum (reduce) functions of ReQL:

```
table.map( r.row("field0").split(".").count() ).sum()
```

We run this query five times for every cluster size and then calculate the average runtime. The results table below shows these averages.

Nodes | 1 | 2 | 3 | 4 | 8 | 12 | 16
------|---|---|---|---|---|----|----
Query Runtime (seconds) | 58859.76 | 32101.36 | 23245.40 | 15188.86 | 9567.36 | 7429.62 | 4397.48

With a single server, our query takes 59 seconds to complete. The automatic query parallelization in RethinkDB results in practically linear scalability, as the same query is executed in just above 4 seconds on 16 servers. The graph, shown in the results overview section, demonstrates the inverse execution time (queries per second) of the query.

## Conclusion

We wanted to provide a reasonably comprehensive RethinkDB test that covers a variety of different workloads. Given limited time, we chose to use the YCSB testing framework as a reliable and community-approved means of conducting rigorous testing on our database. We saw that most of the tests resulted in near-linear scalability as we moved from a single RethinkdB instance to a 16 node cluster. Although most of the tests resulted in performance metrics that suggest linear horizontal scalability, we know that there are plenty of improvements to make as our database evolves.

## Ongoing

Near to the release of this performance report, we will be releasing RethinkDB 2.3  where we will again publish our next set of performance metrics during the lifetime of the 2.3 release and additionally measure performance while scaling up to 16 servers and beyond.

### Notes

* We were fortunate enough to receive free credits from Rackspace to perform the majority of these tests and are very grateful for their contributions to open source software. All of [Rackspace’s OnMetal offerings can be found here](https://www.rackspace.com/cloud/servers/onmetal).
* We have published all relevant performance testing code and final results in the [rethinkdb/preformance-reports repository on Github](https://github.com/rethinkdb/performance-reports)
* We’d love to answer any questions you have about these tests. Come join us at [http://slack.rethinkdb.com](http://slack.rethinkdb.com) and feel free to ask more specific questions we don’t answer here by pinging @danielmewes or @dalanmiller.
* Recently, the team behind BigchainDB - a scalable blockchain database built on top of RethinkDB - has benchmarked RethinkDB on a 32-server cluster running on Amazon's EC2. They measured throughputs of  more than a million writes per second. Their conclusion: "There is linear scaling in write performance with the number of nodes". The full report is available at [https://www.bigchaindb.com/whitepaper/](https://www.bigchaindb.com/whitepaper/)
