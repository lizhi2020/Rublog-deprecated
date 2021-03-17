/*
use std::fs;
use std::net::{TcpListener,TcpStream};
use std::path::{PathBuf,Path};
use std::env;
use comrak::{markdown_to_html, ComrakOptions};
use json::parse;
use json::JsonValue;
use std::io::{Read,Write};

fn default_config()-> JsonValue{

    let seq: String=String::from(r#"
{
    "author":"Unknow"
}

    "#);

    let p=PathBuf::from("config.json");
    fs::write(p, seq.as_str()).expect("默认配置创建失败");

    let config=parse(seq.as_str()).expect("返回json对象失败");
    return config;
}
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    println!("{:?}",buffer);
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "main.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    //let contents = fs::read_to_string(filename).expect(filename);
    let contents = String::from("<html>hello</html>");
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    
    //let ten_millis = time::Duration::from_millis(10000); 
    //thread::sleep(ten_millis);				//睡眠一段时间，模拟处理时间很长
}
fn server()->std::io::Result<()>{
    let listener=TcpListener::bind("127.0.0.1:8080")?;
    for stream in listener.incoming(){
        handle_client(stream?);
    }
    Ok(())
}
fn build(tera:& tera::Tera){
    let curdir=env::current_dir().unwrap();
    println!("curdir: {}", curdir.display());
    println!("curexe: {}", env::current_exe().unwrap().display());

    //读取配置文件
    let config=PathBuf::from("config.json");
    let config=fs::read_to_string(config);
    let config_json: JsonValue;
    match config {
        Ok(config)=>{
            println!("找到配置文件");
            config_json=parse(config.as_str()).unwrap();
        },
        _=>{
            config_json=default_config();
        }
    };

    let config = config_json;
    println!("{}",config.pretty(4));

    let target_path=PathBuf::from("target");
    if target_path.is_dir(){
        println!("目标文件夹已存在，这可能导致问题");
    }
    else{
        //然后试图创建输出文件夹./build
        fs::create_dir(PathBuf::from("build")).expect("目标文件夹创建失败，程序终止");
    }
    
    //遍历相对路径./source下文本文件内容,并输出到target下


    let source_dir = PathBuf::from("source");

    for entry in source_dir.read_dir().expect("read error"){
        if let Ok(entry)=entry{
            if entry.path().is_file() {
                println!("{:?}",entry.path());
                let f = fs::read_to_string(entry.path()).unwrap();
                let mut p=PathBuf::from("build/blog");
                p.push(entry.path().file_name().unwrap());
                p.set_extension("html");
                println!("{:?}",p);

                let output=markdown_to_html(f.as_str(), &ComrakOptions::default());
                println!("{}",output);
                
                let mut context=Context::new();
                context.insert("post_context", &output);

                let content=tera.render("post.html", &context).unwrap();

                
                fs::write(p, content).expect(entry.path().to_str().unwrap());
            };
        }
    };
}

use tera::Tera;
use tera::Context;

*/
fn main() {

    println!("hello world");
   /*
    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let mut context = Context::new();
    context.insert("title","hello world");
    context.insert("home_page_title", "少年东方曜");
    context.insert("about_url", "about.html");
    context.insert("blog_url", "blog/index.html");
    //context.insert("vat_rate", &0.20);
    //context.insert("users");
    //println!("{}",tera.render("home.html", &context).unwrap());

    //生成首页
    let content=tera.render("home.html",&context).unwrap();
    fs::write(PathBuf::from("build/home.html"), content).unwrap();

    
    //生成about
    context.insert("title", "About");
    context.insert("about_context", "爱好技术，喜欢交流");
    context.insert("author_name", "li");
    let content=tera.render("about.html",&context).unwrap();
    fs::write(PathBuf::from("build/about.html"), content).unwrap();

    //生成blog/
    build(& tera);
 
    let mut arg=env::args();
    match arg.nth(1){
        Some(p)=>{
            if p== String::from("server"){
                println!("start server");
                server().unwrap();
            }
            else if p== String::from("build"){
                println!("start build");
                build();
            }
            else{
                println!("unknow command");
            }
        }
        None=>{
            println!("no command!");
        }
    }
*/
}