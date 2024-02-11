# HTML-Render
A simple discord bot designed to recieve post requests with HTML as a body, render them into an image using an auxiliary selenium webserver, and then send them to a discord channel. Used for rendering faxes to a staff channel for ss13.

## Setup
To set it up, you'll have to compile it on your local machine which requires cargo and rust to be installed. Simply run cargo build and the binary should be put somewhere under target/
When the program is run, it'll generate a new config file wherever the working directory is, you can edit this to suit your own needs.

Notably this requires a WebDriver server to be running on 127.0.0.1:4444, personally I'm using and recommend the selenium docker dynamic grid setup described here: https://github.com/SeleniumHQ/docker-selenium?tab=readme-ov-file#dynamic-grid
Simply having geckodriver running will work as well, but it will break if it gets too many requests too quickly (i.e. multifaxes)
