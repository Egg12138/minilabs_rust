use std::env;

fn main() {
    for (var, value) in env::vars_os() {
        println!("{var:?}: {value:?}");

        print!("【var_os checking:】");
        match env::var_os(var) {
            Some(has) =>  {print!(" {has:?}");}
            None => {print!(" -- ");
        }
            
    }
        
}}
