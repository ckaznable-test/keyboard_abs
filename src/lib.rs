use std::{sync::{Arc, Weak}, collections::HashMap};

use bevy::prelude::KeyCode;

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

pub struct Keyboard {
    map: HashMap<KeyCode, ControllerConnectedNode<KeyCode>>,
}

impl Controller<KeyCode> for Keyboard {
    fn get_controller_map(&self) -> &HashMap<KeyCode, ControllerConnectedNode<KeyCode>> {
        &self.map
    }
}

impl Default for Keyboard {
    fn default() -> Self {
        let mut map = HashMap::<KeyCode, ControllerConnectedNode<KeyCode>>::new();

        let q = ControllerNode::new(KeyCode::Q);
        let w = ControllerNode::new(KeyCode::W);
        let e = ControllerNode::new(KeyCode::E);
        let r = ControllerNode::new(KeyCode::R);
        let t = ControllerNode::new(KeyCode::T);
        let y = ControllerNode::new(KeyCode::Y);
        let u = ControllerNode::new(KeyCode::U);
        let i = ControllerNode::new(KeyCode::I);
        let o = ControllerNode::new(KeyCode::O);
        let p = ControllerNode::new(KeyCode::P);

        let a = ControllerNode::new(KeyCode::A);
        let s = ControllerNode::new(KeyCode::S);
        let d = ControllerNode::new(KeyCode::D);
        let f = ControllerNode::new(KeyCode::F);
        let g = ControllerNode::new(KeyCode::G);
        let h = ControllerNode::new(KeyCode::H);
        let j = ControllerNode::new(KeyCode::J);
        let k = ControllerNode::new(KeyCode::K);
        let l = ControllerNode::new(KeyCode::L);

        let z = ControllerNode::new(KeyCode::Z);
        let x = ControllerNode::new(KeyCode::X);
        let c = ControllerNode::new(KeyCode::C);
        let v = ControllerNode::new(KeyCode::V);
        let b = ControllerNode::new(KeyCode::B);
        let n = ControllerNode::new(KeyCode::N);
        let m = ControllerNode::new(KeyCode::M);

        map.insert(KeyCode::Q, ControllerConnectedNode::new(
            q.clone(),
            ControllerNeighbors::Neighbors2(w.clone().into(), a.clone().into())
        ));

        map.insert(KeyCode::W, ControllerConnectedNode::new(
            w.clone(),
            ControllerNeighbors::Neighbors4(
                q.clone().into(), e.clone().into(),
                a.clone().into(), s.clone().into(),
            )
        ));

        map.insert(KeyCode::E, ControllerConnectedNode::new(
            e.clone(),
            ControllerNeighbors::Neighbors4(
                w.clone().into(), r.clone().into(),
                s.clone().into(), d.clone().into(),
            )
        ));

        map.insert(KeyCode::R, ControllerConnectedNode::new(
            r.clone(),
            ControllerNeighbors::Neighbors4(
                e.clone().into(), t.clone().into(),
                d.clone().into(), f.clone().into(),
            )
        ));

        map.insert(KeyCode::T, ControllerConnectedNode::new(
            t.clone(),
            ControllerNeighbors::Neighbors4(
                r.clone().into(), y.clone().into(),
                f.clone().into(), g.clone().into(),
            )
        ));

        map.insert(KeyCode::Y, ControllerConnectedNode::new(
            y.clone(),
            ControllerNeighbors::Neighbors4(
                t.clone().into(), u.clone().into(),
                g.clone().into(), h.clone().into(),
            )
        ));

        map.insert(KeyCode::U, ControllerConnectedNode::new(
            u.clone(),
            ControllerNeighbors::Neighbors4(
                y.clone().into(), i.clone().into(),
                h.clone().into(), j.clone().into(),
            )
        ));

        map.insert(KeyCode::I, ControllerConnectedNode::new(
            i.clone(),
            ControllerNeighbors::Neighbors4(
                u.clone().into(), i.clone().into(),
                j.clone().into(), k.clone().into(),
            )
        ));

        map.insert(KeyCode::O, ControllerConnectedNode::new(
            o.clone(),
            ControllerNeighbors::Neighbors4(
                i.clone().into(), p.clone().into(),
                k.clone().into(), l.clone().into(),
            )
        ));

        map.insert(KeyCode::P, ControllerConnectedNode::new(
            e.clone(),
            ControllerNeighbors::Neighbors2(o.clone().into(), l.clone().into())
        ));

        map.insert(KeyCode::A, ControllerConnectedNode::new(
            a.clone(),
            ControllerNeighbors::Neighbors4(
                q.clone().into(), w.clone().into(),
                s.clone().into(), z.clone().into(),
            )
        ));

        map.insert(KeyCode::S, ControllerConnectedNode::new(
            s.clone(),
            ControllerNeighbors::Neighbors6(
                w.clone().into(), e.clone().into(),
                a.clone().into(), d.clone().into(),
                z.clone().into(), x.clone().into(),
            )
        ));

        map.insert(KeyCode::D, ControllerConnectedNode::new(
            d.clone(),
            ControllerNeighbors::Neighbors6(
                e.clone().into(), r.clone().into(),
                s.clone().into(), f.clone().into(),
                x.clone().into(), c.clone().into(),
            )
        ));

        map.insert(KeyCode::F, ControllerConnectedNode::new(
            f.clone(),
            ControllerNeighbors::Neighbors6(
                r.clone().into(), t.clone().into(),
                d.clone().into(), g.clone().into(),
                c.clone().into(), v.clone().into(),
            )
        ));

        map.insert(KeyCode::G, ControllerConnectedNode::new(
            g.clone(),
            ControllerNeighbors::Neighbors6(
                t.clone().into(), y.clone().into(),
                f.clone().into(), h.clone().into(),
                v.clone().into(), b.clone().into(),
            )
        ));

        map.insert(KeyCode::H, ControllerConnectedNode::new(
            h.clone(),
            ControllerNeighbors::Neighbors6(
                y.clone().into(), u.clone().into(),
                g.clone().into(), j.clone().into(),
                b.clone().into(), n.clone().into(),
            )
        ));

        map.insert(KeyCode::J, ControllerConnectedNode::new(
            j.clone(),
            ControllerNeighbors::Neighbors6(
                u.clone().into(), i.clone().into(),
                h.clone().into(), k.clone().into(),
                n.clone().into(), m.clone().into(),
            )
        ));

        map.insert(KeyCode::K, ControllerConnectedNode::new(
            k.clone(),
            ControllerNeighbors::Neighbors5(
                i.clone().into(), o.clone().into(),
                j.clone().into(), l.clone().into(),
                m.clone().into()
            )
        ));

        map.insert(KeyCode::L, ControllerConnectedNode::new(
            l.clone(),
            ControllerNeighbors::Neighbors3(k.clone().into(), p.clone().into(), o.clone().into())
        ));

        map.insert(KeyCode::Z, ControllerConnectedNode::new(
            z.clone(),
            ControllerNeighbors::Neighbors3(a.clone().into(), s.clone().into(), x.clone().into())
        ));

        map.insert(KeyCode::X, ControllerConnectedNode::new(
            x.clone(),
            ControllerNeighbors::Neighbors4(s.clone().into(), d.clone().into(), c.clone().into(), z.clone().into())
        ));

        map.insert(KeyCode::C, ControllerConnectedNode::new(
            c.clone(),
            ControllerNeighbors::Neighbors4(d.clone().into(), f.clone().into(), v.clone().into(), x.clone().into())
        ));

        map.insert(KeyCode::V, ControllerConnectedNode::new(
            v.clone(),
            ControllerNeighbors::Neighbors4(f.clone().into(), g.clone().into(), b.clone().into(), c.clone().into())
        ));

        map.insert(KeyCode::B, ControllerConnectedNode::new(
            b.clone(),
            ControllerNeighbors::Neighbors4(g.clone().into(), h.clone().into(), n.clone().into(), v.clone().into())
        ));

        map.insert(KeyCode::N, ControllerConnectedNode::new(
            n.clone(),
            ControllerNeighbors::Neighbors4(h.clone().into(), j.clone().into(), m.clone().into(), b.clone().into())
        ));

        map.insert(KeyCode::M, ControllerConnectedNode::new(
            m.clone(),
            ControllerNeighbors::Neighbors3(j.clone().into(), k.clone().into(), n.clone().into())
        ));

        Self { map }
    }
}
