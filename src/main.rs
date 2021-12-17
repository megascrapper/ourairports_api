use ourairports_api::ourairports::countries::get_countries_csv;
use ourairports_api::ourairports::ToJsonString;

fn main() {
    let mut data_dir = std::env::current_dir().unwrap();
    data_dir.push("tmp");
    let countries_data = get_countries_csv(&data_dir).unwrap();
    for (_, country) in countries_data.iter() {
        println!("{}", country.to_json_string().unwrap())
    }
}