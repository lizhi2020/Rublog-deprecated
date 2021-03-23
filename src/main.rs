use tera::{Tera,Context};
use comrak::{markdown_to_html, ComrakOptions};
use structopt::StructOpt;

mod webclient;

#[derive(StructOpt,Debug)]
#[structopt(about = "the stupid content tracker")]
enum Rub {
    Init {
        name:String
    },
    New {
        pattern:String,
        name:String
    },
    Clear {
        name:Option<String>
    },
    Build,
    Update
}

fn clear(name:&Option<String>){
    let mut path=env::current_dir().unwrap();
    if let Some(name)=name{
        path.push(name);
    }

    for entry in ["src","templates","build"].iter(){
        path.push(entry);
        fs::remove_dir_all(&path).unwrap_or_else(|e|{
            println!("{}{:?}",e,path);
        });
        path.pop();
    }

}
//渲染
fn build()->std::io::Result<()>{
    //初始化 tera
    let mut tera = match Tera::new("templates/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    tera.autoescape_on(vec![]);

    //先拷贝templates中的css文件
    for entry in fs::read_dir(PathBuf::from("templates")).unwrap(){
        let src=entry?.path();

        let mut dst=PathBuf::from("build");
        dst.push(src.file_name().unwrap());

        if src.extension().unwrap() == "css"{
            fs::write(dst,fs::read(src).unwrap())?;
        }

    }
    //只对md文件渲染,如果不是md，则直接拷贝
    for entry in fs::read_dir(PathBuf::from("src")).unwrap(){

        let entry=entry?;
        let path=entry.path();
        let mut dst=PathBuf::from("build");
        dst.push(path.file_name().unwrap());
        if path.extension().unwrap() != "md"{
            fs::write(&dst, fs::read(&path).unwrap()).unwrap();
            println!("create {:?} read {:?}",&dst,&path);
        }
        else{
            dst.set_extension("html");

            //此处执行了渲染并输出
            //感觉不太好
            render(&tera,&path,&dst).unwrap();
        }
        
    };
    Ok(())
}

use serde::Serialize;
#[derive(Debug,Serialize)]
struct Item{
    name:String,
    url:String
}
//上下文,源文件,目标文件
fn render(tera:&Tera,src:&PathBuf,dst:&PathBuf)->std::io::Result<()>{
    let src=fs::read_to_string(src)?;
    let post_context=markdown_to_html(&src, &ComrakOptions::default());

    //根据源文件生成上下文
    let mut context=Context::new();

    let mut post_list = Vec::new();
    for entry in fs::read_dir(PathBuf::from("src")).unwrap(){
        let file_name=entry?.file_name().into_string().unwrap();

        let pos=file_name.rfind('.').unwrap();
        let (file_name,_)=file_name.split_at(pos);
        let file_name=file_name.to_string();
        
        let mut url=String::new();
        url.push_str(file_name.as_str());
        url.push_str(".html");

        post_list.push(Item{name:file_name,url:url});
    };
    context.insert("dir_list", &post_list);
    context.insert("post_context", &post_context);

    let template_type:&str;
    if dst.file_name().unwrap().to_str().unwrap() == "index.html"{
        template_type="index.html";
    }
    else{
        template_type="post.html";
    }

    let content=tera.render(template_type, &context).unwrap();

    fs::write(dst, content).unwrap();
    Ok(())
}
fn new(pattern:String,name:String){
    if pattern==String::from("post"){
        let mut path=PathBuf::from("src");
        path.push(name);

        path.set_extension("md");
        fs::write(path, "").unwrap();
    }
    else{
        panic!("unkonw pattern");
    }
}
fn main() {

    let args= Rub::from_args();
    match args{
        Rub::Init{name}=>{
            init_project(&name);
        }
        Rub::New{pattern,name}=>{
            new(pattern,name);
        }
        Rub::Clear{name}=>{
            clear(&name);
        }
        Rub::Build=>{
            build().unwrap();
        }
        Rub::Update=>{
            update().unwrap();
        }
    }
}
fn update()->std::io::Result<()>{
    let target="https://raw.githubusercontent.com/lizhi2020/Rublog/main/templates/prism-okaidia.css";
    let test = webclient::get_file(target)?;
    let mut dst=env::current_exe()?;
    dst.pop();
    dst.push("templates");
    dst.push("prism-okaidia.css");

    fs::write(&dst, test)?;
    println!("{:?}",dst);
    Ok(())
}
use std::{fs,path::PathBuf};
use std::io::ErrorKind;
use std::env;

///创建项目目录
fn init_project(path:&String){
    let mut path=PathBuf::from(path);

    //先尝试生成项目根目录
    match fs::create_dir(&path){
        Ok(())=>{
            println!("create dir:{:?}",path)
        }
        Err(error)=>{
            if error.kind()==ErrorKind::AlreadyExists{
                println!("dir:{:?} has existed",path)
            }
            else{
                panic!("unknow error happened when creating dir:{:?}",path)
            }
        }
    };

    // 创建子目录
    for i in ["src","build","templates"].iter(){
        path.push(i);
        match fs::create_dir(&path){
            Ok(())=>{
                println!("create dir:{:?}",path)
            }
            Err(error)=>{
                if error.kind()==ErrorKind::AlreadyExists{
                    println!("warning dir:{:?} has exited",path)
                }
                else{
                    panic!("unknow error happened when creating dir:{:?}",path)
                }
            }
        };
        path.pop();
    }

    //执行拷贝templates 操作
    //将exe路径下的template 拷贝到 项目中

    let mut template_dir=env::current_exe().unwrap();
    template_dir.pop();
    template_dir.push("templates");

    if !template_dir.is_dir(){
        template_dir=env::current_dir().unwrap();
        template_dir.push("templates");
    }

    for entry in template_dir.read_dir().expect(template_dir.to_str().unwrap()){
        if let Ok(entry)=entry{
            if entry.path().is_file() {
                let f = fs::read_to_string(entry.path()).unwrap();
                println!("read file:{:?}",entry.path());

                let mut p=PathBuf::from(&path);
                p.push("templates");
                p.push(entry.path().file_name().unwrap());
                fs::write(&p, f).expect(entry.path().to_str().unwrap());
                println!("create file:{:?}",p);
            };
        }
    }
}