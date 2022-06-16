use std::io::Read;
use std::net::TcpStream;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use lockfree::map::Map;


fn foo_reader(number: u32, mapp: Arc<Map<u32, String>>) {
    loop{
        println!("number of current thread---{}", &number);
        let mut conn;
        for j in mapp.iter() {
            conn = j.1.clone();
            if  j.0 == 6 {
                mapp.remove(&6);
            }
            println!("{}---{}", j.0 ,conn);
        }
        thread::sleep(Duration::from_millis(10));
    }
}

fn foo_writer(mut number: u32, mapp: Arc<Map<u32, String>>) {
    while(number != 10){
        mapp.insert(number, format!("random message --- {}", &number));
        thread::sleep(Duration::from_millis(10));
        number += 1;
    }
}

fn main() {
    let mut mapp: Map<u32, String> = Map::new();
    let mut mapp = Arc::new(mapp);
    mapp.insert(0, String::from("Zero"));
    mapp.insert(1, String::from("One"));
    mapp.insert(2, String::from("Two"));
    mapp.insert(3, String::from("Three"));
    mapp.insert(4, String::from("Four"));
    let copy_mapp = mapp.clone();
        thread::spawn(move || {
            foo_writer(5, copy_mapp);
        });
    for i in 0..2 {
        let copy_mapp = mapp.clone();
        thread::spawn(move || {
            foo_reader(i, copy_mapp);
        });
    }

    thread::sleep(Duration::from_millis(100));
    println!("lasted messages");
    for j in mapp.iter() {
        let conn = j.1.clone();
        println!("{}---{}", j.0 ,conn);
    }
}
