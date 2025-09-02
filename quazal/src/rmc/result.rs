/// This module defines various error types and their conversions.
use std::convert::TryFrom;

use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;

/// Represents a general error type, encompassing various specific error categories.
#[derive(Debug, Clone, Copy)]
pub enum Error {
    /// Core error.
    Core(CoreError),
    /// DDL error.
    DDL(DDLError),
    /// RendezVous error.
    RendezVous(RendezVousError),
    /// Python core error.
    PythonCore(PythonCoreError),
    /// Transport error.
    Transport(TransportError),
    /// DO core error.
    DOCore(DOCoreError),
    /// FPD error.
    FPD(FPDError),
    /// Ranking error.
    Ranking(RankingError),
    /// Authentication error.
    Authentication(AuthenticationError),
    /// Data store error.
    DataStore(DataStoreError),
    /// Service item error.
    ServiceItem(ServiceItemError),
    /// Matchmake referee error.
    MatchmakeReferee(MatchmakeRefereeError),
    /// Subscriber error.
    Subscriber(SubscriberError),
    /// Ranking2 error.
    Ranking2(Ranking2Error),
    /// Smart device voice chat error.
    SmartDeviceVoiceChat(SmartDeviceVoiceChatError),
    /// Screening error.
    Screening(ScreeningError),
    /// Custom error.
    Custom(CustomError),
    /// Ess error.
    Ess(EssError),
}

impl From<Error> for u32 {
    /// Converts an `Error` enum into a `u32` representation.
    /// The `u32` error code is structured as follows:
    /// - The most significant bit (MSB) `0x8000_0000` is set to indicate a Quazal error.
    /// - The next 16 bits (bits 16-30) represent the error category.
    /// - The least significant 16 bits (bits 0-15) represent the specific error code within that category.
    fn from(err: Error) -> Self {
        let code = match err {
            Error::Core(inner) => (1 << 16) | u32::from(u16::from(inner)),
            Error::DDL(inner) => (2 << 16) | u32::from(u16::from(inner)),
            Error::RendezVous(inner) => (3 << 16) | u32::from(u16::from(inner)),
            Error::PythonCore(inner) => (4 << 16) | u32::from(u16::from(inner)),
            Error::Transport(inner) => (5 << 16) | u32::from(u16::from(inner)),
            Error::DOCore(inner) => (6 << 16) | u32::from(u16::from(inner)),
            Error::FPD(inner) => (0x65 << 16) | u32::from(u16::from(inner)),
            Error::Ranking(inner) => (0x67 << 16) | u32::from(u16::from(inner)),
            Error::Authentication(inner) => (0x68 << 16) | u32::from(u16::from(inner)),
            Error::DataStore(inner) => (0x69 << 16) | u32::from(u16::from(inner)),
            Error::ServiceItem(inner) => (0x6c << 16) | u32::from(u16::from(inner)),
            Error::MatchmakeReferee(inner) => (0x6f << 16) | u32::from(u16::from(inner)),
            Error::Subscriber(inner) => (0x70 << 16) | u32::from(u16::from(inner)),
            Error::Ranking2(inner) => (0x71 << 16) | u32::from(u16::from(inner)),
            Error::SmartDeviceVoiceChat(inner) => (0x72 << 16) | u32::from(u16::from(inner)),
            Error::Screening(inner) => (0x73 << 16) | u32::from(u16::from(inner)),
            Error::Custom(inner) => (0x74 << 16) | u32::from(u16::from(inner)),
            Error::Ess(inner) => (0x75 << 16) | u32::from(u16::from(inner)),
        };
        // Set the MSB to indicate a Quazal error
        code | 0x8000_0000
    }
}

impl TryFrom<u32> for Error {
    type Error = u32;

    /// Attempts to convert a `u32` error code back into an `Error` enum.
    /// It checks the MSB to ensure it's a Quazal error, then extracts the category
    /// and specific error code to reconstruct the enum.
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        #![allow(clippy::cast_possible_truncation)]

        // Check if the MSB is set, indicating a Quazal error
        if value & 0x8000_0000 == 0 {
            return Err(value);
        }

        // Clear the MSB to get the actual error code
        let code = value ^ 0x8000_0000;
        match code {
            // Extract the category (higher 16 bits) and specific error code (lower 16 bits)
            v if v >> 16 == 1 => Ok(Self::Core(CoreError::try_from(v as u16).or(Err(value))?)),
            v if v >> 16 == 2 => Ok(Self::DDL(DDLError::try_from(v as u16).or(Err(value))?)),
            v if v >> 16 == 3 => Ok(Self::RendezVous(RendezVousError::try_from(v as u16).or(Err(value))?)),
            v if v >> 16 == 4 => Ok(Self::PythonCore(PythonCoreError::try_from(v as u16).or(Err(value))?)),
            v if v >> 16 == 5 => Ok(Self::Transport(TransportError::try_from(v as u16).or(Err(value))?)),
            v if v >> 16 == 6 => Ok(Self::DOCore(DOCoreError::try_from(v as u16).or(Err(value))?)),
            v if v >> 16 == 0x65 => Ok(Self::FPD(FPDError::try_from(v as u16).or(Err(value))?)),
            v if v >> 16 == 0x67 => Ok(Self::Ranking(RankingError::try_from(v as u16).or(Err(value))?)),
            v if v >> 16 == 0x68 => Ok(Self::Authentication(AuthenticationError::try_from(v as u16).or(Err(value))?)),
            v if v >> 16 == 0x69 => Ok(Self::DataStore(DataStoreError::try_from(v as u16).or(Err(value))?)),
            v if v >> 16 == 0x6c => Ok(Self::ServiceItem(ServiceItemError::try_from(v as u16).or(Err(value))?)),
            v if v >> 16 == 0x6f => Ok(Self::MatchmakeReferee(MatchmakeRefereeError::try_from(v as u16).or(Err(value))?)),
            v if v >> 16 == 0x70 => Ok(Self::Subscriber(SubscriberError::try_from(v as u16).or(Err(value))?)),
            v if v >> 16 == 0x71 => Ok(Self::Ranking2(Ranking2Error::try_from(v as u16).or(Err(value))?)),
            v if v >> 16 == 0x72 => Ok(Self::SmartDeviceVoiceChat(SmartDeviceVoiceChatError::try_from(v as u16).or(Err(value))?)),
            v if v >> 16 == 0x73 => Ok(Self::Screening(ScreeningError::try_from(v as u16).or(Err(value))?)),
            v if v >> 16 == 0x74 => Ok(Self::Custom(CustomError::try_from(v as u16).or(Err(value))?)),
            v if v >> 16 == 0x75 => Ok(Self::Ess(EssError::try_from(v as u16).or(Err(value))?)),
            _ => Err(value),
        }
    }
}

/// Represents a core error code.
#[derive(TryFromPrimitive, IntoPrimitive, Debug, Clone, Copy)]
#[repr(u16)]
pub enum CoreError {
    Unknown = 0x0001,
    NotImplemented = 0x0002,
    InvalidPointer = 0x0003,
    OperationAborted = 0x0004,
    Exception = 0x0005,
    AccessDenied = 0x0006,
    InvalidHandle = 0x0007,
    InvalidIndex = 0x0008,
    OutOfMemory = 0x0009,
    InvalidArgument = 0x000A,
    Timeout = 0x000B,
    InitializationFailure = 0x000C,
    CallInitiationFailure = 0x000D,
    RegistrationError = 0x000E,
    BufferOverflow = 0x000F,
    InvalidLockState = 0x0010,
    InvalidSequence = 0x0011,
    SystemError = 0x0012,
    Cancelled = 0x0013,
}

/// Represents a DDL error code.
#[derive(TryFromPrimitive, IntoPrimitive, Debug, Clone, Copy)]
#[repr(u16)]
pub enum DDLError {
    InvalidSignature = 0x0001,
    IncorrectVersion = 0x0002,
}

/// Represents a RendezVous error code.
#[derive(TryFromPrimitive, IntoPrimitive, Debug, Clone, Copy)]
#[repr(u16)]
pub enum RendezVousError {
    ConnectionFailure = 0x0001,
    NotAuthenticated = 0x0002,
    InvalidUsername = 0x0064,
    InvalidPassword = 0x0065,
    UsernameAlreadyExists = 0x0066,
    AccountDisabled = 0x0067,
    AccountExpired = 0x0068,
    ConcurrentLoginDenied = 0x0069,
    EncryptionFailure = 0x006A,
    InvalidPID = 0x006B,
    MaxConnectionsReached = 0x006C,
    InvalidGID = 0x006D,
    InvalidControlScriptID = 0x006E,
    InvalidOperationInLiveEnvironment = 0x006F,
    DuplicateEntry = 0x0070,
    ControlScriptFailure = 0x0071,
    ClassNotFound = 0x0072,
    SessionVoid = 0x0073,
    DDLMismatch = 0x0075,
    InvalidConfiguration = 0x0076,
    SessionFull = 0x00C8,
    InvalidGatheringPassword = 0x00C9,
    WithoutParticipationPeriod = 0x00CA,
    PersistentGatheringCreationMax = 0x00CB,
    PersistentGatheringParticipationMax = 0x00CC,
    DeniedByParticipants = 0x00CD,
    ParticipantInBlackList = 0x00CE,
    GameServerMaintenance = 0x00CF,
    OperationPostpone = 0x00D0,
    OutOfRatingRange = 0x00D1,
    ConnectionDisconnected = 0x00D2,
    InvalidOperation = 0x00D3,
    NotParticipatedGathering = 0x00D4,
    MatchmakeSessionUserPasswordUnmatch = 0x00D5,
    MatchmakeSessionSystemPasswordUnmatch = 0x00D6,
    UserIsOffline = 0x00D7,
    AlreadyParticipatedGathering = 0x00D8,
    PermissionDenied = 0x00D9,
    NotFriend = 0x00DA,
    SessionClosed = 0x00DB,
    DatabaseTemporarilyUnavailable = 0x00DC,
    InvalidUniqueId = 0x00DD,
    MatchmakingWithdrawn = 0x00DE,
    LimitExceeded = 0x00DF,
    AccountTemporarilyDisabled = 0x00E0,
    PartiallyServiceClosed = 0x00E1,
    ConnectionDisconnectedForConcurrentLogin = 0x00E2,
}

/// Represents a Python core error code.
#[derive(TryFromPrimitive, IntoPrimitive, Debug, Clone, Copy)]
#[repr(u16)]
pub enum PythonCoreError {
    Exception = 0x0001,
    TypeError = 0x0002,
    IndexError = 0x0003,
    InvalidReference = 0x0004,
    CallFailure = 0x0005,
    MemoryError = 0x0006,
    KeyError = 0x0007,
    OperationError = 0x0008,
    ConversionError = 0x0009,
    ValidationError = 0x000A,
}

/// Represents a transport error code.
#[derive(TryFromPrimitive, IntoPrimitive, Debug, Clone, Copy)]
#[repr(u16)]
pub enum TransportError {
    Unknown = 0x0001,
    ConnectionFailure = 0x0002,
    InvalidUrl = 0x0003,
    InvalidKey = 0x0004,
    InvalidURLType = 0x0005,
    DuplicateEndpoint = 0x0006,
    IOError = 0x0007,
    Timeout = 0x0008,
    ConnectionReset = 0x0009,
    IncorrectRemoteAuthentication = 0x000A,
    ServerRequestError = 0x000B,
    DecompressionFailure = 0x000C,
    ReliableSendBufferFullFatal = 0x000D,
    UPnPCannotInit = 0x000E,
    UPnPCannotAddMapping = 0x000F,
    NatPMPCannotInit = 0x0010,
    NatPMPCannotAddMapping = 0x0011,
    UnsupportedNAT = 0x0013,
    DnsError = 0x0014,
    ProxyError = 0x0015,
    DataRemaining = 0x0016,
    NoBuffer = 0x0017,
    NotFound = 0x0018,
    TemporaryServerError = 0x0019,
    PermanentServerError = 0x001A,
    ServiceUnavailable = 0x001B,
    ReliableSendBufferFull = 0x001C,
    InvalidStation = 0x001D,
    InvalidSubStreamID = 0x001E,
    PacketBufferFull = 0x001F,
    NatTraversalError = 0x0020,
    NatCheckError = 0x0021,
}

/// Represents a DO core error code.
#[derive(TryFromPrimitive, IntoPrimitive, Debug, Clone, Copy)]
#[repr(u16)]
pub enum DOCoreError {
    StationNotReached = 0x0001,
    TargetStationDisconnect = 0x0002,
    LocalStationLeaving = 0x0003,
    ObjectNotFound = 0x0004,
    InvalidRole = 0x0005,
    CallTimeout = 0x0006,
    RMCDispatchFailed = 0x0007,
    MigrationInProgress = 0x0008,
    NoAuthority = 0x0009,
    NoTargetStationSpecified = 0x000A,
    JoinFailed = 0x000B,
    JoinDenied = 0x000C,
    ConnectivityTestFailed = 0x000D,
    Unknown = 0x000E,
    UnfreedReferences = 0x000F,
    JobTerminationFailed = 0x0010,
    InvalidState = 0x0011,
    FaultRecoveryFatal = 0x0012,
    FaultRecoveryJobProcessFailed = 0x0013,
    StationInconsitency = 0x0014,
    AbnormalMasterState = 0x0015,
    VersionMismatch = 0x0016,
}

/// Represents an FPD error code.
#[derive(TryFromPrimitive, IntoPrimitive, Debug, Clone, Copy)]
#[repr(u16)]
pub enum FPDError {
    NotInitialized = 0x0000,
    AlreadyInitialized = 0x0001,
    NotConnected = 0x0002,
    Connected = 0x0003,
    InitializationFailure = 0x0004,
    OutOfMemory = 0x0005,
    RmcFailed = 0x0006,
    InvalidArgument = 0x0007,
    InvalidLocalAccountID = 0x0008,
    InvalidPrincipalID = 0x0009,
    InvalidLocalFriendCode = 0x000A,
    LocalAccountNotExists = 0x000B,
    LocalAccountNotLoaded = 0x000C,
    LocalAccountAlreadyLoaded = 0x000D,
    FriendAlreadyExists = 0x000E,
    FriendNotExists = 0x000F,
    FriendNumMax = 0x0010,
    NotFriend = 0x0011,
    FileIO = 0x0012,
    P2PInternetProhibited = 0x0013,
    Unknown = 0x0014,
    InvalidState = 0x0015,
    AddFriendProhibited = 0x0017,
    InvalidAccount = 0x0019,
    BlacklistedByMe = 0x001A,
    FriendAlreadyAdded = 0x001C,
    MyFriendListLimitExceed = 0x001D,
    RequestLimitExceed = 0x001E,
    InvalidMessageID = 0x001F,
    MessageIsNotMine = 0x0020,
    MessageIsNotForMe = 0x0021,
    FriendRequestBlocked = 0x0022,
    NotInMyFriendList = 0x0023,
    FriendListedByMe = 0x0024,
    NotInMyBlacklist = 0x0025,
    IncompatibleAccount = 0x0026,
    BlockSettingChangeNotAllowed = 0x0027,
    SizeLimitExceeded = 0x0028,
    OperationNotAllowed = 0x0029,
    NotNetworkAccount = 0x002A,
    NotificationNotFound = 0x002B,
    PreferenceNotInitialized = 0x002C,
    FriendRequestNotAllowed = 0x002D,
}

/// Represents a ranking error code.
#[derive(TryFromPrimitive, IntoPrimitive, Debug, Clone, Copy)]
#[repr(u16)]
pub enum RankingError {
    NotInitialized = 0x0001,
    InvalidArgument = 0x0002,
    RegistrationError = 0x0003,
    NotFound = 0x0005,
    InvalidScore = 0x0006,
    InvalidDataSize = 0x0007,
    PermissionDenied = 0x0009,
    Unknown = 0x000A,
    NotImplemented = 0x000B,
}

/// Represents an authentication error code.
#[derive(TryFromPrimitive, IntoPrimitive, Debug, Clone, Copy)]
#[repr(u16)]
pub enum AuthenticationError {
    NASAuthenticateError = 0x0001,
    TokenParseError = 0x0002,
    HttpConnectionError = 0x0003,
    HttpDNSError = 0x0004,
    HttpGetProxySetting = 0x0005,
    TokenExpired = 0x0006,
    ValidationFailed = 0x0007,
    InvalidParam = 0x0008,
    PrincipalIdUnmatched = 0x0009,
    MoveCountUnmatch = 0x000A,
    UnderMaintenance = 0x000B,
    UnsupportedVersion = 0x000C,
    ServerVersionIsOld = 0x000D,
    Unknown = 0x000E,
    ClientVersionIsOld = 0x000F,
    AccountLibraryError = 0x0010,
    ServiceNoLongerAvailable = 0x0011,
    UnknownApplication = 0x0012,
    ApplicationVersionIsOld = 0x0013,
    OutOfService = 0x0014,
    NetworkServiceLicenseRequired = 0x0015,
    NetworkServiceLicenseSystemError = 0x0016,
    NetworkServiceLicenseError3 = 0x0017,
    NetworkServiceLicenseError4 = 0x0018,
}

/// Represents a data store error code.
#[derive(TryFromPrimitive, IntoPrimitive, Debug, Clone, Copy)]
#[repr(u16)]
pub enum DataStoreError {
    Unknown = 0x0001,
    InvalidArgument = 0x0002,
    PermissionDenied = 0x0003,
    NotFound = 0x0004,
    AlreadyLocked = 0x0005,
    UnderReviewing = 0x0006,
    Expired = 0x0007,
    InvalidCheckToken = 0x0008,
    SystemFileError = 0x0009,
    OverCapacity = 0x000A,
    OperationNotAllowed = 0x000B,
    InvalidPassword = 0x000C,
    ValueNotEqual = 0x000D,
}

/// Represents a service item error code.
#[derive(TryFromPrimitive, IntoPrimitive, Debug, Clone, Copy)]
#[repr(u16)]
pub enum ServiceItemError {
    Unknown = 0x0001,
    InvalidArgument = 0x0002,
    EShopUnknownHttpError = 0x0003,
    EShopResponseParseError = 0x0004,
    NotOwned = 0x0005,
    InvalidLimitationType = 0x0006,
    ConsumptionRightShortage = 0x0007,
}

/// Represents a matchmake referee error code.
#[derive(TryFromPrimitive, IntoPrimitive, Debug, Clone, Copy)]
#[repr(u16)]
pub enum MatchmakeRefereeError {
    Unknown = 0x0001,
    InvalidArgument = 0x0002,
    AlreadyExists = 0x0003,
    NotParticipatedGathering = 0x0004,
    NotParticipatedRound = 0x0005,
    StatsNotFound = 0x0006,
    RoundNotFound = 0x0007,
    RoundArbitrated = 0x0008,
    RoundNotArbitrated = 0x0009,
}

/// Represents a subscriber error code.
#[derive(TryFromPrimitive, IntoPrimitive, Debug, Clone, Copy)]
#[repr(u16)]
pub enum SubscriberError {
    Unknown = 0x0001,
    InvalidArgument = 0x0002,
    OverLimit = 0x0003,
    PermissionDenied = 0x0004,
}

/// Represents a ranking2 error code.
#[derive(TryFromPrimitive, IntoPrimitive, Debug, Clone, Copy)]
#[repr(u16)]
pub enum Ranking2Error {
    Unknown = 0x0001,
    InvalidArgument = 0x0002,
    InvalidScore = 0x0003,
}

/// Represents a smart device voice chat error code.
#[derive(TryFromPrimitive, IntoPrimitive, Debug, Clone, Copy)]
#[repr(u16)]
pub enum SmartDeviceVoiceChatError {
    Unknown = 0x0001,
    InvalidArgument = 0x0002,
    InvalidResponse = 0x0003,
    InvalidAccessToken = 0x0004,
    Unauthorized = 0x0005,
    AccessError = 0x0006,
    UserNotFound = 0x0007,
    RoomNotFound = 0x0008,
    RoomNotActivated = 0x0009,
    ApplicationNotSupported = 0x000A,
    InternalServerError = 0x000B,
    ServiceUnavailable = 0x000C,
    UnexpectedError = 0x000D,
    UnderMaintenance = 0x000E,
    ServiceNoLongerAvailable = 0x000F,
    AccountTemporarilyDisabled = 0x0010,
    PermissionDenied = 0x0011,
    NetworkServiceLicenseRequired = 0x0012,
    AccountLibraryError = 0x0013,
    GameModeNotFound = 0x0014,
}

/// Represents a screening error code.
#[derive(TryFromPrimitive, IntoPrimitive, Debug, Clone, Copy)]
#[repr(u16)]
pub enum ScreeningError {
    Unknown = 0x0001,
    InvalidArgument = 0x0002,
    NotFound = 0x0003,
}

/// Represents a custom error code.
#[derive(TryFromPrimitive, IntoPrimitive, Debug, Clone, Copy)]
#[repr(u16)]
pub enum CustomError {
    Unknown = 0x0001,
}

/// Represents an Ess error code.
#[derive(TryFromPrimitive, IntoPrimitive, Debug, Clone, Copy)]
#[repr(u16)]
pub enum EssError {
    Unknown = 0x0001,
    GameSessionError = 0x0002,
    GameSessionMaintenance = 0x0003,
}
