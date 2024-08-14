//! ## Decoding and Encoding example GTFS data example
//! 
//! ```rs
//! use prost::Message;
//! 
//! let url = "https://lbtgtfs.lbtransit.com/TMGTFSRealTimeWebService/Vehicle/VehiclePositions.pb";
//! let response = reqwest::get(url).await.unwrap();
//! let bytes = response.bytes().await.unwrap();
//! let data: Result<gtfs_realtime::FeedMessage, prost::DecodeError> = prost::Message::decode(bytes.as_ref());
//! let data = data.unwrap();
//! 
//! //encode back into protobuf
//! let rencoded = data.encode_to_vec();
//! ```

#[macro_use]
extern crate serde;

// Include the `items` module, which is generated from items.proto.
include!(concat!(env!("OUT_DIR"), "/transit_realtime.rs"));
