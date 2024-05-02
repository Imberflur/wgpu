//! FXC doesn't allow `continue` in switches, so we track `continue` statements with a bool variable
//! and check it after the switch.
//!
//! See <https://github.com/gfx-rs/wgpu/issues/4485>
//!
//! Also for hlsl and glsl we write degenerate single body switches as `do {} while(false);` loops
//! since some consumers (e.g. FXC) have bugs handling this type of switch. In this scenario, we
//! need to forward continue statements to the outer loop.
//!
//! See <https://github.com/gfx-rs/wgpu/issues/4514>

enum Nesting {
    /// Currently nested in at least one loop.
    ///
    /// `continue` should apply to the current loop.
    ///
    /// * When entering a nested switch, add a `Forward` state to the stack.
    /// * When entering an inner loop, increment the depth.
    /// * When exiting the loop, decrement the depth (and pop if it reaches 0).
    Loop { depth: u32 },
    /// Currently nested in at least one switch that needs to forward continues.
    ///
    /// This includes switches transformed into `do {} while(false)` loops, but doesn't need to
    /// include regular switches in backends that can support `continue` within switches.
    ///
    /// `continue` should be forwarded to surrounding loop.
    ///
    /// * When entering a nested loop, add a `Loop` state to the stack.
    /// * When entering an inner switch, increment the depth.
    /// * When exiting the switch, decrement the depth (and pop if it reaches 0).
    Switch { depth: u32, variable_id: u32 },
}

pub(crate) enum ExitControlFlow {
    None,
    /// Emit `if (continue_variable) { continue; }`
    Continue {
        variable_id: u32,
    },
    /// Emit `if (continue_variable) { break; }`
    ///
    /// Used when nesting switches.
    ///
    /// Outer switch will be exited by the break, and then its associated check will check this
    /// same variable and see that it is set.
    Break {
        variable_id: u32,
    },
}

/// Utility for tracking nesting of loops and switches to orchestrate forwarding of continue
/// statements inside of a switch to the enclosing loop.
///
/// See [module docs](self) for why we need this.
#[derive(Default)]
pub(crate) struct ContinueCtx {
    stack: Vec<Nesting>,
    next_id: u32,
}

impl ContinueCtx {
    /// Resets internal state completely including the ID generation used for unique variable
    /// names.
    ///
    /// Use this to reuse memory between writing sessions.
    pub fn clear(&mut self) {
        self.next_id = 0;
        self.stack.clear();
    }

    /// Updates internal state to record entering a loop.
    pub fn enter_loop(&mut self) {
        match self.stack.last_mut() {
            None | Some(&mut Nesting::Switch { .. }) => {
                self.stack.push(Nesting::Loop { depth: 1 });
            }
            Some(&mut Nesting::Loop { ref mut depth }) => *depth += 1,
        }
    }

    /// Updates internal state to record exiting a loop.
    pub fn exit_loop(&mut self) {
        match self.stack.last_mut() {
            None => {
                log::error!("Unexpected empty stack when exiting loop");
            }
            Some(&mut Nesting::Loop { ref mut depth }) => {
                *depth -= 1;
                if *depth == 0 {
                    self.stack.pop();
                }
            }
            Some(&mut Nesting::Switch { .. }) => {
                log::error!("Unexpected switch state when exiting loop");
            }
        }
    }

    /// Updates internal state and returns `Some(variable_id)` if nested in a loop and a new
    /// variable needs to be declared for forwarding continues.
    ///
    /// `variable_id` can be used to derive a unique variable name.
    pub fn enter_switch(&mut self) -> Option<u32> {
        match self.stack.last_mut() {
            // If stack is empty we are not in loop. So we need a variable for forwarding continue
            // statements when writing a switch.
            None => None,
            Some(&mut Nesting::Loop { .. }) => {
                let variable_id = self.next_id;
                // Always increment to avoid conflicting variable names from adjacent switches.
                self.next_id += 1;
                self.stack.push(Nesting::Switch {
                    depth: 1,
                    variable_id,
                });
                Some(variable_id)
            }
            Some(&mut Nesting::Switch { ref mut depth, .. }) => {
                *depth += 1;
                // We already have a variable we can use.
                None
            }
        }
    }

    /// Updates internal state and returns whether this switch needs to be followed by a statement
    /// to forward continues.
    pub fn exit_switch(&mut self) -> ExitControlFlow {
        match self.stack.last_mut() {
            None => ExitControlFlow::None,
            Some(&mut Nesting::Loop { .. }) => {
                log::error!("Unexpected loop state when exiting switch");
                ExitControlFlow::None
            }
            Some(&mut Nesting::Switch {
                ref mut depth,
                variable_id,
            }) => {
                *depth -= 1;
                if *depth == 0 {
                    self.stack.pop();
                    ExitControlFlow::Continue { variable_id }
                } else {
                    ExitControlFlow::Break { variable_id }
                }
            }
        }
    }

    /// Checks if a continue statement can be emitted directly (i.e. not in a switch) or if it
    /// needs to be forwarded via setting a variable to `true` and breaking out of the switch.
    pub fn needs_forwarding(&self) -> Option<u32> {
        if let Some(&Nesting::Switch { variable_id, .. }) = self.stack.last() {
            Some(variable_id)
        } else {
            None
        }
    }
}
