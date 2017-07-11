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

use serde_bencode::de;
use serde_bytes::ByteBuf;
use serde::Serializer;

fn path_serialize<S>(x: &Option<Vec<String>>, s: S) -> Result<S::Ok, S::Error> where S: Serializer {
    if let Some(obj) = x.as_ref() {
        let filepath = obj.join("/");
        s.serialize_str(&filepath)
    } else {
        s.serialize_unit()
    }
}

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
    #[serde(serialize_with="path_serialize")]
    path: Option<Vec<String>>,
    #[serde(default)]
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
    announce_list: Option<Vec<Vec<String>>>,
    #[serde(default)]
    creation_date: Option<i64>,
    #[serde(default)]
    comment: Option<String>,
    #[serde(default)]
    created_by: Option<String>,
}

// fn print_optional<T>(opt: &Option<T>, prefix: &str) where T: std::fmt::Debug {
//     if let Some(ref val) = *opt {
//         println!("{}{:?}", prefix, val);
//     }
// }

// fn render_torrent(torrent: &Torrent) {
//     let j = serde_json::to_string(torrent).unwrap();
//     println!("{}", j);

//     println!("name:\t\t{:?}", torrent.info.name);
//     print_optional(&torrent.announce, "announce:\t");
//     print_optional(&torrent.nodes, "nodes:\t\t");
//     if let Some(ref al) = torrent.announce_list {
//         for a in al {
//             println!("announce list:\t{}", a[0]);
//         }
//     }
//     print_optional(&torrent.httpseeds, "httpseeds:\t");
//     print_optional(&torrent.creation_date, "creation date:\t");
//     print_optional(&torrent.comment, "comment:\t");
//     print_optional(&torrent.created_by, "created by:\t");
//     print_optional(&torrent.encoding, "encoding:\t");
//     println!("piece length:\t{}", torrent.info.piece_length);
//     print_optional(&torrent.info.private, "private:\t");
//     print_optional(&torrent.info.root_hash, "root hash:\t");
//     print_optional(&torrent.info.md5sum, "md5sum:\t\t");
//     print_optional(&torrent.info.path, "path:\t\t");
//     if let &Some(ref files) = &torrent.info.files {
//         for f in files {
//             let filepath = f.path.join("/");
//             println!("file path:\t{:?}", filepath);
//             println!("file length:\t{}", f.length);
//             print_optional(&f.md5sum, "file md5sum:\t");
//         }
//     }
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    let path_string = args.get(1).expect("Missing string argument");
    let path = Path::new(path_string);
    let mut f = fs::File::open(path).unwrap();
    let mut buffer: Vec<u8> = Vec::new();
    match f.read_to_end(&mut buffer) {

        Ok(_) => {
            match de::from_bytes::<Torrent>(&buffer) {
                Ok(t) => {
                    //render_torrent(&t)
                    let j = serde_json::to_string(&t).unwrap();
                    println!("{}", j)
                },
                Err(e) => println!("ERROR: {:?}", e),
            }
        }
        Err(e) => println!("ERROR: {:?}", e),
    }
}
