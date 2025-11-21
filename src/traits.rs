pub trait KeypadInterface<'a> {
	type Key: 'a;

	fn read(&mut self) -> Option<Self::Key>;
}
