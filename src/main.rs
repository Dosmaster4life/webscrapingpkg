
extern crate reqwest; // Used to make get request to get html data

use std::fs::File; 
use std::io::{Write};
use std::env;
use reqwest::Client;
use html_parser::{Dom, Node,Result};
use scraper::{Html, Selector};




#[tokio::main] // required to make main method async


async fn main() 
{
    let client = Client::new();  // New connection, would use bearer token or other information for authentication for sites behind login

    let terminal_args: Vec<String> = env::args().collect(); // save arguments in vector

    let url = &terminal_args[1]; // user chooses a website url as first argument
    let tag = &terminal_args[2]; // user chooses a css selector/tag for second argument

    let response_information = client.get(url)
    .send()
    .await // Wait for connection established
    .expect("no response from site")// most likely due to site blocking get request or a timeout
    .text()
    .await// Wait for data
    .expect("access denied"); // Most likly due to denied access
    
    let path_link = "AllData.txt".to_owned();
    filewriter(&response_information, &path_link);
    
   parse(&response_information,&tag);

}

fn filewriter(result: &String, path: &String)  // write information to file
{
    let mut f = File::create(&path)
    .expect("Unable to create file");

    f.write_all(result.as_bytes()) // Write all information to file in readable format
    .expect("Unable to write website data");

}

fn parse(data : &str, tag : &str) -> Result<()>  // parse data with css selector/tag 
{

    let mut list_vector = Vec::new();
    let mut list_vector_search = Vec::new();
    let html_data = Html::parse_fragment(data);
    let selector_grabber = Selector::parse(tag)// Search data with tag/css selector typed
    .unwrap();

    for element in html_data.select(&selector_grabber)  // Loop through items in html data that matches selector
    {
        println!("{}",element.inner_html()); // Prints out inner html
        list_vector_search.push(element.inner_html()); // Inner html is text seen by user

    }

    let combined_search = list_vector_search // Write all items in vector
    .join("\n");
    let path_link_search = "Search.txt" // convert data type to 
    .to_owned(); // Clones data from borrowed information 

    filewriter(&combined_search, &path_link_search); // Write search results to the file

    let dom = Dom::parse(data)?;
    let parsed_dom_children = dom.children // Vector Node storing the parsed dom children
    .get(0)
    .unwrap()
    .into_iter();

     let link_data = parsed_dom_children.filter_map(|item| match item // Parse the dom children
    {
        Node::Element(ref element) 
        if element.name == "a" => element.attributes["href"] // All Links found on site are cloned and saved
        .clone(),
        _ => None,
    }
);

    println!("\nLinks Found are below:");

    for (index, link) in link_data.enumerate()  
    {
        println!("{}: {}", index + 1, link); // Prints links for user to view in console
        list_vector.push(link); // Stores links in vector for further usage to be stored in a file
    }
    
    let combined = list_vector.join("\n"); // join vector seperated by spaces
    let path_link = "Links.txt".to_owned(); // convert to String

    filewriter(&combined, &path_link); // write links to file
    
    println!("\nAllData.txt contains all html, Search.txt has your search results, Links.txt has any links on the site.");

    Ok(()) // Required, contains the success value
}



