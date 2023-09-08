use std::sync::{Arc, Weak};

use bevy::utils::HashMap;

mod keyboard;

pub trait ControllerBuilder<T> {
    fn get_controller_map(&self) -> &HashMap<T, ControllerConnectedNode<T>>;
}

#[derive(Clone)]
pub struct ControllerLink<T>(Weak<ControllerNode<T>>);
impl<T> From<Arc<ControllerNode<T>>> for ControllerLink<T> {
    fn from(node: Arc<ControllerNode<T>>) -> Self {
        ControllerLink(Arc::downgrade(&node))
    }
}

pub struct ControllerNode<T>(T);
impl<T> ControllerNode<T> {
    pub fn new(node: T) -> Arc<Self> {
        Arc::new(Self(node))
    }
}

pub struct ControllerConnectedNode<T> {
    pub node: Arc<ControllerNode<T>>,
    pub neighbors: Vec<ControllerLink<T>>,
}

impl<T> ControllerConnectedNode<T> {
    pub fn new(node: Arc<ControllerNode<T>>, neighbors: Vec<ControllerLink<T>>) -> Self {
        Self {
            node,
            neighbors,
        }
    }
}

impl<T> Default for ControllerConnectedNode<T> where T: Default {
    fn default() -> Self {
        todo!()
    }
}
