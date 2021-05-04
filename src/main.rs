use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Debug, Formatter, Error, Display};

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
    leaf: bool,
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
            leaf: self.leaf,
        }
    }
}

impl<K: Debug + Clone + Ord, V> Debug for RBNode<K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "RBNode [ key: {:?}, value: {:?}, color: {:?}, parent: {:#?}, left: {:?}, right: {:?}, leaf: {:?} ]", self.key, self.value, self.color, self.parent, self.left, self.right, self.leaf)?;
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
            leaf: false,
        }
    }

    #[inline]
    fn is_leaf(&self) -> bool { self.leaf }

    #[inline]
    fn is_node(&self) -> bool { !self.leaf }

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
    pub fn new() -> RBTree<K, V> {
        RBTree {
            root: None,
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn find(&self, k: K) -> Option<*mut RBNode<K, V>> {
        let mut node: *mut RBNode<K, V> = if let Some(node) = self.root {
            node
        } else {
            return None;
        };
        loop {
            unsafe {
                if (*node).key == k { return Some(node); }
                if (*node).key > k {
                    node = if let Some(next_node) = (*node).left {
                        next_node
                    } else {
                        return None;
                    };
                } else {
                    node = if let Some(next_node) = (*node).right {
                        next_node
                    } else {
                        return None;
                    };
                }
            }
        }
    }

    pub fn find_minimum(&self, partial: *mut RBNode<K, V>) -> *mut RBNode<K, V> {
        unsafe {
            if (*partial).left.is_none() { return partial; }
            return self.find_minimum((*partial).left.unwrap());
        }
    }

    pub fn insert(&mut self, key: K, value: &V) -> Result<(), String> {
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

    pub fn remove(&mut self, key: K) -> Result<(), String> {
        let remove_node = self.find(key.clone()).ok_or(format!("remove error. There is no key({:?}) in rb-tree.", key))?;
        self.len -= 1;
        unsafe {
            let mut origin_color = (*remove_node).color;
            // fixupの対象は、
            // 削除対象の左右どちらかの子ノードが存在しない場合は、昇格予定のノード
            // 削除対象の左右どちらの子ノードも存在する場合は、削除対象の右側の部分木のミニマムノードの右の子ノード
            // なので、fixupの対象が移動する前に存在していたノードの色が黒だった場合、fixupが発生する
            // 赤だった場合は特にこれが黒になろうと赤のままであろうと性質4(赤が連続してはいけない) or 性質5(任意の葉までパスにおける黒のノードの数は同じ)に違反する可能性はない
            // また、fixupの対象が赤の場合は確実に根ではないことが確定するので、性質2にも違反しない
            let mut fixup_node: Option<*mut RBNode<K, V>> = None;
            if (*remove_node).left.is_none() { // 削除対象の左の子が存在していない場合、削除対象のところに右の子を持ってくる
                let mut right_node = (*remove_node).right;

                fixup_node = if right_node.is_some() {
                    right_node
                } else {
                    // 根だったらNoneを返す
                    if self.root == Some(remove_node) {
                        None
                    } else {
                        // leaf_nodeはすでに削除対象ノードの位置に存在するものとしておく
                        let mut leaf_node = (*remove_node).clone();
                        leaf_node.color = Color::Black;
                        leaf_node.leaf = true;
                        Some(&mut leaf_node as *mut RBNode<K, V>)
                    }
                };
                // fixup_nodeが葉ノードの場合後で親から辿れるようにしたいので、削除ノードが親のどちら側のこのノードだったか記録しておく
                let remove_is_left = (*remove_node).is_left();

                if right_node.is_some() {
                    // 右の子の親を削除対象ノードの親の設定にする
                    (*right_node.unwrap()).parent = (*remove_node).parent;
                }
                // 削除対象の親の(右 or 左の)子を右の子にする
                self.transparent(remove_node, right_node);

                // delete_fixupを実行する場合、親から葉にも辿れるようにしておかないと全部右側判定される
                // 葉の親の(左 or 右)の子を設定するが、delete_fixupの中でその線は削除される予定
                if origin_color == Color::Black {
                    if fixup_node.is_some() && (*fixup_node.unwrap()).is_leaf() {
                        if remove_is_left {
                            (*(*fixup_node.unwrap()).parent.unwrap()).left = fixup_node;
                        } else {
                            (*(*fixup_node.unwrap()).parent.unwrap()).right = fixup_node;
                        }
                    };
                }
            } else if (*remove_node).right.is_none() { // 左の子が存在して、右の子が存在していない場合、削除対象のところに左の子を持ってくる
                let mut left_node = (*remove_node).left;
                // fixup_node = left_node;
                fixup_node = left_node;
                if left_node.is_some() {
                    // 左の子の親を削除対象ノードの親の設定にする
                    (*left_node.unwrap()).parent = (*remove_node).parent;
                }
                // 削除対象の親の(右 or 左の)子を左の子にする
                self.transparent(remove_node, left_node);
            } else { // 右の子も左の子も存在した場合、削除対象の右部分木内でのミニマムなノードとそのミニマムノードの右の子を入れ替えた後で、削除対象の位置にミニマムノードを持ってくる
                let minimum_node = self.find_minimum((*remove_node).right.unwrap());
                origin_color = (*minimum_node).color;
                let minimum_right_node = (*minimum_node).right;
                // fixup_node = minimum_right_node;
                fixup_node = if minimum_right_node.is_some() {
                    minimum_right_node
                } else {
                    let mut leaf_node = (*minimum_node).clone();
                    leaf_node.color = Color::Black;
                    (*leaf_node.parent.unwrap()).left = Some(&mut leaf_node as *mut RBNode<K, V>);
                    leaf_node.leaf = true;
                    Some(&mut leaf_node as *mut RBNode<K, V>)
                };
                // 削除対象がミニマムの親のNodeだった場合、ミニマムノードの右の子の親をミニマムノードにする
                if (*minimum_node).parent == Some(remove_node) {
                    if minimum_right_node.is_some() {
                        (*minimum_right_node.unwrap()).parent = Some(minimum_node);
                    }
                } else { // 削除対象がミニマムの親じゃなかった場合
                    // ミニマムの右の子の親をミニマムの親に設定する
                    if minimum_right_node.is_some() {
                        (*minimum_right_node.unwrap()).parent = (*minimum_node).parent;
                    }
                    self.transparent(minimum_node, minimum_right_node);
                    // 削除対象ノードの位置にミニマムノードを持ってくる
                    // 削除対象ノードの右の子をミニマムの右の子にする
                    (*minimum_node).right = (*remove_node).right;
                    (*(*minimum_node).right.unwrap()).parent = Some(minimum_node);
                }
                // 削除対象の親の(右 or 左の)子をミニマムノードにする
                self.transparent(remove_node, Some(minimum_node));
                // 削除対象ノードとミニマムノードを入れ替えて、色も同じにする
                (*minimum_node).left = (*remove_node).left;
                (*(*remove_node).left.unwrap()).parent = Some(minimum_node);
                (*minimum_node).color = (*remove_node).color;

                // delete_fixupを実行する場合、親から葉にも辿れるようにしておかないと全部右側判定される
                // 葉の親の左の子を設定するが、delete_fixupの中でその線は削除される
                if origin_color == Color::Black {
                    (*(*fixup_node.unwrap()).parent.unwrap()).left = if minimum_right_node.is_some() {
                        minimum_right_node
                    } else {
                        fixup_node
                    };
                }
            }
            if origin_color == Color::Black {
                // delete_fixupにて下記の場合を修正する
                // 1) 削除対象ノードが根だった場合にfixup_nodeが赤ノードだった場合(性質2に違反する)
                // 2) 削除対象ノードの親が赤ノードでfixup_nodeノードが赤だった場合(性質4に違反する)
                // 3) 削除対象ノード or 削除対象ノードに左右の子ノードが存在し、削除対象ノードの部分木内のミニマムノードの色が黒の場合、黒ノードの数が1減ってしまう(性質5に違反)
                // 性質5への違反は、新しく配置されたノードの色を特黒ノードとして、黒のノードが一つ追加されるとして計算すれば解消するが、この場合、性質1のノードが赤か黒であることに違反する
                // このノードは赤黒か黒黒の状態となり、正し、属性は依然として赤か黒のどちらかとなっている。この状態をうまく修正する
                self.delete_fixup(fixup_node);
            }
        }
        Ok(())
    }

    fn change_structure(&mut self, node: *mut RBNode<K, V>) {
        unsafe {
            let mut current_node = node;
            // 親が赤かつ根じゃない限り続ける
            while (*current_node).parent().is_some() && (*current_node).parent().unwrap().is_red() && (((*current_node).parent().unwrap()).parent().is_some() && (*current_node).parent.unwrap() != self.root.unwrap()) {
                let parent = (*current_node).parent.unwrap();
                // 親が左の子の場合
                if (*parent).is_left() {
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
                    (*(*(*node).parent.unwrap()).parent.unwrap()).left = (*node).parent;
                }
            }
            if (*node).parent().unwrap().parent().is_none() { // 回転した結果、子が根になった場合、treeの根を更新する
                self.root = (*node).parent;
            }
        }
    }

    // 昇格する予定のNodeに対しての上から降るパスのみを更新する
    // 昇格する予定のNodeからの上へのパスはすでに更新されていることが前提
    fn transparent(&mut self, removal_node: *mut RBNode<K, V>, promotion_node: Option<*mut RBNode<K, V>>) {
        unsafe {
            // 根の場合
            if (*removal_node).parent.is_none() && self.root.unwrap() == removal_node {
                self.root = promotion_node;
            } else if (*removal_node).is_left() {
                (*(*removal_node).parent.unwrap()).left = promotion_node;
            } else {
                (*(*removal_node).parent.unwrap()).right = promotion_node;
            }
            if promotion_node.is_some() { (*promotion_node.unwrap()).parent = (*removal_node).parent; }
        }
    }

    // fn delete_fixup(&mut self, promotion_node: Option<*mut RBNode<K, V>>) {
    fn delete_fixup(&mut self, promotion_node: Option<*mut RBNode<K, V>>) {
        if promotion_node.is_none() { return; }
        let mut node = promotion_node.unwrap();
        unsafe {
            // 以下の条件のいづれかが成立するまで、木の中の特黒を持ち上げる
            // 1) nodeが赤黒nodeを指す。この場合はwhileを抜けた後で普通の黒に彩色する
            // 2) nodeが根を指す。この場合には単純に特黒を取り除く
            // 3) 適切な回転と再彩色を行ってループを停止する
            // このループの中ではnodeは常に根ではない黒黒を指す
            while (self.root != Some(node) && (*node).parent().is_some()) && (*node).is_black() {
                if (*node).is_left() {
                    // nodeが特黒の場合、性質5のため兄弟は確実に存在している。
                    let mut brother = (*(*node).parent.unwrap()).right.unwrap();
                    // 場合1) 兄弟ノードが赤の場合、その子ノードは黒であり、兄弟ノードを黒、親ノードを赤に変更し、親ノードを左回転することで、場合2 or 場合3 or 場合4にする
                    // 場合2, 3, 4は兄弟ノードが黒の場合の対応である
                    if (*brother).is_red() {
                        (*brother).color = Color::Black;
                        (*(*node).parent.unwrap()).color = Color::Red;
                        self.rotate((*node).parent.unwrap(), Rotate::Left);
                        // 兄弟ノードが赤の場合、その子ノードは確実に二つ存在し、色は黒である
                        // これは、insert時に親と叔父が赤だった場合、祖父を赤にして、親と叔父を黒にすることによって性質4を解消する際に生じる状態？
                        brother = (*(*node).parent.unwrap()).right.unwrap();
                    }
                    // 場合2) 兄弟ノードが黒かつ、兄弟ノードの子の両方が黒または存在しない or 片方が黒で片方が存在しない場合
                    if (*brother).is_black() && ((*brother).right().is_none() || (*brother).right().unwrap().is_black()) && ((*brother).left().is_none() || (*brother).left().unwrap().is_black()) {
                        // 兄弟ノードを赤にしてから特黒ノードをを親にする
                        // もし、特黒ノード(親)が赤だった場合その時点でループが終了する
                        // 場合1を経てきた場合は上の条件を満たすので終了する
                        (*brother).color = Color::Red;

                        // nodeが葉だったら、親の左の子をNoneにして繋がりを消す
                        if (*node).is_leaf() { (*(*node).parent.unwrap()).left = None; }

                        node = (*brother).parent.unwrap();
                    } else {
                        // 場合3) 兄弟ノードが(葉 or 黒)かつ兄弟ノードの右の子が黒かつ兄弟ノードの左の子が赤
                        if (*brother).right.is_none() || (*brother).right().unwrap().is_black() {
                            // 兄弟の左の子の色を黒にし、兄弟ノードを赤にした上で兄弟ノードを起点に右回転し、元の兄弟ノードの位置に上がって来た兄弟ノードの左の子を兄弟ノードとする
                            // これにより場合4に変換される
                            (*(*brother).left.unwrap()).color = Color::Black;
                            (*brother).color = Color::Red;
                            self.rotate(brother, Rotate::Right);
                            brother = (*brother).parent.unwrap();
                        }
                        // 場合4) 兄弟ノードが黒かつ、兄弟ノードの右の子が赤
                        // 兄弟ノードの色を親ノードの色に変換し、親ノードの色を黒に、兄弟ノードの右の子の色を黒にし、親ノードを左回転させて、ノードをrootにしてループを停止する
                        (*brother).color =(*node).parent().unwrap().color;
                        (*(*node).parent.unwrap()).color = Color::Black;
                        (*(*brother).right.unwrap()).color = Color::Black;
                        self.rotate((*node).parent.unwrap(), Rotate::Left);

                        // nodeが葉だったら、親の左の子をNoneにして繋がりを消す
                        if (*node).is_leaf() { (*(*node).parent.unwrap()).left = None; }

                        // 停止
                        node = self.root.unwrap();
                        self.root = Some(node);
                    }
                } else {
                    // nodeが特黒の場合、性質5のため兄弟は確実に存在している。
                    let mut brother = (*(*node).parent.unwrap()).left.unwrap();
                    // 場合1) 兄弟ノードが赤の場合、その子ノードは黒であり、兄弟ノードを黒、親ノードを赤に変更し、親ノードを右回転することで、場合2 or 場合3 or 場合4にする
                    // 場合2, 3, 4は兄弟ノードが黒の場合の対応である
                    if (*brother).is_red() {
                        (*brother).color = Color::Black;
                        (*(*node).parent.unwrap()).color = Color::Red;
                        self.rotate((*node).parent.unwrap(), Rotate::Right);
                        // 兄弟ノードが赤の場合、その子ノードは確実に二つ存在し、色は黒である
                        // これは、insert時に親と叔父が赤だった場合、祖父を赤にして、親と叔父を黒にすることによって性質4を解消する際に生じる状態？
                        brother = (*(*node).parent.unwrap()).left.unwrap();
                    }
                    // 場合2) 兄弟ノードが黒かつ、兄弟ノードの子の両方が黒または存在しない or 片方が黒で片方が存在しない場合
                    if (*brother).is_black() && ((*brother).left().is_none() || (*brother).left().unwrap().is_black()) && ((*brother).right().is_none() || (*brother).right().unwrap().is_black()) {
                        // 兄弟ノードを赤にしてから特黒ノードをを親にする
                        // もし、特黒ノード(親)が赤だった場合その時点でループが終了する
                        // 場合1を経てきた場合は上の条件を満たすので終了する
                        (*brother).color = Color::Red;

                        // nodeが葉だったら、親の右の子をNoneにして繋がりを消す
                        if (*node).is_leaf() { (*(*node).parent.unwrap()).right = None; }

                        node = (*brother).parent.unwrap();
                    } else {
                        // 場合3) 兄弟ノードが(葉 or 黒)かつ兄弟ノードの左の子が黒かつ兄弟ノードの右の子が赤
                        if (*brother).left.is_none() || (*brother).left().unwrap().is_black() {
                            // 兄弟の右の子の色を黒にし、兄弟ノードを赤にした上で兄弟ノードを起点に左回転し、元の兄弟ノードの位置に上がって来た兄弟ノードの右の子を兄弟ノードとする
                            // これにより場合4に変換される
                            (*(*brother).right.unwrap()).color = Color::Black;
                            (*brother).color = Color::Red;
                            self.rotate(brother, Rotate::Left);
                            brother = (*brother).parent.unwrap();
                        }
                        // 場合4) 兄弟ノードが黒かつ、兄弟ノードの左の子が赤
                        // 兄弟ノードの色を親ノードの色に変換し、親ノードの色を黒に、兄弟ノードの左の子の色を黒にし、親ノードを右回転させて、ノードをrootにしてループを停止する
                        (*brother).color =(*node).parent().unwrap().color;
                        (*(*node).parent.unwrap()).color = Color::Black;
                        (*(*brother).left.unwrap()).color = Color::Black;
                        self.rotate((*node).parent.unwrap(), Rotate::Right);

                        // nodeが葉だったら、親の右の子をNoneにして繋がりを消す
                        if (*node).is_leaf() { (*(*node).parent.unwrap()).right = None; }

                        // 停止
                        node = self.root.unwrap();
                        self.root = Some(node);
                    }
                }
            }
            (*node).color = Color::Black;
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

    #[test]
    fn find_minimum() {
        unsafe {
            let mut tree: RBTree<usize, usize> = RBTree::new();
            tree.insert(10, &2);
            let node = tree.find_minimum(tree.root.unwrap());
            assert_eq!(10, unsafe { (*node).key });
            println!("find_minimum, key: {:#?}", (*node).key);
            tree.insert(3, &1);
            let node = tree.find_minimum(tree.root.unwrap());
            assert_eq!(3, unsafe { (*node).key });
            println!("find_minimum, key: {:#?}", (*node).key);
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
            let node = tree.find_minimum(tree.root.unwrap());
            assert_eq!(1, unsafe { (*node).key });
            println!("find_minimum, key: {:#?}", (*node).key);

            let node = tree.find_minimum((*tree.root.unwrap()).left().unwrap().right.unwrap());
            assert_eq!(5, unsafe { (*node).key });
            println!("find_minimum, key: {:#?}", (*node).key);

            let node = tree.find_minimum((*tree.root.unwrap()).right.unwrap());
            assert_eq!(20, unsafe { (*node).key });
            println!("find_minimum, key: {:#?}", (*node).key);

            let node = tree.find_minimum((*tree.root.unwrap()).right().unwrap().right.unwrap());
            assert_eq!(30, unsafe { (*node).key });
            println!("find_minimum, key: {:#?}", (*node).key);

            let node = tree.find_minimum((*tree.root.unwrap()).right().unwrap().right().unwrap().right.unwrap());
            assert_eq!(50, unsafe { (*node).key });
            println!("find_minimum, key: {:#?}", (*node).key);
        }
    }

    #[test]
    fn find() {
        unsafe {
            let mut tree: RBTree<usize, usize> = RBTree::new();
            let node = tree.find(15);
            assert_eq!(None, node);

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
            let node = tree.find(3);
            assert_eq!(3, unsafe { (*node.unwrap()).key });
            assert_eq!(Color::Black, unsafe { (*node.unwrap()).color });

            let node = tree.find(40);
            assert_eq!(40, unsafe { (*node.unwrap()).key });
            assert_eq!(Color::Red, unsafe { (*node.unwrap()).color });

            let node = tree.find(15);
            assert_eq!(None, node);
        }
    }

    #[test]
    fn remove() {
        unsafe {
            let mut tree: RBTree<usize, usize> = RBTree::new();
            tree.insert(10, &2);
            tree.remove(10);
            assert_eq!(0, tree.len());

            tree.insert(10, &2);
            tree.insert(3, &1);
            tree.remove(10);
            assert_eq!(1, tree.len());
            assert_eq!(3, (*tree.root.unwrap()).key);
            assert_eq!(Color::Black, (*tree.root.unwrap()).color);

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
            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).key }, unsafe { (*tree.root.unwrap()).color });
            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().color });
            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().color });
            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().left().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().left().unwrap().color });
            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().color });
            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().left().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().left().unwrap().color });
            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().color });
            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().left().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().left().unwrap().color });
            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().left().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().left().unwrap().color });
            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().right().unwrap().color });

            // 削除の場合3 + delete_fixupの場合4
            tree.remove(20);
            assert_eq!(10, tree.len());

            // println!(" ======= ");
            // println!(" remove 10 ");
            // println!("key: {:#?}, color: {:?}", (*tree.root.unwrap()).key, (*tree.root.unwrap()).color);
            assert_eq!(25, (*tree.root.unwrap()).key);
            assert_eq!(Color::Black, (*tree.root.unwrap()).color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().color });
            assert_eq!(3, (*tree.root.unwrap()).left().unwrap().key);
            assert_eq!(Color::Red, (*tree.root.unwrap()).left().unwrap().color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().color });
            assert_eq!(50, (*tree.root.unwrap()).right().unwrap().key);
            assert_eq!(Color::Red, (*tree.root.unwrap()).right().unwrap().color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().left().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().left().unwrap().color });
            assert_eq!(1, (*tree.root.unwrap()).left().unwrap().left().unwrap().key);
            assert_eq!(Color::Black, (*tree.root.unwrap()).left().unwrap().left().unwrap().color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().color });
            assert_eq!(8, (*tree.root.unwrap()).left().unwrap().right().unwrap().key);
            assert_eq!(Color::Black, (*tree.root.unwrap()).left().unwrap().right().unwrap().color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().left().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().left().unwrap().color });
            assert_eq!(30, (*tree.root.unwrap()).right().unwrap().left().unwrap().key);
            assert_eq!(Color::Black, (*tree.root.unwrap()).right().unwrap().left().unwrap().color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().color });
            assert_eq!(60, (*tree.root.unwrap()).right().unwrap().right().unwrap().key);
            assert_eq!(Color::Black, (*tree.root.unwrap()).right().unwrap().right().unwrap().color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().left().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().left().unwrap().color });
            assert_eq!(5, (*tree.root.unwrap()).left().unwrap().right().unwrap().left().unwrap().key);
            assert_eq!(Color::Red, (*tree.root.unwrap()).left().unwrap().right().unwrap().left().unwrap().color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().right().unwrap().color });
            assert_eq!(9, (*tree.root.unwrap()).left().unwrap().right().unwrap().right().unwrap().key);
            assert_eq!(Color::Red, (*tree.root.unwrap()).left().unwrap().right().unwrap().right().unwrap().color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().left().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().left().unwrap().right().unwrap().color });
            assert_eq!(40, (*tree.root.unwrap()).right().unwrap().left().unwrap().right().unwrap().key);
            assert_eq!(Color::Red, (*tree.root.unwrap()).right().unwrap().left().unwrap().right().unwrap().color);

            // 削除の場合1 + delete_fixupの場合4
            tree.remove(40);
            assert_eq!(9, tree.len());

            // println!(" ======= ");
            // println!(" remove 40 ");

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).key }, unsafe { (*tree.root.unwrap()).color });
            assert_eq!(25, (*tree.root.unwrap()).key);
            assert_eq!(Color::Black, (*tree.root.unwrap()).color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().color });
            assert_eq!(3, (*tree.root.unwrap()).left().unwrap().key);
            assert_eq!(Color::Red, (*tree.root.unwrap()).left().unwrap().color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().color });
            assert_eq!(50, (*tree.root.unwrap()).right().unwrap().key);
            assert_eq!(Color::Red, (*tree.root.unwrap()).right().unwrap().color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().left().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().left().unwrap().color });
            assert_eq!(1, (*tree.root.unwrap()).left().unwrap().left().unwrap().key);
            assert_eq!(Color::Black, (*tree.root.unwrap()).left().unwrap().left().unwrap().color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().color });
            assert_eq!(8, (*tree.root.unwrap()).left().unwrap().right().unwrap().key);
            assert_eq!(Color::Black, (*tree.root.unwrap()).left().unwrap().right().unwrap().color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().left().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().left().unwrap().color });
            assert_eq!(30, (*tree.root.unwrap()).right().unwrap().left().unwrap().key);
            assert_eq!(Color::Black, (*tree.root.unwrap()).right().unwrap().left().unwrap().color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().color });
            assert_eq!(60, (*tree.root.unwrap()).right().unwrap().right().unwrap().key);
            assert_eq!(Color::Black, (*tree.root.unwrap()).right().unwrap().right().unwrap().color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().left().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().left().unwrap().color });
            assert_eq!(5, (*tree.root.unwrap()).left().unwrap().right().unwrap().left().unwrap().key);
            assert_eq!(Color::Red, (*tree.root.unwrap()).left().unwrap().right().unwrap().left().unwrap().color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().right().unwrap().color });
            assert_eq!(9, (*tree.root.unwrap()).left().unwrap().right().unwrap().right().unwrap().key);
            assert_eq!(Color::Red, (*tree.root.unwrap()).left().unwrap().right().unwrap().right().unwrap().color);


            // 削除の場合1 + delete_fixupの場合4(左のケース)
            tree.remove(1);
            assert_eq!(8, tree.len());
            // println!(" ======= ");
            // println!(" remove 1 ");

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).key }, unsafe { (*tree.root.unwrap()).color });
            assert_eq!(25, (*tree.root.unwrap()).key);
            assert_eq!(Color::Black, (*tree.root.unwrap()).color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().color });
            assert_eq!(8, (*tree.root.unwrap()).left().unwrap().key);
            assert_eq!(Color::Red, (*tree.root.unwrap()).left().unwrap().color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().color });
            assert_eq!(50, (*tree.root.unwrap()).right().unwrap().key);
            assert_eq!(Color::Red, (*tree.root.unwrap()).right().unwrap().color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().left().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().left().unwrap().color });
            assert_eq!(3, (*tree.root.unwrap()).left().unwrap().left().unwrap().key);
            assert_eq!(Color::Black, (*tree.root.unwrap()).left().unwrap().left().unwrap().color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().color });
            assert_eq!(9, (*tree.root.unwrap()).left().unwrap().right().unwrap().key);
            assert_eq!(Color::Black, (*tree.root.unwrap()).left().unwrap().right().unwrap().color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().left().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().left().unwrap().color });
            assert_eq!(30, (*tree.root.unwrap()).right().unwrap().left().unwrap().key);
            assert_eq!(Color::Black, (*tree.root.unwrap()).right().unwrap().left().unwrap().color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().color });
            assert_eq!(60, (*tree.root.unwrap()).right().unwrap().right().unwrap().key);
            assert_eq!(Color::Black, (*tree.root.unwrap()).right().unwrap().right().unwrap().color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().left().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().left().unwrap().right().unwrap().color });
            assert_eq!(5, (*tree.root.unwrap()).left().unwrap().left().unwrap().right().unwrap().key);
            assert_eq!(Color::Red, (*tree.root.unwrap()).left().unwrap().left().unwrap().right().unwrap().color);


            // 削除の場合1 + delete_fixupの場合 3 & 4(右のケース)を通る
            tree.remove(9);
            assert_eq!(7, tree.len());
            // println!(" ======= ");
            // println!(" remove 9 ");
            // println!("parent node: {:#?}", unsafe { (*tree.root.unwrap()).clone() });

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).key }, unsafe { (*tree.root.unwrap()).color });
            assert_eq!(25, (*tree.root.unwrap()).key);
            assert_eq!(Color::Black, (*tree.root.unwrap()).color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().color });
            assert_eq!(5, (*tree.root.unwrap()).left().unwrap().key);
            assert_eq!(Color::Red, (*tree.root.unwrap()).left().unwrap().color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().color });
            assert_eq!(50, (*tree.root.unwrap()).right().unwrap().key);
            assert_eq!(Color::Red, (*tree.root.unwrap()).right().unwrap().color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().left().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().left().unwrap().color });
            assert_eq!(3, (*tree.root.unwrap()).left().unwrap().left().unwrap().key);
            assert_eq!(Color::Black, (*tree.root.unwrap()).left().unwrap().left().unwrap().color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().color });
            assert_eq!(8, (*tree.root.unwrap()).left().unwrap().right().unwrap().key);
            assert_eq!(Color::Black, (*tree.root.unwrap()).left().unwrap().right().unwrap().color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().left().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().left().unwrap().color });
            assert_eq!(30, (*tree.root.unwrap()).right().unwrap().left().unwrap().key);
            assert_eq!(Color::Black, (*tree.root.unwrap()).right().unwrap().left().unwrap().color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().color });
            assert_eq!(60, (*tree.root.unwrap()).right().unwrap().right().unwrap().key);
            assert_eq!(Color::Black, (*tree.root.unwrap()).right().unwrap().right().unwrap().color);


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
            assert_eq!(12, tree.len());

            // println!(" ======= ");
            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).key }, unsafe { (*tree.root.unwrap()).color });
            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().color });
            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().color });
            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().left().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().left().unwrap().color });
            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().color });
            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().left().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().left().unwrap().color });
            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().color });
            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().left().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().left().unwrap().color });
            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().right().unwrap().color });
            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().left().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().left().unwrap().color });
            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().right().unwrap().color });
            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().right().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().right().unwrap().right().unwrap().color });

            // 削除の場合1 + delete_fixupの場合 1 & 2(左のケース)を通る
            tree.remove(20);
            assert_eq!(11, tree.len());
            // println!(" ======= ");
            // println!(" remove 20 ");

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).key }, unsafe { (*tree.root.unwrap()).color });
            assert_eq!(10, (*tree.root.unwrap()).key);
            assert_eq!(Color::Black, (*tree.root.unwrap()).color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().color });
            assert_eq!(3, (*tree.root.unwrap()).left().unwrap().key);
            assert_eq!(Color::Black, (*tree.root.unwrap()).left().unwrap().color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().color });
            assert_eq!(40, (*tree.root.unwrap()).right().unwrap().key);
            assert_eq!(Color::Black, (*tree.root.unwrap()).right().unwrap().color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().left().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().left().unwrap().color });
            assert_eq!(1, (*tree.root.unwrap()).left().unwrap().left().unwrap().key);
            assert_eq!(Color::Black, (*tree.root.unwrap()).left().unwrap().left().unwrap().color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().color });
            assert_eq!(8, (*tree.root.unwrap()).left().unwrap().right().unwrap().key);
            assert_eq!(Color::Black, (*tree.root.unwrap()).left().unwrap().right().unwrap().color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().left().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().left().unwrap().color });
            assert_eq!(25, (*tree.root.unwrap()).right().unwrap().left().unwrap().key);
            assert_eq!(Color::Black, (*tree.root.unwrap()).right().unwrap().left().unwrap().color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().color });
            assert_eq!(50, (*tree.root.unwrap()).right().unwrap().right().unwrap().key);
            assert_eq!(Color::Black, (*tree.root.unwrap()).right().unwrap().right().unwrap().color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().left().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().left().unwrap().color });
            assert_eq!(5, (*tree.root.unwrap()).left().unwrap().right().unwrap().left().unwrap().key);
            assert_eq!(Color::Red, (*tree.root.unwrap()).left().unwrap().right().unwrap().left().unwrap().color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().right().unwrap().color });
            assert_eq!(9, (*tree.root.unwrap()).left().unwrap().right().unwrap().right().unwrap().key);
            assert_eq!(Color::Red, (*tree.root.unwrap()).left().unwrap().right().unwrap().right().unwrap().color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().left().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().left().unwrap().right().unwrap().color });
            assert_eq!(30, (*tree.root.unwrap()).right().unwrap().left().unwrap().right().unwrap().key);
            assert_eq!(Color::Red, (*tree.root.unwrap()).right().unwrap().left().unwrap().right().unwrap().color);

            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().right().unwrap().color });
            assert_eq!(60, (*tree.root.unwrap()).right().unwrap().right().unwrap().right().unwrap().key);
            assert_eq!(Color::Red, (*tree.root.unwrap()).right().unwrap().right().unwrap().right().unwrap().color);

            // println!("node(25): {:#?}", unsafe { (*tree.root.unwrap()).right().unwrap().left().unwrap() });
            assert_eq!(None, (*tree.root.unwrap()).right().unwrap().left().unwrap().left);


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
            tree.insert(19, &6);
            assert_eq!(13, tree.len());

            // println!(" ======= ");
            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).key }, unsafe { (*tree.root.unwrap()).color });
            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().color });
            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().color });
            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().left().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().left().unwrap().color });
            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().color });
            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().left().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().left().unwrap().color });
            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().color });
            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().left().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().left().unwrap().color });
            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().right().unwrap().color });
            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().left().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().left().unwrap().color });
            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().right().unwrap().color });
            // println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().right().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().right().unwrap().right().unwrap().color });

            // 削除の場合2 + delete_fixupなしで最後に黒になって終わる場合
            tree.remove(20);
            assert_eq!(12, tree.len());
            println!(" ======= ");
            println!(" remove 20 ");

            println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).key }, unsafe { (*tree.root.unwrap()).color });
            assert_eq!(10, (*tree.root.unwrap()).key);
            assert_eq!(Color::Black, (*tree.root.unwrap()).color);

            println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().color });
            assert_eq!(3, (*tree.root.unwrap()).left().unwrap().key);
            assert_eq!(Color::Black, (*tree.root.unwrap()).left().unwrap().color);

            println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().color });
            assert_eq!(25, (*tree.root.unwrap()).right().unwrap().key);
            assert_eq!(Color::Black, (*tree.root.unwrap()).right().unwrap().color);

            println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().left().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().left().unwrap().color });
            assert_eq!(1, (*tree.root.unwrap()).left().unwrap().left().unwrap().key);
            assert_eq!(Color::Black, (*tree.root.unwrap()).left().unwrap().left().unwrap().color);

            println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().color });
            assert_eq!(8, (*tree.root.unwrap()).left().unwrap().right().unwrap().key);
            assert_eq!(Color::Black, (*tree.root.unwrap()).left().unwrap().right().unwrap().color);

            println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().left().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().left().unwrap().color });
            assert_eq!(19, (*tree.root.unwrap()).right().unwrap().left().unwrap().key);
            assert_eq!(Color::Black, (*tree.root.unwrap()).right().unwrap().left().unwrap().color);

            println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().color });
            assert_eq!(40, (*tree.root.unwrap()).right().unwrap().right().unwrap().key);
            assert_eq!(Color::Red, (*tree.root.unwrap()).right().unwrap().right().unwrap().color);

            println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().left().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().left().unwrap().color });
            assert_eq!(5, (*tree.root.unwrap()).left().unwrap().right().unwrap().left().unwrap().key);
            assert_eq!(Color::Red, (*tree.root.unwrap()).left().unwrap().right().unwrap().left().unwrap().color);

            println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).left().unwrap().right().unwrap().right().unwrap().color });
            assert_eq!(9, (*tree.root.unwrap()).left().unwrap().right().unwrap().right().unwrap().key);
            assert_eq!(Color::Red, (*tree.root.unwrap()).left().unwrap().right().unwrap().right().unwrap().color);

            println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().left().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().left().unwrap().color });
            assert_eq!(30, (*tree.root.unwrap()).right().unwrap().right().unwrap().left().unwrap().key);
            assert_eq!(Color::Black, (*tree.root.unwrap()).right().unwrap().right().unwrap().left().unwrap().color);

            println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().right().unwrap().color });
            assert_eq!(50, (*tree.root.unwrap()).right().unwrap().right().unwrap().right().unwrap().key);
            assert_eq!(Color::Black, (*tree.root.unwrap()).right().unwrap().right().unwrap().right().unwrap().color);

            println!("key: {:#?}, color: {:?}", unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().right().unwrap().right().unwrap().key }, unsafe { (*tree.root.unwrap()).right().unwrap().right().unwrap().right().unwrap().right().unwrap().color });
            assert_eq!(60, (*tree.root.unwrap()).right().unwrap().right().unwrap().right().unwrap().right().unwrap().key);
            assert_eq!(Color::Red, (*tree.root.unwrap()).right().unwrap().right().unwrap().right().unwrap().right().unwrap().color);
        }
    }
}