fn main() {
  // Export the environment variable RUSTC_VERSION, so it can be imported as const &str at compile time
  let version = rustc_version::version().unwrap();
  println!("cargo:rustc-env=RUSTC_VERSION={}", version);
}