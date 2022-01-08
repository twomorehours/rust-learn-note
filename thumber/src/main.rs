// 总结
// 1. axum从path取参数用Path<Params>, Params实现Deserialize
// 2. axum可以增加layer并且传入handler使用
// 3. async环境下不能使用std::sync::Mutex
// 4. 定义接口声明能力 -> 实现接口提供能力   选择接口 -> 选择接口实现使用接口声明的能力
// 5. 声明接口时，如果接口的能需要全局状态实现则使用&self, 如果接口的能力不需要全局状态则使用self

use axum::{
    extract::{Extension, Path},
    http::{HeaderMap, HeaderValue, StatusCode},
    routing::get,
    AddExtensionLayer, Router,
};
use bytes::Bytes;
use image::ImageOutputFormat;
use lru::LruCache;
use pb::ImageSpec;
use percent_encoding::{percent_decode, percent_encode, NON_ALPHANUMERIC};
use serde::Deserialize;
use std::{
    collections::hash_map::DefaultHasher, convert::TryInto, hash::Hasher, net::SocketAddr,
    sync::Arc,
};
// async环境下不能使用 std::sync::Mutex
use tokio::sync::Mutex;
use tower::ServiceBuilder;

mod pb;
use pb::*;

mod engine;
use engine::*;

#[derive(Deserialize)]
struct Params {
    spec: String,
    url: String,
}

type Cache = Arc<Mutex<LruCache<u64, Bytes>>>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // print_test_url("http://ehr.neusoft.com/neuitools/styles/themes/aero/PNG24/logon.png");

    tracing_subscriber::fmt::init();

    let cache: Cache = Arc::new(Mutex::new(LruCache::new(100)));

    let app = Router::new()
        .route("/image/:spec/:url", get(generate))
        .layer(
            ServiceBuilder::new()
                .layer(AddExtensionLayer::new(cache))
                .into_inner(),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

// 从path中取参数
async fn generate(
    path: Path<Params>,
    cache: Extension<Cache>,
) -> anyhow::Result<(HeaderMap, Vec<u8>), StatusCode> {
    let url = percent_decode(path.url.as_bytes()).decode_utf8_lossy();
    let image_spec: ImageSpec = path
        .spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let data = retrieve_image(&url, cache.0)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    // todo 处理图片
    let mut engine: Photon = data.try_into().map_err(|_| StatusCode::BAD_REQUEST)?;
    engine.apply(&image_spec.specs);
    let data = engine.generate(ImageOutputFormat::Jpeg(85));

    let mut header_map = HeaderMap::new();
    header_map.append("content-type", HeaderValue::from_static("image/jpeg"));
    Ok((header_map, data.to_vec()))
}

async fn retrieve_image(url: &str, cache: Cache) -> anyhow::Result<Bytes> {
    let mut hasher = DefaultHasher::new();
    hasher.write(url.as_bytes());
    let hash = hasher.finish();

    let mut g = cache.lock().await;
    let data = match g.get_mut(&hash) {
        Some(data) => {
            tracing::info!("cache hit {}", url);
            data.clone()
        }
        None => {
            tracing::info!("retrieve {}", url);
            let data = reqwest::get(url).await?.bytes().await?;
            g.put(hash, data.clone());
            data
        }
    };
    Ok(data)
}

// 需要抽出函数的时候
// 如果一个操作是同步的 需要一个直接执行逻辑的fn
// 如果一个操作时异步的 需要一个能返回异步任务（包含逻辑）的fn(async fn)
// #[warn(dead_code)]
// fn print_test_url(url: &str) {
//     use std::borrow::Borrow;
//     let spec1 = Spec::new_resize(600, 800);
//     let spec2 = Spec::new_watermark(20, 20);
//     let image_spec = ImageSpec::new(vec![spec1, spec2]);
//     let s: String = image_spec.borrow().into();
//     let test_image = percent_encode(url.as_bytes(), NON_ALPHANUMERIC).to_string();
//     println!("test url: http://localhost:3000/image/{}/{}", s, test_image);
// }
