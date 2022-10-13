use std::{
	borrow::BorrowMut,
	sync::{
		atomic::{AtomicBool, Ordering},
		Arc,
	},
};

use anyhow::{anyhow, Error, Result};
use crossbeam_channel as channel;
use parking_lot::Mutex;

use super::{action::ViewAction, render_slice::RenderSlice, view_data::ViewData};

fn map_send_err(_: channel::SendError<ViewAction>) -> Error {
	anyhow!("Unable to send data")
}

/// Represents a message sender and receiver for passing actions between threads.
#[derive(Clone, Debug)]
pub struct Sender {
	poisoned: Arc<AtomicBool>,
	paused: Arc<AtomicBool>,
	sender: channel::Sender<ViewAction>,
	render_slice: Arc<Mutex<RenderSlice>>,
}

impl Sender {
	/// Create a new instance.
	#[inline]
	#[must_use]
	pub fn new(sender: channel::Sender<ViewAction>) -> Self {
		Self {
			poisoned: Arc::new(AtomicBool::new(false)),
			sender,
			paused: Arc::new(AtomicBool::new(false)),
			render_slice: Arc::new(Mutex::new(RenderSlice::new())),
		}
	}

	/// Clone the poisoned flag.
	#[inline]
	#[must_use]
	pub fn clone_poisoned(&self) -> Arc<AtomicBool> {
		Arc::clone(&self.poisoned)
	}

	/// Is the sender poisoned, and not longer accepting actions.
	#[inline]
	#[must_use]
	pub fn is_poisoned(&self) -> bool {
		self.poisoned.load(Ordering::Acquire)
	}

	/// Is the sender paused from refreshing the view.
	#[inline]
	#[must_use]
	pub fn is_paused(&self) -> bool {
		self.paused.load(Ordering::Relaxed)
	}

	/// Clone the render slice.
	#[inline]
	#[must_use]
	pub fn clone_render_slice(&self) -> Arc<Mutex<RenderSlice>> {
		Arc::clone(&self.render_slice)
	}

	/// Queue a start action.
	///
	/// # Errors
	/// Results in an error if the sender has been closed.
	#[inline]
	pub fn start(&self) -> Result<()> {
		self.paused.store(false, Ordering::Relaxed);
		self.sender.send(ViewAction::Start).map_err(map_send_err)
	}

	/// Queue a stop action.
	///
	/// # Errors
	/// Results in an error if the sender has been closed.
	#[inline]
	pub fn stop(&self) -> Result<()> {
		self.paused.store(true, Ordering::Relaxed);
		self.sender.send(ViewAction::Stop).map_err(map_send_err)
	}

	/// Queue an end action.
	///
	/// # Errors
	/// Results in an error if the sender has been closed.
	#[inline]
	pub fn end(&self) -> Result<()> {
		self.stop()?;
		self.sender.send(ViewAction::End).map_err(map_send_err)
	}

	/// Queue a scroll up action.
	#[inline]
	pub fn scroll_top(&self) {
		self.render_slice.lock().borrow_mut().record_scroll_top();
	}

	/// Queue a scroll up action.
	#[inline]
	pub fn scroll_bottom(&self) {
		self.render_slice.lock().borrow_mut().record_scroll_bottom();
	}

	/// Queue a scroll up action.
	#[inline]
	pub fn scroll_up(&self) {
		self.render_slice.lock().borrow_mut().record_scroll_up();
	}

	/// Queue a scroll down action.
	#[inline]
	pub fn scroll_down(&self) {
		self.render_slice.lock().borrow_mut().record_scroll_down();
	}

	/// Queue a scroll left action.
	#[inline]
	pub fn scroll_left(&self) {
		self.render_slice.lock().borrow_mut().record_scroll_left();
	}

	/// Queue a scroll right action.
	#[inline]
	pub fn scroll_right(&self) {
		self.render_slice.lock().borrow_mut().record_scroll_right();
	}

	/// Queue a scroll up a page action.
	#[inline]
	pub fn scroll_page_up(&self) {
		self.render_slice.lock().borrow_mut().record_page_up();
	}

	/// Queue a scroll down a page action.
	#[inline]
	pub fn scroll_page_down(&self) {
		self.render_slice.lock().borrow_mut().record_page_down();
	}

	/// Queue a resize action.
	#[inline]
	pub fn resize(&self, width: u16, height: u16) {
		self.render_slice
			.lock()
			.borrow_mut()
			.record_resize(width as usize, height as usize);
	}

	/// Sync the `ViewData` and queue a render action.
	///
	/// # Errors
	/// Results in an error if the sender has been closed.
	#[inline]
	pub fn render(&self, view_data: &ViewData) -> Result<()> {
		self.render_slice.lock().borrow_mut().sync_view_data(view_data);
		self.sender.send(ViewAction::Render).map_err(map_send_err)
	}
}

#[cfg(test)]
mod tests {
	use std::sync::atomic::Ordering;

	use crate::{
		testutil::{render_view_line, with_view_sender},
		ViewData,
		ViewLine,
	};

	#[test]
	fn poisoned() {
		with_view_sender(|context| {
			context.sender.clone_poisoned().store(true, Ordering::Release);
			assert!(context.sender.is_poisoned());
		});
	}

	#[test]
	fn start_success() {
		with_view_sender(|context| {
			context.sender.start().unwrap();
			context.assert_sent_messages(vec!["Start"]);
			assert!(!context.sender.is_paused());
		});
	}

	#[test]
	fn start_error() {
		with_view_sender(|mut context| {
			context.drop_receiver();
			assert_eq!(context.sender.start().unwrap_err().to_string(), "Unable to send data");
		});
	}

	#[test]
	fn stop_success() {
		with_view_sender(|context| {
			context.sender.stop().unwrap();
			context.assert_sent_messages(vec!["Stop"]);
			assert!(context.sender.is_paused());
		});
	}

	#[test]
	fn stop_error() {
		with_view_sender(|mut context| {
			context.drop_receiver();
			assert_eq!(context.sender.stop().unwrap_err().to_string(), "Unable to send data");
		});
	}

	#[test]
	fn end_success() {
		with_view_sender(|context| {
			context.sender.end().unwrap();
			context.assert_sent_messages(vec!["Stop", "End"]);
		});
	}

	#[test]
	fn end_error() {
		with_view_sender(|mut context| {
			context.drop_receiver();
			assert_eq!(context.sender.end().unwrap_err().to_string(), "Unable to send data");
		});
	}

	#[test]
	fn scroll_top() {
		with_view_sender(|context| {
			context.sender.scroll_top();
			context.assert_render_action(&["ScrollTop"]);
		});
	}

	#[test]
	fn scroll_bottom() {
		with_view_sender(|context| {
			context.sender.scroll_bottom();
			context.assert_render_action(&["ScrollBottom"]);
		});
	}

	#[test]
	fn scroll_up() {
		with_view_sender(|context| {
			context.sender.scroll_up();
			context.assert_render_action(&["ScrollUp"]);
		});
	}

	#[test]
	fn scroll_down() {
		with_view_sender(|context| {
			context.sender.scroll_down();
			context.assert_render_action(&["ScrollDown"]);
		});
	}

	#[test]
	fn scroll_left() {
		with_view_sender(|context| {
			context.sender.scroll_left();
			context.assert_render_action(&["ScrollLeft"]);
		});
	}

	#[test]
	fn scroll_right() {
		with_view_sender(|context| {
			context.sender.scroll_right();
			context.assert_render_action(&["ScrollRight"]);
		});
	}

	#[test]
	fn scroll_page_up() {
		with_view_sender(|context| {
			context.sender.scroll_page_up();
			context.assert_render_action(&["PageUp"]);
		});
	}

	#[test]
	fn scroll_page_down() {
		with_view_sender(|context| {
			context.sender.scroll_page_down();
			context.assert_render_action(&["PageDown"]);
		});
	}

	#[test]
	fn resize() {
		with_view_sender(|context| {
			context.sender.resize(10, 20);
			context.assert_render_action(&["Resize(10, 20)"]);
		});
	}

	#[test]
	fn render() {
		with_view_sender(|context| {
			context.sender.resize(300, 100);
			context
				.sender
				.render(&ViewData::new(|updater| updater.push_line(ViewLine::from("Foo"))))
				.unwrap();
			assert_eq!(
				render_view_line(context.sender.clone_render_slice().lock().get_lines().first().unwrap()),
				"{Normal}Foo"
			);
		});
	}
}