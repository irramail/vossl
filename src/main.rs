extern crate redis;

use redis::{Client, Commands, Connection, RedisResult};
use jsonrpc_http_server::jsonrpc_core::{IoHandler, Value, Params, Error};
use jsonrpc_http_server::{ServerBuilder};
use std::time::{SystemTime};

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

fn fetch_a_stat(id: &str, hash: &str, time: &str, date: &str) -> redis::RedisResult<isize> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    let mut id_date_time = id.to_string();
    id_date_time.push('_');
    id_date_time.push_str(date);
    id_date_time.push('_');
    id_date_time.push_str(time);

    let _ : () = con.set(id_date_time, hash)?;

    con.get("my_key")
}

fn delete(key: &str) -> redis::RedisResult<isize> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    con.del(key)
}

fn get_a_stat() -> redis::RedisResult<String>  {

    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    let keys: Vec<String> = con.keys("*_*_*")?;
    let _ : () = con.set("html", " " )?;

    let now_time : u64 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    let mut uniq_html_first = String::new();
    for key in keys {
        let content: String = con.get(key.to_string())?;
        let v: Vec<&str> = key.split('_').collect();
        if !uniq_html_first.contains(content.as_str()) {
            uniq_html_first.push_str(content.as_str());
            con.append(format!("{}_html", now_time), format!("\"{}_{}_{} {}\",", &v[1], &v[2], &v[0], content))?;

        }
        let _ = delete(key.as_str());
    }

    let mut keys: Vec<String> = con.keys("*_html")?;
    keys.sort();
    let mut uniq_html = String::new();
    for _ in 1..10 {
        let content: String = con.get(keys.pop().unwrap())?;
        if !uniq_html.contains(content.as_str()) {
            uniq_html.push_str(content.as_str());
            con.append("html", content)?;
        }

    }
    con.get("html")
}

fn get_names() -> redis::RedisResult<String> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    let mut keys: Vec<String> = con.keys("name_*")?;
    let _ : () = con.set("all_names", " " )?;

    keys.sort();

    for key in keys {
        //println!("{:?}", key);
        let content: String = con.get(key.to_string())?;
        let v: Vec<&str> = key.split('_').collect();

        con.append(format!("all_names", ), format!("\"{}_{}\",", &v[1], content))?;
    }

    con.get("all_names")
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
        let _ = fetch_a_stat( &w[0], &w[1], &w[2], &w[3]);
        Ok(Value::String("ok".to_string()))
    });

    io.add_method("get_tracks",  | _params | {
        let tracks = get_a_stat().unwrap();
        Ok(Value::String((format!("[{}]", &tracks[..tracks.len()-1])).to_string()))
    });

    io.add_method("get_names",  | _params | {
        let names = get_names().unwrap();
        Ok(Value::String((format!("[{}]", &names[..names.len()-1])).to_string()))
    });

    let server = ServerBuilder::new(io)
        .threads(3)
        .start_http(&"127.0.0.1:3030".parse().unwrap())
        .unwrap();

    server.wait();
}
