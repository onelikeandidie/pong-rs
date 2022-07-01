use bevy::utils::HashMap;

use crate::shared::network::packet_events::ClientInputData;

#[derive(Debug, Default)]
pub struct ClientList {
    pub list: HashMap<u32, ClientData>
}

impl ClientList {
    pub fn new_client_input(&mut self, handle: Option<usize>) {
        if let Some(handle) = handle {
            self.list.insert(handle as u32, ClientData::default());
        } else {
            self.list.insert(*self.list.keys().max().unwrap_or(&0), ClientData::default());
        }
    }
    pub fn update_client_input(&mut self, handle: usize, new_input: ClientInputData) -> Result<(), ()> {
        let client_data = self.list.get_mut(&(handle as u32));
        match client_data {
            Some(mut input) => {
                input.input = new_input;
                return Ok(());
            },
            None => Err(()),
        }
    }
    pub fn clear_client_input(&mut self, handle: usize) -> Result<(), ()> {
        let client_data = self.list.get_mut(&(handle as u32));
        match client_data {
            Some(mut input) => {
                input.input = ClientInputData::default();
                return Ok(());
            },
            None => Err(()),
        }
    }
    pub fn get(&self, handle: usize) -> Option<&ClientData> {
        self.list.get(&(handle as u32))
    }
}

#[derive(Debug, Default)]
pub struct ClientData {
    pub input: ClientInputData
}