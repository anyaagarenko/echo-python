use echo_python::check_source;
use std::path::Path;

#[test]
fn reports_line_and_column() {
    let diags = check_source(Path::new("t.py"), "x = [2, 1]\n").expect("lint");
    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0].row, 1);
    assert_eq!(diags[0].column, 5);
}
