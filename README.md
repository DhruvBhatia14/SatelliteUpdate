# SatelliteUpdate
Task:
Write a Rust based app using the Rocket backend framework, that stores the following fields in a sqlite database, and provides routes to handle HTTP requests to return the value of the requested field. 
Time since last message received from the satellite 
Time since last command sent to satellite
Estimated time to next satellite pass
Current TLE (two line element) of the satellite
Estimate time to epoch (when does TLE expire)
A list of previous entered commands with associated time stamp

For now all the data for these fields can be random (just use common sense formatting for things like date - times)

Useful resources / links:
https://rocket.rs/
https://developer.mozilla.org/en-US/docs/Web/HTTP/Overview
