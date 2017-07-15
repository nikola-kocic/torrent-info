use std::env;
use std::fs;
use std::path::Path;
use std::io::{Read};

extern crate serde_bencode;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_bytes;
extern crate serde_json;
extern crate sha1;

use serde_bencode::{de};
use serde_bytes::ByteBuf;

extern crate bip_bencode;
use std::default::Default;
use bip_bencode::{BencodeRef, BRefAccess, BDecodeOpt};


#[derive(Debug, Serialize, Deserialize)]
struct Node(String, i64);

#[derive(Debug, Serialize, Deserialize)]
struct File {
    path: Vec<String>,
    length: i64,
    #[serde(default)]
    md5sum: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Info {
    name: String,
    #[serde(skip_serializing)]
    pieces: ByteBuf,
    #[serde(default)]
    #[serde(rename="piece length")]
    piece_length: i64,
    #[serde(default)]
    md5sum: Option<String>,
    #[serde(default)]
    length: Option<i64>,
    #[serde(default)]
    files: Option<Vec<File>>,
    #[serde(default)]
    private: Option<u8>,
    #[serde(default)]
    // #[serde(serialize_with="path_serialize")]
    path: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename="root hash")]
    root_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Torrent {
    info: Info,
    #[serde(default)]
    announce: Option<String>,
    #[serde(default)]
    nodes: Option<Vec<Node>>,
    #[serde(default)]
    encoding: Option<String>,
    #[serde(default)]
    httpseeds: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename="announce-list")]
    announce_list: Option<Vec<Vec<String>>>,
    #[serde(default)]
    #[serde(rename="creation date")]
    creation_date: Option<i64>,
    #[serde(default)]
    comment: Option<String>,
    #[serde(default)]
    #[serde(rename="created by")]
    created_by: Option<String>,
}

#[derive(Debug, Serialize)]
struct TorrentInfo {
    torrent: Torrent,
    info_hash: String
}

fn get_info_hash(buffer: &[u8]) -> String {
    let bencode = BencodeRef::decode(buffer, BDecodeOpt::default()).unwrap();
    let bencode_dict = bencode.dict().unwrap();
    let info_ref = bencode_dict.lookup(b"info").unwrap();
    let info_bytes = info_ref.buffer();

    let mut m = sha1::Sha1::new();
    m.update(info_bytes);
    m.digest().to_string()
}

fn get_torrent_info(buffer: &[u8]) -> TorrentInfo {
    let info_hash = get_info_hash(&buffer);
    let t = de::from_bytes::<Torrent>(&buffer).unwrap();
    TorrentInfo { torrent: t, info_hash: info_hash}
}

fn read_file(path_string: &str) -> Vec<u8> {
    let path = Path::new(path_string);
    let mut f = fs::File::open(path).unwrap();
    let mut buffer: Vec<u8> = Vec::new();
    f.read_to_end(&mut buffer).unwrap();
    buffer
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path_string = args.get(1).expect("Missing string argument");
    let buffer = read_file(path_string);
    let torrent_info = get_torrent_info(&buffer);
    let j = serde_json::to_string_pretty(&torrent_info).unwrap();
    println!("{}", j);
}
