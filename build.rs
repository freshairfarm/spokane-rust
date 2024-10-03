use copy_to_output::copy_to_output;

fn main() {
    copy_to_output("src/static", &std::env::var("PROFILE").unwrap()).unwrap();
}