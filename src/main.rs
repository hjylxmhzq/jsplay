use std::io::Write;

use clap::Parser;
use quick_js::console::Level;
use quick_js::{Context, JsValue};
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    file: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let ctx = Context::builder()
        .console(|_: Level, args: Vec<JsValue>| {
            println!("{}", format_multi_js_value(args));
        })
        .build()
        .unwrap();

    let file = args.file;
    if let Some(file) = file {
        let content = std::fs::read_to_string(file).unwrap();
        ctx.eval(&content).unwrap();
        return Ok(());
    }

    std::io::stdout().flush().unwrap();
    let mut rl = DefaultEditor::new()?;
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                let ret = ctx.eval(&line);
                match ret {
                    Ok(val) => {
                        println!("{}", format_js_value(val));
                    }
                    Err(err) => {
                        println!("Error: {}", err);
                    }
                }
                std::io::stdout().flush().unwrap();
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
    Ok(())
}

fn format_multi_js_value(vec: Vec<JsValue>) -> String {
    let vec: Vec<String> = vec
        .into_iter()
        .map(|item| {
            let r = format_js_value(item);
            r
        })
        .collect();
    format!("{}", vec.join(" "))
}

fn format_js_value(val: JsValue) -> String {
    match val {
        JsValue::String(s) => s,
        JsValue::Array(arr) => {
            let vec: Vec<String> = arr
                .into_iter()
                .map(|item| {
                    let r = format_js_value(item);
                    r
                })
                .collect();
            format!("[{}]", vec.join(", "))
        }
        JsValue::Null => "null".to_owned(),
        JsValue::Undefined => "undefined".to_owned(),
        JsValue::Bool(b) => b.to_string(),
        JsValue::Float(f) => f.to_string(),
        JsValue::Int(i) => i.to_string(),
        JsValue::Object(_) => "[Object object]".to_owned(),
        JsValue::__NonExhaustive => "other value".to_owned(),
    }
}
