use std::fs::File;
use std::io::{self, BufReader};
use std::time::Instant;
use md5::Context;
use std::io::Read;

fn main() -> io::Result<()> {
    // 记录开始时间
    let start = Instant::now();

    // 使用 BufReader 提高读取效率
    let file = File::open("../video.mkv")?;
    let mut reader = BufReader::new(file);
    
    // 创建 MD5 上下文
    let mut context = Context::new();

    // 创建缓冲区并读取文件
    let mut buffer = vec![0; 8192000];
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        context.consume(&buffer[..bytes_read]);
    }

    // 获取 MD5 散列值
    let result = context.compute();

    // 记录结束时间
    let end = Instant::now();

    // 计算时间差并转换为毫秒
    let duration = end.duration_since(start);
    let elapsed_ms = duration.as_millis();

    // 将结果打印为十六进制字符串
    println!("MD5 Hash: {:x}", result);
    println!("Time elapsed: {} ms", elapsed_ms);

    Ok(())
}
