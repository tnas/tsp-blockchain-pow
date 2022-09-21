use std::fs;

fn main() {
    // let _weights = Box::new([[0; 86000]; 86000]);

    let paths = fs::read_dir("./tsplib95").unwrap();

    for path in paths {

        let path = path.unwrap().path();

        if path.ends_with("a280.tsp") {
            println!("TSP Instance: {}", path.display());
        }
    }
}
