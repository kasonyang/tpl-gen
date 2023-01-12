use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::exit;
use std::str::FromStr;
use tera::{Context, Tera};
use toml::Value;

#[derive(Debug)]
struct AppError {
    message: String,
}

struct CmdOption {
    tpl_path: String,
    data_path: Option<String>,
    out_path: Option<String>,
    vars: HashMap<String, String>,
}

fn main() {
    execute().unwrap_or_else(|e| {
        eprintln!("{}", e.message);
    });
}

fn execute() -> Result<(),AppError> {
    let ops = load_args();
    let tpl_fn = &ops.tpl_path;
    let vars = ops.vars;
    let mut tera = Tera::default();
    tera.add_template_file(Path::new(tpl_fn), Some(tpl_fn))
        .map_err(|e| app_error(format!("failed to parse template file:{}", e)))?;
    let mut ctx = Context::new();
    if let Some(data_path) = ops.data_path {
        let value = load_config(data_path.as_str())?;
        let val_table = value.as_table().unwrap();
        val_table.iter().for_each(|(k,v)| {
            ctx.insert(k, v);
        });
    }
    vars.iter().for_each(|(k, v) | {
        ctx.insert(k, v);
    });
    let result = tera.render(tpl_fn, &ctx)
        .map_err(|e| app_error(format!("failed to render template file:{}", e)))?;
    if let Some(out_path) = ops.out_path {
        let mut file = File::create(out_path)
            .map_err(|e| app_error(format!("failed to create output file:{}", e)))?;
        file.write_all(result.as_bytes())
            .map_err(|e| app_error(format!("failed to write output file:{}", e)))?;
    } else {
        println!("{}", result);
    }
    Ok(())
}

fn load_args() -> CmdOption {
    let args : Vec<String> = env::args().collect();
    let mut i = 1;
    let mut out_path = None;
    let mut tpl_path = String::new();
    let mut data_path = None;
    let mut vars = HashMap::new();
    while i < args.len() {
        let arg = &args[i];
        i += 1;
        if arg == "-o" {
            out_path = Some(args[i].to_string());
            i += 1;
        } else if arg == "-d" {
            data_path = Some(args[i].to_string());
            i += 1;
        } else if arg.starts_with("-v:") {
            vars.insert(arg[3..].to_string(), args[i].to_string());
            i += 1;
        } else {
            tpl_path = arg.to_string();
            break;
        }
    }
    if tpl_path.is_empty() {
        print_usage();
        exit(-1);
    }
    return CmdOption {
        tpl_path,
        data_path,
        out_path,
        vars,
    }
}

fn print_usage() {
    println!("tpl-gen [-d DATA_FILE] [-o OUT_FILE] [-v:VAR_NAME VAR_VALUE]... TEMPLATE_FILE ")
}

fn load_config(path: &str) -> Result<Value,AppError> {
    let mut config_content = String::new();
    let mut config_file = File::open(path)
        .map_err(|e| app_error(format!("failed to load data file: {}", e)))?;
    config_file.read_to_string(&mut config_content)
        .map_err(|err| app_error(format!("failed to read data file: {}", err)))?;
    let value = Value::from_str(&mut config_content)
        .map_err(|e| app_error(format!("failed to parse data file: {}", e)))?;
    Ok(value)
}

fn app_error(message: String) -> AppError {
    AppError {
        message,
    }
}
