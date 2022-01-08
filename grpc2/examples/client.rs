use std::time::Duration;

use futures::stream;
use proto::{route_guide_client::RouteGuideClient, Point, Rectangle, RouteNote};
use rand::{prelude::ThreadRng, Rng};
use tokio::time::{self, Instant};
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = RouteGuideClient::connect("http://localhost:50051").await?;

    let resp = client
        .get_feature(Request::new(Point::new(409146138, -746188906)))
        .await?;
    println!("{:?}", resp.into_inner());

    let rectangle = Rectangle {
        lo: Some(Point {
            latitude: 400000000,
            longitude: -750000000,
        }),
        hi: Some(Point {
            latitude: 420000000,
            longitude: -730000000,
        }),
    };

    let mut stream = client
        .list_features(Request::new(rectangle))
        .await?
        .into_inner();

    while let Ok(Some(data)) = stream.message().await {
        println!("{:?}", data);
    }

    let mut rng = rand::thread_rng();
    let point_count: i32 = rng.gen_range(2..100);

    let mut points = vec![];
    for _ in 0..=point_count {
        points.push(random_point(&mut rng))
    }

    let resp = client
        .record_route(Request::new(stream::iter(points)))
        .await?;
    println!("{:?}", resp.into_inner());
    let mut now = Instant::now();

    let outbound = async_stream::stream! {
        let mut ticker = time::interval(Duration::from_secs(1));
        while let _ = ticker.tick().await {
            let elapsed = now.elapsed();
            let note = RouteNote {
                location: Some(Point {
                    latitude: 409146138 + elapsed.as_secs() as i32,
                    longitude: -746188906,
                }),
                message: format!("at {:?}", elapsed),
            };

            yield note;
        }
    };

    let mut inbound = client
        .route_chat(Request::new(outbound))
        .await?
        .into_inner();
    while let Ok(Some(note)) = inbound.message().await {
        println!("{:?}", note);
    }

    Ok(())
}

fn random_point(rng: &mut ThreadRng) -> Point {
    let latitude = (rng.gen_range(0..180) - 90) * 10_000_000;
    let longitude = (rng.gen_range(0..360) - 180) * 10_000_000;
    Point {
        latitude,
        longitude,
    }
}
