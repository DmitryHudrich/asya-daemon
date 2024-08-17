use criterion::BenchmarkId;
use criterion::{criterion_group, criterion_main, Criterion};
use multimap::multimap;
use multimap::MultiMap;

use aska::service::fetchservice::parse;

fn get_params() -> MultiMap<String, String> {
    multimap! (
        "issys_name" => "1",
        "issys_osv" => "1",
        "issys_name"=> "1",
        "issys_kernelv" => "1",
        "issys_hostname" => "1",
        "issys_osv"=> "1",
        "issys_osvlong"=> "1",
        "issys_uptime" => "1",
        "issys_distroid" => "1",
        "issys_name" => "1",
        "issys_kernelv" => "1",
        "issys_hostname" => "1",
        "issys_osv" => "1",
        "issys_osvlong" => "1",
        "issys_uptime" => "1",
        "issys_distroid" => "1",
        "ihram_totalmem" => "1",
        "ihram_usedmem" => "1",
        "ihram_freemem" => "1",
        "ihram_availmem" => "1",
        "ihram_totalswp" => "1",
        "ihram_freeswp" => "1",
        "ihram_usedswp" => "1",
        "ihcpu_name" => "1",
        "ihcpu_cores" => "1",
    )
    .into_iter()
    .map(|(k, v)| {
        (
            k.to_string(),
            v.into_iter().map(|e| e.to_string()).collect::<String>(),
        )
    })
    .collect::<MultiMap<String, String>>()
}

pub fn parse_all_params(c: &mut Criterion) {
    let params = get_params();
    c.bench_with_input(BenchmarkId::new("parse all params", ""), &params, |b, p| {
        b.iter(|| parse(p.clone()))
    });
}

pub fn parse_every_param(c: &mut Criterion) {
    let mut group = c.benchmark_group("from_elem");

    for param in get_params().iter() {
        let mut multimap = MultiMap::new();
        multimap.insert(param.0.clone(), param.1.clone());
        group.bench_with_input(BenchmarkId::from_parameter(param.0), &multimap, |b, p| {
            b.iter(|| parse(p.clone()));
        });
    }
    group.finish();
}

criterion_group!(benches, parse_all_params, parse_every_param);
criterion_main!(benches);
