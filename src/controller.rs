pub struct ControllerStack {
    // Stores controllers directly, but will pass out references
    controllers: ~[~TraitController]
}

impl ControllerStack {
	// Pushes a ownable reference of the Controller on to the stack
	pub fn push(&mut self, controller: ~TraitController) {
		self.controllers.push(controller);
	}

	// Used in loops to get the active controller
	// but not manipulate the stack
	//
	// Essentially this method takes an instance of
	// TraitController and passes its lifetime to
	// anything returned
	pub fn peek<'a>(&'a self) -> &'a ~TraitController {
		return &'a self.controllers[self.controllers.len() - 1];
	}
}

pub struct ControllerMessage {
	message: ~str,
}

impl TraitController for ControllerMessage {
	fn construct(&self) {}
	fn game_loop(&self) {
		println!("ControllerMessage: { }", self.message);
	}
	fn destruct(&self) {}
}

pub trait TraitController {
	fn construct(&self);
	fn game_loop(&self);
	fn destruct(&self);
}