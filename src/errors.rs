use sysdeezer as sys;

pub enum DeezerError {
  NoError,
  ErrorArg,
  ErrorState,
  NotImplemented,
  AsyncCanceled,

  NotEnoughMemory,
  OsError,
  Unsupported,
  ClassNotFound,
  JsonParsing,
  XmlParsing,
  Parsing,
  ClassInstantiation,
  RunnableAlreadyStarted,
  RunnableNotStarted,
  CacheResourceOpenFailed,
  FsFull,
  FileExists,
  IoError,

  CategoryConnect,
  ConnectSessionLoginFailed,
  UserProfilePermDenied,
  CacheDirectoryPermDenied,
  ConnectSessionNotOnline,
  ConnectSessionOfflineMode,
  ConnectNoOfflineCache,

  CategoryPlayer,
  PlayerQueuelistNoneSet,
  PlayerQueuelistBadIndex,
  PlayerQueuelistNoMedia,
  PlayerQueuelistNoRights,
  PlayerQueuelistRightTimeout,
  PlayerQueuelistRadioTooManySkip,
  PlayerQueuelistNoMoreTrack,
  PlayerPauseNotStarted,
  PlayerPauseAlreadyPaused,
  PlayerUnpauseNotStarted,
  PlayerUnpauseNotPaused,
  PlayerSeekNotSeekableNotStarted,
  PlayerSeekNotSeekableNoDuration,
  PlayerSeekNotSeekableNotIndexed,
  PlayerSeekNotSeekable,

  CategoryMediastreamer,
  MediastreamerBadUrlScheme,
  MediastreamerBadUrlHost,
  MediastreamerBadUrlTrack,
  MediastreamerNotAvailableOffline,
  MediastreamerNotReadable,
  MediastreamerNoDuration,
  MediastreamerNotIndexed,
  MediastreamerSeekNotSeekable,
  MediastreamerNoData,
  MediastreamerEndOfStream,
  MediastreamerAlreadyMapped,
  MediastreamerNotMapped,
  
  CategoryOffline,
  OfflineFsFull,

  PlayerBadUrl,
  PlayerInitial,
  Unknown
}

pub fn ffi_error_to_enum(error: sys::dz_error_t) -> DeezerError {
  match error {
    sys::dz_error_t_DZ_ERROR_NO_ERROR => DeezerError::NoError,
    sys::dz_error_t_DZ_ERROR_ERROR_ARG => DeezerError::ErrorArg,
    sys::dz_error_t_DZ_ERROR_ERROR_STATE => DeezerError::ErrorState,
    sys::dz_error_t_DZ_ERROR_NOT_IMPLEMENTED => DeezerError::NotImplemented,
    sys::dz_error_t_DZ_ERROR_ASYNC_CANCELED => DeezerError::AsyncCanceled,
      
    sys::dz_error_t_DZ_ERROR_NOT_ENOUGH_MEMORY => DeezerError::NotEnoughMemory,
    sys::dz_error_t_DZ_ERROR_OS_ERROR => DeezerError::OsError,
    sys::dz_error_t_DZ_ERROR_UNSUPPORTED => DeezerError::Unsupported,
    sys::dz_error_t_DZ_ERROR_CLASS_NOT_FOUND => DeezerError::ClassNotFound,
    sys::dz_error_t_DZ_ERROR_JSON_PARSING => DeezerError::JsonParsing,
    sys::dz_error_t_DZ_ERROR_XML_PARSING => DeezerError::XmlParsing,
    sys::dz_error_t_DZ_ERROR_PARSING => DeezerError::Parsing,
    sys::dz_error_t_DZ_ERROR_CLASS_INSTANTIATION => DeezerError::ClassInstantiation,
    sys::dz_error_t_DZ_ERROR_RUNNABLE_ALREADY_STARTED => DeezerError::RunnableAlreadyStarted,
    sys::dz_error_t_DZ_ERROR_RUNNABLE_NOT_STARTED => DeezerError::RunnableNotStarted,
    sys::dz_error_t_DZ_ERROR_CACHE_RESOURCE_OPEN_FAILED => DeezerError::CacheResourceOpenFailed,
    sys::dz_error_t_DZ_ERROR_FS_FULL => DeezerError::FsFull,
    sys::dz_error_t_DZ_ERROR_FILE_EXISTS => DeezerError::FileExists,
    sys::dz_error_t_DZ_ERROR_IO_ERROR => DeezerError::IoError,

    sys::dz_error_t_DZ_ERROR_CATEGORY_CONNECT => DeezerError::CategoryConnect,
    sys::dz_error_t_DZ_ERROR_CONNECT_SESSION_LOGIN_FAILED => DeezerError::ConnectSessionLoginFailed,
    sys::dz_error_t_DZ_ERROR_USER_PROFILE_PERM_DENIED => DeezerError::UserProfilePermDenied,
    sys::dz_error_t_DZ_ERROR_CACHE_DIRECTORY_PERM_DENIED => DeezerError::CacheDirectoryPermDenied,
    sys::dz_error_t_DZ_ERROR_CONNECT_SESSION_NOT_ONLINE => DeezerError::ConnectSessionNotOnline,
    sys::dz_error_t_DZ_ERROR_CONNECT_SESSION_OFFLINE_MODE => DeezerError::ConnectSessionOfflineMode,
    sys::dz_error_t_DZ_ERROR_CONNECT_NO_OFFLINE_CACHE => DeezerError::ConnectNoOfflineCache,

    sys::dz_error_t_DZ_ERROR_CATEGORY_PLAYER => DeezerError::CategoryPlayer,
    sys::dz_error_t_DZ_ERROR_PLAYER_QUEUELIST_NONE_SET => DeezerError::PlayerQueuelistNoneSet,
    sys::dz_error_t_DZ_ERROR_PLAYER_QUEUELIST_BAD_INDEX => DeezerError::PlayerQueuelistBadIndex,
    sys::dz_error_t_DZ_ERROR_PLAYER_QUEUELIST_NO_MEDIA => DeezerError::PlayerQueuelistNoMedia,
    sys::dz_error_t_DZ_ERROR_PLAYER_QUEUELIST_NO_RIGHTS => DeezerError::PlayerQueuelistNoRights,
    sys::dz_error_t_DZ_ERROR_PLAYER_QUEUELIST_RIGHT_TIMEOUT => DeezerError::PlayerQueuelistRightTimeout,
    sys::dz_error_t_DZ_ERROR_PLAYER_QUEUELIST_RADIO_TOO_MANY_SKIP => DeezerError::PlayerQueuelistRadioTooManySkip,
    sys::dz_error_t_DZ_ERROR_PLAYER_QUEUELIST_NO_MORE_TRACK => DeezerError::PlayerQueuelistNoMoreTrack,
    sys::dz_error_t_DZ_ERROR_PLAYER_PAUSE_NOT_STARTED => DeezerError::PlayerPauseNotStarted,
    sys::dz_error_t_DZ_ERROR_PLAYER_PAUSE_ALREADY_PAUSED => DeezerError::PlayerPauseAlreadyPaused,
    sys::dz_error_t_DZ_ERROR_PLAYER_UNPAUSE_NOT_STARTED => DeezerError::PlayerUnpauseNotStarted,
    sys::dz_error_t_DZ_ERROR_PLAYER_UNPAUSE_NOT_PAUSED => DeezerError::PlayerUnpauseNotPaused,
    sys::dz_error_t_DZ_ERROR_PLAYER_SEEK_NOT_SEEKABLE_NOT_STARTED => DeezerError::PlayerSeekNotSeekableNotStarted,
    sys::dz_error_t_DZ_ERROR_PLAYER_SEEK_NOT_SEEKABLE_NO_DURATION => DeezerError::PlayerSeekNotSeekableNoDuration,
    sys::dz_error_t_DZ_ERROR_PLAYER_SEEK_NOT_SEEKABLE_NOT_INDEXED => DeezerError::PlayerSeekNotSeekableNotIndexed,
    sys::dz_error_t_DZ_ERROR_PLAYER_SEEK_NOT_SEEKABLE => DeezerError::PlayerSeekNotSeekable,

    sys::dz_error_t_DZ_ERROR_CATEGORY_MEDIASTREAMER => DeezerError::CategoryMediastreamer,
    sys::dz_error_t_DZ_ERROR_MEDIASTREAMER_BAD_URL_SCHEME => DeezerError::MediastreamerBadUrlScheme,
    sys::dz_error_t_DZ_ERROR_MEDIASTREAMER_BAD_URL_HOST => DeezerError::MediastreamerBadUrlHost,
    sys::dz_error_t_DZ_ERROR_MEDIASTREAMER_BAD_URL_TRACK => DeezerError::MediastreamerBadUrlTrack,
    sys::dz_error_t_DZ_ERROR_MEDIASTREAMER_NOT_AVAILABLE_OFFLINE => DeezerError::MediastreamerNotAvailableOffline,
    sys::dz_error_t_DZ_ERROR_MEDIASTREAMER_NOT_READABLE => DeezerError::MediastreamerNotReadable,
    sys::dz_error_t_DZ_ERROR_MEDIASTREAMER_NO_DURATION => DeezerError::MediastreamerNoDuration,
    sys::dz_error_t_DZ_ERROR_MEDIASTREAMER_NOT_INDEXED => DeezerError::MediastreamerNotIndexed,
    sys::dz_error_t_DZ_ERROR_MEDIASTREAMER_SEEK_NOT_SEEKABLE => DeezerError::MediastreamerSeekNotSeekable,
    sys::dz_error_t_DZ_ERROR_MEDIASTREAMER_NO_DATA => DeezerError::MediastreamerNoData,
    sys::dz_error_t_DZ_ERROR_MEDIASTREAMER_END_OF_STREAM => DeezerError::MediastreamerEndOfStream,
    sys::dz_error_t_DZ_ERROR_MEDIASTREAMER_ALREADY_MAPPED => DeezerError::MediastreamerAlreadyMapped,
    sys::dz_error_t_DZ_ERROR_MEDIASTREAMER_NOT_MAPPED => DeezerError::MediastreamerNotMapped,
      
    sys::dz_error_t_DZ_ERROR_CATEGORY_OFFLINE => DeezerError::CategoryOffline,
    sys::dz_error_t_DZ_ERROR_OFFLINE_FS_FULL => DeezerError::OfflineFsFull,
      
    sys::dz_error_t_DZ_ERROR_PLAYER_BAD_URL => DeezerError::PlayerBadUrl,
    _ => DeezerError::Unknown
  }
}