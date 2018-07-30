use super::{Service, Interface, Object};
use std::collections::LinkedList;

/// Transaction allows making multiple changes to object as a single
/// operation. This allows to revert changes if one of the changes
/// fails.
pub struct ObjectTransaction {
    cmds    : LinkedList<Command>,
}

/// Visibility of objects and services.
enum Visibility {

    /// Visible to everything from everywhere.
    Public,

    /// Visible only to services and sub-objects and their services.
    Internal,

    /// Visible only for parent object and services at current hierarchy level.
    Private,
}

/// Commands used in object transactions.
enum Command {

    /// Add public service.
    AddPubSrv(Service),

    /// Add internal service.
    AddIntSrv(Service),

    /// Add private service.
    AddPrivSrv(Service),

    /// Remove public service.
    RemPubSrv(Service),

    /// Remove internal service.
    RemIntSrv(Service),

    /// Remove private service.
    RemPrivSrv(Service),

    /// Change public service visibility.
    ChgPubSrvVis(Service, Visibility),

    /// Change internal service visibility.
    ChgIntSrvVis(Service, Visibility),

    /// Change private service visibility.
    ChgPrivSrvVis(Service, Visibility),

    /// Change service visibility.
    ChgSrvVis(Service, Visibility),

    /// New public sub-object.
    NewPubSubObj(Object),

    /// New internal sub-object.
    NewIntSubObj(Object),

    /// New private sub-object.
    NewPrivSubObj(Object),

    /// Remove public sub-object.
    RemPubSubObj(Object),

    /// Remove internal sub-object.
    RemIntSubObj(Object),

    /// Remove private sub-object.
    RemPrivSubObj(Object),

    /// Change sub-object visibility.
    ChgSubObjVis(Object, Visibility),

    /// Change public sub-object visibility.
    ChgPubSubObjVis(Object, Visibility),

    /// Change internal sub-object visibility.
    ChgIntSubObjVis(Object, Visibility),

    /// Change private sub-object visibility.
    ChgPrivSubObjVis(Object, Visibility),

    /// Implement interface.
    ImplInt(Interface),

    /// Unimplement interface.
    UnimplInt(Interface),
}

impl ObjectTransaction {

    pub fn new() -> Self {
        unimplemented!()
    }

    /// Add new public service.
    pub fn add_public_service(&mut self) {
        unimplemented!()
    }

    /// Add new internal service.
    pub fn add_internal_service(&mut self) {
        unimplemented!()
    }

    /// Add new private service.
    pub fn add_private_service(&mut self) {
        unimplemented!()
    }

    /// Add service and set it's visibility to given value.
    pub fn add_service(&mut self) {
        unimplemented!()
    }

    pub fn remove_public_service(&mut self) {
        unimplemented!()
    }

    pub fn remove_internal_service(&mut self) {
        unimplemented!()
    }

    pub fn remove_private_service(&mut self) {
        unimplemented!()
    }

    /// Change visibility of given service. If service initial visibility
    /// isn't passed then all lists will be checked to find requested service
    /// and then the visibility will be changed.
    pub fn change_service_access(&mut self) {
        unimplemented!()
    }

    /// Create new public sub-object.
    pub fn new_public_sub_object(&mut self) {
        unimplemented!()
    }

    /// Create new internal sub-object.
    pub fn new_internal_sub_object(&mut self) {
        unimplemented!()
    }

    /// Create new private sub-object.
    pub fn new_private_sub_object(&mut self) {
        unimplemented!()
    }

    /// Delete private sub-object.
    pub fn remove_private_sub_object(&mut self) {
        unimplemented!()
    }

    /// Delete internal sub-object.
    pub fn remove_internal_sub_object(&mut self) {
        unimplemented!()
    }

    /// Delete public sub-object.
    pub fn remove_public_sub_object(&mut self) {
        unimplemented!()
    }

    /// Change the visibility of sub-object from given value to new value.
    /// If initial value is not passed then every list of objects will be
    /// checked to find it and change the visibility to appropriate.
    pub fn change_sub_object_access(&mut self) {
        unimplemented!()
    }

    /// Mark this object as interface implementer.
    pub fn implement_interface(&mut self) {
        unimplemented!()
    }

    /// Tell this object now doesn't implement given interface.
    pub fn unimplement_interface(&mut self) {
        unimplemented!()
    }
}
