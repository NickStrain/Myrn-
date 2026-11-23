// Temporary: print the top vector matches for a few queries.
fn main() {
    let cwd = std::env::current_dir().unwrap();
    let e = myrn_engine::api::Engine::open(&cwd, None).unwrap();
    for q in std::env::args().skip(1) {
        let qv = e.embed(&[q.clone()]).unwrap().pop().unwrap();
        let mut stmt = e.db().conn.prepare("SELECT id, vec FROM vectors").unwrap();
        let mut hits: Vec<(String, f32)> = stmt
            .query_map([], |r| {
                Ok((r.get::<_, String>(0)?, r.get::<_, Vec<u8>>(1)?))
            })
            .unwrap()
            .filter_map(|r| r.ok())
            .map(|(id, b)| (id, myrn_embed::dot(&qv, &myrn_embed::from_bytes(&b))))
            .collect();
        hits.sort_by(|a, b| b.1.total_cmp(&a.1));
        println!("== {q}");
        for (id, s) in hits.iter().take(6) {
            println!("  {s:.3} {id}");
        }
        println!("  ... 20th {:.3}, 100th {:.3}", hits[19].1, hits[99].1);
    }
}
