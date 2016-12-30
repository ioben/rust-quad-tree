use std::mem;
use std::fmt::Debug;
use std::ops::{Div, Deref, DerefMut};

use super::primitives::*;

pub trait Locatable {
    fn position(&self) -> &Point;
}

#[derive(Debug)]
pub enum RawVoxel<T> {
    SubCell([Voxel<T>; ::N]),
    Value(T)
}

impl<T> RawVoxel<T> {
    fn subcell() -> RawVoxel<T> {
        RawVoxel::SubCell( //TODO - Why mem::zeroed?
            unsafe { mem::zeroed() }
        )
    }

    fn value(value: T) -> RawVoxel<T> {
        RawVoxel::Value(value)
    }
}

#[derive(Debug)]
pub struct Voxel<T>(Option<Box<RawVoxel<T>>>);

impl<T> Voxel<T> {
    fn subcell() -> Voxel<T> {
        Voxel(Some(Box::new(RawVoxel::subcell())))
    }

    fn value(value: T) -> Voxel<T> {
        Voxel(Some(Box::new(RawVoxel::value(value))))
    }
}

impl<T> Deref for Voxel<T> {
    type Target = Option<Box<RawVoxel<T>>>;

    fn deref(&self) -> &Option<Box<RawVoxel<T>>> {
        &self.0
    }
}

impl<T> DerefMut for Voxel<T> {
    fn deref_mut(&mut self) -> &mut Option<Box<RawVoxel<T>>> {
        &mut self.0
    }
}

pub struct Tree<T> {
    size: Point,
    root: Voxel<T>,
    // items: Vec<T>,
}

impl<T: Locatable + Debug> Tree<T> {
    pub fn new(size: Point) -> Self {
        Tree {
            size: size,
            root: Voxel(None),
            // items: vec![],
        }
    }

    pub fn add(&mut self, item: T) {
        Self::add_to(&mut self.root, Point::zero(), self.size, item);
    }

    fn add_to(voxel: &mut Voxel<T>, offset: Point, size: Point, value: T) {
        // let mut raw_voxel = voxel.take();

        let mut offset = offset;
        let mut size = size.to_owned();

        match *voxel {
            Voxel(None) => { // Insert
                println!(" ===== ADDING {:?}", value);
                let mut value = Some(Box::new(RawVoxel::value(value)));
                **voxel = value;
            },
            Voxel(Some(box RawVoxel::SubCell(ref mut subcell))) => { // Recurse
                println!(" ===== RECURSING {:?}", subcell);

                // Get child index
                let mut child_index: usize = 0;

                for i in 0..::D {
                    child_index <<= 1;

                    println!("COMPARISON: {} < {}", size[i] + offset[i], value.position()[i]);
                    if offset[i] + size[i] < value.position()[i] {
                        child_index |= 1;
                        offset[i] += size[i];
                    }
                }

                println!("Child Index: {:b}", child_index);
                println!("Child Relative Position: {:?}", offset);

                let mut child_voxel = &mut subcell[child_index];
                Self::add_to(child_voxel, offset, size / 2f32, value);
            },
            Voxel(Some(box RawVoxel::Value(_))) => {
                let raw_voxel = voxel.take();
                println!(" ===== SUBDIVIDING {:?}", raw_voxel);

                // Too many items, create new subcell, add v then value to it.
                // Make subcell, add v to subcell, recurse
                let mut subcell = Some(Box::new(RawVoxel::<T>::subcell()));
                **voxel = subcell;

                println!("OFFSET: {:?}", offset);
                println!("SUBCELL: {:?}", voxel);

                if let Some(box RawVoxel::Value(current_value)) = raw_voxel {
                    // Can't add two values at the same pos yet
                    if current_value.position() == value.position() {
                        unimplemented!();
                    }

                    Self::add_to(voxel, offset, size / 2f32, current_value);
                }

                Self::add_to(voxel, offset, size / 2f32, value);
                println!("SUBCELL2: {:?}", voxel);
            },
        };
    }

    fn contains(&mut self, point: Point) {

    }

    fn intersects(&mut self, point: Rectangle) {

    }

    fn print(&self) {
        Self::print_voxel(&self.root, String::new(), 0usize);
    }

    fn print_voxel(voxel: &Voxel<T>, path: String, depth: usize) {
        match voxel {
            &Voxel(None) => { },
            &Voxel(Some(box RawVoxel::SubCell(ref subcells))) => {
                for (index, subcell) in subcells.iter().enumerate() {
                    Self::print_voxel(subcell, format!("{}.{:b}", path, index), depth + 1);
                }
            },
            &Voxel(Some(box RawVoxel::Value(ref v))) => {
                println!("{}{}: {:?}", " ".repeat(depth * 4), path, v);
            },
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use ::primitives::Point;

    // #[test]
    // fn it_works() {
        // assert!(true);
    // }

    #[test]
    fn it_can_add_items() {
        let mut tree: Tree<Point> = Tree::new(Point::from(100f32));

        // tree.add(Point::from(40f32));
        // tree.add(Point::from(60f32));
        tree.add(Point::new([70f32, 70f32]));
        tree.add(Point::new([30f32, 30f32]));
        tree.add(Point::new([70f32, 30f32]));
        tree.add(Point::new([30f32, 70f32]));

        tree.print();
        // println!("ROOT: {:?}", tree.root);
    }
}
