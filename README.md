# TV Prog Rust API

This is a Rust implementation of a TV program API. 

## Motivation

The original API was written in Java Spring Boot, but it consumes too much memory. 
This Rust version aims to provide a more efficient and faster alternative.

The current API runs on a small VPS with only 2GB of RAM. The deployed version consumes at least 1.5GB of RAM, which is too much for such a simple API.
The goal is to reduce the memory footprint while maintaining functionality.

## Done
- [x] Call XMLTV API to fetch TV program data
- [x] Parse XMLTV data and store it in a suitable format
- [x] Docker support for easy deployment
- [x] Implement the API endpoints (channels, programs, etc.)
- [x] Fix Program API object (rating, categories, etc...)
- [x] CI/CD pipeline for automated testing and deployment
