extern crate chrono;
extern crate flexi_logger;
#[macro_use]
extern crate log;
extern crate serde;
extern crate serde_db;
#[macro_use]
extern crate serde_derive;

mod mock_db;
mod util;

use flexi_logger::{LogSpecification, ReconfigurationHandle};
use mock_db::{MValue as MV, Resultset};

#[test] // cargo test --test test_resultset_1x1 -- --nocapture
pub fn test_resultset_1x1() {
    let mut loghandle = util::init_logger("info");

    match evaluate_field_rs(&mut loghandle) {
        Err(e) => {
            error!("test_resultset_1x1() failed with {:?}", e);
            assert!(false)
        }
        Ok(_) => debug!("test_resultset_1x1() ended successful"),
    }
}

// Test the various ways to evaluate a resultset
fn evaluate_field_rs(loghandle: &mut ReconfigurationHandle) -> mock_db::Result<()> {
    #[derive(Deserialize)]
    struct TestDataMin {
        f1: String,
    };

    loghandle.set_new_spec(LogSpecification::parse("info"));
    info!("=== Single value (1x1) ===");
    {
        info!("Convert a 1x1 resultset into a Vec<struct>");
        let vd: Vec<TestDataMin> = get_resultset_string(1).into_typed()?;
        assert_eq!(&vd[0].f1, "a");
    }
    {
        info!("Convert a 1x1 resultset into a Vec<field>");
        let vs: Vec<String> = get_resultset_string(1).into_typed()?;
        assert_eq!(&vs[0], "a");
    }
    {
        info!("Convert a 1x1 resultset into a struct");
        let d: TestDataMin = get_resultset_string(1).into_typed()?;
        assert_eq!(&d.f1, "a");
    }
    {
        info!("Convert a 1x1 resultset into a field");
        let s: String = get_resultset_string(1).into_typed()?;
        assert_eq!(&s, "a");
    }
    Ok(())
}

////////////////////////////////////////////////////////
fn get_resultset_string(len: usize) -> Resultset {
    assert!(len < 60);
    let mut rs = Resultset::new(vec!["f1"]);
    for i in 0..len {
        rs.push(vec![MV::String(String::from_utf8(vec![b'a' + i as u8]).unwrap())]);
    }
    rs
}
