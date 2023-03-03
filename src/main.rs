use std::env::current_dir;
use std::io::Write;

use clap::Parser;
use rquickjs::{
    BuiltinLoader, BuiltinResolver, FileResolver, Func, NativeLoader, Object, ScriptLoader, Type,
    Value,
};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    file: Option<String>,
}

fn main() {
    let args = Args::parse();
    use rquickjs::{Context, Runtime};

    let resolver = (FileResolver::default().with_path("./"),);
    let loader = (ScriptLoader::default(),);

    let rt = Runtime::new().unwrap();
    rt.set_loader(resolver, loader);
    let ctx = Context::full(&rt).unwrap();

    ctx.with(|ctx| {
        let console: Object = Object::new(ctx).unwrap();
        let log_fn = |v: Value| {
            println!("{}", format_js_value(v));
        };
        console.set("log", Func::from(log_fn)).unwrap();
        let globals = ctx.globals();
        globals.set("console", console).unwrap()
    });

    let file = args.file;
    if let Some(file) = file {
        let content = std::fs::read_to_string(&file).unwrap();
        ctx.with(move |ctx| {
            let file = current_dir().unwrap().join(file);
            let module = ctx
                .compile(
                    file.canonicalize().unwrap().to_string_lossy().to_string(),
                    content,
                )
                .unwrap();
            println!("{:?}", module.as_value());
        });
        return;
    }
    std::io::stdout().flush().unwrap();
    let mut rl = DefaultEditor::new().unwrap();
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();
                ctx.with(|ctx| {
                    let ret: rquickjs::Result<Value> = ctx.eval(line);
                    match ret {
                        Ok(val) => {
                            let val = val;
                            println!("{}", format_js_value(val));
                        }
                        Err(err) => {
                            println!("Error: {}", err);
                        }
                    }
                    std::io::stdout().flush().unwrap();
                })
            }
            Err(ReadlineError::Interrupted) => {
                println!("exit");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("exit");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

fn format_js_value(v: Value) -> String {
    let t = v.type_of();
    let s = match t {
        Type::String => {
            let s = v.as_string().unwrap().to_string().unwrap();
            format!(r#""{s}""#)
        },
        Type::Bool => v.as_bool().unwrap().to_string(),
        Type::Float => v.as_float().unwrap().to_string(),
        Type::Int => v.as_int().unwrap().to_string(),
        Type::Null => "null".to_owned(),
        Type::Module => {
            let m = v.as_module().unwrap();
            let module_name: String = m.name().unwrap();
            format!("module({})", module_name)
        }
        Type::Array => {
            let arr = v.as_array().unwrap();
            let mut arr1 = vec![];
            for item in arr.iter() {
                let v: Value = item.unwrap();
                let inner = format_js_value(v);
                arr1.push(inner);
            }
            format!("[{}]", arr1.join(", "))
        }
        Type::Object => {
            let obj = v.as_object().unwrap();
            let keys = obj.keys();
            let mut arr = vec![];
            for key in keys {
                let key: String = key.unwrap();
                let val: Value = obj.get(&key).unwrap();
                let val_repr = format_js_value(val);
                arr.push(format!("{}: {}", key, val_repr));
            }
            format!("{{ {} }}", arr.join(", "))
        },
        Type::Function => {
            let func = v.as_function().unwrap();
            let func_name: String = func.as_object().get("name").unwrap();
            format!("function {}(...)", func_name)
        },
        _ => "Other Type".to_owned()
    };
    format!("{}", s)
}
