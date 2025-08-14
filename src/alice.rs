

pub struct Engine {
    cached_values: HashMap<String, Data>,
}

pub struct Connection {
    pub name: String,
    pub token: String, 
}

pub struct Instance {
    pub connections: Vec<Connection>,
}

pub struct AliceServer {
    pub instances: Vec<Instance>,
}


impl AliceServer {
    pub fn init() -> Self {
        let mut instances = Vec::new();
        Self { instances }
    }
}
