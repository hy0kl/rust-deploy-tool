extern crate rustc_serialize;
use rustc_serialize::json::{self, Json, ToJson};

use std::fmt;
use std::env;

use std::fs::File;
use std::io::Read;


struct WorkConf {
    project: String,
    operate: String,
    head:    String,
}

impl fmt::Debug for WorkConf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WorkConf:{}\"project\": {}, \"operate\": {}, \"head\": {}{}",
            "{", self.project, self.operate, self.head, "}")
    }
}

// Automatically generate `RustcDecodable` and `RustcEncodable` trait
// implementations
#[derive(RustcDecodable, RustcEncodable)]
struct DeployConf {
    user: String,
    path: String,
    hosts_conf: Vec<String>,
}

fn usage(argv_0: &String) {
    println!("-----USAGE----");
    println!("{} project deploy             deploy project with latest <head>.", argv_0);
    println!("{} project rollback <head>    rollback with <head>.", argv_0);
    std::process::exit(0);
}

static DEBUG: bool = true;

fn main() {
    let args: Vec<_> = env::args().collect();
    let argc = args.len();

    if argc < 3 {
        usage(&args[0]);
    }

    // 处理命令行参数
    let project = args[1].to_string();
    let operate = args[2].to_string();
    let head = if argc > 3 { args[3].to_string() } else { "".to_string() };

    let work_conf = WorkConf{
        project: project,
        operate: operate,
        head:    head,
    };
    if DEBUG { println!("{:?}", work_conf); }

    let config_filename = format!("./conf/{}.json", work_conf.project);
    if DEBUG { println!("config_filename: {}", config_filename); }

    let mut data = String::new();
    let mut f = File::open(config_filename)
        .ok()
        .expect("Can NOT open file.");
    f.read_to_string(&mut data)
        .ok()
        .expect("Can NOT read cofing file.");
    if DEBUG { println!("deploy_conf: {}", data)}
    let deploy_conf: DeployConf = json::decode(&data)
        .ok()
        .expect("Can NOT parse JSON config data.");
    if DEBUG { println!("JSON: {}", json::encode(&deploy_conf).unwrap()); }
}
