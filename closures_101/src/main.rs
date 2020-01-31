 std::thread;

struct City{

    name : String,
    population : i64,
    country : String,
    avg_yearly_income_shs : u64,
    avg_age : f16,
    area_km2 : f64
}

impl City {
    fn new() -> Self {
        City{
            name: "".to_string(),
            population : 0,
            country : "".to_string(),
            avg_yearly_income_shs : 0,
            avg_age = 0.0,
            area_km2 : 0.0
        }
    }
    fn get_statistic<T>(&self, stat : &str) -> Option<T>
    {   
        match stat {
            "name" => Some(&self.name),
            "population" => Some(&self.population),
            "country"=> Some(self.country),
            "avg_yearly_income_shs" => Some(&self.avg_yearly_income_shs),
            "avg_age" =>Some (&self.avg_age),
            "area_km2"=> Some(&self.area_km2),
            _ => None
        }
        
    }
}


fn start_sorting(cities : Vec<City>, stat : Statistic) -> thread::JoinHandle<Vec<City>>
{
    let key_func = |city : &City| -> i64 { -city.get_statistic(stat) };

    thread::spawn( || {
        cities.sort_by_key(key_fn);
        cities
    })
}

fn main() {
    let nairobi = City { name : "Nairobi".to_string(),
    population : 4_000_000,
    country:"Kenya".to_string(), avg_yearly_income_shs:40000,
    avg_age:19, area_km2:200_000};
    
    let newyork = City { name:"New York".to_string(), population:12_000_000,
    country:"USA".to_string(), avg_yearly_income_shs : 1_000_000,
    avg_age:32, area_km2 : 14_000_000};

    let tokyo = City {name : "Tokyo".to_string(), population: 36_000_000,
    country: "Japan".to_string(), avg_yearly_income_shs : 1_600_000,
    avg_age : 62, area_km2 : 46_700_000 };

    let mut cities = vec![nairobi, newyork, tokyo];
    sorted_queue = start_sorting(mut cities, population);

    println!("Output of sorting ==> {}",sorted_queue);

}
