// General Codes
pub const CONNECTION_REQUEST: u8 = 1;
pub const CONNECTION_REQUEST_REJECTED: u8 = 3;
pub const BROADCAST_MESSAGE: u8 = 4;
pub const DISCONNECT: u8 = 6;

// User codes
pub const USER_DIRECT_MESSAGE: u8 = 11;
pub const USER_BLOCK: u8 = 12;
pub const USER_UNBLOCK: u8 = 13;

// Group codes
pub const GROUP_CREATE: u8 = 21;
pub const GROUP_JOIN: u8 = 22;
pub const GROUP_LEAVE: u8 = 23;
pub const GROUP_MESSAGE: u8 = 24;
pub const GROUP_LIST: u8 = 25;
pub const GROUP_PARTICIPANTS_LIST: u8 = 26;
pub const GROUP_BAN: u8 = 27;
pub const GROUP_DELETE: u8 = 28;
