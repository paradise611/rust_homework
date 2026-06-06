use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    //导入日志数据
    let logs = vec![
        "service=auth level=ERROR msg=login failed",
        "service=order level=WARN msg=slow query",
        "service=payment level=ERROR msg=timeout",
        "service=auth level=ERROR msg=password wrong",
        "service=order level=INFO msg=create order",
        "service=payment level=WARN msg=retry",
    ];

    //统计结果共享同一份数据，使用多线程
    let level_count: Arc<Mutex<HashMap<String, usize>>> = Arc::new(Mutex::new(HashMap::new()));
    let service_error_count: Arc<Mutex<HashMap<String, usize>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let mut handles = vec![];

    for log in logs {
        //克隆Arc以便在线程中使用
        let level_count = Arc::clone(&level_count);
        let service_error_count = Arc::clone(&service_error_count);

        let handle = thread::spawn(move || {
            //解析日志
            let parts: Vec<&str> = log.split_whitespace().collect();

            let mut level = "";
            let mut service = "";

            for part in parts {
                if let Some(stripped) = part.strip_prefix("level=") {
                    level = stripped; //提取level的值
                } else if let Some(stripped) = part.strip_prefix("service=") {
                    service = stripped; //提取service的值
                }
            }

            //level_count计数
            {
                let mut level_map = level_count.lock().unwrap();
                *level_map.entry(level.to_string()).or_insert(0) += 1;
            }

            //如果level是ERROR，service_error_count计数
            if level == "ERROR" {
                let mut service_map = service_error_count.lock().unwrap();
                *service_map.entry(service.to_string()).or_insert(0) += 1;
            }
        });

        handles.push(handle);
    }

    //等待所有线程结束
    for handle in handles {
        handle.join().unwrap();
    }

    //输出level_count结果
    let level_count = level_count.lock().unwrap();
    println!("level_count:");
    println!("{:#?}", *level_count);

    //输出service_error_count结果
    let service_error_count = service_error_count.lock().unwrap();
    println!("service_error_count:");
    println!("{:#?}", *service_error_count);
}
