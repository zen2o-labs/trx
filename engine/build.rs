fn main() {
    println!("cargo:rerun-if-changed=../grammar/src/trx.pest");
}