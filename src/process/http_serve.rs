use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    serve, Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::{fs, net::TcpListener};
use tracing::info;

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(dir: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on {}", dir, addr);

    let state = HttpServeState { path: dir };
    let router = Router::new()
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));
    let listener = TcpListener::bind(addr).await?;

    serve(listener, router).await?;

    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let res_path = std::path::Path::new(&state.path).join(path);
    if !res_path.exists() {
        (StatusCode::NOT_FOUND, format!("文件不存在: {:?}", res_path))
    } else {
        match fs::read_to_string(res_path).await {
            Ok(data) => (StatusCode::OK, data.to_string()),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("读取文件失败: {:?}", e),
            ),
        }
    }
}
