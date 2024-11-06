const GOLDEN_RATIO : f32 = 1.618034; 

fn main () {
    let string: &str = "My Litteral";
    let indices: Vec<f32> = string.char_indices().map(|(index, _)| index as f32).collect();
    let mut golden_indices : Vec<f32> = Vec::new();
    for i in indices {
        golden_indices.push(i * GOLDEN_RATIO)
    }
    println!("{:?}", golden_indices);
}