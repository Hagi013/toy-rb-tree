use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Debug, Formatter, Error, Display};
use std::path::Component::RootDir;

fn main() {
    let mut tree: RBTree<usize, usize> = RBTree::new();
    tree.insert(10, &2);
    tree.insert(3, &1);
    tree.insert(1, &2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Color {
    Black, Red,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Rotate {
    Left, Right,
}

struct RBNode<K: Ord + Clone, V> {
    key: K,
    value: *mut V,
    color: Color,
    parent: Option<*mut RBNode<K, V>>,
    left: Option<*mut RBNode<K, V>>,
    right: Option<*mut RBNode<K, V>>,
}

impl<K: Ord + Clone + Debug, V> RBNode<K, V> {
    #[inline]
    fn pair(self) -> (K, *mut V) {
        (self.key, self.value)
    }
}

impl<K: Ord + Clone + Debug, V> PartialOrd for RBNode<K, V> {
    fn partial_cmp(&self, other: &RBNode<K, V>) -> Option<Ordering> {
        Some(self.key.cmp(&other.key))
    }
}

impl<K: Ord + Clone + Debug, V> Ord for RBNode<K, V> {
    fn cmp(&self, other: &RBNode<K, V>) -> Ordering {
        self.key.cmp(&other.key)
    }
}

impl<K: Ord + Clone + Debug, V> PartialEq for RBNode<K, V> {
    fn eq(&self, other: &RBNode<K, V>) -> bool { self.key == other.key }
}

impl<K: Ord + Clone + Debug, V> Eq for RBNode<K, V> {}

impl<K: Ord + Clone + Debug + Clone, V> Clone for RBNode<K, V> {
    fn clone(&self) -> RBNode<K, V> {
        RBNode {
            key: self.key.clone(),
            value: self.value,
            color: self.color,
            parent: self.parent,
            left: self.left,
            right: self.right,
        }
    }
}

impl<K: Debug + Clone + Ord, V> Debug for RBNode<K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "RBNode [ key: {:?}, value: {:?}, color: {:?}, parent: {:#?}, left: {:?}, right: {:?} ]", self.key, self.value, self.color, self.parent, self.left, self.right)?;
        Ok(())
    }
}

impl<K: Ord + Clone + Debug, V> RBNode<K, V> {
    fn new(key: K, value: &V) -> RBNode<K, V> {
        RBNode {
            key,
            value: value as *const V as *mut V,
            color: Color::Red,
            parent: None,
            left: None,
            right: None,
        }
    }

    #[inline]
    fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    #[inline]
    fn set_red_color(&mut self) {
        self.set_color(Color::Red);
    }

    #[inline]
    fn set_black_color(&mut self) {
        self.set_color(Color::Black);
    }

    #[inline]
    fn get_color(&mut self) {
        self.color;
    }

    #[inline]
    fn is_red(&self) -> bool {
        self.color == Color::Red
    }

    #[inline]
    fn parent_is_red(&self) -> bool {
        if let Some(parent) = self.parent() {
            parent.is_red()
        } else {
            false
        }
    }

    #[inline]
    fn is_black(&self) -> bool {
        self.color == Color::Black
    }

    #[inline]
    fn parent_is_black(&self) -> bool {
        if let Some(parent) = self.parent() {
            parent.is_black()
        } else {
            false
        }
    }

    #[inline]
    fn parent(&self) -> Option<RBNode<K, V>> {
        if self.parent.is_some() {
            unsafe { Some((*self.parent.unwrap()).clone()) }
        } else {
            None
        }
    }

    #[inline]
    fn left(&self) -> Option<RBNode<K, V>> {
        if self.left.is_some() {
            unsafe { Some((*self.left.unwrap()).clone()) }
        } else {
            None
        }
    }

    #[inline]
    fn is_left(&self) -> bool {
        let p = self.parent();
        if let Some(parent) = p {
            if let Some(left) = parent.left() { return left == *self }
        }
        false
    }


    #[inline]
    fn right(&self) -> Option<RBNode<K, V>> {
        if self.right.is_some() {
            unsafe { Some((*self.right.unwrap()).clone()) }
        } else {
            None
        }
    }

    #[inline]
    fn is_right(&self) -> bool {
        let p = self.parent();
        if let Some(parent) = p {
            if let Some(right) = parent.right() {
                return right == *self
            }
        }
        false
    }

    #[inline]
    fn next(&self) -> Option<RBNode<K, V>> {
        if let Some(right) = self.right() {
            return Some(right.min_node());
        }
        if self.is_left() {
            if let Some(parent) = self.parent() {
                return Some(parent);
            }
        }
        None
    }

    #[inline]
    fn prev(&self) -> Option<RBNode<K, V>> {
        if let Some(left) = self.left() {
            return Some(left.max_node());
        }
        if self.is_right() {
            if let Some(parent) = self.parent() {
                return Some(parent);
            }
        }
        None
    }

    #[inline]
    fn min_node(&self) -> RBNode<K, V> {
        let next_left = self.left();
        if let Some(l) = next_left {
            return l.min_node();
        }
        (*self).clone()
    }

    #[inline]
    fn max_node(&self) -> RBNode<K, V> {
        let next_right = self.right();
        if let Some(r) = next_right {
            return r.max_node();
        }
        (*self).clone()
    }
}

struct RBTree<K: Ord + Clone + Debug, V> {
    root: Option<*mut RBNode<K, V>>,
    len: usize,
}

impl<K: Ord + Clone + Debug + Debug, V> fmt::Debug for RBTree<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        let root = self.root;
        let mut node = root;
        if node.is_none() {
            write!(f, "]")?;
            return Ok(());
        }
        unsafe { write!(f, "RBNode( Key: {:?}, Value: {:?}, Color: {:?}), ", &(*node.unwrap()).key, &(*node.unwrap()).value, &(*node.unwrap()).color)?; }
        let mut current_node = unsafe { (*node.unwrap()).min_node() };
        loop {
            write!(f, "RBNode( Key: {:?}, Value: {:?}, Color: {:?}), ", current_node.key, current_node.value, current_node.color)?;
            current_node = match current_node.next() {
                Some(n) => n,
                None => break,
            }
        }
        write!(f, "]")?;
        return Ok(());
    }
}

impl<K: Ord + Clone + Debug, V> RBTree<K, V> {
    fn new() -> RBTree<K, V> {
        RBTree {
            root: None,
            len: 0,
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn insert(&mut self, key: K, value: &V) -> Result<(), String> {
        if self.len == 0 && self.root.is_none() {
            let mut root: *mut RBNode<K, V> = Box::into_raw(Box::from(RBNode::new(key, value)));
            unsafe { (*root).set_color(Color::Black); }
            self.root = Some(root);
            self.len = 1;
            return Ok(());
        }
        let mut current_node_ptr = self.root.ok_or("Error when self.root? in insert method.".to_owned())?;
        loop {
            unsafe {
                if (*current_node_ptr).key == key {
                    unsafe { (*current_node_ptr).value = value as *const V as *mut V };
                    break;
                }
                if (*current_node_ptr).key > key { // keyが現在のnodeのkeyよりも小さい場合
                    if let Some(left) = (*current_node_ptr).left { // 現在のnodeに左側の子が存在した場合は次にそいつと比較する
                        current_node_ptr = left;
                        continue;
                    } else { // 現在のnodeに左側に子が存在しなかった場合はinsertする
                        // insert
                        let node: *mut RBNode<K, V> = Box::into_raw(Box::from(RBNode::new(key, value)));
                        (*current_node_ptr).left = Some(node);
                        (*node).parent = Some(current_node_ptr);
                        self.change_structure(node);
                        self.len += 1;
                        break;
                    }
                }
                if (*current_node_ptr).key < key { // keyが現在のnodeのkeyよりも大きい場合、次にそいつと比較する
                    if let Some(right) = (*current_node_ptr).right {
                        current_node_ptr = right;
                        continue;
                    } else { // 現在のnodeに右側に子が存在しなかった場合はinsertする
                        // insert
                        let node: *mut RBNode<K, V> = Box::into_raw(Box::from(RBNode::new(key, value)));
                        (*current_node_ptr).right = Some(node);
                        (*node).parent = Some(current_node_ptr);
                        self.change_structure(node);
                        self.len += 1;
                        break;
                    }
                }
            }
        }
        Ok(())
    }

    fn change_structure(&mut self, node: *mut RBNode<K, V>) {
        unsafe {
            let mut current_node = node;
            // println!("(*current_node).key: {:?}", (*current_node).key);
            // println!("(*current_node).parent().is_some() && (*current_node).parent().unwrap().is_red() && (((*current_node).parent().unwrap()).parent().is_some() && (*current_node).parent.unwrap() != self.root.unwrap(): {:?}", (*current_node).parent().is_some() && (*current_node).parent().unwrap().is_red() && (((*current_node).parent().unwrap()).parent().is_some() && (*current_node).parent.unwrap() != self.root.unwrap()));
            // 親が赤かつ根じゃない限り続ける
            while (*current_node).parent().is_some() && (*current_node).parent().unwrap().is_red() && (((*current_node).parent().unwrap()).parent().is_some() && (*current_node).parent.unwrap() != self.root.unwrap()) {
                let parent = (*current_node).parent.unwrap();
                // 親が左の子の場合
                if (*parent).is_left() {
                    // println!("(*current_node).parent(): {:?}", (*current_node).parent());
                    // もし叔父が存在しなかった場合、2段登って祖父を右回転させて、祖父(回転後は兄弟)の色を赤に親のの色を黒にして次に進む
                    if (*parent).parent().unwrap().right.is_none() {
                        current_node = (*parent).parent.unwrap();
                        self.rotate(current_node, Rotate::Right);
                        (*(*current_node).parent.unwrap()).color = Color::Black;
                        (*current_node).color = Color::Red;
                        continue;
                    }
                    let uncle = (*parent).parent().unwrap().right.unwrap();
                    // 場合1: 叔父が赤の場合、叔父と親を黒にして、祖父を赤にして二段登る
                    if (*uncle).is_red() {
                        (*uncle).color = Color::Black;
                        (*parent).color = Color::Black;
                        (*(*parent).parent.unwrap()).color = Color::Red;
                        if (*current_node).parent().unwrap().parent().is_none() { break; }
                        current_node = (*current_node).parent().unwrap().parent.unwrap();
                    } else {
                        // 場合2: 叔父が黒で自分が右の子の場合、親を左回転させる
                        if (*current_node).is_right() {
                            current_node = (*current_node).parent.unwrap();
                            self.rotate(current_node, Rotate::Left);
                        }
                        // 場合3: 叔父が黒で自分が左の子の場合、祖父を赤、親を黒にして、祖父を起点に右回転させる
                        (*(*(*current_node).parent.unwrap()).parent.unwrap()).color = Color::Red;
                        (*(*current_node).parent.unwrap()).color = Color::Black;
                        self.rotate((*current_node).parent().unwrap().parent.unwrap(), Rotate::Right);
                    }
                } else { // 親が右の子の場合
                    // もし叔父が存在しなかった場合、2段登って祖父を左回転させて、祖父(回転後は兄弟)の色を赤に親のの色を黒にして次に進む
                    if (*parent).parent().unwrap().left.is_none() {
                        current_node = (*parent).parent.unwrap();
                        self.rotate(current_node, Rotate::Left);
                        (*(*current_node).parent.unwrap()).color = Color::Black;
                        (*current_node).color = Color::Red;
                        continue;
                    }

                    let uncle = (*parent).parent().unwrap().left.unwrap();
                    // 場合1: 叔父が赤の場合黒にする
                    if (*uncle).is_red() {
                        (*uncle).color = Color::Black;
                        (*parent).color = Color::Black;
                        (*(*parent).parent.unwrap()).color = Color::Red;
                        if (*current_node).parent().unwrap().parent().is_none() { break; }
                        current_node = (*current_node).parent().unwrap().parent.unwrap();
                    } else {
                        // 場合2: 叔父が黒で自分が左の子の場合、親を右回転させる
                        if (*current_node).is_left() {
                            current_node = (*current_node).parent.unwrap();
                            self.rotate(current_node, Rotate::Right);
                        }
                        // 場合3: 叔父が黒で自分が右の子の場合、祖父を赤、親を黒にして、祖父を起点に左回転させる
                        (*(*(*current_node).parent.unwrap()).parent.unwrap()).color = Color::Red;
                        (*(*current_node).parent.unwrap()).color = Color::Black;
                        self.rotate((*current_node).parent().unwrap().parent.unwrap(), Rotate::Left);
                    }
                }
            }
            (*self.root.unwrap()).color = Color::Black;
        }
    }

    fn rotate(&mut self, node: *mut RBNode<K, V>, rotate: Rotate) {
        unsafe {
            if rotate == Rotate::Left {
                if (*node).right.is_none() { return; }
                (*(*node).right.unwrap()).parent = (*node).parent; // 右の子の親を自分の親にする
                (*node).parent = (*node).right; // 自分の親を右の子にする
                (*node).right = (*node).right().unwrap().left; // 右の子(今は親)の左の子を自分の右の子にする
                if (*node).right.is_some() { (*(*node).right.unwrap()).parent = Some(node); } // 移動してきた右の子の親を自分にする
                (*(*node).parent.unwrap()).left = Some(node); // 自分の親の左の子を自分にする
            }
            if rotate == Rotate::Right {
                if (*node).left.is_none() { return; }
                (*(*node).left.unwrap()).parent = (*node).parent; // 左の子の親を自分の親にする
                (*node).parent = (*node).left; // 自分の親を左の子にする
                (*node).left = (*node).left().unwrap().right; // 左の子(今は親)の右の子を自分の左の子にする
                if (*node).left.is_some() { (*(*node).left.unwrap()).parent = Some(node); } // 移動してきた左の子の親を自分にする
                (*(*node).parent.unwrap()).right = Some(node); // 自分の親の右の子を自分にする
            }

            if (*node).parent().unwrap().parent().is_some() { // もともとの自分の親(今は祖父)に紐づく子を自分の元子(今は親)に紐付ける
                if (*node).parent().unwrap().parent().unwrap().right == Some(node) {
                    (*(*(*node).parent.unwrap()).parent.unwrap()).right = (*node).parent;
                } else {
                    println!("wrong rotating, 3 {:?}", (*node).parent());
                    (*(*(*node).parent.unwrap()).parent.unwrap()).left = (*node).parent;
                }
            }
            if (*node).parent().unwrap().parent().is_none() { // 回転した結果、子が根になった場合、treeの根を更新する
                self.root = (*node).parent;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn len() {
        let tree: RBTree<usize, usize> = RBTree::new();
        assert_eq!(tree.len(), 0);
    }

    #[test]
    fn insert() {
        let mut tree: RBTree<usize, usize> = RBTree::new();
        tree.insert(10, &2);
        tree.insert(3, &1);
        tree.insert(1, &2);
        tree.insert(5, &3);
        tree.insert(20, &4);
        tree.insert(25, &5);
        tree.insert(30, &5);
        tree.insert(40, &6);
        tree.insert(8, &6);
        tree.insert(9, &6);
        tree.insert(50, &6);
        tree.insert(60, &6);
        assert_eq!(tree.len(), 12);
        // println!("{:#?}", tree);
        assert_eq!(10, unsafe { (*tree.root.unwrap()).key });
        assert_eq!(3, unsafe { (*tree.root.unwrap()).left().unwrap().key });
        assert_eq!(25, unsafe { (*tree.root.unwrap()).right().unwrap().key });
        assert_eq!(1, unsafe { (*tree.root.unwrap()).left().unwrap().left().unwrap().key });
        println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).key }, unsafe { (*tree.root.unwrap()).color });
        println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().color });
        println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().color });
        println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().left().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().left().unwrap().color });
        println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().color });
        println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().left().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().left().unwrap().color });
        println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().color });
        println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().left().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().left().unwrap().color });
        println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().right().unwrap().color });
        println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().left().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().left().unwrap().color });
        println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().right().unwrap().color });
        println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().right().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().right().unwrap().right().unwrap().color });
    }
}