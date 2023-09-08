use std::{sync::{Arc, Weak}, collections::HashMap};

mod keyboard;

pub trait Controller<T> {
    fn get_controller_map(&self) -> &HashMap<T, ControllerConnectedNode<T>>;
}

#[derive(Default)]
pub enum ControllerNeighbors<T> {
    #[default]
    Empty,
    Neighbors(T),
    Neighbors2(T, T),
    Neighbors3(T, T, T),
    Neighbors4(T, T, T, T),
    Neighbors5(T, T, T, T, T),
    Neighbors6(T, T, T, T, T, T),
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
    pub neighbors: ControllerNeighbors<ControllerLink<T>>,
}

impl<T> ControllerConnectedNode<T> {
    pub fn new(node: Arc<ControllerNode<T>>, neighbors: ControllerNeighbors<ControllerLink<T>>) -> Self {
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
