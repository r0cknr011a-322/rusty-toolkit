fn main() {
    println!("cargo::rerun-if-changed=map.ld");
    println!("cargo::rustc-link-arg=-Tmap.ld");
}
