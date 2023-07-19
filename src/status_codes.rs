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
pub const GROUP_CREATE_ACK: u8 = 21;
pub const GROUP_DELETE_ACK: u8 = 22;
pub const GROUP_JOIN_REQ: u8 = 23;
pub const GROUP_JOIN_REQ_REJECTED: u8 = 24;
pub const GROUP_JOIN_ACK: u8 = 25;
pub const GROUP_LEAVE_REQ: u8 = 26;
pub const GROUP_LEAVE_ACK: u8 = 27;
pub const GROUP_MESSAGE_REQ: u8 = 28;
pub const GROUP_MESSAGE_ACK: u8 = 29;
pub const GROUP_PARTICIPANTS_LIST_ACK: u8 = 30;
pub const GROUP_INFO_ACK: u8 = 31;
pub const GROUP_BAN_ACK: u8 = 32;
pub const GROUP_UNBAN_ACK: u8 = 33;
