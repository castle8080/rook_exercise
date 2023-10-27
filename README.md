# Rook Exercise

This is another toy rust project I started to try out web dev using Rust. For this one I am trying out:
* HTMX
* Sled
* Maud

## Interesting Bits for Me
* I like seeing a disk based key value store for Rust (sled). I had originally wanted to try out unqlite, but found sled was easy to add and setup. I wanted to try sled, because I wanted to try out using an embedded db which would work with document style save and search. I realize that using sled, will make it more difficult for me to use anything like a secondary index though.
* I was trying to figure out how I better handle errors with HTML generation on the server side. Using converstions for error types is pretty eas for APIs, but I haven't seen something as easy when generating HTML based on errors.
* This is the first project where I have tried passing a repository to a request processor where I wanted to use a trait for the repository. I am passing it as a dynamic implementation, which means there will be virtual method look ups. I think this is fine for a repository and wouldn't want to try and put generic parmaeters to the handler functions.
* I am figuring more about how to use Maud. So far it has been pretty easy to use.
* I am curious to see if I can use HTMX to update more than 1 section of a page at a time.
