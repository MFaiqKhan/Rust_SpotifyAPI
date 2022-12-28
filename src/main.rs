use reqwest; // reqwest is use to make http request
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
// reqwest::header is use to set header for http request, and in this case we are setting up accept, authorization and content-type header
// reqsest will call the api and get the response in json format
use serde::{Deserialize, Serialize}; // serde is use to deserialize the json response
use std::env; // env is use to get the environment variable

#[derive(Serialize, Deserialize, Debug)] // this is use to derive the Serialize and Deserialize trait
struct ExternalUrls {
    spotify: String, // basically this is spotify url
}

#[derive(Serialize, Deserialize, Debug)]
struct Artist {
    name: String,
    external_urls: ExternalUrls, // A struct
}

#[derive(Serialize, Deserialize, Debug)]
struct Album {
    name: String,
    artists: Vec<Artist>, // A vector of Artist struct, here vector is use because there can be multiple artist in a album
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct Track {
    name: String,
    href: String, // this is the url of the track
    popularity: u32,
    /// u32 is use because popularity is a unsigned integer
    album: Album, // A struct
    external_urls: ExternalUrls, // A struct
}

#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    tracks: Items<Track>, // this is a struct, here Items is a generic struct and track is a generic type
}

#[derive(Serialize, Deserialize, Debug)]
struct Items<T> {
    items: Vec<T>, // A vector of T, here T is a generic type, what will it do is that it will take the type of the struct which is passed to it and make a vector of that type
}

// just calling the API and getting the response is not enough, we need to deserialize the response(structurize)
//and then print it
fn print_tracks(tracks: Vec<&Track>) { // our api response will get the tracks and then we will pass it to this function
    for track in tracks {
        // this will iterate over the tracks
        println!(
            "Track: {name} from the album {album}",
            name = track.name,
            album = track.album.name, 
        );
        println!( // this will print the artist name
            "{}", 
            track // this will get the track
            .album // this will get the album
            .artists // this will get the artists
            .iter() // this will iterate over the artists
            .map(|artist| artist.name.to_string()) // this will map the artist name to a string
            .collect::<String>() // this will collect the string into a string , <String> is use to tell the compiler that we are collecting the string into a string and not a vector
        );
        println!("Spotify URL: {url}", url = track.external_urls.spotify);
        println!("Popularity: {popularity}", popularity = track.popularity);
        println!("Preview URL: {url}", url = track.href);
        println!(" --------------------------------------------- ");
    }
}

#[tokio::main] // this is use to make the main function as async function
async fn main() {
    let args: Vec<String> = env::args().collect(); // this will get the arguments passed to the program, the API key and the search query
    let search_query = &args[1]; // this will get the search query // first argument is the program name, second is the search query and third is the API key
    let api_key = &args[2]; // this will get the API key

    let spotify_api_url = format!(
        "https://api.spotify.com/v1/search?q={query}&type=track,artist",
        query = search_query // we are changing the query to the search query
    ); // this will format the url of the spotify api
       // this is the url of the spotify api

    let client = reqwest::Client::new(); // this will create a new client, which will be use to make the request
    let response = client
        .get(spotify_api_url) // this will make a get request to the url
        .header(AUTHORIZATION, format!("Bearer {}", api_key)) // this will set the authorization header,  Bearer is a type of authorization and api_key is the API key
        .header(CONTENT_TYPE, "application/json") // this will set the content-type header, we are sending the request in json format
        .header(ACCEPT, "application/json") // this will set the accept header, we are accepting the response in json format
        .send() // this will send the request
        .await // this will make the function async
        .unwrap(); // this will unwrap the response, if there is any error it will panic
    match response.status() {
        // here we are matching the status code of the response
        reqwest::StatusCode::OK => {
            // if the status code is 200
            match response.json::<APIResponse>().await {
                // // this will deserialize the response, apiresponse is the struct which we have created, it will deserialize the response into the struct
                Ok(parsed) => {
                    print_tracks(parsed.tracks.items.iter().collect()); // parsed.tracks.items is a vector of track, so we are converting it into a vector of track, it is iter because we are iterating over it and then collecting it into a vector
                },
                Err(_) => {
                    // _ means any thing which is not ok
                    println!("Error while parsing the response");
                    std::process::exit(1); // this will exit the program with exit code 1
                }
            }
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            // if the status code is 401
            println!("Invalid API key");
            std::process::exit(1); // this will exit the program with exit code 1
        }
        _ => {
            // if the status code is not 200 or 401
            println!("Error while making the request");
            std::process::exit(1); // this will exit the program with exit code 1
        }
    }
}
