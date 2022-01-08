// 判断enum类型的值应该是期子类型的值

use anyhow::Result;
use futures::{Stream, StreamExt};
use proto::route_guide_server::RouteGuideServer;
use proto::{route_guide_server::RouteGuide, Feature, Point, Rectangle, RouteNote, RouteSummary};
use serde::Deserialize;
use std::collections::HashMap;
use std::pin::Pin;
use std::{fs::File, sync::Arc};
use tokio::sync::mpsc;
use tokio::time::Instant;
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

#[derive(Debug)]
struct RouteGuideService {
    features: Arc<Vec<Feature>>,
}

impl RouteGuideService {
    fn new(features: Vec<Feature>) -> Self {
        Self {
            features: Arc::new(features),
        }
    }
}

#[tonic::async_trait]
impl RouteGuide for RouteGuideService {
    async fn get_feature(&self, request: Request<Point>) -> Result<Response<Feature>, Status> {
        let request = request.into_inner();
        for f in self.features.iter() {
            if f.location.as_ref() == Some(&request) {
                return Ok(Response::new(f.clone()));
            }
        }
        Ok(Response::new(Feature::default()))
    }

    type ListFeaturesStream = ReceiverStream<Result<Feature, Status>>;

    async fn list_features(
        &self,
        request: Request<Rectangle>,
    ) -> Result<Response<Self::ListFeaturesStream>, Status> {
        let (sender, receiver) = mpsc::channel(4);
        let features = self.features.clone();

        tokio::spawn(async move {
            let request = request.into_inner();
            for f in features.iter() {
                if f.location.as_ref().map(|loc| in_range(loc, &request)) == Some(true) {
                    sender.send(Ok(f.clone())).await.unwrap();
                }
            }
            println!(" /// done sending");
        });

        Ok(Response::new(ReceiverStream::new(receiver)))
    }

    async fn record_route(
        &self,
        request: Request<tonic::Streaming<Point>>,
    ) -> Result<Response<RouteSummary>, Status> {
        let mut stream = request.into_inner();
        let now = Instant::now();
        let mut last_point = None;
        let mut summary = RouteSummary::default();

        while let Some(Ok(point)) = stream.next().await {
            summary.point_count += 1;

            for f in self.features.iter() {
                if f.location.as_ref() == Some(&point) {
                    summary.feature_count += 1;
                }
            }

            last_point
                .as_ref()
                .map(|p| summary.distance += calc_distance(p, &point));
            last_point = Some(point);
        }

        summary.elapsed_time = now.elapsed().as_secs() as i32;

        Ok(Response::new(summary))
    }

    type RouteChatStream = Pin<Box<dyn Stream<Item = Result<RouteNote, Status>> + Send + 'static>>;

    async fn route_chat(
        &self,
        request: Request<tonic::Streaming<RouteNote>>,
    ) -> Result<Response<Self::RouteChatStream>, Status> {
        let mut notes = HashMap::new();
        let mut stream = request.into_inner();

        let output = async_stream::try_stream! {
            while let Some(Ok(note)) = stream.next().await {
                let location = note.location.clone().unwrap();
                let note_list =notes.entry(location).or_insert(vec![]);
                note_list.push(note);
                for note in note_list.iter(){
                    yield note.clone();
                }
            }
        };

        Ok(Response::new(Box::pin(output)))
    }
}

#[derive(Debug, Deserialize)]
struct DataFeature {
    location: DataLocation,
    name: String,
}

#[derive(Debug, Deserialize)]
struct DataLocation {
    latitude: i32,
    longitude: i32,
}

impl From<DataLocation> for Point {
    fn from(value: DataLocation) -> Self {
        Self::new(value.latitude, value.longitude)
    }
}

pub fn load() -> Vec<Feature> {
    let file = File::open("examples/db.json").expect("failed to open data file");

    let decoded: Vec<DataFeature> =
        serde_json::from_reader(&file).expect("failed to deserialize features");

    decoded
        .into_iter()
        .map(|feature| Feature::new(Some(feature.location.into()), feature.name))
        .collect()
}

fn in_range(point: &Point, rect: &Rectangle) -> bool {
    use std::cmp;

    let lo = rect.lo.as_ref().unwrap();
    let hi = rect.hi.as_ref().unwrap();

    let left = cmp::min(lo.longitude, hi.longitude);
    let right = cmp::max(lo.longitude, hi.longitude);
    let top = cmp::max(lo.latitude, hi.latitude);
    let bottom = cmp::min(lo.latitude, hi.latitude);

    point.longitude >= left
        && point.longitude <= right
        && point.latitude >= bottom
        && point.latitude <= top
}

/// Calculates the distance between two points using the "haversine" formula.
/// This code was taken from http://www.movable-type.co.uk/scripts/latlong.html.
fn calc_distance(p1: &Point, p2: &Point) -> i32 {
    const CORD_FACTOR: f64 = 1e7;
    const R: f64 = 6_371_000.0; // meters

    let lat1 = p1.latitude as f64 / CORD_FACTOR;
    let lat2 = p2.latitude as f64 / CORD_FACTOR;
    let lng1 = p1.longitude as f64 / CORD_FACTOR;
    let lng2 = p2.longitude as f64 / CORD_FACTOR;

    let lat_rad1 = lat1.to_radians();
    let lat_rad2 = lat2.to_radians();

    let delta_lat = (lat2 - lat1).to_radians();
    let delta_lng = (lng2 - lng1).to_radians();

    let a = (delta_lat / 2f64).sin() * (delta_lat / 2f64).sin()
        + (lat_rad1).cos() * (lat_rad2).cos() * (delta_lng / 2f64).sin() * (delta_lng / 2f64).sin();

    let c = 2f64 * a.sqrt().atan2((1f64 - a).sqrt());

    (R * c) as i32
}

#[tokio::main]

async fn main() -> Result<()> {
    let route_guide_service = RouteGuideService::new(load());
    let addr = "0.0.0.0:50051".parse()?;

    Server::builder()
        .add_service(RouteGuideServer::new(route_guide_service))
        .serve(addr)
        .await?;

    Ok(())
}
