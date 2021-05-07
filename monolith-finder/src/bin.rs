use monolith_finder::finder::search_monoliths;
use monolith_finder::worldgen::ChunkGenerator;

fn main() {
    let chunk_gen = ChunkGenerator::new(8676641231682978167);
    let result = search_monoliths(&chunk_gen, -2624 / 4, 4343 / 4, 4, 4);
    print!("Got: {:#?}", result);
}
