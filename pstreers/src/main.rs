
fn main() {
    println!("PSTREE: ");
    if let Err(e) = pstreers::get_args().and_then(pstree::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
