pub struct PauseControl
{
	paused: bool
}

impl PauseControl
{
	pub const fn new() -> Self 
	{
		PauseControl
		{ 
			paused: false,
		}
	}

	pub const fn is_not_paused(&self) -> bool
	{
		!self.paused
	}

	pub fn pause(&mut self)
	{
		self.paused = true;
	}

	pub fn unpause(&mut self)
	{
		self.paused = false;
	}

	pub fn toggle(&mut self)
	{
		self.paused = !self.paused;
	}

	pub const fn is_paused(&self) -> bool
	{
		self.paused
	}
}