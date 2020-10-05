# StreamsDecoder

## Preparation
Install rust if you don't have it already, find the instructions here https://www.rust-lang.org/tools/install

Make sure you also have the build dependencies installed, if not run:  
`sudo apt install build-essential`  
`sudo apt install pkg-config`  
`sudo apt install libssl-dev`  
`sudo apt update`  


## Installing streams_decoder
Download streams_decoder server:  
`git clone https://github.com/AleBuser/streams_decoder`  
`cd streams_decoder`  
  
Configure the Streams Gateway:  
`nano config.json`  
 
Set the *node* to a high-throughput node to make request response faster. 

  
## Runnig the Server:  
  
Run the Server:  
`cargo run --release`  
This starts the server  

To decode a streams channel send the channel address in a GET request as *text/plain* in the body, the endpoint is */decode_channel*:  
`  
curl --location --request GET 'localhost:8585/decode_channel' \
--header 'Content-Type: text/plain' \
--data-raw '1c6fddc2f0344892403ebc4fb6b94c4b308147c9fddb6340a27c89b7c4a28c390000000000000000'
`