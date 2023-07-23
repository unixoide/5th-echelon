enum CoreError {
    Unknown,
    NotImplemented,
    InvalidPointer,
    OperationAborted,
    Exception,
    AccessDenied,
    InvalidHandle,
    InvalidIndex,
    OutOfMemory,
    InvalidArgument,
    Timeout,
    InitializationFailure,
    CallInitiationFailure,
    RegistrationError,
    BufferOverflow,
    InvalidLockState,
    InvalidSequence,
    SystemError,
    Cancelled,
}

impl From<u32> for CoreError {
    fn from(v: u32) -> Self {
        match v {
            0x10001 => Self::Unknown,
            0x10002 => Self::NotImplemented,
            0x10003 => Self::InvalidPointer,
            0x10004 => Self::OperationAborted,
            0x10005 => Self::Exception,
            0x10006 => Self::AccessDenied,
            0x10007 => Self::InvalidHandle,
            0x10008 => Self::InvalidIndex,
            0x10009 => Self::OutOfMemory,
            0x1000a => Self::InvalidArgument,
            0x1000b => Self::Timeout,
            0x1000c => Self::InitializationFailure,
            0x1000d => Self::CallInitiationFailure,
            0x1000e => Self::RegistrationError,
            0x1000f => Self::BufferOverflow,
            0x10010 => Self::InvalidLockState,
            0x10011 => Self::InvalidSequence,
            0x10012 => Self::SystemError,
            0x10013 => Self::Cancelled,
        }
    }
}

impl From<CoreError> for u32 {
    fn from(v: CoreError) -> Self {
        match v {
            CoreError::Unknown => 0x10001,
            CoreError::NotImplemented => 0x10002,
            CoreError::InvalidPointer => 0x10003,
            CoreError::OperationAborted => 0x10004,
            CoreError::Exception => 0x10005,
            CoreError::AccessDenied => 0x10006,
            CoreError::InvalidHandle => 0x10007,
            CoreError::InvalidIndex => 0x10008,
            CoreError::OutOfMemory => 0x10009,
            CoreError::InvalidArgument => 0x1000a,
            CoreError::Timeout => 0x1000b,
            CoreError::InitializationFailure => 0x1000c,
            CoreError::CallInitiationFailure => 0x1000d,
            CoreError::RegistrationError => 0x1000e,
            CoreError::BufferOverflow => 0x1000f,
            CoreError::InvalidLockState => 0x10010,
            CoreError::InvalidSequence => 0x10011,
            CoreError::SystemError => 0x10012,
            CoreError::Cancelled => 0x10013,
        }
    }
}

enum DDLError {
    InvalidSignature,
    IncorrectVersion,
}

impl From<u32> for DDLError {
    fn from(v: u32) -> Self {
        match v {
            0x20001 => Self::InvalidSignature,
            0x20002 => Self::IncorrectVersion,
        }
    }
}

impl From<DDLError> for u32 {
    fn from(v: DDLError) -> Self {
        match v {
            DDLError::InvalidSignature => 0x20001,
            DDLError::IncorrectVersion => 0x20002,
        }
    }
}

enum RendezVousError {
    ConnectionFailure,
    NotAuthenticated,
    InvalidUsername,
    InvalidPassword,
    UsernameAlreadyExists,
    AccountDisabled,
    AccountExpired,
    ConcurrentLoginDenied,
    EncryptionFailure,
    InvalidPID,
    MaxConnectionsReached,
    InvalidGID,
    InvalidControlScriptID,
    InvalidOperationInLiveEnvironment,
    DuplicateEntry,
    ControlScriptFailure,
    ClassNotFound,
    SessionVoid,
    DDLMismatch,
    InvalidConfiguration,
    SessionFull,
    InvalidGatheringPassword,
    WithoutParticipationPeriod,
    PersistentGatheringCreationMax,
    PersistentGatheringParticipationMax,
    DeniedByParticipants,
    ParticipantInBlackList,
    GameServerMaintenance,
    OperationPostpone,
    OutOfRatingRange,
    ConnectionDisconnected,
    InvalidOperation,
    NotParticipatedGathering,
    MatchmakeSessionUserPasswordUnmatch,
    MatchmakeSessionSystemPasswordUnmatch,
    UserIsOffline,
    AlreadyParticipatedGathering,
    PermissionDenied,
    NotFriend,
    SessionClosed,
    DatabaseTemporarilyUnavailable,
    InvalidUniqueId,
    MatchmakingWithdrawn,
    LimitExceeded,
    AccountTemporarilyDisabled,
    PartiallyServiceClosed,
    ConnectionDisconnectedForConcurrentLogin,
}

impl From<u32> for RendezVousError {
    fn from(v: u32) -> Self {
        match v {
            0x30001 => Self::ConnectionFailure,
            0x30002 => Self::NotAuthenticated,
            0x30064 => Self::InvalidUsername,
            0x30065 => Self::InvalidPassword,
            0x30066 => Self::UsernameAlreadyExists,
            0x30067 => Self::AccountDisabled,
            0x30068 => Self::AccountExpired,
            0x30069 => Self::ConcurrentLoginDenied,
            0x3006a => Self::EncryptionFailure,
            0x3006b => Self::InvalidPID,
            0x3006c => Self::MaxConnectionsReached,
            0x3006d => Self::InvalidGID,
            0x3006e => Self::InvalidControlScriptID,
            0x3006f => Self::InvalidOperationInLiveEnvironment,
            0x30070 => Self::DuplicateEntry,
            0x30071 => Self::ControlScriptFailure,
            0x30072 => Self::ClassNotFound,
            0x30073 => Self::SessionVoid,
            0x30075 => Self::DDLMismatch,
            0x30076 => Self::InvalidConfiguration,
            0x300c8 => Self::SessionFull,
            0x300c9 => Self::InvalidGatheringPassword,
            0x300ca => Self::WithoutParticipationPeriod,
            0x300cb => Self::PersistentGatheringCreationMax,
            0x300cc => Self::PersistentGatheringParticipationMax,
            0x300cd => Self::DeniedByParticipants,
            0x300ce => Self::ParticipantInBlackList,
            0x300cf => Self::GameServerMaintenance,
            0x300d0 => Self::OperationPostpone,
            0x300d1 => Self::OutOfRatingRange,
            0x300d2 => Self::ConnectionDisconnected,
            0x300d3 => Self::InvalidOperation,
            0x300d4 => Self::NotParticipatedGathering,
            0x300d5 => Self::MatchmakeSessionUserPasswordUnmatch,
            0x300d6 => Self::MatchmakeSessionSystemPasswordUnmatch,
            0x300d7 => Self::UserIsOffline,
            0x300d8 => Self::AlreadyParticipatedGathering,
            0x300d9 => Self::PermissionDenied,
            0x300da => Self::NotFriend,
            0x300db => Self::SessionClosed,
            0x300dc => Self::DatabaseTemporarilyUnavailable,
            0x300dd => Self::InvalidUniqueId,
            0x300de => Self::MatchmakingWithdrawn,
            0x300df => Self::LimitExceeded,
            0x300e0 => Self::AccountTemporarilyDisabled,
            0x300e1 => Self::PartiallyServiceClosed,
            0x300e2 => Self::ConnectionDisconnectedForConcurrentLogin,
        }
    }
}

impl From<RendezVousError> for u32 {
    fn from(v: RendezVousError) -> Self {
        match v {
            RendezVousError::ConnectionFailure => 0x30001,
            RendezVousError::NotAuthenticated => 0x30002,
            RendezVousError::InvalidUsername => 0x30064,
            RendezVousError::InvalidPassword => 0x30065,
            RendezVousError::UsernameAlreadyExists => 0x30066,
            RendezVousError::AccountDisabled => 0x30067,
            RendezVousError::AccountExpired => 0x30068,
            RendezVousError::ConcurrentLoginDenied => 0x30069,
            RendezVousError::EncryptionFailure => 0x3006a,
            RendezVousError::InvalidPID => 0x3006b,
            RendezVousError::MaxConnectionsReached => 0x3006c,
            RendezVousError::InvalidGID => 0x3006d,
            RendezVousError::InvalidControlScriptID => 0x3006e,
            RendezVousError::InvalidOperationInLiveEnvironment => 0x3006f,
            RendezVousError::DuplicateEntry => 0x30070,
            RendezVousError::ControlScriptFailure => 0x30071,
            RendezVousError::ClassNotFound => 0x30072,
            RendezVousError::SessionVoid => 0x30073,
            RendezVousError::DDLMismatch => 0x30075,
            RendezVousError::InvalidConfiguration => 0x30076,
            RendezVousError::SessionFull => 0x300c8,
            RendezVousError::InvalidGatheringPassword => 0x300c9,
            RendezVousError::WithoutParticipationPeriod => 0x300ca,
            RendezVousError::PersistentGatheringCreationMax => 0x300cb,
            RendezVousError::PersistentGatheringParticipationMax => 0x300cc,
            RendezVousError::DeniedByParticipants => 0x300cd,
            RendezVousError::ParticipantInBlackList => 0x300ce,
            RendezVousError::GameServerMaintenance => 0x300cf,
            RendezVousError::OperationPostpone => 0x300d0,
            RendezVousError::OutOfRatingRange => 0x300d1,
            RendezVousError::ConnectionDisconnected => 0x300d2,
            RendezVousError::InvalidOperation => 0x300d3,
            RendezVousError::NotParticipatedGathering => 0x300d4,
            RendezVousError::MatchmakeSessionUserPasswordUnmatch => 0x300d5,
            RendezVousError::MatchmakeSessionSystemPasswordUnmatch => 0x300d6,
            RendezVousError::UserIsOffline => 0x300d7,
            RendezVousError::AlreadyParticipatedGathering => 0x300d8,
            RendezVousError::PermissionDenied => 0x300d9,
            RendezVousError::NotFriend => 0x300da,
            RendezVousError::SessionClosed => 0x300db,
            RendezVousError::DatabaseTemporarilyUnavailable => 0x300dc,
            RendezVousError::InvalidUniqueId => 0x300dd,
            RendezVousError::MatchmakingWithdrawn => 0x300de,
            RendezVousError::LimitExceeded => 0x300df,
            RendezVousError::AccountTemporarilyDisabled => 0x300e0,
            RendezVousError::PartiallyServiceClosed => 0x300e1,
            RendezVousError::ConnectionDisconnectedForConcurrentLogin => 0x300e2,
        }
    }
}

enum PythonCoreError {
    Exception,
    TypeError,
    IndexError,
    InvalidReference,
    CallFailure,
    MemoryError,
    KeyError,
    OperationError,
    ConversionError,
    ValidationError,
}

impl From<u32> for PythonCoreError {
    fn from(v: u32) -> Self {
        match v {
            0x40001 => Self::Exception,
            0x40002 => Self::TypeError,
            0x40003 => Self::IndexError,
            0x40004 => Self::InvalidReference,
            0x40005 => Self::CallFailure,
            0x40006 => Self::MemoryError,
            0x40007 => Self::KeyError,
            0x40008 => Self::OperationError,
            0x40009 => Self::ConversionError,
            0x4000a => Self::ValidationError,
        }
    }
}

impl From<PythonCoreError> for u32 {
    fn from(v: PythonCoreError) -> Self {
        match v {
            PythonCoreError::Exception => 0x40001,
            PythonCoreError::TypeError => 0x40002,
            PythonCoreError::IndexError => 0x40003,
            PythonCoreError::InvalidReference => 0x40004,
            PythonCoreError::CallFailure => 0x40005,
            PythonCoreError::MemoryError => 0x40006,
            PythonCoreError::KeyError => 0x40007,
            PythonCoreError::OperationError => 0x40008,
            PythonCoreError::ConversionError => 0x40009,
            PythonCoreError::ValidationError => 0x4000a,
        }
    }
}

enum TransportError {
    Unknown,
    ConnectionFailure,
    InvalidUrl,
    InvalidKey,
    InvalidURLType,
    DuplicateEndpoint,
    IOError,
    Timeout,
    ConnectionReset,
    IncorrectRemoteAuthentication,
    ServerRequestError,
    DecompressionFailure,
    ReliableSendBufferFullFatal,
    UPnPCannotInit,
    UPnPCannotAddMapping,
    NatPMPCannotInit,
    NatPMPCannotAddMapping,
    UnsupportedNAT,
    DnsError,
    ProxyError,
    DataRemaining,
    NoBuffer,
    NotFound,
    TemporaryServerError,
    PermanentServerError,
    ServiceUnavailable,
    ReliableSendBufferFull,
    InvalidStation,
    InvalidSubStreamID,
    PacketBufferFull,
    NatTraversalError,
    NatCheckError,
}

impl From<u32> for TransportError {
    fn from(v: u32) -> Self {
        match v {
            0x50001 => Self::Unknown,
            0x50002 => Self::ConnectionFailure,
            0x50003 => Self::InvalidUrl,
            0x50004 => Self::InvalidKey,
            0x50005 => Self::InvalidURLType,
            0x50006 => Self::DuplicateEndpoint,
            0x50007 => Self::IOError,
            0x50008 => Self::Timeout,
            0x50009 => Self::ConnectionReset,
            0x5000a => Self::IncorrectRemoteAuthentication,
            0x5000b => Self::ServerRequestError,
            0x5000c => Self::DecompressionFailure,
            0x5000d => Self::ReliableSendBufferFullFatal,
            0x5000e => Self::UPnPCannotInit,
            0x5000f => Self::UPnPCannotAddMapping,
            0x50010 => Self::NatPMPCannotInit,
            0x50011 => Self::NatPMPCannotAddMapping,
            0x50013 => Self::UnsupportedNAT,
            0x50014 => Self::DnsError,
            0x50015 => Self::ProxyError,
            0x50016 => Self::DataRemaining,
            0x50017 => Self::NoBuffer,
            0x50018 => Self::NotFound,
            0x50019 => Self::TemporaryServerError,
            0x5001a => Self::PermanentServerError,
            0x5001b => Self::ServiceUnavailable,
            0x5001c => Self::ReliableSendBufferFull,
            0x5001d => Self::InvalidStation,
            0x5001e => Self::InvalidSubStreamID,
            0x5001f => Self::PacketBufferFull,
            0x50020 => Self::NatTraversalError,
            0x50021 => Self::NatCheckError,
        }
    }
}

impl From<TransportError> for u32 {
    fn from(v: TransportError) -> Self {
        match v {
            TransportError::Unknown => 0x50001,
            TransportError::ConnectionFailure => 0x50002,
            TransportError::InvalidUrl => 0x50003,
            TransportError::InvalidKey => 0x50004,
            TransportError::InvalidURLType => 0x50005,
            TransportError::DuplicateEndpoint => 0x50006,
            TransportError::IOError => 0x50007,
            TransportError::Timeout => 0x50008,
            TransportError::ConnectionReset => 0x50009,
            TransportError::IncorrectRemoteAuthentication => 0x5000a,
            TransportError::ServerRequestError => 0x5000b,
            TransportError::DecompressionFailure => 0x5000c,
            TransportError::ReliableSendBufferFullFatal => 0x5000d,
            TransportError::UPnPCannotInit => 0x5000e,
            TransportError::UPnPCannotAddMapping => 0x5000f,
            TransportError::NatPMPCannotInit => 0x50010,
            TransportError::NatPMPCannotAddMapping => 0x50011,
            TransportError::UnsupportedNAT => 0x50013,
            TransportError::DnsError => 0x50014,
            TransportError::ProxyError => 0x50015,
            TransportError::DataRemaining => 0x50016,
            TransportError::NoBuffer => 0x50017,
            TransportError::NotFound => 0x50018,
            TransportError::TemporaryServerError => 0x50019,
            TransportError::PermanentServerError => 0x5001a,
            TransportError::ServiceUnavailable => 0x5001b,
            TransportError::ReliableSendBufferFull => 0x5001c,
            TransportError::InvalidStation => 0x5001d,
            TransportError::InvalidSubStreamID => 0x5001e,
            TransportError::PacketBufferFull => 0x5001f,
            TransportError::NatTraversalError => 0x50020,
            TransportError::NatCheckError => 0x50021,
        }
    }
}

enum DOCoreError {
    StationNotReached,
    TargetStationDisconnect,
    LocalStationLeaving,
    ObjectNotFound,
    InvalidRole,
    CallTimeout,
    RMCDispatchFailed,
    MigrationInProgress,
    NoAuthority,
    NoTargetStationSpecified,
    JoinFailed,
    JoinDenied,
    ConnectivityTestFailed,
    Unknown,
    UnfreedReferences,
    JobTerminationFailed,
    InvalidState,
    FaultRecoveryFatal,
    FaultRecoveryJobProcessFailed,
    StationInconsitency,
    AbnormalMasterState,
    VersionMismatch,
}

impl From<u32> for DOCoreError {
    fn from(v: u32) -> Self {
        match v {
            0x60001 => Self::StationNotReached,
            0x60002 => Self::TargetStationDisconnect,
            0x60003 => Self::LocalStationLeaving,
            0x60004 => Self::ObjectNotFound,
            0x60005 => Self::InvalidRole,
            0x60006 => Self::CallTimeout,
            0x60007 => Self::RMCDispatchFailed,
            0x60008 => Self::MigrationInProgress,
            0x60009 => Self::NoAuthority,
            0x6000a => Self::NoTargetStationSpecified,
            0x6000b => Self::JoinFailed,
            0x6000c => Self::JoinDenied,
            0x6000d => Self::ConnectivityTestFailed,
            0x6000e => Self::Unknown,
            0x6000f => Self::UnfreedReferences,
            0x60010 => Self::JobTerminationFailed,
            0x60011 => Self::InvalidState,
            0x60012 => Self::FaultRecoveryFatal,
            0x60013 => Self::FaultRecoveryJobProcessFailed,
            0x60014 => Self::StationInconsitency,
            0x60015 => Self::AbnormalMasterState,
            0x60016 => Self::VersionMismatch,
        }
    }
}

impl From<DOCoreError> for u32 {
    fn from(v: DOCoreError) -> Self {
        match v {
            DOCoreError::StationNotReached => 0x60001,
            DOCoreError::TargetStationDisconnect => 0x60002,
            DOCoreError::LocalStationLeaving => 0x60003,
            DOCoreError::ObjectNotFound => 0x60004,
            DOCoreError::InvalidRole => 0x60005,
            DOCoreError::CallTimeout => 0x60006,
            DOCoreError::RMCDispatchFailed => 0x60007,
            DOCoreError::MigrationInProgress => 0x60008,
            DOCoreError::NoAuthority => 0x60009,
            DOCoreError::NoTargetStationSpecified => 0x6000a,
            DOCoreError::JoinFailed => 0x6000b,
            DOCoreError::JoinDenied => 0x6000c,
            DOCoreError::ConnectivityTestFailed => 0x6000d,
            DOCoreError::Unknown => 0x6000e,
            DOCoreError::UnfreedReferences => 0x6000f,
            DOCoreError::JobTerminationFailed => 0x60010,
            DOCoreError::InvalidState => 0x60011,
            DOCoreError::FaultRecoveryFatal => 0x60012,
            DOCoreError::FaultRecoveryJobProcessFailed => 0x60013,
            DOCoreError::StationInconsitency => 0x60014,
            DOCoreError::AbnormalMasterState => 0x60015,
            DOCoreError::VersionMismatch => 0x60016,
        }
    }
}

enum FPDError {
    NotInitialized,
    AlreadyInitialized,
    NotConnected,
    Connected,
    InitializationFailure,
    OutOfMemory,
    RmcFailed,
    InvalidArgument,
    InvalidLocalAccountID,
    InvalidPrincipalID,
    InvalidLocalFriendCode,
    LocalAccountNotExists,
    LocalAccountNotLoaded,
    LocalAccountAlreadyLoaded,
    FriendAlreadyExists,
    FriendNotExists,
    FriendNumMax,
    NotFriend,
    FileIO,
    P2PInternetProhibited,
    Unknown,
    InvalidState,
    AddFriendProhibited,
    InvalidAccount,
    BlacklistedByMe,
    FriendAlreadyAdded,
    MyFriendListLimitExceed,
    RequestLimitExceed,
    InvalidMessageID,
    MessageIsNotMine,
    MessageIsNotForMe,
    FriendRequestBlocked,
    NotInMyFriendList,
    FriendListedByMe,
    NotInMyBlacklist,
    IncompatibleAccount,
    BlockSettingChangeNotAllowed,
    SizeLimitExceeded,
    OperationNotAllowed,
    NotNetworkAccount,
    NotificationNotFound,
    PreferenceNotInitialized,
    FriendRequestNotAllowed,
}

impl From<u32> for FPDError {
    fn from(v: u32) -> Self {
        match v {
            0x650000 => Self::NotInitialized,
            0x650001 => Self::AlreadyInitialized,
            0x650002 => Self::NotConnected,
            0x650003 => Self::Connected,
            0x650004 => Self::InitializationFailure,
            0x650005 => Self::OutOfMemory,
            0x650006 => Self::RmcFailed,
            0x650007 => Self::InvalidArgument,
            0x650008 => Self::InvalidLocalAccountID,
            0x650009 => Self::InvalidPrincipalID,
            0x65000a => Self::InvalidLocalFriendCode,
            0x65000b => Self::LocalAccountNotExists,
            0x65000c => Self::LocalAccountNotLoaded,
            0x65000d => Self::LocalAccountAlreadyLoaded,
            0x65000e => Self::FriendAlreadyExists,
            0x65000f => Self::FriendNotExists,
            0x650010 => Self::FriendNumMax,
            0x650011 => Self::NotFriend,
            0x650012 => Self::FileIO,
            0x650013 => Self::P2PInternetProhibited,
            0x650014 => Self::Unknown,
            0x650015 => Self::InvalidState,
            0x650017 => Self::AddFriendProhibited,
            0x650019 => Self::InvalidAccount,
            0x65001a => Self::BlacklistedByMe,
            0x65001c => Self::FriendAlreadyAdded,
            0x65001d => Self::MyFriendListLimitExceed,
            0x65001e => Self::RequestLimitExceed,
            0x65001f => Self::InvalidMessageID,
            0x650020 => Self::MessageIsNotMine,
            0x650021 => Self::MessageIsNotForMe,
            0x650022 => Self::FriendRequestBlocked,
            0x650023 => Self::NotInMyFriendList,
            0x650024 => Self::FriendListedByMe,
            0x650025 => Self::NotInMyBlacklist,
            0x650026 => Self::IncompatibleAccount,
            0x650027 => Self::BlockSettingChangeNotAllowed,
            0x650028 => Self::SizeLimitExceeded,
            0x650029 => Self::OperationNotAllowed,
            0x65002a => Self::NotNetworkAccount,
            0x65002b => Self::NotificationNotFound,
            0x65002c => Self::PreferenceNotInitialized,
            0x65002d => Self::FriendRequestNotAllowed,
        }
    }
}

impl From<FPDError> for u32 {
    fn from(v: FPDError) -> Self {
        match v {
            FPDError::NotInitialized => 0x650000,
            FPDError::AlreadyInitialized => 0x650001,
            FPDError::NotConnected => 0x650002,
            FPDError::Connected => 0x650003,
            FPDError::InitializationFailure => 0x650004,
            FPDError::OutOfMemory => 0x650005,
            FPDError::RmcFailed => 0x650006,
            FPDError::InvalidArgument => 0x650007,
            FPDError::InvalidLocalAccountID => 0x650008,
            FPDError::InvalidPrincipalID => 0x650009,
            FPDError::InvalidLocalFriendCode => 0x65000a,
            FPDError::LocalAccountNotExists => 0x65000b,
            FPDError::LocalAccountNotLoaded => 0x65000c,
            FPDError::LocalAccountAlreadyLoaded => 0x65000d,
            FPDError::FriendAlreadyExists => 0x65000e,
            FPDError::FriendNotExists => 0x65000f,
            FPDError::FriendNumMax => 0x650010,
            FPDError::NotFriend => 0x650011,
            FPDError::FileIO => 0x650012,
            FPDError::P2PInternetProhibited => 0x650013,
            FPDError::Unknown => 0x650014,
            FPDError::InvalidState => 0x650015,
            FPDError::AddFriendProhibited => 0x650017,
            FPDError::InvalidAccount => 0x650019,
            FPDError::BlacklistedByMe => 0x65001a,
            FPDError::FriendAlreadyAdded => 0x65001c,
            FPDError::MyFriendListLimitExceed => 0x65001d,
            FPDError::RequestLimitExceed => 0x65001e,
            FPDError::InvalidMessageID => 0x65001f,
            FPDError::MessageIsNotMine => 0x650020,
            FPDError::MessageIsNotForMe => 0x650021,
            FPDError::FriendRequestBlocked => 0x650022,
            FPDError::NotInMyFriendList => 0x650023,
            FPDError::FriendListedByMe => 0x650024,
            FPDError::NotInMyBlacklist => 0x650025,
            FPDError::IncompatibleAccount => 0x650026,
            FPDError::BlockSettingChangeNotAllowed => 0x650027,
            FPDError::SizeLimitExceeded => 0x650028,
            FPDError::OperationNotAllowed => 0x650029,
            FPDError::NotNetworkAccount => 0x65002a,
            FPDError::NotificationNotFound => 0x65002b,
            FPDError::PreferenceNotInitialized => 0x65002c,
            FPDError::FriendRequestNotAllowed => 0x65002d,
        }
    }
}

enum RankingError {
    NotInitialized,
    InvalidArgument,
    RegistrationError,
    NotFound,
    InvalidScore,
    InvalidDataSize,
    PermissionDenied,
    Unknown,
    NotImplemented,
}

impl From<u32> for RankingError {
    fn from(v: u32) -> Self {
        match v {
            0x670001 => Self::NotInitialized,
            0x670002 => Self::InvalidArgument,
            0x670003 => Self::RegistrationError,
            0x670005 => Self::NotFound,
            0x670006 => Self::InvalidScore,
            0x670007 => Self::InvalidDataSize,
            0x670009 => Self::PermissionDenied,
            0x67000a => Self::Unknown,
            0x67000b => Self::NotImplemented,
        }
    }
}

impl From<RankingError> for u32 {
    fn from(v: RankingError) -> Self {
        match v {
            RankingError::NotInitialized => 0x670001,
            RankingError::InvalidArgument => 0x670002,
            RankingError::RegistrationError => 0x670003,
            RankingError::NotFound => 0x670005,
            RankingError::InvalidScore => 0x670006,
            RankingError::InvalidDataSize => 0x670007,
            RankingError::PermissionDenied => 0x670009,
            RankingError::Unknown => 0x67000a,
            RankingError::NotImplemented => 0x67000b,
        }
    }
}

enum AuthenticationError {
    NASAuthenticateError,
    TokenParseError,
    HttpConnectionError,
    HttpDNSError,
    HttpGetProxySetting,
    TokenExpired,
    ValidationFailed,
    InvalidParam,
    PrincipalIdUnmatched,
    MoveCountUnmatch,
    UnderMaintenance,
    UnsupportedVersion,
    ServerVersionIsOld,
    Unknown,
    ClientVersionIsOld,
    AccountLibraryError,
    ServiceNoLongerAvailable,
    UnknownApplication,
    ApplicationVersionIsOld,
    OutOfService,
    NetworkServiceLicenseRequired,
    NetworkServiceLicenseSystemError,
    NetworkServiceLicenseError3,
    NetworkServiceLicenseError4,
}

impl From<u32> for AuthenticationError {
    fn from(v: u32) -> Self {
        match v {
            0x680001 => Self::NASAuthenticateError,
            0x680002 => Self::TokenParseError,
            0x680003 => Self::HttpConnectionError,
            0x680004 => Self::HttpDNSError,
            0x680005 => Self::HttpGetProxySetting,
            0x680006 => Self::TokenExpired,
            0x680007 => Self::ValidationFailed,
            0x680008 => Self::InvalidParam,
            0x680009 => Self::PrincipalIdUnmatched,
            0x68000a => Self::MoveCountUnmatch,
            0x68000b => Self::UnderMaintenance,
            0x68000c => Self::UnsupportedVersion,
            0x68000d => Self::ServerVersionIsOld,
            0x68000e => Self::Unknown,
            0x68000f => Self::ClientVersionIsOld,
            0x680010 => Self::AccountLibraryError,
            0x680011 => Self::ServiceNoLongerAvailable,
            0x680012 => Self::UnknownApplication,
            0x680013 => Self::ApplicationVersionIsOld,
            0x680014 => Self::OutOfService,
            0x680015 => Self::NetworkServiceLicenseRequired,
            0x680016 => Self::NetworkServiceLicenseSystemError,
            0x680017 => Self::NetworkServiceLicenseError3,
            0x680018 => Self::NetworkServiceLicenseError4,
        }
    }
}

impl From<AuthenticationError> for u32 {
    fn from(v: AuthenticationError) -> Self {
        match v {
            AuthenticationError::NASAuthenticateError => 0x680001,
            AuthenticationError::TokenParseError => 0x680002,
            AuthenticationError::HttpConnectionError => 0x680003,
            AuthenticationError::HttpDNSError => 0x680004,
            AuthenticationError::HttpGetProxySetting => 0x680005,
            AuthenticationError::TokenExpired => 0x680006,
            AuthenticationError::ValidationFailed => 0x680007,
            AuthenticationError::InvalidParam => 0x680008,
            AuthenticationError::PrincipalIdUnmatched => 0x680009,
            AuthenticationError::MoveCountUnmatch => 0x68000a,
            AuthenticationError::UnderMaintenance => 0x68000b,
            AuthenticationError::UnsupportedVersion => 0x68000c,
            AuthenticationError::ServerVersionIsOld => 0x68000d,
            AuthenticationError::Unknown => 0x68000e,
            AuthenticationError::ClientVersionIsOld => 0x68000f,
            AuthenticationError::AccountLibraryError => 0x680010,
            AuthenticationError::ServiceNoLongerAvailable => 0x680011,
            AuthenticationError::UnknownApplication => 0x680012,
            AuthenticationError::ApplicationVersionIsOld => 0x680013,
            AuthenticationError::OutOfService => 0x680014,
            AuthenticationError::NetworkServiceLicenseRequired => 0x680015,
            AuthenticationError::NetworkServiceLicenseSystemError => 0x680016,
            AuthenticationError::NetworkServiceLicenseError3 => 0x680017,
            AuthenticationError::NetworkServiceLicenseError4 => 0x680018,
        }
    }
}

enum DataStoreError {
    Unknown,
    InvalidArgument,
    PermissionDenied,
    NotFound,
    AlreadyLocked,
    UnderReviewing,
    Expired,
    InvalidCheckToken,
    SystemFileError,
    OverCapacity,
    OperationNotAllowed,
    InvalidPassword,
    ValueNotEqual,
}

impl From<u32> for DataStoreError {
    fn from(v: u32) -> Self {
        match v {
            0x690001 => Self::Unknown,
            0x690002 => Self::InvalidArgument,
            0x690003 => Self::PermissionDenied,
            0x690004 => Self::NotFound,
            0x690005 => Self::AlreadyLocked,
            0x690006 => Self::UnderReviewing,
            0x690007 => Self::Expired,
            0x690008 => Self::InvalidCheckToken,
            0x690009 => Self::SystemFileError,
            0x69000a => Self::OverCapacity,
            0x69000b => Self::OperationNotAllowed,
            0x69000c => Self::InvalidPassword,
            0x69000d => Self::ValueNotEqual,
        }
    }
}

impl From<DataStoreError> for u32 {
    fn from(v: DataStoreError) -> Self {
        match v {
            DataStoreError::Unknown => 0x690001,
            DataStoreError::InvalidArgument => 0x690002,
            DataStoreError::PermissionDenied => 0x690003,
            DataStoreError::NotFound => 0x690004,
            DataStoreError::AlreadyLocked => 0x690005,
            DataStoreError::UnderReviewing => 0x690006,
            DataStoreError::Expired => 0x690007,
            DataStoreError::InvalidCheckToken => 0x690008,
            DataStoreError::SystemFileError => 0x690009,
            DataStoreError::OverCapacity => 0x69000a,
            DataStoreError::OperationNotAllowed => 0x69000b,
            DataStoreError::InvalidPassword => 0x69000c,
            DataStoreError::ValueNotEqual => 0x69000d,
        }
    }
}

enum ServiceItemError {
    Unknown,
    InvalidArgument,
    EShopUnknownHttpError,
    EShopResponseParseError,
    NotOwned,
    InvalidLimitationType,
    ConsumptionRightShortage,
}

impl From<u32> for ServiceItemError {
    fn from(v: u32) -> Self {
        match v {
            0x6c0001 => Self::Unknown,
            0x6c0002 => Self::InvalidArgument,
            0x6c0003 => Self::EShopUnknownHttpError,
            0x6c0004 => Self::EShopResponseParseError,
            0x6c0005 => Self::NotOwned,
            0x6c0006 => Self::InvalidLimitationType,
            0x6c0007 => Self::ConsumptionRightShortage,
        }
    }
}

impl From<ServiceItemError> for u32 {
    fn from(v: ServiceItemError) -> Self {
        match v {
            ServiceItemError::Unknown => 0x6c0001,
            ServiceItemError::InvalidArgument => 0x6c0002,
            ServiceItemError::EShopUnknownHttpError => 0x6c0003,
            ServiceItemError::EShopResponseParseError => 0x6c0004,
            ServiceItemError::NotOwned => 0x6c0005,
            ServiceItemError::InvalidLimitationType => 0x6c0006,
            ServiceItemError::ConsumptionRightShortage => 0x6c0007,
        }
    }
}

enum MatchmakeRefereeError {
    Unknown,
    InvalidArgument,
    AlreadyExists,
    NotParticipatedGathering,
    NotParticipatedRound,
    StatsNotFound,
    RoundNotFound,
    RoundArbitrated,
    RoundNotArbitrated,
}

impl From<u32> for MatchmakeRefereeError {
    fn from(v: u32) -> Self {
        match v {
            0x6f0001 => Self::Unknown,
            0x6f0002 => Self::InvalidArgument,
            0x6f0003 => Self::AlreadyExists,
            0x6f0004 => Self::NotParticipatedGathering,
            0x6f0005 => Self::NotParticipatedRound,
            0x6f0006 => Self::StatsNotFound,
            0x6f0007 => Self::RoundNotFound,
            0x6f0008 => Self::RoundArbitrated,
            0x6f0009 => Self::RoundNotArbitrated,
        }
    }
}

impl From<MatchmakeRefereeError> for u32 {
    fn from(v: MatchmakeRefereeError) -> Self {
        match v {
            MatchmakeRefereeError::Unknown => 0x6f0001,
            MatchmakeRefereeError::InvalidArgument => 0x6f0002,
            MatchmakeRefereeError::AlreadyExists => 0x6f0003,
            MatchmakeRefereeError::NotParticipatedGathering => 0x6f0004,
            MatchmakeRefereeError::NotParticipatedRound => 0x6f0005,
            MatchmakeRefereeError::StatsNotFound => 0x6f0006,
            MatchmakeRefereeError::RoundNotFound => 0x6f0007,
            MatchmakeRefereeError::RoundArbitrated => 0x6f0008,
            MatchmakeRefereeError::RoundNotArbitrated => 0x6f0009,
        }
    }
}

enum SubscriberError {
    Unknown,
    InvalidArgument,
    OverLimit,
    PermissionDenied,
}

impl From<u32> for SubscriberError {
    fn from(v: u32) -> Self {
        match v {
            0x700001 => Self::Unknown,
            0x700002 => Self::InvalidArgument,
            0x700003 => Self::OverLimit,
            0x700004 => Self::PermissionDenied,
        }
    }
}

impl From<SubscriberError> for u32 {
    fn from(v: SubscriberError) -> Self {
        match v {
            SubscriberError::Unknown => 0x700001,
            SubscriberError::InvalidArgument => 0x700002,
            SubscriberError::OverLimit => 0x700003,
            SubscriberError::PermissionDenied => 0x700004,
        }
    }
}

enum Ranking2Error {
    Unknown,
    InvalidArgument,
    InvalidScore,
}

impl From<u32> for Ranking2Error {
    fn from(v: u32) -> Self {
        match v {
            0x710001 => Self::Unknown,
            0x710002 => Self::InvalidArgument,
            0x710003 => Self::InvalidScore,
        }
    }
}

impl From<Ranking2Error> for u32 {
    fn from(v: Ranking2Error) -> Self {
        match v {
            Ranking2Error::Unknown => 0x710001,
            Ranking2Error::InvalidArgument => 0x710002,
            Ranking2Error::InvalidScore => 0x710003,
        }
    }
}

enum SmartDeviceVoiceChatError {
    Unknown,
    InvalidArgument,
    InvalidResponse,
    InvalidAccessToken,
    Unauthorized,
    AccessError,
    UserNotFound,
    RoomNotFound,
    RoomNotActivated,
    ApplicationNotSupported,
    InternalServerError,
    ServiceUnavailable,
    UnexpectedError,
    UnderMaintenance,
    ServiceNoLongerAvailable,
    AccountTemporarilyDisabled,
    PermissionDenied,
    NetworkServiceLicenseRequired,
    AccountLibraryError,
    GameModeNotFound,
}

impl From<u32> for SmartDeviceVoiceChatError {
    fn from(v: u32) -> Self {
        match v {
            0x720001 => Self::Unknown,
            0x720002 => Self::InvalidArgument,
            0x720003 => Self::InvalidResponse,
            0x720004 => Self::InvalidAccessToken,
            0x720005 => Self::Unauthorized,
            0x720006 => Self::AccessError,
            0x720007 => Self::UserNotFound,
            0x720008 => Self::RoomNotFound,
            0x720009 => Self::RoomNotActivated,
            0x72000a => Self::ApplicationNotSupported,
            0x72000b => Self::InternalServerError,
            0x72000c => Self::ServiceUnavailable,
            0x72000d => Self::UnexpectedError,
            0x72000e => Self::UnderMaintenance,
            0x72000f => Self::ServiceNoLongerAvailable,
            0x720010 => Self::AccountTemporarilyDisabled,
            0x720011 => Self::PermissionDenied,
            0x720012 => Self::NetworkServiceLicenseRequired,
            0x720013 => Self::AccountLibraryError,
            0x720014 => Self::GameModeNotFound,
        }
    }
}

impl From<SmartDeviceVoiceChatError> for u32 {
    fn from(v: SmartDeviceVoiceChatError) -> Self {
        match v {
            SmartDeviceVoiceChatError::Unknown => 0x720001,
            SmartDeviceVoiceChatError::InvalidArgument => 0x720002,
            SmartDeviceVoiceChatError::InvalidResponse => 0x720003,
            SmartDeviceVoiceChatError::InvalidAccessToken => 0x720004,
            SmartDeviceVoiceChatError::Unauthorized => 0x720005,
            SmartDeviceVoiceChatError::AccessError => 0x720006,
            SmartDeviceVoiceChatError::UserNotFound => 0x720007,
            SmartDeviceVoiceChatError::RoomNotFound => 0x720008,
            SmartDeviceVoiceChatError::RoomNotActivated => 0x720009,
            SmartDeviceVoiceChatError::ApplicationNotSupported => 0x72000a,
            SmartDeviceVoiceChatError::InternalServerError => 0x72000b,
            SmartDeviceVoiceChatError::ServiceUnavailable => 0x72000c,
            SmartDeviceVoiceChatError::UnexpectedError => 0x72000d,
            SmartDeviceVoiceChatError::UnderMaintenance => 0x72000e,
            SmartDeviceVoiceChatError::ServiceNoLongerAvailable => 0x72000f,
            SmartDeviceVoiceChatError::AccountTemporarilyDisabled => 0x720010,
            SmartDeviceVoiceChatError::PermissionDenied => 0x720011,
            SmartDeviceVoiceChatError::NetworkServiceLicenseRequired => 0x720012,
            SmartDeviceVoiceChatError::AccountLibraryError => 0x720013,
            SmartDeviceVoiceChatError::GameModeNotFound => 0x720014,
        }
    }
}

enum ScreeningError {
    Unknown,
    InvalidArgument,
    NotFound,
}

impl From<u32> for ScreeningError {
    fn from(v: u32) -> Self {
        match v {
            0x730001 => Self::Unknown,
            0x730002 => Self::InvalidArgument,
            0x730003 => Self::NotFound,
        }
    }
}

impl From<ScreeningError> for u32 {
    fn from(v: ScreeningError) -> Self {
        match v {
            ScreeningError::Unknown => 0x730001,
            ScreeningError::InvalidArgument => 0x730002,
            ScreeningError::NotFound => 0x730003,
        }
    }
}

enum CustomError {
    Unknown,
}

impl From<u32> for CustomError {
    fn from(v: u32) -> Self {
        match v {
            0x740001 => Self::Unknown,
        }
    }
}

impl From<CustomError> for u32 {
    fn from(v: CustomError) -> Self {
        match v {
            CustomError::Unknown => 0x740001,
        }
    }
}

enum EssError {
    Unknown,
    GameSessionError,
    GameSessionMaintenance,
}

impl From<u32> for EssError {
    fn from(v: u32) -> Self {
        match v {
            0x750001 => Self::Unknown,
            0x750002 => Self::GameSessionError,
            0x750003 => Self::GameSessionMaintenance,
        }
    }
}

impl From<EssError> for u32 {
    fn from(v: EssError) -> Self {
        match v {
            EssError::Unknown => 0x750001,
            EssError::GameSessionError => 0x750002,
            EssError::GameSessionMaintenance => 0x750003,
        }
    }
}

enum Error {
    Core(CoreError),
    DDL(DDLError),
    RendezVous(RendezVousError),
    PythonCore(PythonCoreError),
    Transport(TransportError),
    DOCore(DOCoreError),
    FPD(FPDError),
    Ranking(RankingError),
    Authentication(AuthenticationError),
    DataStore(DataStoreError),
    ServiceItem(ServiceItemError),
    MatchmakeReferee(MatchmakeRefereeError),
    Subscriber(SubscriberError),
    Ranking2(Ranking2Error),
    SmartDeviceVoiceChat(SmartDeviceVoiceChatError),
    Screening(ScreeningError),
    Custom(CustomError),
    Ess(EssError),
}

impl From<u32> for Error {
    fn from(v: u32) -> Self {
        match v >> 16 {
            0x1 => CoreError::from(v),
            0x2 => DDLError::from(v),
            0x3 => RendezVousError::from(v),
            0x4 => PythonCoreError::from(v),
            0x5 => TransportError::from(v),
            0x6 => DOCoreError::from(v),
            0x65 => FPDError::from(v),
            0x67 => RankingError::from(v),
            0x68 => AuthenticationError::from(v),
            0x69 => DataStoreError::from(v),
            0x6c => ServiceItemError::from(v),
            0x6f => MatchmakeRefereeError::from(v),
            0x70 => SubscriberError::from(v),
            0x71 => Ranking2Error::from(v),
            0x72 => SmartDeviceVoiceChatError::from(v),
            0x73 => ScreeningError::from(v),
            0x74 => CustomError::from(v),
            0x75 => EssError::from(v),
        }
    }
}

impl From<Error> for u32 {
    fn from(v: Error) -> Self {
        match v {
            Error::Core(e) => CoreError::from(e),
            Error::DDL(e) => DDLError::from(e),
            Error::RendezVous(e) => RendezVousError::from(e),
            Error::PythonCore(e) => PythonCoreError::from(e),
            Error::Transport(e) => TransportError::from(e),
            Error::DOCore(e) => DOCoreError::from(e),
            Error::FPD(e) => FPDError::from(e),
            Error::Ranking(e) => RankingError::from(e),
            Error::Authentication(e) => AuthenticationError::from(e),
            Error::DataStore(e) => DataStoreError::from(e),
            Error::ServiceItem(e) => ServiceItemError::from(e),
            Error::MatchmakeReferee(e) => MatchmakeRefereeError::from(e),
            Error::Subscriber(e) => SubscriberError::from(e),
            Error::Ranking2(e) => Ranking2Error::from(e),
            Error::SmartDeviceVoiceChat(e) => SmartDeviceVoiceChatError::from(e),
            Error::Screening(e) => ScreeningError::from(e),
            Error::Custom(e) => CustomError::from(e),
            Error::Ess(e) => EssError::from(e),
        }
    }
}
