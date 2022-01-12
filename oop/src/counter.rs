use std::{
    collections::HashMap,
    ops::Range,
    slice::SliceIndex,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, SystemTime},
};

use anyhow::Result;
use rand::Rng;

#[derive(Debug, Clone, Default)]
struct Metric {
    api: String,
    ts: i64,
    duration: i64,
}

impl Metric {
    fn new(api: impl Into<String>, ts: i64, duration: i64) -> Self {
        Self {
            api: api.into(),
            ts,
            duration,
        }
    }
}

trait MetricsCollector {
    fn collect(&self, metric: Metric) -> Result<()>;
}

struct DefaultMetricCollector<Store> {
    store: Arc<Mutex<Store>>,
}

impl<Store: MetricStorage> DefaultMetricCollector<Store> {
    fn new(store: Arc<Mutex<Store>>) -> DefaultMetricCollector<Store> {
        Self { store }
    }
}

impl<Store: MetricStorage> MetricsCollector for DefaultMetricCollector<Store> {
    fn collect(&self, metric: Metric) -> Result<()> {
        let mut store = self.store.lock().unwrap();
        Ok(store.save(metric)?)
    }
}

trait MetricStorage {
    fn save(&mut self, metric: Metric) -> Result<()>;
    fn load_all(&self, start: i64, end: i64) -> Result<Vec<(String, Vec<Metric>)>>;
}

#[derive(Default, Debug)]
struct MemoryMetricStorage {
    metrics: HashMap<String, Vec<Metric>>,
}

impl MetricStorage for MemoryMetricStorage {
    fn save(&mut self, metric: Metric) -> Result<()> {
        let entry = self.metrics.entry(metric.api.clone()).or_insert(vec![]);
        entry.push(metric);
        Ok(())
    }

    fn load_all(&self, start: i64, end: i64) -> Result<Vec<(String, Vec<Metric>)>> {
        let metrics = self
            .metrics
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    v.clone()
                        .into_iter()
                        .filter(|m| m.ts >= start && m.ts <= end)
                        .collect(),
                )
            })
            .collect();

        Ok(metrics)
    }
}

#[derive(Debug)]
struct AggregateResult {
    avg: i64,
}

trait Aggregate {
    fn aggregate(self) -> AggregateResult;
}

struct DefaultAggregate {
    metrics: Vec<Metric>,
}

impl DefaultAggregate {
    fn new(metrics: Vec<Metric>) -> DefaultAggregate {
        Self { metrics }
    }
}

impl Aggregate for DefaultAggregate {
    fn aggregate(self) -> AggregateResult {
        let avg = self.metrics.iter().map(|m| m.duration).sum::<i64>() / self.metrics.len() as i64;
        AggregateResult { avg }
    }
}

trait MetricReorter {
    fn report(&self) -> Result<()>;
}

struct DefaultMetricReport<Store> {
    store: Arc<Mutex<Store>>,
}

impl<Store: MetricStorage> DefaultMetricReport<Store> {
    fn new(store: Arc<Mutex<Store>>) -> DefaultMetricReport<Store> {
        Self { store }
    }
}

impl<Store: MetricStorage> MetricReorter for DefaultMetricReport<Store> {
    fn report(&self) -> Result<()> {
        let store = self.store.lock().unwrap();
        let metrics = store.load_all(0, i64::MAX)?;
        metrics
            .into_iter()
            .for_each(|(k, v)| println!("{} {:?}", k, DefaultAggregate::new(v).aggregate()));

        Ok(())
    }
}

pub fn main() {
    let storage = Arc::new(Mutex::new(MemoryMetricStorage::default()));
    let collector = DefaultMetricCollector::new(storage.clone());
    let reporter = DefaultMetricReport::new(storage);

    thread::spawn(move || loop {
        reporter.report().unwrap();
        thread::sleep(Duration::from_secs(1));
    });

    let mut rng = rand::thread_rng();
    loop {
        let ts = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();
        collector
            .collect(Metric::new(
                "api_a",
                ts.as_millis() as i64,
                rng.gen_range(0..100),
            ))
            .unwrap();
        thread::sleep(Duration::from_secs(1));
    }
}

fn gen_iter<T: Default>(r: Range<i32>) -> Box<dyn Iterator<Item = T>> {
    Box::new(r.into_iter().map(|_| T::default()))
}

fn to_iter(vec: Option<Vec<i32>>) -> Option<Box<dyn Iterator<Item = i32>>> {
    match vec {
        Some(v) => Some(Box::new(v.into_iter())),
        None => None,
    }
}

mod tests {
    use super::*;

    #[test]
    fn gen_iter_works() {
        assert_eq!(gen_iter::<i32>(1..3).collect::<Vec<_>>(), vec![0, 0]);
    }
}
