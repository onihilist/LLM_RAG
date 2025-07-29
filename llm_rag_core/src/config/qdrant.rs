
pub struct QdrantConfig {
    pub logger_config: QdrantLoggerConfig,
    pub storage_config: QdrantStorageConfig,
    pub service_config: QdrantServiceConfig,
    pub cluster_config: QdrantClusterConfig,
}

pub struct QdrantLoggerConfig {
    pub format: String,
    pub logger_config_on_disk: QdrantLoggerOnDiskConfig,
}

pub struct QdrantLoggerOnDiskConfig {
    pub enabled: bool,
    pub log_file: String,
    pub log_level: String,
    pub format: String
}

// ========= START OF STORAGE CONFIG

pub struct QdrantStorageConfig {
    pub storage_path: String,
    pub snapshot_path: String,
    pub snapshot_config: QdrantSnapshotConfig,
    pub temp_path: String,
    pub on_disk_payload: bool, // default: true
    pub update_concurrency: String,
    pub wal_config: QdrantWalConfig,
    pub node_type: String,
    pub performance_config: QdrantPerformanceConfig,
    pub optimizer_config: QdrantOptimizerConfig,
    pub optimizer_overwrite_config: Option<QdrantOptimizerOverwriteConfig>,
    pub hnsw_index_config: QdrantHNSWIndexConfig,
    pub shard_tranfer_method: String,
    pub collection_config: QdrantCollectionConfig
}

pub struct QdrantSnapshotConfig {
    pub snapshot_storage: String, // local or s3
    pub qdrant_snapshot_s3config: Option<QdrantSnapshotS3Config>
}

pub struct QdrantSnapshotS3Config {
    pub bucket: String,
    pub region: String,
    pub access_key: String,
    pub secret_key: String
}

pub struct QdrantWalConfig {
    pub wal_capacity_mb: u16,
    pub wal_segments_ahead: u8,
}

pub struct QdrantPerformanceConfig {
    pub max_search_thread: u8,
    pub max_optimization_threads: u8,
    pub optimizer_cpu_budget: i8,
    pub update_rate_limit: String,
}

pub struct QdrantOptimizerConfig {
    pub deleted_threshold: f32,
    pub vacuum_min_vector_number: u16,
    pub default_segment_number: u16,
    pub max_segment_size_kb: u32,
    pub memmap_threshold_kb: u32, // 1Kb = 1 vector of size 256
    pub indexing_threshold_kb: u32, // default: 20000
    pub flush_interval_sec: u8, // default: 5
    pub max_optimization_threads: u8, // null = automatic sature CPU, 0 = disabled
}

pub struct QdrantOptimizerOverwriteConfig {
    pub deleted_threshold: f32,
    pub vacuum_min_vector_number: u16,
    pub default_segment_number: u16,
    pub max_segment_size_kb: u32,
    pub memmap_threshold_kb: u32,
    pub indexing_threshold_kb: u32,
    pub flush_interval_sec: u8,
    pub max_optimization_threads: u8,
}

pub struct QdrantHNSWIndexConfig {
    pub m: u8,
    pub ef_construct: u8,
    pub full_scan_threshold_kb: u32,
    pub max_indexing_threads: u8,
    pub on_disk: bool,
    pub payload_m: String
}

pub struct QdrantCollectionConfig {
    pub replication_factor: u8,
    pub write_consistency_factor: u8,
    pub vector: QdrantCollectionVectorConfig,
    pub quantization: u16,
    pub strict_mode: QdrantCollectionStrictModeConfig
}

pub struct QdrantCollectionVectorConfig{
    pub on_disk: bool,
    pub shard_number_per_node: u8
}

pub struct QdrantCollectionStrictModeConfig {
    pub enabled: bool,
    pub max_query_limit: u16,
    pub max_timeout: u16,
    pub unindexed_filtering_retrieve: bool,
    pub unindexed_filtering_update: bool,
    pub search_max_hnsw_ef: u8,
    pub search_allow_exact: bool,
    pub search_max_oversampling: bool
}

// ========= END OF STORAGE CONFIG

pub struct QdrantServiceConfig {
    pub max_request_size_mb: u16,
    pub max_workers: u8,
    pub host: String,
    pub http_port: u16, // default: 6333
    pub grcp_port: u16, // default: none/disabled or 6334
    pub enable_cors: bool, // default: true
    pub enable_tls: bool, // default: false
    pub verify_https_client_certificate: bool, // default: false
    pub api_key: Option<String>,
    pub read_only_api_key: Option<String>,
    pub jwt_rbac: Option<bool>, // default: false
    pub hardware_reporting: Option<bool>, // default: true
}

pub struct QdrantClusterConfig {
    pub enabled: bool, // default: false
    pub p2p_config: QdrantClusterP2PConfig,

}

pub struct QdrantClusterP2PConfig {
    pub port: u16, // default: 6335
    pub enable_tls: bool, // default: false
    pub consensus_tick_period_ms: u16, // default: 100
    pub telemetry_disabled: bool, // default: false
}

pub struct QdrantTLSConfig {
    pub cert_path: String,
    pub key_path: String,
    pub ca_cert_path: String,
    pub cert_ttl: u16 // default: 3600
}
