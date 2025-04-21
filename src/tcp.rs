use std::{net::Ipv4Addr, u8};

// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |          Source Port          |       Destination Port        |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                        Sequence Number                        |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                    Acknowledgment Number                      |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |  Data |           |U|A|P|R|S|F|                               |
// | Offset| Reserved  |R|C|S|S|Y|I|            Window             |
// |       |           |G|K|H|T|N|N|                               |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |           Checksum            |         Urgent Pointer        |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                    Options                    |    Padding    |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                             data                              |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

/// |----------------------|-------------|----------------------------------------------------------------------------------|
/// | Field                | Size (bits) | Description                                                                      |
/// |----------------------|-------------|----------------------------------------------------------------------------------|
/// | Source Port          | 16          | Identifies the source application port on the sender's host.                     |
/// | Destination Port     | 16          | Identifies the destination application port on the receiver's host.              |
/// | Sequence Number      | 32          | Specifies the sequence number of the first byte of data in this segment.         |
/// | Acknowledgment Number| 32          | Specifies the next sequence number the sender of the segment expects to receive. |
/// | Data Offset          | 4           | Indicates the size of the TCP header in 32-bit words.                            |
/// | Reserved             | 3           | Reserved for future use; should be set to zero.                                  |
/// | Flags                | 9           | Includes control flags such as URG, ACK, PSH, RST, SYN, FIN.                     |
/// | Window Size          | 16          | Specifies the size of the sender's receive window (the buffer space available).  |
/// | Checksum             | 16          | Used for error-checking the header and data.                                     |
/// | Urgent Pointer       | 16          | If the URG flag is set, this field points to the last urgent byte in the data.   |
/// | Options (optional)   | Variable    | May include options like MSS, timestamp, etc.                                    |
/// | Padding              | Variable    | Added to ensure the header is a multiple of 32 bits in length.                   |
/// | Data                 | Variable    | Contains the application data being transmitted.                                 |
/// |----------------------|-------------|----------------------------------------------------------------------------------|

/// Header TCP
#[derive(Debug)]
pub struct TCP {
    pub source: Ipv4Addr,
    pub destination: Ipv4Addr,
    pub sequence: u32,
    pub acknowledgment: u32,
    pub data_offset: u8,
    pub reserved: u8,
    pub flags: u16,
    pub window_size: u16,
    pub checksum: u16,
    pub urgent_pointer: u16,
    pub options: Vec<u8>,
    pub padding: Vec<u8>,
    pub data: Vec<u8>,
}


/// Implementation of methods for the TCP header, including a constructor (`new`),
/// getters (field reading) and setters (fluent field modification).
impl TCP {
    /// Constructor to create a new instance of a TCP packet.
    /// All fields must be provided at creation time.
    pub fn new(
        source: Ipv4Addr,
        destination: Ipv4Addr,
        sequence: u32,
        acknowledgment: u32,
        data_offset: u8,
        reserved: u8,
        flags: u16,
        window_size: u16,
        checksum: u16,
        urgent_pointer: u16,
        options: Vec<u8>,
        padding: Vec<u8>,
        data: Vec<u8>,
    ) -> Self {
        TCP {
            source,
            destination,
            sequence,
            acknowledgment,
            data_offset,
            reserved,
            flags,
            window_size,
            checksum,
            urgent_pointer,
            options,
            padding,
            data,
        }
    }

    // --- GETTER METHODS ---

    /// Returns the source IP address.
    pub fn get_source(&mut self) -> Ipv4Addr {
        self.source
    }

    /// Returns the destination IP address.
    pub fn get_destination(&mut self) -> Ipv4Addr {
        self.destination
    }

    /// Returns the sequence number.
    pub fn get_sequence(&mut self) -> u32 {
        self.sequence
    }

    /// Returns the acknowledgment number.
    pub fn get_acknowledgement(&mut self) -> u32 {
        self.acknowledgment
    }

    /// Returns the data offset.
    pub fn get_data_offset(&mut self) -> u8 {
        self.data_offset
    }

    /// Returns the reserved field.
    pub fn get_reserved(&mut self) -> u8 {
        self.reserved
    }

    /// Returns the TCP flags.
    pub fn get_flags(&mut self) -> u16 {
        self.flags
    }

    /// Returns the window size.
    pub fn get_window_size(&mut self) -> u16 {
        self.window_size
    }

    /// Returns the checksum value.
    pub fn get_checksum(&mut self) -> u16 {
        self.checksum
    }

    /// Returns the urgent pointer.
    pub fn get_urgent_ponter(&mut self) -> u16 {
        self.urgent_pointer
    }

    /// Returns the options (optional header field).
    pub fn get_options(&mut self) -> Vec<u8> {
        self.options.clone()
    }

    /// Returns the padding.
    pub fn get_padding(&mut self) -> Vec<u8> {
        self.padding.clone()
    }

    /// Returns the data carried by the TCP packet.
    pub fn get_data(&mut self) -> Vec<u8> {
        self.data.clone()
    }

    // --- SETTER METHODS ---

    /// Sets the source IP address.
    pub fn set_source(mut self, source: Ipv4Addr) -> Self {
        self.source = source;
        self
    }

    /// Sets the destination IP address.
    pub fn set_destination(mut self, destination: Ipv4Addr) -> Self {
        self.destination = destination;
        self
    }

    /// Sets the sequence number.
    pub fn set_sequence(mut self, sequence: u32) -> Self {
        self.sequence = sequence;
        self
    }

    /// Sets the acknowledgment number.
    pub fn set_acknowledgement(mut self, acknowledgment: u32) -> Self {
        self.acknowledgment = acknowledgment;
        self
    }

    /// Sets the data offset.
    pub fn set_data_offset(mut self, data_offset: u8) -> Self {
        self.data_offset = data_offset;
        self
    }

    /// Sets the reserved field.
    pub fn set_reserved(mut self, reserved: u8) -> Self {
        self.reserved = reserved;
        self
    }

    /// Sets the flags.
    pub fn set_flags(mut self, flags: u16) -> Self {
        self.flags = flags;
        self
    }

    /// Sets the window size.
    pub fn set_window_size(mut self, window_size: u16) -> Self {
        self.window_size = window_size;
        self
    }

    /// Sets the checksum.
    pub fn set_checksum(mut self, checksum: u16) -> Self {
        self.checksum = checksum;
        self
    }

    /// Sets the urgent pointer.
    pub fn set_urgent_ponter(mut self, urgent_pointer: u16) -> Self {
        self.urgent_pointer = urgent_pointer;
        self
    }

    /// Sets the options.
    pub fn set_options(mut self, options: Vec<u8>) -> Self {
        self.options = options;
        self
    }

    /// Sets the padding.
    pub fn set_padding(mut self, padding: Vec<u8>) -> Self {
        self.padding = padding;
        self
    }

    /// Sets the packet's data.
    pub fn set_data(mut self, data: Vec<u8>) -> Self {
        self.data = data;
        self
    }
}



