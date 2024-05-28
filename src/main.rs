use std::io;
use serde::Deserialize;



#[derive(Deserialize,Debug)]

struct WeatherResponse {
    weather:Vec<Weather>,
    main:Main,
    wind:Wind,
    name:String,
}

#[derive(Deserialize,Debug)]

struct Weather{
    description:String,
}

#[derive(Deserialize,Debug)]

struct Main {
    temp:f64,
    humidity:f64,
    pressure:f64
}


#[derive(Deserialize,Debug)]

struct Wind {
    speed:f64,
}


// function to get the weather data
fn get_weather_data(city:&str,country_code:&str,api_key:&str)->Result<WeatherResponse,reqwest::Error>{

    let url=format!("https://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}&units=metric",city,country_code,api_key);

    let response=reqwest::blocking::get(&url)?;
    let result=response.json::<WeatherResponse>()?;

    Ok(result)
}

//function to display the weather data
fn display_weather_data(data:WeatherResponse){
    let name=data.name;
    let description:&String=&data.weather[0].description;
    let temperature:f64=data.main.temp;
    let humidity:f64=data.main.humidity;
    let pressure:f64=data.main.pressure;
    let wind_speed:f64=data.wind.speed;

    let weather_text=format!(
    "Weather in {}: {} {}
        > Temperature: {}°C
        > Humidity: {}%
        > Pressure: {:.1} hPa,
        > Wind Speed: {:.1} m/s
    ",
    name,
    description,
    get_temp_emoji(temperature),
    temperature,
    humidity,
    pressure,
    wind_speed
    );


    println!("{}",weather_text);
}


//function to get emoji based on the temperature
fn get_temp_emoji(temperature:f64) ->String {
    if temperature < 0.0 {
        return "🥶".to_string();
    }else if temperature >= 0.0 && temperature <= 10.0 {
        return "💭".to_string();
    }else if temperature >=10.0 && temperature <= 20.0 {
        return "⛅".to_string();
    }else if temperature>= 20.0 && temperature <= 30.0 {
        return "🌤️".to_string();
    }else{
        return "🥵".to_string();
    }
}

fn main(){
    println!("\n🦀🦀 🦀🦀🦀 🦀🦀🦀🦀🦀 🦀🦀🦀 🦀🦀\n");
    println!("Welcome to the Weather Station!");

    loop {
        println!("\nPlease enter the name of your city:");
        let mut city=String::new();
        io::stdin().read_line(&mut city).expect("Failed to read input! 😞");
        let city=city.trim();

        println!("Please enter the country code (e.g, IN for India):");
        let mut country_code=String::new();
        io::stdin().read_line(&mut country_code).expect("Failed to read input! 😞");
        let country_code=country_code.trim();

        let api_key="72bf9e7145a6a0f89951a3f895c97ec9";

        match get_weather_data(city,country_code,api_key) {
            Ok(result) => {
                display_weather_data(result);
            }, 
            _ => {
                println!("Failed to get the weather data! 😞");
            }
        }

        println!("Do you want to continue (yes/no):");
        let mut choice=String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input! 😞");
        let choice=choice.trim();

        if choice != "yes"{
            break;
        }

    }
}


