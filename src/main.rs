use std::env;
use std::fs;
use std::path::Path;
use std::io::Read;
use std::collections::HashMap;

extern crate bip_bencode;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate sha1;

use bip_bencode::{BencodeRef, BencodeRefKind, BDecodeOpt, BRefAccess};

#[derive(Debug, Serialize)]
struct TorrentInfo {
    torrent: serde_json::Value,
    info_hash: String,
}

fn get_info_hash(bencode_root: &BencodeRef) -> String {
    let bencode_dict = bencode_root.dict().unwrap();
    let info_ref = bencode_dict.lookup(b"info").unwrap();
    let info_bytes = info_ref.buffer();

    let mut m = sha1::Sha1::new();
    m.update(info_bytes);
    m.digest().to_string()
}

fn bencode_to_json(v: &BencodeRef) -> serde_json::Value {
    let k = v.kind();
    match k {
        BencodeRefKind::Int(n) => json!(n),
        BencodeRefKind::Bytes(n) => {
            match std::str::from_utf8(n) {
                Ok(s) => json!(s),
                Err(_) => json!(vec![n]),
            }
        }
        BencodeRefKind::List(n) => {
            let mut vec = Vec::new();
            for element in n {
                vec.push(bencode_to_json(element));
            }
            json!(vec)
        }
        BencodeRefKind::Dict(n) => {
            let mut map = HashMap::new();
            let list = n.to_list();
            for (k, v) in list {
                let k_str = std::str::from_utf8(k).unwrap().to_owned();
                map.insert(k_str, bencode_to_json(v));
            }
            json!(map)
        }
    }
}

fn get_torrent_info(buffer: &[u8]) -> Result<TorrentInfo, bip_bencode::BencodeParseError> {
    let bencode = BencodeRef::decode(buffer, BDecodeOpt::default())?;
    let info_hash = get_info_hash(&bencode);
    let mut torrent_json = bencode_to_json(&bencode);
    {
        torrent_json.as_object_mut().map(|o| {
            o.get_mut("info").map(|iv| {
                iv.as_object_mut().map(|io| io.remove("pieces"))
            })
        });
    }
    Ok(TorrentInfo {
        torrent: torrent_json,
        info_hash: info_hash,
    })
}

fn read_file(path_string: &str) -> std::io::Result<Vec<u8>> {
    let path = Path::new(path_string);
    let mut f = fs::File::open(path)?;
    let mut buffer: Vec<u8> = Vec::new();
    f.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path_string = args.get(1).expect("Missing string argument");
    let buffer = read_file(path_string).unwrap();
    let torrent_info = get_torrent_info(&buffer).unwrap();
    let j = serde_json::to_string_pretty(&torrent_info).unwrap();
    println!("{}", j);
}
