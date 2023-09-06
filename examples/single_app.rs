use seahorse::{App, Context, Flag, FlagType};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let name = "single_app";

    let app = App::new(name)
        .author(env!("CARGO_PKG_AUTHORS"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .usage("single_app [args]")
        .version(env!("CARGO_PKG_VERSION"))
        .app_data(String::from("some app_name"))
        .action(action)
        .flag(
            Flag::new("bye", FlagType::Bool)
                .description("single_app args --bye(-b)")
                .alias("b"),
        );

    app.run(args);
}

fn action(c: &Context) {
  
    // NOTE: this strange access pattern 
    if let Some(app_data) =  c.extensions_mut().get::<String>(){

        println!("app-data: {:#?}", app_data) ;
    }

    let d = std::cell::Ref::map(c.extensions(), |ext| ext.get::<String>().unwrap());
    println!("app-data2: {:#?}", *d) ;


    if c.bool_flag("bye") {
        println!("Bye, {:?}", c.args);
    } else {
        println!("Hello, {:?}", c.args);
    }
}
