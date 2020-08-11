// 标准线程库
use std::thread;
// 标准网络库
use std::net::{TcpListener, TcpStream, Shutdown};
// 标准IO库
use std::io::{Read, Write};

// 处理客户端发来的内容
fn handle_client(mut stream: TcpStream) {
    // 存储字符串的缓冲区
    let mut data = [0 as u8; 2000]; // 2000字节的缓冲区
    // 将tcp数据读取进缓冲区。只要不出Error就一直while循环处理数据
    while match stream.read(&mut data) {
        // 如果能成功接收到数据
        Ok(size) => {
            // 将chilent的数据原样返回
            stream.write(&data[0..size]).unwrap();  // echo everything!
            // Ok返回True，while条件满足，继续循环
            true 
        },
        // 如果出现错误
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            // 出现错误，关闭tcp接收端
            stream.shutdown(Shutdown::Both).unwrap();
            // 返回False，终止while循环
            false
        }
    } {}
}

// 服务器入口程序
fn main() {
    // 监听tcp端口
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    // 接收tcp数据
    for stream in listener.incoming() {
        // 模式匹配接收到的数据
        match stream {
            // 正确接收到了数据
            Ok(stream) => {
                // 有客户端连接到了服务器
                println!("New connection: {}", stream.peer_addr().unwrap());
                // 新建线程处理接收到的数据
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            // 出现错误
            Err(e) => {
                // 打印错误
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // 关闭tcp监听
    drop(listener); // close the socket server
    println!("Close server......");
}