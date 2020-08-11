// 标准网络库
use std::net::{TcpStream};
// 标准IO库
use std::io::{Read, Write, stdin};
// 字符串库
use std::str::from_utf8;

// 客户端入口程序
fn main() {
    let result = loop {
        // 连接服务器端口
        match TcpStream::connect("localhost:3333") {
            // 成功连接到服务器
            Ok(mut stream) => {
                // 打印成功连接到服务器
                println!("Successfully connected to server in port 3333");
                // 等待用户输入数据
                let mut buf = String::new();
                println!("input something");
                // 输入数据
                stdin().read_line(&mut buf).ok().expect("Error!");
                // 去掉输入数据中多余的空格、回车，转换成字节流，作为发送数据
                let msg = buf.trim().as_bytes();
                // 发送数据
                stream.write(msg).unwrap();
                // 打印发送出的数据
                let send_text = from_utf8(&msg).unwrap();
                println!("Sent:    {}, awaiting reply...", send_text);
            
                // 服务器返回数据的缓冲区
                let mut data = [0 as u8; 2000]; // 2000字节的缓冲区
                // 接收服务器返回数据
                match stream.read(&mut data) {
                    // 成功接收到数据
                    Ok(_) => {
                        // 打印收到的回复信息
                        let receive_text = from_utf8(&data).unwrap();
                        println!("Receive: {}", receive_text);
                    },
                    // 未能成功接收数据，打印当前的Error
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                    }
                }
            },
            // 没有正确连接到服务器
            Err(e) => {
                println!("Failed to connect: {}", e);
            }
        }
    };
    // 打印客户端终止信息
    println!("Terminated.");
}