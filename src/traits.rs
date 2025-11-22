pub trait KeypadInterface {
	type Key: Copy;

	fn read(&mut self) -> Option<Self::Key>;
}
