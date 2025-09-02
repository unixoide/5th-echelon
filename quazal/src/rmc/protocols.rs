/// This module re-exports various RMC protocols.
mod ticket_granting_protocol;

/// Re-exports the `TicketGrantingProtocol`.
#[deprecated]
pub use ticket_granting_protocol::TicketGrantingProtocol;
/// Re-exports the `TicketGrantingProtocolTrait`.
#[deprecated]
pub use ticket_granting_protocol::TicketGrantingProtocolTrait;

impl TicketGrantingProtocolTrait for TicketGrantingProtocol {}
