# tokio-minimal-threading-fd-issue

Reproducing an issue we've seen. With a large number of concurrent things that require resources, tokio's runtime configurations (particularly `max_threads`) don't effectively limit how many request connections are made simultaneously. This results in file descriptor limits being exhausted in the case of making requests.

Hoping we're configuring something wrong and there's a way for the scheduler to be made to stop executing new futures if it already has too many threads.

You may need to change the number of futures created to hit your file descriptor limit.

```
λ ulimit -n
1024
λ cargo test --release -- --nocapture
```

## info from rust discord
```
Protty
doesn't max_threads control how many OS threads it can spawn - not related to network resources?
in order to limit a maximum number of futures at a time, could try looking into throttling systems like async Semaphores as those found in futures-intrusive for example

mumu
@Protty for this case then we'd use rlimit or something to get the fd limit and create a GenericSemaphore with permits around that number, then make that semaphore accessible anywhere a socket would be opened so that aquire().await could be called before making the request?

clintfred
@Protty @mumu I think the relevant line from the max_threads API docs is:

    Otherwise as core_threads are always active, it limits additional threads (e.g. for blocking annotations) as max_threads - core_threads.


What is meant by "blocking annotations" I wonder?

Protty
@clintfred tokio supports block_in_place() and spawn_blocking(). They serve as a way to offload futures which would normally block the scheduler (e.g. file io) to a separate thread pool meant for running blocking code so that it doesn't limit non-blocking execution (e.g. network io). Theres core_threads OS threads meant for running non-blocking code, and I assume the maximum threads it can spawn including both core_threads and potentially blocking  ones is max_threads

Ralith
@mumu that's a reasonable approach, yes
number of threads and number of concurrent futures are completely unrelated values; you can even use a single thread if you like
```

So we were thinking about what was being scheduled wrong, futures and threads aren't really connected in the way we were thinking about them.
