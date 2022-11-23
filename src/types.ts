export type PlayerNativeObject = never

export interface LibrespotModule {
  // Non spirc player
  create_player: (
    config: FullConstructorConfig,
    callback: (event: PlayerEvent) => void
  ) => Promise<PlayerNativeObject>

  play: () => Promise<void>
  pause: () => Promise<void>
  seek: (timeMs: number) => Promise<void>
  set_volume: (volume: number) => Promise<void>
  close_player: () => Promise<void>
  get_device_id: () => string
  get_token: (scopes: string) => Promise<Token | undefined>
  load_track: (
    trackUri: string,
    autoPlay: boolean,
    start_pos: number
  ) => Promise<void>

  // Spirc player
  create_player_spirc: (
    config: FullConstructorConfig,
    callback: (event: PlayerEvent) => void
  ) => Promise<PlayerNativeObject>

  play_spirc: () => Promise<void>
  pause_spirc: () => Promise<void>
  seek_spirc: (timeMs: number) => Promise<void>
  set_volume_spirc: (volume: number) => Promise<void>
  close_player_spirc: () => Promise<void>
  get_device_id_spirc: () => string
  get_token_spirc: (scopes: string) => Promise<Token | undefined>
}

export interface FetchConfig {
  method: "GET" | "POST" | "PUT"
  body?: Record<string, unknown>
  headers?: Record<string, string | string[] | number>
  search?: Record<string, string>
  auth?: string
}

export interface NormalizationConfig {
  normalization: boolean
  normalizationPregain: number
  normalizationType: "auto" | "album" | "track"
  normalizationMethod: "dynamic" | "basic"
  normalizationAttackCF: number
  normalizationKneeDB: number
  normalizationReleaseCF: number
  normalizationThreshold: number
}

export interface ConnectConfig {
  name: string
  deviceType:
    | "computer"
    | "tablet"
    | "smartphone"
    | "speaker"
    | "tv"
    | "avr"
    | "stb"
    | "audiodongle"
    | "gameconsole"
    | "castaudio"
    | "castvideo"
    | "automobile"
    | "smartwatch"
    | "chromebook"
    | "carthing"
    | "homething"
  initialVolume: number
  hasVolumeControl: boolean
}

export interface ConstructorConfig {
  auth: Partial<AuthDetails>
  save_tokens?: boolean
  cache_path?: string
  pos_update_interval?: number
  backend?: string
  gapless?: boolean
  bitrate?: "96" | "160" | "320"
  passThrough?: boolean
  normalizationConfig?: Partial<NormalizationConfig>
  connectConfig?: Partial<ConnectConfig>
}

export interface FullConstructorConfig {
  auth: AuthDetails
  save_tokens: boolean
  cache_path: string
  pos_update_interval: number
  backend: string
  gapless: boolean
  bitrate: "96" | "160" | "320"
  passThrough: boolean
  normalizationConfig: NormalizationConfig
  connectConfig: ConnectConfig
}

export interface AuthDetails {
  username: string
  password: string
  authType?:
    | "AUTHENTICATION_USER_PASS"
    | "AUTHENTICATION_USER_PASS"
    | "AUTHENTICATION_STORED_FACEBOOK_CREDENTIALS"
    | "AUTHENTICATION_SPOTIFY_TOKEN"
    | "AUTHENTICATION_FACEBOOK_TOKEN"
}

export type PlayerEventTypes =
  | "Stopped"
  | "Loading"
  | "Preloading"
  | "Playing"
  | "Paused"
  | "TimeToPreloadNextTrack"
  | "EndOfTrack"
  | "Unavailable"
  | "VolumeChanged"
  | "PositionCorrection"
  | "Seeked"
  | "FilterExplicitContentChanged"
  | "TrackChanged"
  | "SessionConnected"
  | "SessionDisconnected"
  | "SessionClientChanged"
  | "ShuffleChanged"
  | "RepeatChanged"
  | "AutoPlayChanged"
  | "PlayerInitialized"
  | "TimeUpdated"
  | "InitializationError"

export type PlayerEvent<T extends PlayerEventTypes = "InitializationError"> = {
  event: T
} & (T extends "Stopped"
  ? {
      play_request_id: bigint
      track_id: string
    }
  : T extends "Loading"
  ? {
      play_request_id: bigint
      track_id: string
      position_ms: number
    }
  : T extends "Preloading"
  ? {
      track_id: string
    }
  : T extends "Playing"
  ? {
      play_request_id: bigint
      track_id: string
      position_ms: number
    }
  : T extends "Paused"
  ? {
      play_request_id: bigint
      track_id: string
      position_ms: number
    }
  : T extends "TimeToPreloadNextTrack"
  ? {
      play_request_id: bigint
      track_id: string
    }
  : T extends "EndOfTrack"
  ? {
      play_request_id: bigint
      track_id: string
    }
  : T extends "Unavailable"
  ? {
      play_request_id: bigint
      track_id: string
    }
  : T extends "VolumeChanged"
  ? {
      volume: number
    }
  : T extends "PositionCorrection"
  ? {
      play_request_id: bigint
      track_id: string
      position_ms: number
    }
  : T extends "Seeked"
  ? {
      play_request_id: bigint
      track_id: string
      position_ms: number
    }
  : T extends "TrackChanged"
  ? {
      audio_item: string
    }
  : T extends "SessionConnected"
  ? {
      connection_id: string
      user_name: string
    }
  : T extends "SessionDisconnected"
  ? {
      connection_id: string
      user_name: string
    }
  : T extends "SessionClientChanged"
  ? {
      client_id: string
      client_name: string
      client_brand_name: string
      client_model_name: string
    }
  : T extends "ShuffleChanged"
  ? {
      shuffle: boolean
    }
  : T extends "RepeatChanged"
  ? {
      repeat: boolean
    }
  : T extends "AutoPlayChanged"
  ? {
      auto_play: boolean
    }
  : T extends "FilterExplicitContentChanged"
  ? {
      filter: boolean
    }
  : T extends "TimeUpdated"
  ? {
      position_ms: number
    }
  : T extends "PlayerInitialized"
  ? undefined
  : T extends "InitializationError"
  ? { error: Error }
  : unknown)

export type TokenScope =
  | "ugc-image-upload"
  | "user-read-playback-state"
  | "user-modify-playback-state"
  | "user-read-currently-playing"
  | "app-remote-control"
  | "streaming"
  | "playlist-read-private"
  | "playlist-read-collaborative"
  | "playlist-modify-private"
  | "playlist-modify-public"
  | "user-follow-modify"
  | "user-follow-read"
  | "user-read-playback-position"
  | "user-top-read"
  | "user-read-recently-played"
  | "user-library-modify"
  | "user-library-read"
  | "user-read-email"
  | "user-read-private"

export type Token = {
  access_token: string
  token_type: "Bearer"
  expires_in: number
  expiry_from_epoch: number
  scopes: TokenScope[]
}