fn main() {
    println!("cargo:rustc-env=DATABASE_URL=postgres://postgres:passwordnya@127.0.0.1/api");
}
