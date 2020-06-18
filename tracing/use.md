# USE method

The goal of the USE method is to identify system bottlenecks. For every
system resource (CPU, mem, network, buses, etc.) we walk down a checklist:

1. __Utilization__: For a time interval, how much of that did we spend doing work?
2. __Saturation__: How much work _couldn't_ process during that interval?
3. __Errors__: How many errors occurred during this time?

## Applying USE

1. Identify resources
2. Choose a resource
3. Are there errors present?
4. Is there high utilization?
5. Is there saturation?
6. If we identified the problem, we're done
7. Move onto the next resource

This way we systematically go over each resource, and check "errors",
"utilization", and "saturation".

## Networking

The strategies to use are:
1. performance monitoring
2. USE method
3. static performance tuning
4. workload characterization

__USE__

1. __Utilization__: Time spent sending or receiving frames.
2. __Saturation__: The degree of extra queuing, buffering, or blocking due
    to a fully utilized interface.
3. __Errors__: TCP / Network protocol errors

Utilization is `current throughput / bandwidth limit` calculated for both
sending and receiving expressed in bytes per second.

Saturation is time spent blocked on network sends. If we trace both the TCP
call, and the call to sending, we can measure the saturation (time spent
waiting to send.)

