# SatelliteUpdate
This project provides a web-based dashboard for tracking and displaying satellite communication information using Rocket and Rusqlite in Rust. The dashboard includes routes to fetch various data points such as the time since the last message was received from the satellite, the time since the last message was sent, the estimated time to the next satellite pass, the current TLE (Two-Line Element set), the estimated time left until epoch, and the previous command sent to the satellite.

The application uses an SQLite database to store and retrieve satellite information. The database file updates.db is created automatically if it doesn't exist.
If the database file does exist, fields keep on getting added to it.
The database operations are handled using Rusqlite for SQLite operations.
The Information struct contains the necessary fields, and the addtodb function inserts this data into the database.

The routes are defined using rocket's #[get("/")] attribute which can handle different HTTP get requests.
Available routes are : 
1. #[get("/")] - Provides the user with information regarding the routes defined.
2. #[get("/time_r")] - Provides the time since the last message from the satellite was received.
3. #[get("/time_s")] - Provides the time since the last message was sent to the satellite.
4. #[get("/est_time")] - Provides the estimated time to the next satellite pass
5. #[get("/cur_TLE")] - Provides the current TLE 
6. #[get("/TLE_exp")] - Provides the time left until epoch
7. #[get("/prev_com")] - Provides the last command which was sent to the satellite.


