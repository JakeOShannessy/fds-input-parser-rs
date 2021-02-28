use crate::decode::*;
// TODO: everything here assumes XBs are sorted, make this true.

/// A trait for objects that MUST have an XB (for example: meshes, which are
/// given a default XB if it's not specified).
pub trait HasXB {
    fn xb(&self) -> XB;
    // / Test two Objects with XBs and determine if they intersect in 3d.
    // fn intersect<Q: HasXB>(&self, b: &Q) -> bool {
    //     self.xb().intersect(&b.xb())
    // }
}

pub trait MightHaveXB {
    fn try_xb(&self) -> Option<XB>;
    fn intersect<Q: MightHaveXB>(&self, b: &Q) -> bool {
        let a_xb = if let Some(xb) = self.try_xb() {xb} else {return  false;};
        let b_xb = if let Some(xb) = b.try_xb() {xb} else {return  false;};
        a_xb.intersect(&b_xb)
    }
}

impl<T: HasXB> MightHaveXB for T {
    /// Return the XB value if it has one.
    fn try_xb(&self) -> Option<XB> {
        Some(self.xb())
    }

    /// Test two Objects with XBs and determine if they intersect in 3d.
    fn intersect<Q: MightHaveXB>(&self, b: &Q) -> bool {
        match b.try_xb() {
            Some(b_xb) => self.xb().intersect(&b_xb),
            _ => false,
        }
    }
}

/// Check if two objects have the same XB. If either does not have an XB, simply
/// return False.
pub fn match_xbs<T: MightHaveXB, Q: MightHaveXB>(a: T, b: Q) -> bool {
    match (a.try_xb(), b.try_xb()) {
        (Some(a_xb), Some(b_xb)) => a_xb == b_xb,
        _ => false,
    }
}
