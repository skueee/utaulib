use src::structs::renderer::Renderer;

fn main() {
    let renderer = Renderer::create("").unwrap();
    println!("{:?}", renderer.voicebank.character)
}
