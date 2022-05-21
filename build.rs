use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
  // Incorporate the README.md file into the top-level Rust library (so we only
  // have to maintain that in one place).
  fs::write(
    PathBuf::from(env::var("OUT_DIR")?).join("README-rustdocified.md"),
    readme_rustdocifier::rustdocify(
      &fs::read_to_string("README.md")?,
      &env::var("CARGO_PKG_NAME")?,
      Some(&env::var("CARGO_PKG_VERSION")?),
      Some("sensible"),
    )?,
  )?;
  println!("cargo:rerun-if-changed=README.md");
  Ok(())
}
