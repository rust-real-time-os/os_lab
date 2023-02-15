// 杜嘉骏 2020212257
// 实现的是没有头节点的双链表
use std::cmp::Ordering;
use std::fmt::{self, Debug};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::ptr::null_mut;

/// 双链表
pub struct LinkedList<T> {
    // TODO: YOUR CODE HERE
    // 实现的是一个没有头节点的双向链表,即没有元素时head = null
    // head: 链表的第一个元素的裸指针(如果没有节点,则是null)
    // length: 链表中元素的长度
    head: * mut Node<T>,
    length: usize,

    // marker: PhantomData<T>, // 可以去掉
}

/// 链表节点
struct Node<T> {
    // TODO: YOUR CODE HERE
    // prev: 前一个节点的指针
    // next: 后一个节点的指针
    // val: 存储节点的值
    prev: * mut Node<T>,
    next: * mut Node<T>,
    val: Option<T>,

    // marker: PhantomData<T>, // 可以去掉
}

/// 链表迭代器
pub struct Iter<'a, T> {
    // TODO: YOUR CODE HERE
    // ptr_front: 指向链表的第一个节点
    // ptr_back: 指向链表的末尾节点
    // left_cnt: 迭代器中剩余元素个数
    ptr_front: * mut Node<T>,
    ptr_back: * mut Node<T>,
    left_cnt: usize,

    marker: PhantomData<&'a T>,
}

/// 链表可变迭代器
pub struct IterMut<'a, T> {
    // TODO: YOUR CODE HERE
    // ptr_front: 指向链表的第一个节点
    // ptr_back: 指向链表的末尾节点
    // left_cnt: 迭代器中剩余元素个数
    ptr_front: * mut Node<T>,
    ptr_back: * mut Node<T>,
    left_cnt: usize,

    marker: PhantomData<&'a mut T>,
}

impl<T> LinkedList<T> {
    /// 创建一个空链表
    pub fn new() -> Self {
        // Self {
        //     // TODO: YOUR CODE HERE
        //     marker: PhantomData,
        // }
        // 没有元素, head = null
        Self {
            head: null_mut() as * mut Node<T>,
            length: 0_usize,
        }
    }

    /// 将元素插入到链表头部
    ///
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_front(1);
    /// assert_eq!(list.front(), Some(&1));
    /// ```
    pub fn push_front(&mut self, _elem: T) {
        // TODO: YOUR CODE HERE
        // 创建新节点
        let temp_node = Node {
            prev: null_mut() as * mut Node<T>,
            next: null_mut() as * mut Node<T>,
            val: Some(_elem),
        };
        let heap_node = Box::new(temp_node);
        let ptr_node = Box::into_raw(heap_node);

        if self.length == 0_usize {
            // 如果原来没有元素
            unsafe {
                (*ptr_node).prev = ptr_node;
                (*ptr_node).next = ptr_node;
            }
        } else {
            // 插入头部
            unsafe {
                (*ptr_node).next = self.head;
                (*ptr_node).prev = (*(self.head)).prev;
                (*((*(self.head)).prev)).next = ptr_node;
                (*(self.head)).prev = ptr_node;
            }
        }
        // 改变head和length
        self.head = ptr_node;
        self.length += 1_usize;
    }

    /// 将元素插入到链表尾部
    ///
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_back(1);
    /// assert_eq!(list.back(), Some(&1));
    /// ```
    pub fn push_back(&mut self, _elem: T) {
        // TODO: YOUR CODE HERE        
        // 创建新节点
        let temp_node = Node {
            prev: null_mut() as * mut Node<T>,
            next: null_mut() as * mut Node<T>,
            val: Some(_elem),
        };
        let heap_node = Box::new(temp_node);
        let ptr_node = Box::into_raw(heap_node);

        if self.length == 0_usize {
            // 如果原来没有元素
            unsafe {
                (*ptr_node).prev = ptr_node;
                (*ptr_node).next = ptr_node;
            }
            self.head = ptr_node;
        } else {
            unsafe {
                (*ptr_node).prev = (*(self.head)).prev;
                (*ptr_node).next = self.head;
                (*((*(self.head)).prev)).next = ptr_node;
                (*(self.head)).prev = ptr_node;
            }
        }
        self.length += 1_usize;
    }

    /// 将第一个元素返回
    ///
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_front(1);
    /// assert_eq!(list.pop_front(), Some(1));
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        // TODO: YOUR CODE HERE
        // 没有节点返回None
        if self.length == 0 {
            return None
        }
        let ptr_front = self.head;

        self.length -= 1_usize;
        // 将头节点删除,改变head
        if self.length == 0_usize {
            self.head = null_mut() as * mut Node<T>;
        } else {
            unsafe {
                (*((*ptr_front).next)).prev = (*ptr_front).prev;
                (*((*ptr_front).prev)).next = (*ptr_front).next;
                self.head = (*ptr_front).next;
            }
        }
        unsafe {
            Box::from_raw(ptr_front)
        }.val
    }

    /// 将最后一个元素返回
    ///
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_back(1);
    /// assert_eq!(list.pop_back(), Some(1));
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        // TODO: YOUR CODE HERE
        // 没有节点返回None
        if self.length == 0 {
            return None
        }
        let ptr_back = unsafe {
            (*(self.head)).prev
        };
        
        self.length -= 1_usize;
        if self.length == 0_usize {
            // 剩余没有节点
            self.head = null_mut() as * mut Node<T>;
        } else {
            unsafe {
                (*(self.head)).prev = (*ptr_back).prev;
                (*((*ptr_back).prev)).next = self.head;
            }
        }
        unsafe {
            Box::from_raw(ptr_back)
        }.val
    }

    /// 返回链表第一个元素的引用  
    ///
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// assert_eq!(list.front(), None);
    /// list.push_front(1);
    /// assert_eq!(list.front(), Some(&1));
    /// ```
    pub fn front(&self) -> Option<&T> {
        // TODO: YOUR CODE HERE
        if self.length == 0 {
            return None
        }
        unsafe {
            (*self.head).val.as_ref()
        }
    }

    /// 返回链表第一个元素的可变引用   
    pub fn front_mut(&mut self) -> Option<&mut T> {
        // TODO: YOUR CODE HERE
        if self.length == 0 {
            return None
        }
        unsafe {
            (*self.head).val.as_mut()
        }
    }

    /// 返回链表最后一个元素的引用
    ///
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// assert_eq!(list.back(), None);
    /// list.push_back(1);
    /// assert_eq!(list.back(), Some(&1));
    /// ```
    pub fn back(&self) -> Option<&T> {
        // TODO: YOUR CODE HERE
        if self.length == 0 {
            return None
        }
        unsafe {
            (*(*self.head).prev).val.as_ref()
        }
    }

    /// 返回链表最后一个元素的可变引用
    pub fn back_mut(&mut self) -> Option<&mut T> {
        // TODO: YOUR CODE HERE
        if self.length == 0 {
            return None
        }
        unsafe {
            (*(*self.head).prev).val.as_mut()
        }
    }

    /// 返回链表长度
    ///
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_back(1);
    /// assert_eq!(list.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        // TODO: YOUR CODE HERE
        self.length
    }

    /// 判断链表是否为空
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 清空链表
    ///
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_back(1);
    /// list.push_back(2);
    /// assert_eq!(list.len(), 2);
    /// list.clear();
    /// assert_eq!(list.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        // Oh look it's drop again
        while self.pop_front().is_some() {}
    }

    /// 返回一个迭代器
    pub fn iter(&self) -> Iter<T> {
        // Iter {
        //     // TODO: YOUR CODE HERE
        //     marker : PhantomData,
        // }
        Iter {
            ptr_front: self.head,
            ptr_back: unsafe {
                (*(self.head)).prev  
            },
            left_cnt: self.length,

            marker: PhantomData,
        }
    }

    /// 返回一个可变迭代器
    pub fn iter_mut(&mut self) -> IterMut<T> {
        // IterMut {
        //     // TODO: YOUR CODE HERE
        //     marker: PhantomData,
        // }
        IterMut {
            ptr_front: self.head,
            ptr_back: unsafe {
                (*(self.head)).prev
            },
            left_cnt: self.length,

            marker: PhantomData,
        }
    }

    /// 获取链表中指定位置的元素
    /// 如果超出范围，使用panic!宏抛出异常
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_back(1);
    /// assert_eq!(list.get(0), &1);
    /// ```
    pub fn get(&self, _at: usize) -> &T {
        // TODO: YOUR CODE HERE
        // 越界panic!
        if _at >= self.length {
            panic!("get时下标出现越界")
        }
        let mut ptr: * mut Node<T> = self.head;
        // 遍历链表
        for _ in 0.._at {
            ptr = unsafe {
                (*ptr).next
            }
        }
        unsafe {
            (*ptr).val.as_ref().unwrap()
        }
    }

    /// 获取链表中指定位置的可变元素
    pub fn get_mut(&mut self, _at: usize) -> &mut T {
        // TODO: YOUR CODE HERE
        // 越界panic!
        if _at >= self.length {
            panic!("get_mut时下标出现越界")
        }
        let mut ptr: * mut Node<T> = self.head;
        // 遍历链表
        for _ in 0.._at {
            ptr = unsafe {
                (*ptr).next
            }
        }
        unsafe {
            (*ptr).val.as_mut().unwrap()
        }
    }

    /// 将元素插入到**下标为i**的位置    
    /// 如果超出范围，使用panic!宏抛出异常
    ///
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.insert(0,1);
    /// list.insert(1,3);
    /// list.insert(1,2);
    /// assert_eq!(list.get(0), &1);
    /// assert_eq!(list.get(1), &2);
    /// assert_eq!(list.get(2), &3);
    /// ```
    pub fn insert(&mut self, _at: usize, _data: T) {
        // TODO: YOUR CODE HERE
        // 越界panic!
        if _at > self.length {
            panic!("insert时下标出现越界")
        } else if _at == 0 {
            // 没有节点相当于push_front
            self.push_front(_data)
        } else if _at == self.length {
            // 相当于在back插入元素
            self.push_back(_data)
        } else {
            // 创建新节点
            let temp_node = Node {
                prev: null_mut() as * mut Node<T>,
                next: null_mut() as * mut Node<T>,
                val: Some(_data),
            };
            let heap_node = Box::new(temp_node);
            let ptr_node = Box::into_raw(heap_node);

            // 遍历到_at - 1下标的节点
            let mut cur: * mut Node<T> = self.head;
            for _ in 0..(_at - 1) {
                cur = unsafe {
                    (*cur).next
                }
            }
            unsafe {
                (*ptr_node).prev = cur;
                (*ptr_node).next = (*cur).next;
                (*((*cur).next)).prev = ptr_node;
                (*cur).next = ptr_node;
            }
            self.length += 1_usize;
        }
    }

    /// 移除链表中下标为i的元素
    /// 如果超出范围，使用panic!宏抛出异常
    ///
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::from_iter(vec![1,2,3]);
    /// assert_eq!(list.remove(1), 2);
    pub fn remove(&mut self, _at: usize) -> T {
        // TODO: YOUR CODE HERE
        // 越界panic!
        if _at >= self.length {
            panic!("remove时下标出现越界")
        } else if _at == 0 {
            // 相当于pop_front
            self.pop_front().unwrap()
        } else if _at + 1 == self.length {
            // 相当于pop_back
            self.pop_back().unwrap()
        } else {
            let mut ptr: * mut Node<T> = self.head;
            // 遍历到前一个节点
            for _ in 0..(_at - 1) {
                ptr = unsafe {
                    (*ptr).next
                }
            }
            let ptr_target: * mut Node<T> = unsafe {
                (*ptr).next
            };
            unsafe {
                (*((*ptr_target).next)).prev = ptr;
                (*ptr).next = (*ptr_target).next;
            }
            self.length -= 1_usize;
            unsafe {
                Box::from_raw(ptr_target)
            }.val.unwrap()
        }
    }

    /// 将链表分割成两个链表，原链表为[0,at-1]，新链表为[at,len-1]。
    /// 如果超出范围，使用panic!宏抛出异常
    ///
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::from_iter(vec![1,2,3,4,5]);
    /// let new = list.split_off(2); // list = [1,2], new = [3,4,5]
    /// assert_eq!(list.len(), 2);
    /// assert_eq!(list.pop_front(), Some(1));
    /// assert_eq!(list.pop_front(), Some(2));
    pub fn split_off(&mut self, _at: usize) -> LinkedList<T> {
        // TODO: YOUR CODE HERE
        // _at的范围在[0, self.length]
        // 越界panic!
        if _at > self.length {
            panic!("split_off时出现下标越界")
        } else if _at == 0 {
            // 直接将self赋值给new_list即可
            let mut new_list = Self::new();
            new_list.head = self.head;
            new_list.length = self.length;
            self.head = null_mut() as * mut Node<T>;
            self.length = 0_usize;
            new_list
        } else if _at == self.length {
            Self::new()
        } else {
            let mut ptr: * mut Node<T> = self.head;
            // 遍历到下标为 _at - 1的节点
            for _ in 0..(_at - 1) {
                ptr = unsafe {
                    (*ptr).next
                }
            }
            let mut new_list = Self::new();
            new_list.head = unsafe {
                (*ptr).next
            };
            new_list.length = self.length - _at;
            self.length = _at;
            unsafe {
                (*((*(self.head)).prev)).next = new_list.head;
                (*(new_list.head)).prev = (*(self.head)).prev;
                (*(self.head)).prev = ptr;
                (*ptr).next = self.head;
            }
            new_list
        }
    }

    /// 查找链表中第一个满足条件的元素
    /// 
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::from_iter(vec![1,2,3]);
    /// assert_eq!(list.find_mut(|x| x % 2 == 0), Some(&mut 2));
    /// assert_eq!(list.find_mut(|x| x % 4 == 0), None);
    /// ```
    pub fn find_mut<P>(&mut self,predicate:P)->Option<&mut T> where P:Fn(&T) -> bool{
        // TODO: YOUR CODE HERE
        let mut ptr = self.head;
        // 遍历节点的值
        for _ in 0..self.len() {
            if predicate(unsafe {(*ptr).val.as_ref()}.unwrap()) {
                return unsafe {
                    (*ptr).val.as_mut()
                }
            }
            ptr = unsafe {(*ptr).next}
        }
        None
    }
}

impl<T: PartialEq> LinkedList<T> {
    /// 判断链表中是否包含某个元素
    ///
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_back(1);
    /// assert_eq!(list.contains(&1), true);
    /// assert_eq!(list.contains(&2), false);
    /// ```
    pub fn contains(&mut self, _data: &T) -> bool {
        // TODO: YOUR CODE HERE
        // 遍历节点的值
        let mut ptr = self.head;
        for _ in 0..self.len() {
            if _data == unsafe {(*ptr).val.as_ref().unwrap()} {
                return true
            }
            ptr = unsafe {
                (*ptr).next
            }
        }
        false
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type IntoIter = Iter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type IntoIter = IterMut<'a, T>;
    type Item = &'a mut T;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    // 返回下一个元素，当没有元素可返回时，返回None
    fn next(&mut self) -> Option<Self::Item> {
        // TODO: YOUR CODE HERE
        if self.left_cnt == 0 {
            None
        } else {
            // 直接返回self.ptr_front中的值即可
            let ptr = self.ptr_front;
            self.left_cnt -= 1_usize;
            self.ptr_front = unsafe {
                (*(self.ptr_front)).next
            };
            unsafe {
                (*ptr).val.as_ref()
            }
        }
    }

    // 返回(self.len, Some(self.len))即可
    fn size_hint(&self) -> (usize, Option<usize>) {
        // TODO: YOUR CODE HERE
        // 返回剩余节点的数量
        (self.left_cnt, Some(self.left_cnt))
    }
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: YOUR CODE HERE
        if self.left_cnt == 0 {
            None
        } else {
            // 直接返回self.ptr_front中的值即可
            let ptr = self.ptr_front;
            self.left_cnt -= 1_usize;
            self.ptr_front = unsafe {
                (*(self.ptr_front)).next
            };
            unsafe {
                (*ptr).val.as_mut()
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        // TODO: YOUR CODE HERE
        // 返回剩余节点的数量
        (self.left_cnt, Some(self.left_cnt))
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    // 返回前一个元素
    // 返回迭代器中最后面的元素
    fn next_back(&mut self) -> Option<Self::Item> {
        // TODO: YOUR CODE HERE
        if self.left_cnt == 0 {
            None
        } else {
            // 直接返回self.ptr_back中的值即可
            let ptr = self.ptr_back;
            self.left_cnt -= 1_usize;
            self.ptr_back = unsafe {
                (*(self.ptr_back)).prev
            };
            unsafe {
                (*ptr).val.as_ref()
            }
        }
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        // TODO: YOUR CODE HERE
        if self.left_cnt == 0 {
            None
        } else {
            // 直接返回self.ptr_back中的值即可
            let ptr = self.ptr_back;
            self.left_cnt -= 1_usize;
            self.ptr_back = unsafe {
                (*(self.ptr_back)).prev
            };
            unsafe {
                (*ptr).val.as_mut()
            }
        }
    }
}

/// 提供归并排序的功能
pub trait MergeSort {
    /// 就地进行归并排序，按从小到大排序
    fn merge_sort(&mut self);
}

impl<T: PartialOrd + Default> LinkedList<T> {
    // 你可以在这里添加你需要的辅助函数
    pub fn merge(&mut self, another:& mut LinkedList<T>) {
        let mut res_list: LinkedList<T> = LinkedList::new();

        // 合并self和another中的节点到res_list中
        while ! self.is_empty() && ! another.is_empty() {
            if self.front() <= another.front() {
                res_list.push_back(self.pop_front().unwrap())
            } else {
                res_list.push_back(another.pop_front().unwrap())
            }
        }

        while ! self.is_empty() {
            res_list.push_back(self.pop_front().unwrap())
        }
        while ! another.is_empty() {
            res_list.push_back(another.pop_front().unwrap())
        }

        // self变为有序的合并链表
        while ! res_list.is_empty() {
            self.push_back(res_list.pop_front().unwrap())
        }
    }
}

impl<T: PartialOrd + Default> MergeSort for LinkedList<T> {
    fn merge_sort(&mut self) {
        // TODO: YOUR CODE HERE
        if self.len() <= 1 {
            return
        }
        // 将链表分为等长的两部分,分别进行归并排序,然后合并即可
        let mut right = self.split_off(self.len() / 2);
        self.merge_sort();
        right.merge_sort();
        self.merge(&mut right);
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        // Pop until we have to stop
        while self.pop_front().is_some() {}
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        let mut new_list = Self::new();
        for item in self {
            new_list.push_back(item.clone());
        }
        new_list
    }
}
impl<T> Extend<T> for LinkedList<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push_back(item);
        }
    }
}
impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        list.extend(iter);
        list
    }
}

impl<T: Debug> Debug for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other)
    }
}

impl<T: Eq> Eq for LinkedList<T> {}

impl<T: PartialOrd> PartialOrd for LinkedList<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.iter().partial_cmp(other)
    }
}

impl<T: Ord> Ord for LinkedList<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(other)
    }
}

impl<T: Hash> Hash for LinkedList<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.len().hash(state);
        for item in self {
            item.hash(state);
        }
    }
}

unsafe impl<T: Send> Send for LinkedList<T> {}
unsafe impl<T: Sync> Sync for LinkedList<T> {}

unsafe impl<'a, T: Send> Send for Iter<'a, T> {}
unsafe impl<'a, T: Sync> Sync for Iter<'a, T> {}

unsafe impl<'a, T: Send> Send for IterMut<'a, T> {}
unsafe impl<'a, T: Sync> Sync for IterMut<'a, T> {}