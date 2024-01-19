use tower_http::compression::CompressionLayer;

pub fn default() -> CompressionLayer {
    CompressionLayer::new()
        .gzip(true)
        .deflate(true)
        .zstd(true)
        .br(true)
}
