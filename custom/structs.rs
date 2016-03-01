use std::convert::Into;
use core::{ContextProperty, ContextInfoOrPropertiesPointerType as PropKind, PlatformId};
use util;
use cl_h;

/// Context properties list.
///
/// [MINIMALLY TESTED]
///
/// TODO: Check for duplicate property assignments.
#[derive(Clone, Debug)]
pub struct ContextProperties(Vec<ContextProperty>);

impl ContextProperties {
	/// Returns an empty new list of context properties
	pub fn new() -> ContextProperties {
		ContextProperties(Vec::with_capacity(4))
	}

	/// Specifies a platform (builder-style).
	pub fn platform<P: Into<PlatformId>>(mut self, platform: P) -> ContextProperties {
		self.0.push(ContextProperty::Platform(platform.into()));
		self
	}

	/// Specifies whether the user is responsible for synchronization between
	/// OpenCL and other APIs (builder-style).
	pub fn interop_user_sync(mut self, sync: bool) -> ContextProperties {
		self.0.push(ContextProperty::InteropUserSync(sync));
		self
	}

	/// Pushes a `ContextProperty` onto this list of properties.
	pub fn and(mut self, prop: ContextProperty) -> ContextProperties {
		self.0.push(prop);
		self
	}

	/// Returns a platform id or none.
	pub fn get_platform(&self) -> Option<PlatformId> {
		let mut platform = None;

		for prop in self.0.iter() {
			if let &ContextProperty::Platform(ref plat) = prop {
				platform = Some(plat.clone());
			}
		}

		platform
	} 

	/// [UNTESTED: Not properly tested]
	/// Converts this list into a packed-byte representation as specified
	/// [here](https://www.khronos.org/registry/cl/sdk/1.2/docs/man/xhtml/clCreateContext.html).
	///
	/// TODO: Evaluate cleaner ways to do this.
	pub fn to_bytes(&self) -> Vec<u8> {
		let mut bytes = Vec::with_capacity(128);

		unsafe { 
			// For each property:
			for prop in self.0.iter() {
				// Convert both the kind of property (a u32) and the value (variable type/size) 
				// into just a core byte vector (Vec<u8>):
				let (kind, val) = match prop {
					&ContextProperty::Platform(ref platform_id_core) => (
						util::into_bytes(PropKind::Platform as cl_h::cl_uint),
						util::into_bytes(platform_id_core.as_ptr() as cl_h::cl_platform_id) 
					),
				    &ContextProperty::InteropUserSync(sync) => (
				    	util::into_bytes(PropKind::InteropUserSync as cl_h::cl_uint),
				    	util::into_bytes(sync as cl_h::cl_bool)
			    	),
				    _ => continue,
				};

				// Property Kind Enum:
				bytes.extend_from_slice(&kind);
				// 32 bits of padding:
				bytes.extend_from_slice(&util::into_bytes(0 as u32));
				// Value:
				bytes.extend_from_slice(&val);
				// 32 bits of padding:
				bytes.extend_from_slice(&util::into_bytes(0 as u32));
			}

			// Add a terminating 0:
			bytes.extend_from_slice(&util::into_bytes(0 as usize));
		}

		bytes.shrink_to_fit();
		bytes
	}
}

impl Into<Vec<ContextProperty>> for ContextProperties {
	fn into(self) -> Vec<ContextProperty> {
		self.0
	}
}

// pub enum ContextInfoOrPropertiesPointerType {
//     Platform = cl_h::CL_CONTEXT_PLATFORM as isize,
//     InteropUserSync = cl_h::CL_CONTEXT_INTEROP_USER_SYNC as isize,
// }

impl Into<Vec<u8>> for ContextProperties {
	fn into(self) -> Vec<u8> {
		self.to_bytes()
	}
}
