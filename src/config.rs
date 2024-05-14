

#[derive(Debug, Clone)]
pub struct Config {
    pub rpc: String,
    pub thread: usize,
    pub time: u64,
    pub json_address_file: String,
    pub key_store_file: String,
    
}


pub struct TransferParams {
    pub amount: u64,
    pub step: usize,
    pub gas_budget: u64,
}


impl Default for TransferParams  {
    fn default() ->Self{
        Self {
            amount: 0,
            step: 0,
            gas_budget: 0,
        }
    }
    
}


impl Default for Config{
    fn default() ->Self{
        Self {
            rpc: "rpc".to_string(),
            thread: 10,
            time: 0,
            json_address_file: "".to_string(),
            key_store_file: "".to_string(),
        }
    }
}




