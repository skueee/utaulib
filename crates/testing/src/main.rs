use src::structs::renderer::Renderer;

fn main() {
    let renderer = Renderer::create("/home/robin-brams/.local/share/OpenUtau/Singers/Kumi - English/").unwrap();
    println!("{:?}", renderer.character)
}
