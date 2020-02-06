# tokio-minimal-threading-fd-issue

Reproducing an issue we've seen. With a large number of concurrent things that require resources, tokio's runtime configurations (particularly `max_threads`) don't effectively limit how many request connections are made simultaneously. This results in file descriptor limits being exhausted in the case of making requests.

Hoping we're configuring something wrong and there's a way for the scheduler to be made to stop executing new futures if it already has too many threads.

You may need to change the number of futures created to hit your file descriptor limit.

```
λ ulimit -n
1024
λ cargo test --release -- --nocapture
```
