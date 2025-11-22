pub trait KeypadInterface {
	type Key;

	fn read(&mut self) -> Option<Self::Key>;
}
