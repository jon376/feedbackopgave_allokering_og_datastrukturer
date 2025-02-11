use std::collections::HashMap;
use std::time::Instant;


struct SensorData {
    id: u32,
    value: f64,
}

fn main() {
    // start tidtagning for performance
    let start = Instant::now();

    // opret en vektor med plads til 1 million elementer
    let mut sensor_vec: Vec<SensorData> = Vec::with_capacity(1000000);
    for i in 0..1000000 {
        sensor_vec.push(SensorData {
            id: i,
            value: i as f64,
        });
    }
    println!("Indlæst {} sensorer i vektoren.", sensor_vec.len());
    println!("Tid til vektor-indlæsning: {:?}", start.elapsed());

    // start tidtagning for HashMap
    let start_hashmap = Instant::now();

    // opret en HashMap med samme kapacitet
    let mut sensor_map: HashMap<u32, f64> = HashMap::with_capacity(1000000);
    for sensor in &sensor_vec {
        sensor_map.insert(sensor.id, sensor.value);
    }

    println!("Indlæst {} sensorer i HashMap.", sensor_map.len());
    println!("Tid til HashMap-indlæsning: {:?}", start_hashmap.elapsed());

    // start tidtagning for søgning i HashMap
    let start_search = Instant::now();

    // søg efter en specifik sensor
    let search_id = 500000;
    if let Some(&value) = sensor_map.get(&search_id) {
        println!("Sensor {} har værdi {}", search_id, value);
    } else {
        println!("Sensor {} ikke fundet", search_id);
    }

    println!("Tid til søgning i HashMap: {:?}", start_search.elapsed());

    // optimering med Box
    let start_box = Instant::now();
    let boxed_sensor_vec = Box::new(sensor_vec);
    println!("Boxet sensor-data: {} elementer", boxed_sensor_vec.len());
    println!("Tid til Box-allokering: {:?}", start_box.elapsed());
}

//del 1 sammenligning med java: Java tog 26ms hvor rust tog 8.57ms med indlæsning i vector

//del 2 sammenligning med java: Java tog 38ms hvor rust tog 184ms med indlæsning i hashmap
//del 2 sammenligning med java: Java tog 4ms hvor rust tog 46.6µs med søgning i hashmap

//del 3 sammenligning med java: Rust tog 24.6µs med at allokere data ved hjælp af Box. Box bruger heapen til at allokere data i stedet for stacken, hvilket er nyttigt til store datasæt, da stacken kan blive for lille. Selvom heap-allokering kan tage lidt længere tid end stack-allokering, gør Box det muligt at arbejde med store eller dynamiske objekter, som ikke kan være på stacken.
//del 3 sammenligning med java: Hvor i java bliver heapen brugt automatisk, når der oprettes et nyt object med "new"