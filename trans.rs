use base::{Service, Interface, Object};
use std::collections::LinkedList;
use super::Master;

/// Transaction allows making multiple changes to object as a single
/// operation. This allows to revert changes if one of the changes
/// fails.
pub struct Transaction<'a> {

    /// Master where object is located.
    master  : &'a Master,

    /// Commands to change the choosen object.
    cmds    : LinkedList<Command>,
}

/// Visibility of objects and services.
pub enum Visibility {

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
    AddPubSrv(Box<Service>),

    /// Add internal service.
    AddIntSrv(Box<Service>),

    /// Add private service.
    AddPrivSrv(Box<Service>),

    /// Remove public service.
    RemPubSrv(Box<Service>),

    /// Remove internal service.
    RemIntSrv(Box<Service>),

    /// Remove private service.
    RemPrivSrv(Box<Service>),

    /// Change public service visibility.
    ChgPubSrvVis(Box<Service>, Visibility),

    /// Change internal service visibility.
    ChgIntSrvVis(Box<Service>, Visibility),

    /// Change private service visibility.
    ChgPrivSrvVis(Box<Service>, Visibility),

    /// Change service visibility.
    ChgSrvVis(Box<Service>, Visibility),

    /// New public sub-object.
    NewPubSubObj(Box<Object>),

    /// New internal sub-object.
    NewIntSubObj(Box<Object>),

    /// New private sub-object.
    NewPrivSubObj(Box<Object>),

    /// Remove public sub-object.
    RemPubSubObj(Box<Object>),

    /// Remove internal sub-object.
    RemIntSubObj(Box<Object>),

    /// Remove private sub-object.
    RemPrivSubObj(Box<Object>),

    /// Change sub-object visibility.
    ChgSubObjVis(Box<Object>, Visibility),

    /// Change public sub-object visibility.
    ChgPubSubObjVis(Box<Object>, Visibility),

    /// Change internal sub-object visibility.
    ChgIntSubObjVis(Box<Object>, Visibility),

    /// Change private sub-object visibility.
    ChgPrivSubObjVis(Box<Object>, Visibility),

    /// Implement interface.
    ImplInt(Box<Interface>),

    /// Unimplement interface.
    UnimplInt(Box<Interface>),
}

/// Transaction error which occurs when transaction failed to be
/// applied.
pub enum TransactionError {
}

impl<'a> Transaction<'a> {

    pub fn new(master: &'a Master) -> Self {
        Transaction {
            master: master,
            cmds : Default::default(),
        }
    }

    /// Add new public service.
    pub fn add_public_service(&mut self, srv: Box<Service>) {
        self.pushcmd(Command::AddPubSrv(srv));
    }

    /// Add new internal service.
    pub fn add_internal_service(&mut self, srv: Box<Service>) {
        self.pushcmd(Command::AddIntSrv(srv));
    }

    /// Add new private service.
    pub fn add_private_service(&mut self, srv: Box<Service>) {
        self.pushcmd(Command::AddPrivSrv(srv));
    }

    /// Add service and set it's visibility to given value.
    pub fn add_service(&mut self, srv: Box<Service>, vis: Visibility) {
        use self::Visibility::*;

        match vis {
            Public      => self.add_public_service(srv),
            Private     => self.add_internal_service(srv),
            Internal    => self.add_internal_service(srv),
        }
    }

    pub fn remove_public_service(&mut self, srv: Box<Service>) {
        self.pushcmd(Command::RemPubSrv(srv));
    }

    pub fn remove_internal_service(&mut self, srv: Box<Service>) {
        self.pushcmd(Command::RemIntSrv(srv));
    }

    pub fn remove_private_service(&mut self, srv: Box<Service>) {
        self.pushcmd(Command::RemPrivSrv(srv));
    }

    /// Change visibility of given service. If service initial visibility
    /// isn't passed then all lists will be checked to find requested service
    /// and then the visibility will be changed.
    pub fn change_service_access(&mut self, srv: Box<Service>,
            newvis: Visibility, oldvis: Option<Visibility>) {
        use self::Visibility::*;

        if oldvis.is_none() {
            self.pushcmd(Command::ChgSrvVis(srv, newvis));
        } else { match oldvis.unwrap() {
            Public      => self.pushcmd(Command::ChgPubSrvVis(srv, newvis)),
            Private     => self.pushcmd(Command::ChgPrivSrvVis(srv, newvis)),
            Internal    => self.pushcmd(Command::ChgIntSrvVis(srv, newvis)),
        }}
    }

    /// Create new public sub-object.
    pub fn new_public_sub_object(&mut self, obj: Box<Object>) {
        self.pushcmd(Command::NewPubSubObj(obj));
    }

    /// Create new internal sub-object.
    pub fn new_internal_sub_object(&mut self, obj: Box<Object>) {
        self.pushcmd(Command::NewIntSubObj(obj));
    }

    /// Create new private sub-object.
    pub fn new_private_sub_object(&mut self, obj: Box<Object>) {
        self.pushcmd(Command::NewPrivSubObj(obj));
    }

    /// Delete private sub-object.
    pub fn remove_private_sub_object(&mut self, obj: Box<Object>) {
        self.pushcmd(Command::RemPrivSubObj(obj));
    }

    /// Delete internal sub-object.
    pub fn remove_internal_sub_object(&mut self, obj: Box<Object>) {
        self.pushcmd(Command::RemIntSubObj(obj));
    }

    /// Delete public sub-object.
    pub fn remove_public_sub_object(&mut self, obj: Box<Object>) {
        self.pushcmd(Command::RemPubSubObj(obj));
    }

    /// Change the visibility of sub-object from given value to new value.
    /// If initial value is not passed then every list of objects will be
    /// checked to find it and change the visibility to appropriate.
    pub fn change_sub_object_access(&mut self, obj: Box<Object>,
            newvis: Visibility, oldvis: Option<Visibility>) {
         use self::Command::{
            ChgSubObjVis,
            ChgPrivSubObjVis,
            ChgPubSubObjVis,
            ChgIntSubObjVis,
         };

         use self::Visibility::*;

         self.pushcmd(
            if oldvis.is_none() {
                ChgSubObjVis(obj, newvis)
            } else {
                match oldvis.unwrap() {
                    Public      => ChgPubSubObjVis(obj, newvis),
                    Private     => ChgPrivSubObjVis(obj, newvis),
                    Internal    => ChgIntSubObjVis(obj, newvis),
                }
            }
         );
    }

    /// Mark this object as interface implementer.
    pub fn implement_interface(&mut self, i: Box<Interface>) {
        self.pushcmd(Command::ImplInt(i));
    }

    /// Tell this object now doesn't implement given interface.
    pub fn unimplement_interface(&mut self, i: Box<Interface>) {
        self.pushcmd(Command::UnimplInt(i));
    }

    fn pushcmd(&mut self, cmd: Command) {
        self.cmds.push_front(cmd)
    }

    /// Applies changes to given object.
    pub fn apply_to_object(&self, obj: &mut Object)
            -> Result<(), TransactionError> {
        use self::Command::*;

        for cmd in self.cmds.iter() {
            match *cmd {
                AddPubSrv(ref srv) => {

                },
                _ => unimplemented!()
            }
        }
        unimplemented!();
    }
}
