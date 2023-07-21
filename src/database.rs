use rusqlite::{Connection, params};

pub fn connect() -> Database {
  let path = std::env::var("DATABASE_URL").unwrap();
  let _ = Connection::open(&path).unwrap().close();

  Database {
    db_path: path,
  }
}

pub struct ServerConfig {
  pub lower_bound: u32,
  pub upper_bound: u32,
}

pub struct Database {
  db_path: String,
}

impl Database {
  pub fn create_db(&self) -> Result<(), rusqlite::Error> {
    let connection = Connection::open(&self.db_path)?;
    connection.execute("CREATE TABLE messages (message TEXT);", [])?;
    Ok(())
  }

  pub fn insert_text_beyond_my_comprehension(&mut self, message: &String, is_link: bool) -> Result<(), rusqlite::Error> {
    let connection = Connection::open(&self.db_path)?;
    let query = if is_link {
      "INSERT INTO links (link) VALUES (?1);"
    } else {
      "INSERT INTO messages (message) VALUES (?1);"
    };
    connection.execute(query, params![&message])?;
    let result = connection.close();
    match result {
      Ok(_) => println!("Closed successfully"),
      Err(e) => println!("Error closing: {:?}", e),
    }
    Ok(())
  }

  pub fn get_all_messages(&mut self) -> Result<Vec<String>, rusqlite::Error> {
    let connection = Connection::open(&self.db_path)?;
    let query: &str = "SELECT * FROM messages;" ;
    let mut statement = connection.prepare(query)?;
    let messages = statement.query_map([], |row| {
      row.get::<_, String>(0)   
    })?.map(|r| r.unwrap()).collect();

    Ok(messages)
  }

  pub fn get_all_links(&mut self) -> Result<Vec<String>, rusqlite::Error> {
    let connection = Connection::open(&self.db_path)?;
    let query: &str = "SELECT * FROM links;" ;
    let mut statement = connection.prepare(query)?;
    let messages = statement.query_map([], |row| {
      row.get::<_, String>(0)   
    })?.map(|r| r.unwrap()).collect();

    Ok(messages)
  }

  pub fn set_guild_config(&mut self, guild_id: &str, lower_bound: u32, upper_bound: u32) -> Result<(), rusqlite::Error> {
    let connection = Connection::open(&self.db_path)?;
    let query: &str = "INSERT INTO server_config (guild_id, lower_bound, upper_bound) VALUES (?1, ?2, ?3);" ;
    connection.execute(query, params![guild_id, lower_bound, upper_bound])?;
    let result = connection.close();
    match result {
      Ok(_) => println!("Closed successfully"),
      Err(e) => println!("Error closing: {:?}", e),
    }
    Ok(())
  }

  pub fn get_guild_config(&mut self, guild_id: &str) -> Result<ServerConfig, rusqlite::Error> {
    let connection = Connection::open(&self.db_path)?;
    let query: &str = "SELECT * FROM server_config WHERE guild_id = ?1;" ;
    let mut statement = connection.prepare(query)?;
    let mut result = statement.query([guild_id])?;
    let mut settings = ServerConfig {
      lower_bound: 0,
      upper_bound: 0,
    };
    while let Some(row) = result.next()? {
      let smth: String = row.get(1).unwrap();
      let smth2: String = row.get(2).unwrap();
      settings.lower_bound = smth.parse::<u32>().unwrap();
      settings.upper_bound = smth2.parse::<u32>().unwrap();
      println!("{} {}", smth, smth2);
    }
    
    Ok(settings)
  }  

  pub fn set_filter(&mut self, filter: &str) -> Result<(), rusqlite::Error> {
    let connection = Connection::open(&self.db_path)?;
    let query: &str = "INSERT INTO filters (filter) VALUES (?1);" ;
    connection.execute(query, params![filter])?;
    let result = connection.close();
    match result {
      Ok(_) => println!("Closed successfully"),
      Err(e) => println!("Error closing: {:?}", e),
    }
    Ok(())
  }

  pub fn get_filters(&mut self) -> Result<Vec<String>, rusqlite::Error> {
    let connection = Connection::open(&self.db_path)?;
    let query: &str = "SELECT * FROM filters;" ;
    let mut statement = connection.prepare(query)?;
    let filters = statement.query_map([], |row| {
      row.get::<_, String>(0)   
    })?.map(|r| r.unwrap()).collect();

    Ok(filters)
  }
}