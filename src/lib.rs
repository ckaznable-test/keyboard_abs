use std::{sync::{Arc, Weak}, hash::Hash};

use bevy::utils::HashMap;

mod keyboard;

pub type ControllerNeighbors<T> = Vec<ControllerLink<T>>;

pub trait ControllerBuilder<T: Hash + Eq + Clone>: Sized {
    fn get_controller_map(&self) -> &HashMap<T, ControllerConnectedNode<T>>;

    fn get_neighbors(&self, key: T) -> Option<&ControllerNeighbors<T>> {
        let map = self.get_controller_map();
        let node = map.get(&key)?;
        Some(&node.neighbors)
    }

    fn get_neighbors_distance_two(&self, key: T) -> Option<ControllerNeighbors<T>> {
        let map = self.get_controller_map();
        let neighbors = self.get_neighbors(key)?;
        let neighbors = neighbors
            .iter()
            .fold(Vec::<ControllerLink<T>>::new(), |mut acc, link| {
                acc.push(ControllerLink(link.0.clone()));

                let arc_node = link.0.upgrade();
                if arc_node.is_none() {
                    return acc;
                }

                let node = arc_node.unwrap();
                if let Some(node) = map.get(&node.0) {
                    let mut n = node.neighbors.clone();
                    acc.append(&mut n);
                }

                acc
            });

        Some(neighbors)
    }

    fn get_mirrors_key(&self) -> Option<&ControllerNode<T>> {
        None
    }

    fn build(self) -> Controller<T> {
        let map = self.get_controller_map();
        let mut neighbors_map = HashMap::<T, ControllerNeighbors<T>>::new();
        let mut neighbors_distance_two_map = HashMap::<T, ControllerNeighbors<T>>::new();

        map.keys().for_each(|k| {
            if let Some(n) = self.get_neighbors(k.clone()) {
                neighbors_map.insert(k.clone(), n.clone());
            }

            if let Some(n) = self.get_neighbors_distance_two(k.clone()) {
                neighbors_distance_two_map.insert(k.clone(), n.clone());
            }
        });

        Controller {
            neighbors_map,
            neighbors_distance_two_map,
        }
    }
}

pub struct Controller<T> {
    pub neighbors_map: HashMap<T, ControllerNeighbors<T>>,
    pub neighbors_distance_two_map: HashMap<T, ControllerNeighbors<T>>,
}

#[derive(Clone)]
pub struct ControllerLink<T>(Weak<ControllerNode<T>>);
impl<T> From<Arc<ControllerNode<T>>> for ControllerLink<T> {
    fn from(node: Arc<ControllerNode<T>>) -> Self {
        ControllerLink(Arc::downgrade(&node))
    }
}

impl<T> PartialEq for ControllerLink<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.ptr_eq(&other.0)
    }
}

#[derive(Clone)]
pub struct ControllerNode<T>(T);
impl<T> ControllerNode<T> {
    pub fn new(node: T) -> Arc<Self> {
        Arc::new(Self(node))
    }
}

pub struct ControllerConnectedNode<T> {
    pub node: Arc<ControllerNode<T>>,
    pub neighbors: ControllerNeighbors<T>,
}

impl<T> ControllerConnectedNode<T> {
    pub fn new(node: Arc<ControllerNode<T>>, neighbors: ControllerNeighbors<T>) -> Self {
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
