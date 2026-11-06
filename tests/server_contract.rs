//! The routing job request as the server reads it: the fields it knows, the checks it makes and the version it speaks.

use eh3d_drillpro::job::JobOptions;
use eh3d_drillpro::work_contract::{
    chain_options, check_rack_options, unknown_rack_fields, MAX_RACK_PLACES,
};
use eh3d_routepro::app::server_route_request;
use eh3d_routepro::hpgl::Polyline;
use eh3d_routepro::tools::Tool;

fn tools() -> Vec<Tool> {
    vec![
        Tool {
            number: 1,
            depth_mm: 1.7,
            iterations: 2,
            feed_xy_mm_min: 800,
            feed_z_mm_min: 300,
            spindle_rpm: 24000,
        },
        Tool {
            number: 2,
            depth_mm: 0.2,
            iterations: 1,
            feed_xy_mm_min: 600,
            feed_z_mm_min: 200,
            spindle_rpm: 30000,
        },
    ]
}

fn paths() -> Vec<Polyline> {
    vec![Polyline {
        pen: 1,
        points: vec![(0, 0), (400, 0), (400, 400)],
    }]
}

#[test]
fn a_request_without_chaining_is_the_request_of_the_first_version() {
    let request = server_route_request("o.plt", &tools(), &paths(), &JobOptions::default());
    assert_eq!(request["kind"], "route");
    assert!(request.get("request_version").is_none());
    assert!(request.get("rack").is_none());
    assert_eq!(chain_options(&request), None);
    assert!(unknown_rack_fields(&request).is_empty());
}

#[test]
fn the_rack_of_a_routing_request_is_one_the_server_accepts_and_reads_back() {
    let options = JobOptions {
        rack_first: 4,
        rack_carry_in: true,
        rack_carry_out: true,
        rack_slots: vec![(1, 6), (2, 3)],
        rack_carry_slot: Some(5),
        ..JobOptions::default()
    };
    // the checks the server makes of it pass
    assert_eq!(check_rack_options(&options, MAX_RACK_PLACES), Ok(()));
    let request = server_route_request("o.plt", &tools(), &paths(), &options);
    assert_eq!(request["request_version"], 2);
    // it sends only the fields the server knows
    assert!(unknown_rack_fields(&request).is_empty());
    // and the server reads from the request the very options the program had
    assert_eq!(chain_options(&request), Some(options));
}

#[test]
fn the_pens_and_the_paths_do_not_change_with_the_chaining() {
    let plain = server_route_request("o.plt", &tools(), &paths(), &JobOptions::default());
    let chained = server_route_request(
        "o.plt",
        &tools(),
        &paths(),
        &JobOptions {
            rack_first: 2,
            ..JobOptions::default()
        },
    );
    for key in ["kind", "name", "tools", "paths", "skip_tools"] {
        assert_eq!(plain[key], chained[key], "{key}");
    }
    let keys =
        |v: &serde_json::Value| -> Vec<String> { v.as_object().unwrap().keys().cloned().collect() };
    // the chaining adds exactly the two fields of the second version
    let added: Vec<String> = keys(&chained)
        .into_iter()
        .filter(|k| !keys(&plain).contains(k))
        .collect();
    assert_eq!(added.len(), 2);
    assert!(added.contains(&"rack".to_string()) && added.contains(&"request_version".to_string()));
}

#[test]
fn places_that_the_server_would_refuse_are_found_before_sending() {
    let two_tools_one_place = JobOptions {
        rack_slots: vec![(1, 3), (2, 3)],
        ..JobOptions::default()
    };
    let problem = check_rack_options(&two_tools_one_place, MAX_RACK_PLACES).unwrap_err();
    assert_eq!(
        problem.to_string(),
        "bad_rack: rack.slots: place 3 is given to two tools"
    );
}
