use std::sync::Arc;

use bevy::{prelude::KeyCode, utils::HashMap};

use crate::{ControllerConnectedNode, ControllerBuilder, ControllerNode, Controller};

pub struct Keyboard {
    pub controller: Controller<KeyCode>,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            controller: KeyboardBuilder::default().build(),
        }
    }
}

pub struct KeyboardBuilder {
    map: HashMap<KeyCode, ControllerConnectedNode<KeyCode>>,
    set: Vec<Arc<ControllerNode<KeyCode>>>
}

impl ControllerBuilder<KeyCode> for KeyboardBuilder {
    fn get_controller_map(&self) -> &HashMap<KeyCode, ControllerConnectedNode<KeyCode>> {
        &self.map
    }

    fn get_key_set(&self) -> &[Arc<ControllerNode<KeyCode>>] {
        &self.set
    }
}

impl Default for KeyboardBuilder {
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
            vec![w.clone().into(), a.clone().into()]
        ));

        map.insert(KeyCode::W, ControllerConnectedNode::new(
            w.clone(),
            vec![
                q.clone().into(), e.clone().into(),
                a.clone().into(), s.clone().into(),
            ]
        ));

        map.insert(KeyCode::E, ControllerConnectedNode::new(
            e.clone(),
            vec![
                w.clone().into(), r.clone().into(),
                s.clone().into(), d.clone().into(),
            ]
        ));

        map.insert(KeyCode::R, ControllerConnectedNode::new(
            r.clone(),
            vec![
                e.clone().into(), t.clone().into(),
                d.clone().into(), f.clone().into(),
            ]
        ));

        map.insert(KeyCode::T, ControllerConnectedNode::new(
            t.clone(),
            vec![
                r.clone().into(), y.clone().into(),
                f.clone().into(), g.clone().into(),
            ]
        ));

        map.insert(KeyCode::Y, ControllerConnectedNode::new(
            y.clone(),
            vec![
                t.clone().into(), u.clone().into(),
                g.clone().into(), h.clone().into(),
            ]
        ));

        map.insert(KeyCode::U, ControllerConnectedNode::new(
            u.clone(),
            vec![
                y.clone().into(), i.clone().into(),
                h.clone().into(), j.clone().into(),
            ]
        ));

        map.insert(KeyCode::I, ControllerConnectedNode::new(
            i.clone(),
            vec![
                u.clone().into(), i.clone().into(),
                j.clone().into(), k.clone().into(),
            ]
        ));

        map.insert(KeyCode::O, ControllerConnectedNode::new(
            o.clone(),
            vec![
                i.clone().into(), p.clone().into(),
                k.clone().into(), l.clone().into(),
            ]
        ));

        map.insert(KeyCode::P, ControllerConnectedNode::new(
            e.clone(),
            vec![o.clone().into(), l.clone().into()]
        ));

        map.insert(KeyCode::A, ControllerConnectedNode::new(
            a.clone(),
            vec![
                q.clone().into(), w.clone().into(),
                s.clone().into(), z.clone().into(),
            ]
        ));

        map.insert(KeyCode::S, ControllerConnectedNode::new(
            s.clone(),
            vec![
                w.clone().into(), e.clone().into(),
                a.clone().into(), d.clone().into(),
                z.clone().into(), x.clone().into(),
            ]
        ));

        map.insert(KeyCode::D, ControllerConnectedNode::new(
            d.clone(),
            vec![
                e.clone().into(), r.clone().into(),
                s.clone().into(), f.clone().into(),
                x.clone().into(), c.clone().into(),
            ]
        ));

        map.insert(KeyCode::F, ControllerConnectedNode::new(
            f.clone(),
            vec![
                r.clone().into(), t.clone().into(),
                d.clone().into(), g.clone().into(),
                c.clone().into(), v.clone().into(),
            ]
        ));

        map.insert(KeyCode::G, ControllerConnectedNode::new(
            g.clone(),
            vec![
                t.clone().into(), y.clone().into(),
                f.clone().into(), h.clone().into(),
                v.clone().into(), b.clone().into(),
            ]
        ));

        map.insert(KeyCode::H, ControllerConnectedNode::new(
            h.clone(),
            vec![
                y.clone().into(), u.clone().into(),
                g.clone().into(), j.clone().into(),
                b.clone().into(), n.clone().into(),
            ]
        ));

        map.insert(KeyCode::J, ControllerConnectedNode::new(
            j.clone(),
            vec![
                u.clone().into(), i.clone().into(),
                h.clone().into(), k.clone().into(),
                n.clone().into(), m.clone().into(),
            ]
        ));

        map.insert(KeyCode::K, ControllerConnectedNode::new(
            k.clone(),
            vec![
                i.clone().into(), o.clone().into(),
                j.clone().into(), l.clone().into(),
                m.clone().into()
            ]
        ));

        map.insert(KeyCode::L, ControllerConnectedNode::new(
            l.clone(),
            vec![k.clone().into(), p.clone().into(), o.clone().into()]
        ));

        map.insert(KeyCode::Z, ControllerConnectedNode::new(
            z.clone(),
            vec![a.clone().into(), s.clone().into(), x.clone().into()]
        ));

        map.insert(KeyCode::X, ControllerConnectedNode::new(
            x.clone(),
            vec![s.clone().into(), d.clone().into(), c.clone().into(), z.clone().into() ]
        ));

        map.insert(KeyCode::C, ControllerConnectedNode::new(
            c.clone(),
            vec![d.clone().into(), f.clone().into(), v.clone().into(), x.clone().into() ]
        ));

        map.insert(KeyCode::V, ControllerConnectedNode::new(
            v.clone(),
            vec![f.clone().into(), g.clone().into(), b.clone().into(), c.clone().into() ]
        ));

        map.insert(KeyCode::B, ControllerConnectedNode::new(
            b.clone(),
            vec![g.clone().into(), h.clone().into(), n.clone().into(), v.clone().into() ]
        ));

        map.insert(KeyCode::N, ControllerConnectedNode::new(
            n.clone(),
            vec![h.clone().into(), j.clone().into(), m.clone().into(), b.clone().into() ]
        ));

        map.insert(KeyCode::M, ControllerConnectedNode::new(
            m.clone(),
            vec![j.clone().into(), k.clone().into(), n.clone().into()]
        ));

        Self {
            map,
            set: vec![
                q, w, e, r, t, y, u, i, o, p,
                a, s, d, f, g, h, j, k, l,
                z, x, c, v, b, n, m
            ]
        }
    }
}
