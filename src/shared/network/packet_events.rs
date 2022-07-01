#[derive(Debug)]
pub enum EventMasks {
    Ping(PingData),
    Connection(ConnectionData),
    ClientInput(ClientInputData),
}

impl EventMasks {
    pub fn to_byte(&self) -> u8 {
        match self {
            Self::Ping(_) => 1 << 0,
            Self::Connection(_) => 1 << 1,
            Self::ClientInput(_) => 1 << 2
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Self::Ping(event) => vec![1 << 0, event.to_byte()],
            Self::Connection(event) => vec![1 << 1, event.to_byte()],
            Self::ClientInput(event) => vec![1 << 2, event.to_byte()]
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ()> {
        match bytes[0] {
            0b00000001 => {
                let event = PingData::from_bytes(bytes).unwrap();
                Ok(Self::Ping(event))
            }
            0b00000010 => {
                let event = ConnectionData::from_bytes(bytes).unwrap();
                Ok(Self::Connection(event))
            },
            0b00000100 => {
                let event = ClientInputData::from_bytes(bytes).unwrap();
                Ok(Self::ClientInput(event))
            },
            _ => Err(())
        }
    }
}

#[derive(Debug)]
pub enum PingData {
    Ping,
    Pong
}

impl PingData {
    pub fn to_byte(&self) -> u8 {
        match self {
            Self::Ping => 1 << 0,
            Self::Pong => 1 << 1
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ()> {
        let bytes =  &bytes[1..]; // Slice out the mask
        match bytes[0] {
            0b00000001 => Ok(Self::Ping),
            0b00000010 => Ok(Self::Pong),
            _ => Err(())
        }
    }
}

#[derive(Debug)]
pub enum ConnectionData {
    Connect,
    Disconnect,
    Ack
}

impl ConnectionData {
    pub fn to_byte(&self) -> u8 {
        match self {
            Self::Connect => 1 << 0,
            Self::Disconnect => 1 << 1,
            Self::Ack => 1 << 2
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ()> {
        let bytes =  &bytes[1..]; // Slice out the mask
        match bytes[0] {
            0b00000001 => Ok(Self::Connect),
            0b00000010 => Ok(Self::Disconnect),
            0b00000100 => Ok(Self::Ack),
            _ => Err(())
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ClientInputData {
    pub top: bool,
    pub right: bool,
    pub bottom: bool,
    pub left: bool,
}

impl ClientInputData {
    pub fn to_byte(&self) -> u8 {
        // 0b----TRBL
        // 0b00000000
        let t = self.top as u8;
        let r = self.right as u8;
        let b = self.bottom as u8;
        let l = self.left as u8;
        let result = 
            t << 0 |
            r << 1 |
            b << 2 |
            l << 3;
        return result;
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ()> {
        let bytes =  &bytes[1..]; // Slice out the mask
        Ok(Self {
            top:    (bytes[0] & 1 << 0) != 0,
            right:  (bytes[0] & 1 << 1) != 0,
            bottom: (bytes[0] & 1 << 2) != 0,
            left:   (bytes[0] & 1 << 3) != 0,
        })
    }
}