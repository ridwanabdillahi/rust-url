#![cfg(debugger_visualizer)]

mod common;
use common::JsonExt;

use std::str::FromStr;

use debugger_test::debugger_test;
use serde_json::Value;
use url::Url;

#[inline(never)]
fn __break() {}

#[debugger_test(
    debugger = "cdb",
    commands = "
    .nvlist

    dx _urls

    dx _urls[0]

    dx _urls[1]

    dx _urls[2]

    dx _urls[3]

    dx _urls[4]",
    expected_statements = r#"
    pattern:debugger_visualizer-.*\.exe \(embedded NatVis ".*debugger_visualizer-0\.natvis"\)"#
)]
fn test_url_visualizer() {
    // Copied from https://github.com/web-platform-tests/wpt/blob/master/url/
    let mut json = Value::from_str(include_str!("urltestdata.json"))
        .expect("JSON parse error in urltestdata.json");

    let _urls = json
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .filter_map(|entry| {
            if entry.is_string() || entry.take_key("failure").is_some() {
                return None;
            }

            let maybe_base = entry
                .take_key("base")
                .expect("missing base key")
                .maybe_string();
            let input = entry.take_string("input");

            let res = if let Some(base) = maybe_base {
                Url::parse(&base)
                    .expect(format!("must be able to parse base: `{}` into a URL", &base).as_str())
                    .join(&input)
            } else {
                Url::parse(&input)
            };

            Some(
                res.expect(
                    format!("must be able to parse input: `{}` into a URL", &input).as_str(),
                ),
            )
        })
        .collect::<Vec<Url>>();

    __break();
}
