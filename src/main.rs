use rusqlite::{params, Connection, Result, Error};
#[macro_use] extern crate rocket;

//route defined for base - just tells what route to add to get which value
#[get("/")]
fn base() -> String{
    "Provide a route after the backslash to display a requested value. Routes you can add are: time_r , time_s , est_time , cur_TLE , TLE_exp , prev_com ".to_string()
}

//route defined for time since last message received from the satellite 
#[get("/time_r")]
fn received() -> String{
    let conn = Connection::open("updates.db").expect("Failed to open database");

    match getval(&conn, "time_r") {
        Ok(most_recent_value) => format!("The most recent value for time_r is: {}", most_recent_value),
        Err(_) => "Failed to fetch the most recent value for time_r".to_string(),
    }
}

//route defined for time since last message was sent to the satellite
#[get("/time_s")]
fn sent() -> String {
    let conn = Connection::open("updates.db").expect("Failed to open database");
    match getval(&conn, "time_s") {
        Ok(most_recent_value) => format!("The most recent value for time_s is: {}", most_recent_value),
        Err(_) => "Failed to fetch the most recent value for time_s".to_string(),
    }
}

//route defined for estimated time to next satellite pass
#[get("/est_time")]
fn timetopass() -> String {
    let conn = Connection::open("updates.db").expect("Failed to open database");
    match getval(&conn, "est_time") {
        Ok(most_recent_value) => format!("The most recent value for est time is: {}", most_recent_value),
        Err(_) => "Failed to fetch the most recent value for est_time".to_string(),
    }
}

//route defined for current tle of the satellite
#[get("/cur_TLE")]
fn currentTLE() -> String {
    let conn = Connection::open("updates.db").expect("Failed to open database");
    match getval(&conn, "cur_TLE") {
        Ok(most_recent_value) => format!("The most recent value for cur TLE is: {}", most_recent_value),
        Err(_) => "Failed to fetch the most recent value for cur TLE".to_string(),
    }
}

//route defined for the estimated time left till epoch
#[get("/TLE_exp")]
fn epoch() -> String {
    let conn = Connection::open("updates.db").expect("Failed to open database");
    match getval(&conn, "TLE_exp") {
        Ok(most_recent_value) => format!("The most recent value for TLEexp is: {}", most_recent_value),
        Err(_) => "Failed to fetch the most recent value for TLEexp".to_string(),
    }
}

//route defined for previous command sent to satellite
#[get("/prev_com")]
fn previous() -> String {
    let conn = Connection::open("updates.db").expect("Failed to open database");
    match getval(&conn, "prev_com") {
        Ok(most_recent_value) => format!("The most recent value for previous communication is: {}", most_recent_value),
        Err(_) => "Failed to fetch the most recent value for previous communication".to_string(),
    }
}

//struct where all the necessary information regarding our satellite will be stored and put in the database
#[derive(Clone)]
struct Information {
    time_r : String,
    time_s : String,
    est_time : String,
    cur_TLE : String,
    TLE_exp : String,
    prev_com : String,
}

//a connection to the database file and the desired information struct is passed as parameters
//using sqlite commands, inputs value for all columns using the values of each field of the struct
fn addtodb(conn : &Connection, info : &Information) -> Result<(), Error> {
    conn.execute(
        "INSERT INTO satupdate (time_r, time_s, est_time, cur_TLE, TLE_exp, prev_com) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (&info.time_r, &info.time_s, &info.est_time, &info.cur_TLE, &info.TLE_exp, &info.prev_com),
    )?;  
    Ok(())
}

//using sqlite commands, the rows are arranged in descending order
//the most recent time is the first row, and then the first row item of that column is returned
fn getval(conn : &Connection, desired_col : &str) -> Result<String, Error>{
    let gate = conn.prepare(&format!("SELECT {} FROM satupdate ORDER BY time_r DESC LIMIT 1",desired_col ));
    gate?.query_row([], |row| {
        row.get(0)
    })
}

//considered as main fn, databse is made and the table insides columns are created.
//struct information is created and and sent to add to db to be added to the database
//routes are mounted using the rocket mount command for the above fields
#[launch]
fn rocket() -> _ {
    let conn = Connection::open("updates.db").expect("Failed to open database");

    conn.execute(
        "create table if not exists satupdate(
            time_r text not null,
            time_s text not null, 
            est_time text not null,
            cur_TLE text not null,
            TLE_exp text not null, 
            prev_com text not null
        )",
        [],
    ).expect("Failed to create table");

    let info = Information {
        time_r: "2024-07-21 01:10:40".to_string(),
        time_s: "2024-07-21 02:25:30".to_string(),
        est_time: "5 minutes".to_string(),
        cur_TLE: "125544U98067A".to_string(),
        TLE_exp: "2024-07-22 10:00:00".to_string(),
        prev_com: "Previous command executed was AbCXyZ.".to_string(),
    };

    addtodb(&conn, &info.clone()).expect("Failed to add to db");
    addtodb(&conn, &info).expect("Failed to add to db");

    rocket::build().mount("/", routes![base, received, sent, timetopass, currentTLE, epoch, previous])
}