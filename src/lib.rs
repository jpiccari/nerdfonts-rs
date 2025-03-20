include!(concat!(env!("OUT_DIR"), "/mod.rs"));

#[test]
fn spot_check_glyphs() {
    assert_eq!(md::MD_PERIODIC_TABLE, '󰢶');
    assert_eq!(fa::FA_500PX, '');
    assert_eq!(fa::FA_BATTERY_2, '');
    assert_eq!(fae::FAE_MOON_CLOUD, '');
}
