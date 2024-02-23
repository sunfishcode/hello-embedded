// Generated by `wit-bindgen` 0.18.0. DO NOT EDIT!
pub mod sketch {
  pub mod embedded {
    
    #[allow(clippy::all)]
    pub mod delay {
      #[used]
      #[doc(hidden)]
      #[cfg(target_arch = "wasm32")]
      static __FORCE_SECTION_REF: fn() = super::super::super::__link_section;
      /// Delay with up to nanosecond precision.
      
      #[derive(Debug)]
      #[repr(transparent)]
      pub struct Delay{
        handle: wit_bindgen::rt::Resource<Delay>,
      }
      
      impl Delay{
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
          Self {
            handle: wit_bindgen::rt::Resource::from_handle(handle),
          }
        }
        
        #[doc(hidden)]
        pub fn into_handle(self) -> u32 {
          wit_bindgen::rt::Resource::into_handle(self.handle)
        }
        
        #[doc(hidden)]
        pub fn handle(&self) -> u32 {
          wit_bindgen::rt::Resource::handle(&self.handle)
        }
      }
      
      
      unsafe impl wit_bindgen::rt::WasmResource for Delay{
        #[inline]
        unsafe fn drop(_handle: u32) {
          #[cfg(not(target_arch = "wasm32"))]
          unreachable!();
          
          #[cfg(target_arch = "wasm32")]
          {
            #[link(wasm_import_module = "sketch:embedded/delay@0.0.0")]
            extern "C" {
              #[link_name = "[resource-drop]delay"]
              fn drop(_: u32);
            }
            
            drop(_handle);
          }
        }
      }
      
      impl Delay {
        #[allow(unused_unsafe, clippy::all)]
        /// Pauses execution for at minimum `ns` nanoseconds. Pause can be
        /// longer if the implementation requires it due to precision/timing
        /// issues.
        pub fn delay_ns(&self,ns: u32,){
          
          #[allow(unused_imports)]
          use wit_bindgen::rt::{alloc, vec::Vec, string::String};
          unsafe {
            
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "sketch:embedded/delay@0.0.0")]
            extern "C" {
              #[link_name = "[method]delay.delay-ns"]
              fn wit_import(_: i32, _: i32, );
            }
            
            #[cfg(not(target_arch = "wasm32"))]
            fn wit_import(_: i32, _: i32, ){ unreachable!() }
            wit_import((self).handle() as i32, wit_bindgen::rt::as_i32(ns));
          }
        }
      }
      
    }
    
    
    #[allow(clippy::all)]
    pub mod digital {
      #[used]
      #[doc(hidden)]
      #[cfg(target_arch = "wasm32")]
      static __FORCE_SECTION_REF: fn() = super::super::super::__link_section;
      /// Operation errors.
      #[repr(u8)]
      #[derive(Clone, Copy, Eq, PartialEq)]
      pub enum ErrorCode {
        /// An error occurred.
        Other,
      }
      impl ErrorCode{
        pub fn name(&self) -> &'static str {
          match self {
            ErrorCode::Other => "other",
          }
        }
        pub fn message(&self) -> &'static str {
          match self {
            ErrorCode::Other => "An error occurred.",
          }
        }
      }
      impl ::core::fmt::Debug for ErrorCode{
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
          f.debug_struct("ErrorCode")
          .field("code", &(*self as i32))
          .field("name", &self.name())
          .field("message", &self.message())
          .finish()
        }
      }
      impl ::core::fmt::Display for ErrorCode{
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
          write!(f, "{} (error {})", self.name(), *self as i32)
        }
      }
      
      #[cfg(feature = "std")]impl std::error::Error for ErrorCode{}
      
      impl ErrorCode{
        pub(crate) unsafe fn _lift(val: u8) -> ErrorCode{
          if !cfg!(debug_assertions) {
            return ::core::mem::transmute(val);
          }
          
          match val {
            0 => ErrorCode::Other,
            
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      
      /// Digital output pin state.
      #[repr(u8)]
      #[derive(Clone, Copy, Eq, PartialEq)]
      pub enum PinState {
        Low,
        High,
      }
      impl ::core::fmt::Debug for PinState {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
          match self {
            PinState::Low => {
              f.debug_tuple("PinState::Low").finish()
            }
            PinState::High => {
              f.debug_tuple("PinState::High").finish()
            }
          }
        }
      }
      
      impl PinState{
        pub(crate) unsafe fn _lift(val: u8) -> PinState{
          if !cfg!(debug_assertions) {
            return ::core::mem::transmute(val);
          }
          
          match val {
            0 => PinState::Low,
            1 => PinState::High,
            
            _ => panic!("invalid enum discriminant"),
          }
        }
      }
      
      /// Single digital input pin.
      
      #[derive(Debug)]
      #[repr(transparent)]
      pub struct InputPin{
        handle: wit_bindgen::rt::Resource<InputPin>,
      }
      
      impl InputPin{
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
          Self {
            handle: wit_bindgen::rt::Resource::from_handle(handle),
          }
        }
        
        #[doc(hidden)]
        pub fn into_handle(self) -> u32 {
          wit_bindgen::rt::Resource::into_handle(self.handle)
        }
        
        #[doc(hidden)]
        pub fn handle(&self) -> u32 {
          wit_bindgen::rt::Resource::handle(&self.handle)
        }
      }
      
      
      unsafe impl wit_bindgen::rt::WasmResource for InputPin{
        #[inline]
        unsafe fn drop(_handle: u32) {
          #[cfg(not(target_arch = "wasm32"))]
          unreachable!();
          
          #[cfg(target_arch = "wasm32")]
          {
            #[link(wasm_import_module = "sketch:embedded/digital@0.0.0")]
            extern "C" {
              #[link_name = "[resource-drop]input-pin"]
              fn drop(_: u32);
            }
            
            drop(_handle);
          }
        }
      }
      
      /// Single digital input pin.
      
      #[derive(Debug)]
      #[repr(transparent)]
      pub struct OutputPin{
        handle: wit_bindgen::rt::Resource<OutputPin>,
      }
      
      impl OutputPin{
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
          Self {
            handle: wit_bindgen::rt::Resource::from_handle(handle),
          }
        }
        
        #[doc(hidden)]
        pub fn into_handle(self) -> u32 {
          wit_bindgen::rt::Resource::into_handle(self.handle)
        }
        
        #[doc(hidden)]
        pub fn handle(&self) -> u32 {
          wit_bindgen::rt::Resource::handle(&self.handle)
        }
      }
      
      
      unsafe impl wit_bindgen::rt::WasmResource for OutputPin{
        #[inline]
        unsafe fn drop(_handle: u32) {
          #[cfg(not(target_arch = "wasm32"))]
          unreachable!();
          
          #[cfg(target_arch = "wasm32")]
          {
            #[link(wasm_import_module = "sketch:embedded/digital@0.0.0")]
            extern "C" {
              #[link_name = "[resource-drop]output-pin"]
              fn drop(_: u32);
            }
            
            drop(_handle);
          }
        }
      }
      
      /// Push-pull output pin that can read its output state.
      
      #[derive(Debug)]
      #[repr(transparent)]
      pub struct StatefulOutputPin{
        handle: wit_bindgen::rt::Resource<StatefulOutputPin>,
      }
      
      impl StatefulOutputPin{
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
          Self {
            handle: wit_bindgen::rt::Resource::from_handle(handle),
          }
        }
        
        #[doc(hidden)]
        pub fn into_handle(self) -> u32 {
          wit_bindgen::rt::Resource::into_handle(self.handle)
        }
        
        #[doc(hidden)]
        pub fn handle(&self) -> u32 {
          wit_bindgen::rt::Resource::handle(&self.handle)
        }
      }
      
      
      unsafe impl wit_bindgen::rt::WasmResource for StatefulOutputPin{
        #[inline]
        unsafe fn drop(_handle: u32) {
          #[cfg(not(target_arch = "wasm32"))]
          unreachable!();
          
          #[cfg(target_arch = "wasm32")]
          {
            #[link(wasm_import_module = "sketch:embedded/digital@0.0.0")]
            extern "C" {
              #[link_name = "[resource-drop]stateful-output-pin"]
              fn drop(_: u32);
            }
            
            drop(_handle);
          }
        }
      }
      
      impl InputPin {
        #[allow(unused_unsafe, clippy::all)]
        /// Is the input pin low?
        pub fn is_low(&self,) -> Result<bool,ErrorCode>{
          
          #[allow(unused_imports)]
          use wit_bindgen::rt::{alloc, vec::Vec, string::String};
          unsafe {
            
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "sketch:embedded/digital@0.0.0")]
            extern "C" {
              #[link_name = "[method]input-pin.is-low"]
              fn wit_import(_: i32, _: i32, );
            }
            
            #[cfg(not(target_arch = "wasm32"))]
            fn wit_import(_: i32, _: i32, ){ unreachable!() }
            wit_import((self).handle() as i32, ptr0);
            let l1 = i32::from(*((ptr0 + 0) as *const u8));
            match l1 {
              0 => {
                let e = {
                  let l2 = i32::from(*((ptr0 + 1) as *const u8));
                  
                  wit_bindgen::rt::bool_lift(l2 as u8)
                };
                Ok(e)
              }
              1 => {
                let e = {
                  let l3 = i32::from(*((ptr0 + 1) as *const u8));
                  
                  ErrorCode::_lift(l3 as u8)
                };
                Err(e)
              }
              _ => wit_bindgen::rt::invalid_enum_discriminant(),
            }
          }
        }
      }
      impl InputPin {
        #[allow(unused_unsafe, clippy::all)]
        /// Is the input pin high?
        pub fn is_high(&self,) -> Result<bool,ErrorCode>{
          
          #[allow(unused_imports)]
          use wit_bindgen::rt::{alloc, vec::Vec, string::String};
          unsafe {
            
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "sketch:embedded/digital@0.0.0")]
            extern "C" {
              #[link_name = "[method]input-pin.is-high"]
              fn wit_import(_: i32, _: i32, );
            }
            
            #[cfg(not(target_arch = "wasm32"))]
            fn wit_import(_: i32, _: i32, ){ unreachable!() }
            wit_import((self).handle() as i32, ptr0);
            let l1 = i32::from(*((ptr0 + 0) as *const u8));
            match l1 {
              0 => {
                let e = {
                  let l2 = i32::from(*((ptr0 + 1) as *const u8));
                  
                  wit_bindgen::rt::bool_lift(l2 as u8)
                };
                Ok(e)
              }
              1 => {
                let e = {
                  let l3 = i32::from(*((ptr0 + 1) as *const u8));
                  
                  ErrorCode::_lift(l3 as u8)
                };
                Err(e)
              }
              _ => wit_bindgen::rt::invalid_enum_discriminant(),
            }
          }
        }
      }
      impl InputPin {
        #[allow(unused_unsafe, clippy::all)]
        /// Wait until the pin is high. If it is already high, resolve
        /// immediately.
        pub fn wait_for_high(&self,) -> Result<(),ErrorCode>{
          
          #[allow(unused_imports)]
          use wit_bindgen::rt::{alloc, vec::Vec, string::String};
          unsafe {
            
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "sketch:embedded/digital@0.0.0")]
            extern "C" {
              #[link_name = "[method]input-pin.wait-for-high"]
              fn wit_import(_: i32, _: i32, );
            }
            
            #[cfg(not(target_arch = "wasm32"))]
            fn wit_import(_: i32, _: i32, ){ unreachable!() }
            wit_import((self).handle() as i32, ptr0);
            let l1 = i32::from(*((ptr0 + 0) as *const u8));
            match l1 {
              0 => {
                let e = ();
                Ok(e)
              }
              1 => {
                let e = {
                  let l2 = i32::from(*((ptr0 + 1) as *const u8));
                  
                  ErrorCode::_lift(l2 as u8)
                };
                Err(e)
              }
              _ => wit_bindgen::rt::invalid_enum_discriminant(),
            }
          }
        }
      }
      impl InputPin {
        #[allow(unused_unsafe, clippy::all)]
        /// Wait until the pin is low. If it is already low, resolve
        /// immediately.
        pub fn wait_for_low(&self,) -> Result<(),ErrorCode>{
          
          #[allow(unused_imports)]
          use wit_bindgen::rt::{alloc, vec::Vec, string::String};
          unsafe {
            
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "sketch:embedded/digital@0.0.0")]
            extern "C" {
              #[link_name = "[method]input-pin.wait-for-low"]
              fn wit_import(_: i32, _: i32, );
            }
            
            #[cfg(not(target_arch = "wasm32"))]
            fn wit_import(_: i32, _: i32, ){ unreachable!() }
            wit_import((self).handle() as i32, ptr0);
            let l1 = i32::from(*((ptr0 + 0) as *const u8));
            match l1 {
              0 => {
                let e = ();
                Ok(e)
              }
              1 => {
                let e = {
                  let l2 = i32::from(*((ptr0 + 1) as *const u8));
                  
                  ErrorCode::_lift(l2 as u8)
                };
                Err(e)
              }
              _ => wit_bindgen::rt::invalid_enum_discriminant(),
            }
          }
        }
      }
      impl InputPin {
        #[allow(unused_unsafe, clippy::all)]
        /// Wait for the pin to undergo a transition from low to high.
        /// 
        /// If the pin is already high, this does *not* resolve immediately,
        /// it’ll wait for the pin to go low and then high again.
        pub fn wait_for_rising_edge(&self,) -> Result<(),ErrorCode>{
          
          #[allow(unused_imports)]
          use wit_bindgen::rt::{alloc, vec::Vec, string::String};
          unsafe {
            
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "sketch:embedded/digital@0.0.0")]
            extern "C" {
              #[link_name = "[method]input-pin.wait-for-rising-edge"]
              fn wit_import(_: i32, _: i32, );
            }
            
            #[cfg(not(target_arch = "wasm32"))]
            fn wit_import(_: i32, _: i32, ){ unreachable!() }
            wit_import((self).handle() as i32, ptr0);
            let l1 = i32::from(*((ptr0 + 0) as *const u8));
            match l1 {
              0 => {
                let e = ();
                Ok(e)
              }
              1 => {
                let e = {
                  let l2 = i32::from(*((ptr0 + 1) as *const u8));
                  
                  ErrorCode::_lift(l2 as u8)
                };
                Err(e)
              }
              _ => wit_bindgen::rt::invalid_enum_discriminant(),
            }
          }
        }
      }
      impl InputPin {
        #[allow(unused_unsafe, clippy::all)]
        /// Wait for the pin to undergo a transition from high to low.
        /// 
        /// If the pin is already low, this does *not* return immediately,
        /// it’ll wait for the pin to go high and then low again.
        pub fn wait_for_falling_edge(&self,) -> Result<(),ErrorCode>{
          
          #[allow(unused_imports)]
          use wit_bindgen::rt::{alloc, vec::Vec, string::String};
          unsafe {
            
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "sketch:embedded/digital@0.0.0")]
            extern "C" {
              #[link_name = "[method]input-pin.wait-for-falling-edge"]
              fn wit_import(_: i32, _: i32, );
            }
            
            #[cfg(not(target_arch = "wasm32"))]
            fn wit_import(_: i32, _: i32, ){ unreachable!() }
            wit_import((self).handle() as i32, ptr0);
            let l1 = i32::from(*((ptr0 + 0) as *const u8));
            match l1 {
              0 => {
                let e = ();
                Ok(e)
              }
              1 => {
                let e = {
                  let l2 = i32::from(*((ptr0 + 1) as *const u8));
                  
                  ErrorCode::_lift(l2 as u8)
                };
                Err(e)
              }
              _ => wit_bindgen::rt::invalid_enum_discriminant(),
            }
          }
        }
      }
      impl InputPin {
        #[allow(unused_unsafe, clippy::all)]
        /// Wait for the pin to undergo any transition, i.e low to high OR high
        /// to low.
        pub fn wait_for_any_edge(&self,) -> Result<(),ErrorCode>{
          
          #[allow(unused_imports)]
          use wit_bindgen::rt::{alloc, vec::Vec, string::String};
          unsafe {
            
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "sketch:embedded/digital@0.0.0")]
            extern "C" {
              #[link_name = "[method]input-pin.wait-for-any-edge"]
              fn wit_import(_: i32, _: i32, );
            }
            
            #[cfg(not(target_arch = "wasm32"))]
            fn wit_import(_: i32, _: i32, ){ unreachable!() }
            wit_import((self).handle() as i32, ptr0);
            let l1 = i32::from(*((ptr0 + 0) as *const u8));
            match l1 {
              0 => {
                let e = ();
                Ok(e)
              }
              1 => {
                let e = {
                  let l2 = i32::from(*((ptr0 + 1) as *const u8));
                  
                  ErrorCode::_lift(l2 as u8)
                };
                Err(e)
              }
              _ => wit_bindgen::rt::invalid_enum_discriminant(),
            }
          }
        }
      }
      impl OutputPin {
        #[allow(unused_unsafe, clippy::all)]
        /// Drives the pin low.
        pub fn set_low(&self,) -> Result<(),ErrorCode>{
          
          #[allow(unused_imports)]
          use wit_bindgen::rt::{alloc, vec::Vec, string::String};
          unsafe {
            
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "sketch:embedded/digital@0.0.0")]
            extern "C" {
              #[link_name = "[method]output-pin.set-low"]
              fn wit_import(_: i32, _: i32, );
            }
            
            #[cfg(not(target_arch = "wasm32"))]
            fn wit_import(_: i32, _: i32, ){ unreachable!() }
            wit_import((self).handle() as i32, ptr0);
            let l1 = i32::from(*((ptr0 + 0) as *const u8));
            match l1 {
              0 => {
                let e = ();
                Ok(e)
              }
              1 => {
                let e = {
                  let l2 = i32::from(*((ptr0 + 1) as *const u8));
                  
                  ErrorCode::_lift(l2 as u8)
                };
                Err(e)
              }
              _ => wit_bindgen::rt::invalid_enum_discriminant(),
            }
          }
        }
      }
      impl OutputPin {
        #[allow(unused_unsafe, clippy::all)]
        /// Drives the pin high.
        pub fn set_high(&self,) -> Result<(),ErrorCode>{
          
          #[allow(unused_imports)]
          use wit_bindgen::rt::{alloc, vec::Vec, string::String};
          unsafe {
            
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "sketch:embedded/digital@0.0.0")]
            extern "C" {
              #[link_name = "[method]output-pin.set-high"]
              fn wit_import(_: i32, _: i32, );
            }
            
            #[cfg(not(target_arch = "wasm32"))]
            fn wit_import(_: i32, _: i32, ){ unreachable!() }
            wit_import((self).handle() as i32, ptr0);
            let l1 = i32::from(*((ptr0 + 0) as *const u8));
            match l1 {
              0 => {
                let e = ();
                Ok(e)
              }
              1 => {
                let e = {
                  let l2 = i32::from(*((ptr0 + 1) as *const u8));
                  
                  ErrorCode::_lift(l2 as u8)
                };
                Err(e)
              }
              _ => wit_bindgen::rt::invalid_enum_discriminant(),
            }
          }
        }
      }
      impl OutputPin {
        #[allow(unused_unsafe, clippy::all)]
        /// Drives the pin high or low depending on the provided value.
        pub fn set_state(&self,state: PinState,) -> Result<(),ErrorCode>{
          
          #[allow(unused_imports)]
          use wit_bindgen::rt::{alloc, vec::Vec, string::String};
          unsafe {
            
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "sketch:embedded/digital@0.0.0")]
            extern "C" {
              #[link_name = "[method]output-pin.set-state"]
              fn wit_import(_: i32, _: i32, _: i32, );
            }
            
            #[cfg(not(target_arch = "wasm32"))]
            fn wit_import(_: i32, _: i32, _: i32, ){ unreachable!() }
            wit_import((self).handle() as i32, state.clone() as i32, ptr0);
            let l1 = i32::from(*((ptr0 + 0) as *const u8));
            match l1 {
              0 => {
                let e = ();
                Ok(e)
              }
              1 => {
                let e = {
                  let l2 = i32::from(*((ptr0 + 1) as *const u8));
                  
                  ErrorCode::_lift(l2 as u8)
                };
                Err(e)
              }
              _ => wit_bindgen::rt::invalid_enum_discriminant(),
            }
          }
        }
      }
      impl StatefulOutputPin {
        #[allow(unused_unsafe, clippy::all)]
        /// Is the pin in drive high mode?
        pub fn is_set_high(&self,) -> Result<bool,ErrorCode>{
          
          #[allow(unused_imports)]
          use wit_bindgen::rt::{alloc, vec::Vec, string::String};
          unsafe {
            
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "sketch:embedded/digital@0.0.0")]
            extern "C" {
              #[link_name = "[method]stateful-output-pin.is-set-high"]
              fn wit_import(_: i32, _: i32, );
            }
            
            #[cfg(not(target_arch = "wasm32"))]
            fn wit_import(_: i32, _: i32, ){ unreachable!() }
            wit_import((self).handle() as i32, ptr0);
            let l1 = i32::from(*((ptr0 + 0) as *const u8));
            match l1 {
              0 => {
                let e = {
                  let l2 = i32::from(*((ptr0 + 1) as *const u8));
                  
                  wit_bindgen::rt::bool_lift(l2 as u8)
                };
                Ok(e)
              }
              1 => {
                let e = {
                  let l3 = i32::from(*((ptr0 + 1) as *const u8));
                  
                  ErrorCode::_lift(l3 as u8)
                };
                Err(e)
              }
              _ => wit_bindgen::rt::invalid_enum_discriminant(),
            }
          }
        }
      }
      impl StatefulOutputPin {
        #[allow(unused_unsafe, clippy::all)]
        /// Is the pin in drive low mode?
        pub fn is_set_low(&self,) -> Result<bool,ErrorCode>{
          
          #[allow(unused_imports)]
          use wit_bindgen::rt::{alloc, vec::Vec, string::String};
          unsafe {
            
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "sketch:embedded/digital@0.0.0")]
            extern "C" {
              #[link_name = "[method]stateful-output-pin.is-set-low"]
              fn wit_import(_: i32, _: i32, );
            }
            
            #[cfg(not(target_arch = "wasm32"))]
            fn wit_import(_: i32, _: i32, ){ unreachable!() }
            wit_import((self).handle() as i32, ptr0);
            let l1 = i32::from(*((ptr0 + 0) as *const u8));
            match l1 {
              0 => {
                let e = {
                  let l2 = i32::from(*((ptr0 + 1) as *const u8));
                  
                  wit_bindgen::rt::bool_lift(l2 as u8)
                };
                Ok(e)
              }
              1 => {
                let e = {
                  let l3 = i32::from(*((ptr0 + 1) as *const u8));
                  
                  ErrorCode::_lift(l3 as u8)
                };
                Err(e)
              }
              _ => wit_bindgen::rt::invalid_enum_discriminant(),
            }
          }
        }
      }
      impl StatefulOutputPin {
        #[allow(unused_unsafe, clippy::all)]
        /// Toggle pin output.
        pub fn toggle(&self,) -> Result<(),ErrorCode>{
          
          #[allow(unused_imports)]
          use wit_bindgen::rt::{alloc, vec::Vec, string::String};
          unsafe {
            
            #[repr(align(1))]
            struct RetArea([u8; 2]);
            let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "sketch:embedded/digital@0.0.0")]
            extern "C" {
              #[link_name = "[method]stateful-output-pin.toggle"]
              fn wit_import(_: i32, _: i32, );
            }
            
            #[cfg(not(target_arch = "wasm32"))]
            fn wit_import(_: i32, _: i32, ){ unreachable!() }
            wit_import((self).handle() as i32, ptr0);
            let l1 = i32::from(*((ptr0 + 0) as *const u8));
            match l1 {
              0 => {
                let e = ();
                Ok(e)
              }
              1 => {
                let e = {
                  let l2 = i32::from(*((ptr0 + 1) as *const u8));
                  
                  ErrorCode::_lift(l2 as u8)
                };
                Err(e)
              }
              _ => wit_bindgen::rt::invalid_enum_discriminant(),
            }
          }
        }
      }
      
    }
    
  }
}
pub mod exports {
  pub mod sketch {
    pub mod embedded {
      
      #[allow(clippy::all)]
      pub mod run {
        #[used]
        #[doc(hidden)]
        #[cfg(target_arch = "wasm32")]
        static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_section;
        pub type OutputPin = super::super::super::super::sketch::embedded::digital::OutputPin;
        pub type Delay = super::super::super::super::sketch::embedded::delay::Delay;
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "sketch:embedded/run@0.0.0#run"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_run(arg0: i32,arg1: i32,) {
            #[allow(unused_imports)]
            use wit_bindgen::rt::{alloc, vec::Vec, string::String};
            
            // Before executing any other code, use this function to run all static
            // constructors, if they have not yet been run. This is a hack required
            // to work around wasi-libc ctors calling import functions to initialize
            // the environment.
            //
            // This functionality will be removed once rust 1.69.0 is stable, at which
            // point wasi-libc will no longer have this behavior.
            //
            // See
            // https://github.com/bytecodealliance/preview2-prototyping/issues/99
            // for more details.
            #[cfg(target_arch="wasm32")]
            wit_bindgen::rt::run_ctors_once();
            
            <_GuestImpl as Guest>::run(super::super::super::super::sketch::embedded::digital::OutputPin::from_handle(arg0 as u32), super::super::super::super::sketch::embedded::delay::Delay::from_handle(arg1 as u32));
          }
        };
        use super::super::super::super::super::Component as _GuestImpl;
        pub trait Guest {
          /// Start the program, with an output pin and a delay mechanism.
          fn run(led: OutputPin,delay: Delay,);
        }
        
      }
      
    }
  }
}

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:blink"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 1091] = [0, 97, 115, 109, 13, 0, 1, 0, 0, 25, 22, 119, 105, 116, 45, 99, 111, 109, 112, 111, 110, 101, 110, 116, 45, 101, 110, 99, 111, 100, 105, 110, 103, 4, 0, 7, 200, 7, 1, 65, 2, 1, 65, 8, 1, 66, 4, 4, 0, 5, 100, 101, 108, 97, 121, 3, 1, 1, 104, 0, 1, 64, 2, 4, 115, 101, 108, 102, 1, 2, 110, 115, 121, 1, 0, 4, 0, 22, 91, 109, 101, 116, 104, 111, 100, 93, 100, 101, 108, 97, 121, 46, 100, 101, 108, 97, 121, 45, 110, 115, 1, 2, 3, 1, 27, 115, 107, 101, 116, 99, 104, 58, 101, 109, 98, 101, 100, 100, 101, 100, 47, 100, 101, 108, 97, 121, 64, 48, 46, 48, 46, 48, 5, 0, 1, 66, 31, 1, 109, 1, 5, 111, 116, 104, 101, 114, 4, 0, 10, 101, 114, 114, 111, 114, 45, 99, 111, 100, 101, 3, 0, 0, 1, 109, 2, 3, 108, 111, 119, 4, 104, 105, 103, 104, 4, 0, 9, 112, 105, 110, 45, 115, 116, 97, 116, 101, 3, 0, 2, 4, 0, 9, 105, 110, 112, 117, 116, 45, 112, 105, 110, 3, 1, 4, 0, 10, 111, 117, 116, 112, 117, 116, 45, 112, 105, 110, 3, 1, 4, 0, 19, 115, 116, 97, 116, 101, 102, 117, 108, 45, 111, 117, 116, 112, 117, 116, 45, 112, 105, 110, 3, 1, 1, 104, 4, 1, 106, 1, 127, 1, 1, 1, 64, 1, 4, 115, 101, 108, 102, 7, 0, 8, 4, 0, 24, 91, 109, 101, 116, 104, 111, 100, 93, 105, 110, 112, 117, 116, 45, 112, 105, 110, 46, 105, 115, 45, 108, 111, 119, 1, 9, 4, 0, 25, 91, 109, 101, 116, 104, 111, 100, 93, 105, 110, 112, 117, 116, 45, 112, 105, 110, 46, 105, 115, 45, 104, 105, 103, 104, 1, 9, 1, 106, 0, 1, 1, 1, 64, 1, 4, 115, 101, 108, 102, 7, 0, 10, 4, 0, 31, 91, 109, 101, 116, 104, 111, 100, 93, 105, 110, 112, 117, 116, 45, 112, 105, 110, 46, 119, 97, 105, 116, 45, 102, 111, 114, 45, 104, 105, 103, 104, 1, 11, 4, 0, 30, 91, 109, 101, 116, 104, 111, 100, 93, 105, 110, 112, 117, 116, 45, 112, 105, 110, 46, 119, 97, 105, 116, 45, 102, 111, 114, 45, 108, 111, 119, 1, 11, 4, 0, 38, 91, 109, 101, 116, 104, 111, 100, 93, 105, 110, 112, 117, 116, 45, 112, 105, 110, 46, 119, 97, 105, 116, 45, 102, 111, 114, 45, 114, 105, 115, 105, 110, 103, 45, 101, 100, 103, 101, 1, 11, 4, 0, 39, 91, 109, 101, 116, 104, 111, 100, 93, 105, 110, 112, 117, 116, 45, 112, 105, 110, 46, 119, 97, 105, 116, 45, 102, 111, 114, 45, 102, 97, 108, 108, 105, 110, 103, 45, 101, 100, 103, 101, 1, 11, 4, 0, 35, 91, 109, 101, 116, 104, 111, 100, 93, 105, 110, 112, 117, 116, 45, 112, 105, 110, 46, 119, 97, 105, 116, 45, 102, 111, 114, 45, 97, 110, 121, 45, 101, 100, 103, 101, 1, 11, 1, 104, 5, 1, 64, 1, 4, 115, 101, 108, 102, 12, 0, 10, 4, 0, 26, 91, 109, 101, 116, 104, 111, 100, 93, 111, 117, 116, 112, 117, 116, 45, 112, 105, 110, 46, 115, 101, 116, 45, 108, 111, 119, 1, 13, 4, 0, 27, 91, 109, 101, 116, 104, 111, 100, 93, 111, 117, 116, 112, 117, 116, 45, 112, 105, 110, 46, 115, 101, 116, 45, 104, 105, 103, 104, 1, 13, 1, 64, 2, 4, 115, 101, 108, 102, 12, 5, 115, 116, 97, 116, 101, 3, 0, 10, 4, 0, 28, 91, 109, 101, 116, 104, 111, 100, 93, 111, 117, 116, 112, 117, 116, 45, 112, 105, 110, 46, 115, 101, 116, 45, 115, 116, 97, 116, 101, 1, 14, 1, 104, 6, 1, 64, 1, 4, 115, 101, 108, 102, 15, 0, 8, 4, 0, 39, 91, 109, 101, 116, 104, 111, 100, 93, 115, 116, 97, 116, 101, 102, 117, 108, 45, 111, 117, 116, 112, 117, 116, 45, 112, 105, 110, 46, 105, 115, 45, 115, 101, 116, 45, 104, 105, 103, 104, 1, 16, 4, 0, 38, 91, 109, 101, 116, 104, 111, 100, 93, 115, 116, 97, 116, 101, 102, 117, 108, 45, 111, 117, 116, 112, 117, 116, 45, 112, 105, 110, 46, 105, 115, 45, 115, 101, 116, 45, 108, 111, 119, 1, 16, 1, 64, 1, 4, 115, 101, 108, 102, 15, 0, 10, 4, 0, 34, 91, 109, 101, 116, 104, 111, 100, 93, 115, 116, 97, 116, 101, 102, 117, 108, 45, 111, 117, 116, 112, 117, 116, 45, 112, 105, 110, 46, 116, 111, 103, 103, 108, 101, 1, 17, 3, 1, 29, 115, 107, 101, 116, 99, 104, 58, 101, 109, 98, 101, 100, 100, 101, 100, 47, 100, 105, 103, 105, 116, 97, 108, 64, 48, 46, 48, 46, 48, 5, 1, 2, 3, 0, 1, 10, 111, 117, 116, 112, 117, 116, 45, 112, 105, 110, 2, 3, 0, 0, 5, 100, 101, 108, 97, 121, 1, 66, 8, 2, 3, 2, 1, 2, 4, 0, 10, 111, 117, 116, 112, 117, 116, 45, 112, 105, 110, 3, 0, 0, 2, 3, 2, 1, 3, 4, 0, 5, 100, 101, 108, 97, 121, 3, 0, 2, 1, 105, 1, 1, 105, 3, 1, 64, 2, 3, 108, 101, 100, 4, 5, 100, 101, 108, 97, 121, 5, 1, 0, 4, 0, 3, 114, 117, 110, 1, 6, 4, 1, 25, 115, 107, 101, 116, 99, 104, 58, 101, 109, 98, 101, 100, 100, 101, 100, 47, 114, 117, 110, 64, 48, 46, 48, 46, 48, 5, 4, 4, 1, 27, 115, 107, 101, 116, 99, 104, 58, 101, 109, 98, 101, 100, 100, 101, 100, 47, 98, 108, 105, 110, 107, 64, 48, 46, 48, 46, 48, 4, 0, 11, 11, 1, 0, 5, 98, 108, 105, 110, 107, 3, 0, 0, 0, 70, 9, 112, 114, 111, 100, 117, 99, 101, 114, 115, 1, 12, 112, 114, 111, 99, 101, 115, 115, 101, 100, 45, 98, 121, 2, 13, 119, 105, 116, 45, 99, 111, 109, 112, 111, 110, 101, 110, 116, 6, 48, 46, 50, 49, 46, 48, 16, 119, 105, 116, 45, 98, 105, 110, 100, 103, 101, 110, 45, 114, 117, 115, 116, 6, 48, 46, 49, 56, 46, 48];

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}
