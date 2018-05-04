extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::env;
use std::fs::File;
use std::error::Error;
use serde_json::{Value};
use futures::{Future, Stream};
use hyper::{Client, StatusCode};
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;

fn get_json(uri: &str) -> Result<Value, Box<Error>> {
	let mut core = Core::new()?;
	let handle = core.handle();

	let connector = HttpsConnector::new(4, &handle)?;
	let client = Client::configure().connector(connector).build(&handle);

	let uri = uri.parse()?;

	let work = client.get(uri).and_then(|res| {
		match res.status() {
			// shitty error handling lol
			StatusCode::Ok => {
				res.body().concat2().and_then(|body| {
			        let v: Value = serde_json::from_slice(&body).unwrap();
			        Ok(v)
    			})
    		},
    		// here return error instead of panic?
			StatusCode::NotFound => panic!("City Not Found!"),
			_ => panic!("Some Request Error: {} !", res.status())
		}
	});

	Ok(core.run(work)?)
}

fn print_data(data: &Value) {
	let temp = &data["main"]["temp"]; //returns "null" when key is invalid
	let hum = &data["main"]["humidity"];
	println!("Tmp {} °C / Hum {} %", temp, hum);
}

#[derive(Deserialize, Debug)]
struct Config {
	api_key: String
}

fn read_config(path: &str) -> Result<Config, Box<Error>> {
	let file = File::open(path)?;
	let config = serde_json::from_reader(file)?;

	Ok(config)
}

fn main() {
	
	//hardcoded for now
	let config = read_config("config.json").unwrap();

	let location = match env::args().skip(1).next() { 
		Some(s) => s,
		None => String::from("Prague")
	};

	let uri = format!("http://api.openweathermap.org/data/2.5/weather?q={}&units=metric&APPID={}",
		location, config.api_key);
	let res = get_json(&uri);

	match res {
		Ok(v) => {
			//println!("{}", serde_json::to_string_pretty(&v).unwrap());
			print_data(&v);
		},
		Err(_) => {
			println!("No Connection");
		}
	}
}