use bloody_falcon::config::load_config;

#[test]
fn load_default_when_missing() {
    let cfg = load_config(Some("/tmp/does-not-exist.toml")).unwrap();
    assert_eq!(cfg.providers.len(), 5);
    assert!(cfg.providers.iter().all(|p| p.enabled));
}

#[test]
fn load_real_file() {
    let cfg = load_config(Some("config/bloodyf4lcon.toml")).unwrap();
    assert!(cfg.timeout_ms > 0);
    assert!(!cfg.providers.is_empty());
}
