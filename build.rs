use copy_to_output::copy_to_output;

fn main() {
    let env_profile = &std::env::var("PROFILE")
        .expect("Unable to access PROFILE environment variable");
    copy_to_output("src/static", env_profile).unwrap();
}