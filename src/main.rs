use std::fmt;
use std::env;

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

fn usage(argv_0: &String) {
    println!("-----USAGE----");
    println!("{} project deploy             deploy project with latest <head>.", argv_0);
    println!("{} project rollback <head>    rollback with <head>.", argv_0);
    std::process::exit(0);
}

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
    //println!("{:?}", work_conf);
}
