#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum LineType {
	Cancel,
	Pick,
	Exec,
	Label,
	Merge,
	Reset,
	UpdateRef,
}

impl ToString for LineType {
	fn to_string(&self) -> String {
		match *self {
			Self::Cancel => String::from("<cancel>"),
			Self::Pick => String::from("pick"),
			Self::Exec => String::from("exec"),
			Self::Label => String::from("label"),
			Self::Merge => String::from("merge"),
			Self::Reset => String::from("reset"),
			Self::UpdateRef => String::from("update-ref"),
		}
	}
}

#[cfg(test)]
mod tests {
	use rstest::rstest;

	use super::*;

	#[rstest]
	#[case::cancel(&LineType::Cancel, "<cancel>")]
	#[case::pick(&LineType::Pick, "pick")]
	#[case::exec(&LineType::Exec, "exec")]
	#[case::label(&LineType::Label, "label")]
	#[case::merge(&LineType::Merge, "merge")]
	#[case::reset(&LineType::Reset, "reset")]
	#[case::update_ref(&LineType::UpdateRef, "update-ref")]
	fn to_string(#[case] line_type: &LineType, #[case] expected: &str) {
		assert_eq!(line_type.to_string(), String::from(expected));
	}
}
