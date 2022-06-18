# Overview

web_scrapepkg is a rust program that allows you to scrape the html data from a website, use a selector to target specific information, and save links from the desired website.

web_scrapepkg is a quick, efficient program that helps the user get websitedata what they need without having to deal with
prompts. The user types cargo run website_here selector_here to quickly scrape neccessary information.

 This program was developed to help people who need to quickly scrape information from a website get it quickly and efficiently. Often I meet people who need to quickly get information from companies they compete to maintain competitive prices. In order to achieve my goal, I used Rust to develop a program that can be quickly run from a ssh script or a batch file to help the client get what they need.


[Software Demo Video](http://youtube.link.goes.here)

# Development Environment

This program was developed through Visual Studio Code on a M1 Mac.

Rust was used along with the reqwest, tokio, html_parser, and scraper libraries.

# Useful Websites

{Make a list of websites that you found helpful in this project}
* [rust-lang](https://www.rust-lang.org/)
* [tutorialsPoint](https://www.tutorialspoint.com/rust/index.htm)

# Future Work

{Make a list of things that you need to fix, improve, and add in the future.}
* Regex implementation for more customizable searches/queries
* Saved Files are incremented rather than overwriting old information
* Text files can be loaded with a list of websites to scrape
* A Configuration file with maximum depth of sites to search so that links are followed and scraped.
