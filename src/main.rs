extern crate rustc_serialize;
use rustc_serialize::json;

use std::fmt;
use std::env;

use std::fs::File;
use std::io::Read;

use std::thread;
use std::process::Command;

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

static DEBUG: bool = false;

fn main() {
    let args: Vec<_> = env::args().collect();
    let argc = args.len();

    if argc < 3 {
        usage(&args[0]);
    }

    // 处理命令行参数
    let project = args[1].to_string();
    let operate = args[2].to_string();
    if ("deploy".to_string() != operate && "rollback".to_string() != operate) || ("rollback".to_string() == operate && argc < 4) {
        usage(&args[0]);
    }

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

    let mut hosts_conf  = Vec::new();
    let hosts_len = deploy_conf.hosts_conf.len();
    for index in 0 .. hosts_len {
        hosts_conf.push(deploy_conf.hosts_conf[index].clone());
    }

    println!("          ===== {} ======\n", work_conf.project);

    let handles: Vec<_> = hosts_conf.into_iter().map(|host| {
        let user = deploy_conf.user.clone();
        let path = deploy_conf.path.clone();
        let operate = work_conf.operate.clone();
        let head    = work_conf.head.clone();
        thread::spawn(move || {
            let cmd = if "deploy".to_string() == operate {
                format!("ssh {}@{} \"cd {} && git pull && git log -1 | awk '{{if (\\$1 ~/commit/) {{print \\$2}}}}' 2>&1\"",
                user, host, path)
            } else {
                format!("ssh {}@{} \"cd {} && git reset --hard {} && git log -1 | awk '{{if (\\$1 ~/commit/) {{print \\$2}}}}' 2>&1\"",
                user, host, path, head)
            };
            if DEBUG { println!("CMD: {}", cmd); }
            let output = Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .output()
                .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

            println!("--- Start {} for {}@{} ---", operate, user, host);
            if DEBUG { println!("status: {}", output.status); }
            println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
            println!("--- End {} for {}@{} ---\n", operate, user, host);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
