# EH3D-ROUTEPRO: requirements

This register is the single source of the state of each capability: `STATUS.md` and the README files cite it and never state a state that it does not. A state is one of `planned`, `prototype`, `implemented` and `validated`. `validated` means that a person has run the capability on a real machine and the evidence of the manifest says `bench` or `field`; nothing here is validated, because a whole routing job has never run on a machine; only the serial protocol it shares with the drilling program has moved a real Bungard machine, and that evidence belongs to that program.

The register has 43 requirements: 34 implemented, 4 prototype, 5 planned and 0 validated. By progress: 34 done, 4 partial, 1 pending and 4 blocked. High coverage is not proof of correctness, so each row says how it is checked: the test that names it, and what it shows.

## How to read the register

* **ID:** `RPRO-NNN`, never reused and never renumbered; a new capability takes the next number.
* **State:** `implemented` is code that is complete and tested on a computer, `prototype` is part of the capability, `planned` is not written.
* **Progress:** `done`, `partial`, `pending` (can be written without hardware) or `blocked` (waits for hardware or for another program of the suite).
* **Evidence:** `simulator` when the test runs against the simulator of the machine, which receives the same bytes that a real machine would, `host-test` when it checks a calculation, a reader or a screen-less piece, `none` when nothing is written yet.
* **Test:** the acceptance criterion: `file::name` is the test `name` of `tests/file.rs`; a name with a slash, such as `protocol/tests/file::name` or `src/file::name`, is the path from the root without the extension. The test must exist. For a capability that is not written the cell says `planned:` and the name of the test that will show it; the day that test exists, the row must change.
* **Limits:** what is not done or not proven; a row that is `partial` or `blocked` always says it.

The check is `tests/reqs.rs`: it reads this table, finds each test and fails when a state, a test or a count does not agree.

## Reading the plot

| ID | Requirement | State | Progress | Evidence | Test | Limits |
|---|---|---|---|---|---|---|
| RPRO-001 | A plot is measured and a rectangle of 300 by 200 mm is read as such. | implemented | done | host-test | `routepro::measure_plot_is_a_300_by_200_mm_rectangle` | - |
| RPRO-002 | Pen-up moves split a polyline, and absolute and relative moves are both read. | implemented | done | host-test | `routepro::pen_up_moves_split_polylines` | - |
| RPRO-003 | Each pen selects a tool. | implemented | done | host-test | `routepro::pens_select_the_tool` | - |
| RPRO-004 | A command that is not understood is counted and ignored, never fatal. | implemented | done | host-test | `routepro::unknown_commands_are_counted_not_fatal` | - |
| RPRO-005 | Every sample plot of the repository is read. | implemented | done | host-test | `routepro::every_real_plt_file_parses` | Only small files written for the tests and the examples of the release; no plot of a real design. |
| RPRO-006 | A plot with an absurd number is turned down with its line. | implemented | done | host-test | `src/app::a_plot_with_an_absurd_number_is_turned_down_with_its_line` | - |
| RPRO-007 | A drawing with a line that never ends, or that is not text, is turned down. | implemented | done | host-test | `src/app::a_drawing_with_a_line_that_never_ends_or_not_text_is_turned_down` | - |
| RPRO-008 | Every example and test plot is inside the limits, and a plot is judged by the line the problem is on. | implemented | done | host-test | `input_limits::every_example_and_every_test_plot_is_inside_the_limits` | - |
| RPRO-009 | A DXF drawing and the outline of a Gerber board load as routing paths. | implemented | done | host-test | `routepro::a_dxf_drawing_and_a_gerber_outline_load_as_routing_paths` | - |
| RPRO-010 | The first routing file of an archive loads. | implemented | done | host-test | `classic_zip::the_first_routing_file_of_an_archive_loads` | - |

## Planning the cut

| ID | Requirement | State | Progress | Evidence | Test | Limits |
|---|---|---|---|---|---|---|
| RPRO-011 | A plan returns the tool to the surface and reaches the end. | implemented | done | host-test | `protocol/tests/route::a_plan_returns_the_tool_to_the_surface_and_reaches_the_end` | - |
| RPRO-012 | The tool reaches exactly the depth asked going down and comes back up. | implemented | done | host-test | `protocol/tests/route::the_tool_reaches_exactly_the_depth_asked_going_down_and_comes_back_up` | - |
| RPRO-013 | Every pass goes a part of the depth deeper and the last one is the whole depth. | implemented | done | host-test | `protocol/tests/route_more::every_pass_goes_a_part_of_the_depth_deeper_and_the_last_one_is_the_whole_depth` | - |
| RPRO-014 | A number of passes that is not believable is bounded and not planned for ever. | implemented | done | host-test | `protocol/tests/route_more::a_number_of_passes_that_is_not_believable_is_bounded_and_not_planned_for_ever` | - |
| RPRO-015 | The parameters of a tool are checked before a plan is made. | implemented | done | host-test | `protocol/tests/route_more::the_parameters_of_a_tool_are_checked_before_a_plan_is_made` | - |
| RPRO-016 | The plan is the same every time. | implemented | done | host-test | `protocol/tests/route_more::the_plan_is_the_same_every_time` | - |
| RPRO-017 | Whatever the path, the tool never rises above where it began and ends there. | implemented | done | host-test | `protocol/tests/route_property::whatever_the_path_the_tool_never_rises_above_where_it_began_and_ends_there` | Seeded generation, not a continuous fuzzer. |
| RPRO-018 | Routing goes through the safety gate of the protocol. | implemented | done | host-test | `protocol/tests/route::routing_goes_through_the_safety_gate` | The gate is a function of the protocol; no machine was behind it. |
| RPRO-019 | On a tilted table the depth of the levelled plan follows it along the path. | implemented | done | host-test | `protocol/tests/route_more::a_tilted_table_makes_the_depth_follow_it_along_the_path` | - |
| RPRO-020 | A table much higher than the depth never lifts the tool over the travel height. | implemented | done | host-test | `protocol/tests/route_more::a_table_much_higher_than_the_depth_never_lifts_the_tool_over_the_travel_height` | - |
| RPRO-021 | The plan of each sample file is the stored one. | implemented | done | host-test | `plan_golden::the_plan_of_each_sample_file_is_the_stored_one` | - |

## Running the job

| ID | Requirement | State | Progress | Evidence | Test | Limits |
|---|---|---|---|---|---|---|
| RPRO-022 | The job runs on the machine and the head comes back to the origin. | prototype | partial | simulator | `route_job::the_job_runs_on_the_machine_and_the_head_comes_back_to_the_origin` | Run on the simulator only; no whole job has run on a real machine. |
| RPRO-023 | A tool with settings the planner cannot use refuses the job naming its pen. | implemented | done | simulator | `route_job::a_tool_with_settings_the_planner_cannot_use_refuses_the_job_naming_its_pen` | - |
| RPRO-024 | By hand, the router stops the spindle before asking for the next cutter and starts it after. | prototype | partial | simulator | `route_job::by_hand_the_router_stops_the_spindle_before_asking_for_the_next_cutter_and_starts_it_after` | The spindle output has never been switched on a real machine. |
| RPRO-025 | With the changer the operator is never asked for a tool. | implemented | done | simulator | `route_job::on_a_machine_with_the_changer_the_operator_is_never_asked_for_a_tool` | - |
| RPRO-026 | The head never cuts outside the work area whatever the places of the tools. | implemented | done | simulator | `route_positions::the_head_never_cuts_outside_the_work_area_whatever_the_places_of_the_tools` | - |
| RPRO-027 | A routing job that follows a drilling job takes the chuck from the place of the last drill. | implemented | done | simulator | `route_positions::a_routing_job_that_follows_a_drilling_job_takes_the_chuck_from_the_place_of_the_last_drill` | - |
| RPRO-028 | The pens switched off and the paths outside the working area are left out. | implemented | done | simulator | `job_options::the_pens_switched_off_are_left_out_of_the_job` | - |
| RPRO-029 | A job that goes on leaves out the paths already done. | implemented | done | simulator | `job_options::a_job_that_goes_on_leaves_out_the_paths_already_done` | - |
| RPRO-030 | The outside of a panel is cut at its tabs. | implemented | done | simulator | `job_options::the_outside_of_a_panel_is_cut_at_its_tabs` | - |

## Server and screens

| ID | Requirement | State | Progress | Evidence | Test | Limits |
|---|---|---|---|---|---|---|
| RPRO-031 | The rack of a routing request is one that the server accepts and reads back. | prototype | partial | host-test | `server_contract::the_rack_of_a_routing_request_is_one_the_server_accepts_and_reads_back` | Checked against the contract of the server, not against a running server. |
| RPRO-032 | A rack the server would refuse is refused before the job is planned. | implemented | done | host-test | `src/app::a_rack_the_server_would_refuse_is_refused_before_the_job_is_planned` | - |
| RPRO-033 | The demo sends the very bytes of the plan of the machine. | implemented | done | simulator | `src/app::the_demo_sends_the_very_bytes_of_the_plan_of_the_machine` | - |
| RPRO-034 | The main screen matches the stored image of the original cell by cell. | prototype | partial | host-test | `classic_golden::main_screen_matches_the_original` | Compared with stored images, not with the original program on a screen. |
| RPRO-035 | The hot letters of the menu are unique in each of the 37 languages. | implemented | done | host-test | `classic::menu_hot_letters_are_unique_in_the_37_languages` | - |
| RPRO-036 | Every language draws every screen. | implemented | done | host-test | `routepro::every_language_renders_every_screen` | - |
| RPRO-037 | The chord angle is edited and saved to the real file, and zero is refused. | implemented | done | host-test | `classic_interaction::a_zero_chord_angle_is_refused` | - |
| RPRO-038 | The basic configuration is rewritten without losing its explanations. | implemented | done | host-test | `release_files::the_basic_configuration_is_rewritten_without_losing_its_explanations` | - |

## Not written yet

| ID | Requirement | State | Progress | Evidence | Test | Limits |
|---|---|---|---|---|---|---|
| RPRO-039 | A whole routing job runs on a real Bungard machine and the board is cut. | planned | blocked | none | `planned:real_job::a_whole_routing_job_runs_on_a_real_machine` | Needs a machine, a board, a cutter and a person ready to stop it. |
| RPRO-040 | The spindle and the vacuum are switched on a real machine by the job. | planned | blocked | none | `planned:real_outputs::the_spindle_and_the_vacuum_are_switched_on_a_real_machine` | Needs a machine. |
| RPRO-041 | The steps per millimetre are measured on a real machine and match the machine file. | planned | blocked | none | `planned:real_steps::the_steps_per_millimetre_match_a_measured_machine` | Needs a machine and a measure. |
| RPRO-042 | The job goes through a running server to a machine. | planned | blocked | none | `planned:real_server::a_job_goes_through_a_running_server_to_a_machine` | Needs the server and a machine. |
| RPRO-043 | The cut is offset by the radius of the cutter. | planned | pending | none | `planned:offset::the_cut_is_offset_by_the_radius_of_the_cutter` | Not written; the plan follows the path as drawn. |
