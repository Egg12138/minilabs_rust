mod pstree;
fn main() {
    if let Ok(_) = pstree::parse_args().run() {
        println!("Hello, world!");
    }
}
