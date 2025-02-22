use view::assert_rendered_output;

use super::*;
use crate::testutil::module_test;

#[test]
fn normal_mode_remove_line_first() {
	module_test(
		&[
			"pick aaa c1",
			"pick bbb c2",
			"pick ccc c3",
			"pick ddd c4",
			"pick eee c5",
		],
		&[Event::from(MetaEvent::Delete)],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				Options AssertRenderOptions::EXCLUDE_STYLE,
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected} > pick bbb      c2{Pad( )}",
				"   pick ccc      c3",
				"   pick ddd      c4",
				"   pick eee      c5"
			);
		},
	);
}

#[test]
fn normal_mode_remove_line_end() {
	module_test(
		&[
			"pick aaa c1",
			"pick bbb c2",
			"pick ccc c3",
			"pick ddd c4",
			"pick eee c5",
		],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::Delete),
		],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				Options AssertRenderOptions::EXCLUDE_STYLE,
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"   pick aaa      c1",
				"   pick bbb      c2",
				"   pick ccc      c3",
				"{Selected} > pick ddd      c4{Pad( )}"
			);
		},
	);
}

#[test]
fn visual_mode_remove_lines_start_index_first() {
	module_test(
		&[
			"pick aaa c1",
			"pick bbb c2",
			"pick ccc c3",
			"pick ddd c4",
			"pick eee c5",
		],
		&[
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::Delete),
		],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				Options AssertRenderOptions::EXCLUDE_STYLE,
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected} > pick ddd      c4{Pad( )}",
				"   pick eee      c5"
			);
			assert_eq!(
				module.visual_index_start.unwrap(),
				module.todo_file.lock().get_selected_line_index()
			);
		},
	);
}

#[test]
fn visual_mode_remove_lines_end_index_first() {
	module_test(
		&[
			"pick aaa c1",
			"pick bbb c2",
			"pick ccc c3",
			"pick ddd c4",
			"pick eee c5",
		],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::Delete),
		],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				Options AssertRenderOptions::EXCLUDE_STYLE,
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"{Selected} > pick ddd      c4{Pad( )}",
				"   pick eee      c5"
			);
			assert_eq!(
				module.visual_index_start.unwrap(),
				module.todo_file.lock().get_selected_line_index()
			);
		},
	);
}

#[test]
fn visual_mode_remove_lines_start_index_last() {
	module_test(
		&[
			"pick aaa c1",
			"pick bbb c2",
			"pick ccc c3",
			"pick ddd c4",
			"pick eee c5",
		],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::MoveCursorUp),
			Event::from(MetaEvent::Delete),
		],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				Options AssertRenderOptions::EXCLUDE_STYLE,
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"   pick aaa      c1",
				"{Selected} > pick bbb      c2{Pad( )}"
			);
			assert_eq!(
				module.visual_index_start.unwrap(),
				module.todo_file.lock().get_selected_line_index()
			);
		},
	);
}

#[test]
fn visual_mode_remove_lines_end_index_last() {
	module_test(
		&[
			"pick aaa c1",
			"pick bbb c2",
			"pick ccc c3",
			"pick ddd c4",
			"pick eee c5",
		],
		&[
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::ToggleVisualMode),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::MoveCursorDown),
			Event::from(MetaEvent::Delete),
		],
		|mut test_context| {
			let mut module = create_list(&Config::new(), test_context.take_todo_file());
			_ = test_context.handle_all_events(&mut module);
			assert_rendered_output!(
				Options AssertRenderOptions::EXCLUDE_STYLE,
				test_context.build_view_data(&mut module),
				"{TITLE}{HELP}",
				"{BODY}",
				"   pick aaa      c1",
				"{Selected} > pick bbb      c2{Pad( )}"
			);
			assert_eq!(
				module.visual_index_start.unwrap(),
				module.todo_file.lock().get_selected_line_index()
			);
		},
	);
}
