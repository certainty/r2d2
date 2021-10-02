use r2d2::engine::storage::{self, lsm};
use r2d2::engine::{Key, Value};
use tempfile::tempdir;

#[test]
fn check_lsm_works() -> anyhow::Result<()> {
    let mut config_builder = storage::lsm::configuration::Builder::default();
    config_builder.with_storage_path(tempdir()?.path().to_path_buf())?;
    let config = config_builder.build()?;
    let mut lsm = lsm::LSM::new(config)?;

    let foo = Key::from("foo");
    let bar = Value::from("bar");

    assert!(lsm.get(&foo)?.is_none());
    assert!(lsm.set(foo.clone(), bar.clone()).is_ok());
    assert_eq!(Some(bar.clone()), lsm.get(&foo)?);

    Ok(())
}

#[test]
fn check_recovery_from_commit_log() -> anyhow::Result<()> {
    let mut config_builder = storage::lsm::configuration::Builder::default();
    config_builder.with_storage_path(tempdir()?.path().to_path_buf())?;
    let config = config_builder.build()?;

    let foo = Key::from("foo");
    let bar = Key::from("bar");
    let baz = Value::from("baz");

    {
        let mut lsm = lsm::LSM::new(config.clone())?;

        assert!(lsm.set(foo.clone(), baz.clone()).is_ok());
        assert!(lsm.set(bar.clone(), baz.clone()).is_ok());
    }

    // now open a new LSM that recreates the state from the commit log
    let lsm = lsm::LSM::new(config)?;

    // keys should be there now
    assert_eq!(Some(baz.clone()), lsm.get(&foo)?);
    assert_eq!(Some(baz.clone()), lsm.get(&bar)?);
    Ok(())
}
