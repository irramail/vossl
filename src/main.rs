extern crate redis;

use redis::{Client, Commands, Connection, RedisResult};
use jsonrpc_http_server::jsonrpc_core::{IoHandler, Value, Params, Error};
use jsonrpc_http_server::{ServerBuilder};

fn parse_arguments (p: Params) -> Result<Vec<String>, Error> {
    let mut result = Vec::new();
    match p {
        Params::Array(array) => {
            for s in &array {
                match s {
                    Value::String(s) => result.push(s.clone()),
                    _ => return Err(Error::invalid_params("expecting strings"))
                }
            }
        }
        _ => return Err(Error::invalid_params("expecting an array of strings"))
    }
    if result.len() < 1 {
        return Err(Error::invalid_params("missing api key"));
    }

    return Ok(result[0..].to_vec());
}

fn fetch_an_integer(id: &str, ver: &str) -> redis::RedisResult<isize> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    let _ : () = con.set(id, ver )?;

    con.get("my_key")
}

fn fetch_an_stat(id: &str, hash: &str, time: &str, date: &str) -> redis::RedisResult<isize> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    let mut id_date_time = id.to_string();
    id_date_time.push_str(date);
    id_date_time.push_str(time);

    let _ : () = con.set(id_date_time, hash)?;

    con.get("my_key")
}

fn main() {
    let mut io = IoHandler::new();
    io.add_method("openssl_version", move |params: Params| {
        let w = parse_arguments(params)?;
        let _ = fetch_an_integer( &w[0], &w[1]);
        Ok(Value::String(w.join("-").to_string()))
    });

    io.add_method("new_track", move |params: Params| {
        let w = parse_arguments(params)?;
        let _ = fetch_an_stat( &w[0], &w[1], &w[2], &w[3]);
        Ok(Value::String(w.join("-").to_string()))
    });

    let server = ServerBuilder::new(io)
        .threads(3)
        .start_http(&"127.0.0.1:3030".parse().unwrap())
        .unwrap();

    server.wait();
}