use redis::streams::{StreamId, StreamKey, StreamMaxlen, StreamReadOptions, StreamReadReply};

use redis::{Commands, RedisResult, Value};

const MYSTREAM: &str = "mystream";

const STREAMS: &[&str] = &[MYSTREAM];

fn main() {
    let client = redis::Client::open("redis://:averystrongpassword@redis:6379").expect("client");

    read_records(&client).expect("simple read");

    clean_up(&client)
}

const BLOCK_MILLIS: usize = 5000;

fn read_records(client: &redis::Client) -> RedisResult<()> {
    let mut con = client.get_connection().expect("conn");

    let opts = StreamReadOptions::default().block(BLOCK_MILLIS);

    let starting_id = "0-0";

    let srr: StreamReadReply = con
        .xread_options(STREAMS, &[starting_id], &opts)
        .expect("read");

    for StreamKey { key, ids } in srr.keys {
        println!("Stream {}", key);
        for StreamId { id, map } in ids {
            println!("\tID {}", id);
            for (n, s) in map {
                if let Value::Data(bytes) = s {
                    println!("\t\t{}: {}", n, String::from_utf8(bytes).expect("utf8"))
                } else {
                    panic!("Weird data")
                }
            }
        }
    }

    Ok(())
}

fn clean_up(client: &redis::Client) {}
