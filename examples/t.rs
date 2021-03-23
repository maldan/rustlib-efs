fn main() {
    println!("{:?}", efs::dir::file_list(""));
    println!("{:?}", efs::dir::list("target/debug"));
    println!("{:?}", efs::dir::list("D:/"));
    println!("{:?}", efs::file::size("Cargo.toml"));
    println!("{:?}", efs::file::as_text("Cargo.toml"));
    println!("{:?}", efs::file::as_binary("Cargo.toml"));
}
