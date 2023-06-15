use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // 建立与mini-redis服务器的连接
    let mut client = client::connect("127.0.0.1:7879").await?;

    // 设置 key: "hello" 和 值: "world"
    client.set("hello", "world".into()).await?;

    // 获取"key=hello"的值
    let result = client.get("hello").await?;

    println!("从服务器端获取到结果={:?}", result);

    Ok(())
}




// async fn say_to_world() -> String {
//     String::from("world")
// }

// #[tokio::main]
// async fn main() {
//     // 此处的函数调用是惰性的，并不会执行 `say_to_world()` 函数体中的代码
//     let op = say_to_world();

//     // 首先打印出 "hello"
//     println!("hello");

//     // 使用 `.await` 让 `say_to_world` 开始运行起来
//     println!("{}", op.await);
// }












