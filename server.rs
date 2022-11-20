use std::sync::atomic;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();

    let instance: u64 = args[1].parse().unwrap();
    let counter_base: u64 = instance << 56;

    let port = &args[2];

    let counter = atomic::AtomicU64::new(counter_base);

    let mut app = tide::new();

    app.at("/")
        .get(move |_| {
            counter.fetch_add(1, atomic::Ordering::SeqCst);
            let c = counter.load(atomic::Ordering::SeqCst);
            println!("count: {c}");
            async move {
                Ok(serde_json::json!({ "count": c }))
            }
        });

    let f = async {
        app.listen("127.0.0.1:".to_owned() + port).await
    };

    futures::executor::block_on(f)
}
