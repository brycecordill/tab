use super::*;

#[test]
fn test_calc_tab_order1() {
    let args = vec![
        String::from("tab"),
        String::from("alice"),
        String::from("owes"),
        String::from("20"),
        String::from("bob"),
    ];
    let conf = Config::new(args).unwrap();

    let tab = calculate_tab(&conf);
    assert_eq!(tab, 20.00);
}
#[test]
fn test_calc_tab_order2() {
    let args = vec![
        String::from("tab"),
        String::from("bob"),
        String::from("owes"),
        String::from("20"),
        String::from("alice"),
    ];
    let conf = Config::new(args).unwrap();

    let tab = calculate_tab(&conf);
    assert_eq!(tab, -20.00);
}
#[test]
fn test_calc_tab_order3() {
    let args = vec![
        String::from("tab"),
        String::from("alice"),
        String::from("recv"),
        String::from("20"),
        String::from("bob"),
    ];
    let conf = Config::new(args).unwrap();

    let tab = calculate_tab(&conf);
    assert_eq!(tab, -20.00);
}
#[test]
fn test_calc_tab_order4() {
    let args = vec![
        String::from("tab"),
        String::from("bob"),
        String::from("recv"),
        String::from("20"),
        String::from("alice"),
    ];
    let conf = Config::new(args).unwrap();

    let tab = calculate_tab(&conf);
    assert_eq!(tab, 20.00);
}