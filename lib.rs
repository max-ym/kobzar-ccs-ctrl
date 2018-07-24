/// Basic elements to make up the network operation environment.
pub mod base;

use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use base::*;

/// Master controller of CCS network.
pub struct Master {

    /// Core object controller of this network.
    core_obj_h  : ObjectHandle,

    /// Interfaces registered in the controller.
    interfaces  : InterfaceMap,
}

/// Stores nodes of interfaces and allows to search for interfaces by
/// vendor paths.
struct InterfaceMap {

    /// Child interfaces.
    map     : HashMap<String, InterfaceMap>,

    /// Interfaces at current node of vendor path.
    ints    : HashSet<Rc<Interface>>,
}

impl Master {

    pub fn new() -> Self {
        let core_obj = Object::new(
                0,
                String::from("org"),
                None
            );

        let mut core_obj_h = ObjectHandle::new(Box::new(core_obj));

        core_obj_h.create_new_object(
                String::from("kobzaros"),
                Visibility::Public,
            );

        Master {
            core_obj_h,
            interfaces: Default::default(),
        }
    }
}

impl Default for InterfaceMap {

    fn default() -> Self {
        InterfaceMap {
            map     : HashMap::new(),
            ints    : HashSet::new(),
        }
    }
}

impl InterfaceMapMaster {

    /// Add interface to the map.
    pub fn add_interface(&mut self, interface: Interface) -> Rc<Interface> {
        let map = self.create_path(interface.vendor());
        let rc = Rc::new(interface);
        map.ints.insert(rc.clone());
        rc
    }

    /// Create (open) the path needed to locate some interface and get
    /// reference to the InterfaceMap at the end of the path.
    ///
    /// # Warning
    /// Must be called from the CCS root InterfaceMap, otherwise the path
    /// will be built relatively to the current node but not as
    /// absolute path.
    fn create_path(&mut self, path: &InterfacePath) -> &mut InterfaceMap {
        unimplemented!()
    }
}
