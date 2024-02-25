//! Implements compression for content served throu endpoints

use tower_http::compression::CompressionLayer;

/// Constructs a default compression layer
pub fn default() -> CompressionLayer {
    CompressionLayer::new()
        .gzip(true)
        .deflate(true)
        .zstd(true)
        .br(true)
}
