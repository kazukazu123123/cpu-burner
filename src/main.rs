use rand::{Rng, SeedableRng};

fn main() {
    let cpus = num_cpus::get();
    println!("[CPU BURNER] Made by kazukazu123123");
    println!("[CPU BURNER] Your cpu count: {}", cpus);
    println!("Now burning you cpu! eat cpu now!");
    for _ in 0..cpus - 1 {
        std::thread::spawn(omoishori);
    }
    
    std::thread::sleep(std::time::Duration::MAX);
}

fn omoishori() {
    let mut rng = rand::rngs::StdRng::from_seed(*b"kazukazu123123____kazukazu123123");
    loop {
        let _: u128 = rng.gen();
    }
}
