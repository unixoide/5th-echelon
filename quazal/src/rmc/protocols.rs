mod ticket_granting_protocol;

#[deprecated]
pub use ticket_granting_protocol::TicketGrantingProtocol;
#[deprecated]
pub use ticket_granting_protocol::TicketGrantingProtocolTrait;

impl TicketGrantingProtocolTrait for TicketGrantingProtocol {}
