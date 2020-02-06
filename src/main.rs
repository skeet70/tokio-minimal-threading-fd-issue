fn main() {
    println!("Hello, world!");
}

#[test]
fn concurrent_spawn() {
    let mut rt = tokio::runtime::Builder::default()
        .enable_all()
        .threaded_scheduler()
        .core_threads(2)
        .max_threads(3)
        .build()
        .unwrap();

    // lots of Futures are spawned onto the shared Runtime
    // modify the size of the test until you hit a file descriptor limit
    rt.block_on(async move {
        let mut handles = vec![];
        for _i in 0..2000 {
            handles.push(tokio::spawn(async move {
                reqwest::get("https://google.com").await
            }));
        }
        let res: Vec<Result<_, _>> = futures::future::join_all(handles).await;

        let succ = res
            .iter()
            .filter(|res| res.as_ref().unwrap().is_ok())
            .count();
        let fail = res
            .iter()
            .filter(|res| res.as_ref().unwrap().is_err())
            .count();

        res.iter()
            .filter(|res| res.as_ref().unwrap().is_err())
            .for_each(|err| print!("{:?}", err.as_ref().unwrap().as_ref().unwrap_err()));
        println!("\nCONCURRENT SPAWN: Succ: {}, Fail: {}", succ, fail);

        assert_eq!(0, fail);
    });
}

#[test]
fn concurrent_join() {
    let mut rt = tokio::runtime::Builder::default()
        .enable_all()
        .threaded_scheduler()
        .core_threads(2)
        .max_threads(3)
        .build()
        .unwrap();

    // One joined future goes onto the shared runtime (?) and that one spawns
    // a bunch of requests simultaneously-ish
    rt.block_on(async move {
        let x = [0u32; 2000]
            .iter()
            .map(|_| reqwest::get("https://google.com"));
        let res = futures::future::join_all(x).await;

        let succ = res.iter().filter(|res| res.is_ok()).count();
        let fail = res.iter().filter(|res| res.is_err()).count();

        res.iter()
            .filter(|res| res.is_err())
            .for_each(|err| print!("{:?}", err.as_ref().unwrap_err()));
        println!("\nCONCURRENT JOIN: Succ: {}, Fail: {}", succ, fail);
        assert_eq!(0, fail);
    });
}
