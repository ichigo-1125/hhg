/*

    Deque

    ----------------------------------------------------------------------------

    # 目次

    - Deque

    - コンストラクタ
    - 要素へのアクセス
    - 状態の取得
    - Dequeの伸縮
    - ソートと検索

    - トレイとの実装

    - macro

*/

#![doc = include_str!("../../doc/collections/deque/deque.md")]

use std::collections::{ VecDeque, vec_deque::Drain };
use std::ops::{ Deref, DerefMut, RangeBounds };


//------------------------------------------------------------------------------
//  Deque
//------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Deque<T>
{
    pub deq: VecDeque<T>,
}


//------------------------------------------------------------------------------
//  コンストラクタ
//------------------------------------------------------------------------------
impl<T> Deque<T>
{
    //--------------------------------------------------------------------------
    //  new
    //
    //  空のDequeを生成する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/deque/new.md")]
    #[inline]
    #[must_use]
    pub fn new() -> Self
    {
        Self { deq: VecDeque::new() }
    }

    //--------------------------------------------------------------------------
    //  with_capacity
    //
    //  容量を指定して空のVectorを生成する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/deque/with_capacity.md")]
    #[inline]
    #[must_use]
    pub fn with_capacity( capacity: usize ) -> Self
    {
        Self { deq: VecDeque::with_capacity(capacity) }
    }
}


//------------------------------------------------------------------------------
//  要素へのアクセス
//------------------------------------------------------------------------------
impl<T> Deque<T>
{
    //--------------------------------------------------------------------------
    //  get
    //
    //  指定したインデックスの要素の参照を取得する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/deque/get.md")]
    #[inline]
    pub fn get( &self, index: usize ) -> Option<&T>
    {
        self.deq.get(index)
    }

    //--------------------------------------------------------------------------
    //  get_mut
    //
    //  指定したインデックスの要素の可変参照を取得する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/deque/get_mut.md")]
    #[inline]
    pub fn get_mut( &mut self, index: usize ) -> Option<&mut T>
    {
        self.deq.get_mut(index)
    }
}


//------------------------------------------------------------------------------
//  状態の取得
//------------------------------------------------------------------------------
impl<T> Deque<T>
{
    //--------------------------------------------------------------------------
    //  len
    //
    //  要素数を取得する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/deque/len.md")]
    #[inline]
    pub fn len( &self ) -> usize
    {
        self.deq.len()
    }

    //--------------------------------------------------------------------------
    //  is_empty
    //
    //  Dequeが空であるか確認する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/deque/is_empty.md")]
    #[inline]
    pub fn is_empty( &self ) -> bool
    {
        self.deq.is_empty()
    }

    //--------------------------------------------------------------------------
    //  capacity
    //
    //  容量を取得する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/deque/capacity.md")]
    #[inline]
    pub fn capacity( &self ) -> usize
    {
        self.deq.capacity()
    }
}


//------------------------------------------------------------------------------
//  Dequeの伸縮
//------------------------------------------------------------------------------
impl<T> Deque<T>
{
    //--------------------------------------------------------------------------
    //  reserve
    //
    //  容量を確保する。将来の再確保に備えて余分に容量を確保する可能性がある。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/deque/reserve.md")]
    #[inline]
    pub fn reserve( &mut self, additional: usize )
    {
        self.deq.reserve(additional);
    }

    //--------------------------------------------------------------------------
    //  reserve_exact
    //
    //  容量を確保する。将来の再確保に備えて余分に容量を確保することはない。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/deque/reserve_exact.md")]
    #[inline]
    pub fn reserve_exact( &mut self, additional: usize )
    {
        self.deq.reserve_exact(additional);
    }

    //--------------------------------------------------------------------------
    //  shrink_to_fit
    //
    //  要素数に合わせて容量を縮小する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/deque/shrink_to_fit.md")]
    #[inline]
    pub fn shrink_to_fit( &mut self )
    {
        self.deq.shrink_to_fit();
    }

    //--------------------------------------------------------------------------
    //  push_front
    //
    //  先頭に要素を追加する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/deque/push_front.md")]
    #[inline]
    pub fn push_front( &mut self, value: T )
    {
        self.deq.push_front(value);
    }

    //--------------------------------------------------------------------------
    //  push_back
    //
    //  末尾に要素を追加する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/deque/push_back.md")]
    #[inline]
    pub fn push_back( &mut self, value: T )
    {
        self.deq.push_back(value);
    }

    //--------------------------------------------------------------------------
    //  pop_front
    //
    //  先頭の要素を取得する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/deque/pop_front.md")]
    #[inline]
    pub fn pop_front( &mut self ) -> Option<T>
    {
        self.deq.pop_front()
    }

    //--------------------------------------------------------------------------
    //  pop_back
    //
    //  末尾の要素を取得する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/deque/pop_back.md")]
    #[inline]
    pub fn pop_back( &mut self ) -> Option<T>
    {
        self.deq.pop_back()
    }

    //--------------------------------------------------------------------------
    //  insert
    //
    //  指定したインデックスに要素を挿入する。それ以降の要素は右シフトする。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/deque/insert.md")]
    #[inline]
    pub fn insert( &mut self, index: usize, element: T )
    {
        self.deq.insert(index, element);
    }

    //--------------------------------------------------------------------------
    //  remove
    //
    //  指定したインデックスの要素を削除して返す。それ以降の要素は左シフトする。
    //  順番を保持しなくてよい場合はswap_remove_frontやswap_remove_backの方が高
    //  速。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/deque/remove.md")]
    #[inline]
    pub fn remove( &mut self, index: usize ) -> Option<T>
    {
        self.deq.remove(index)
    }

    //--------------------------------------------------------------------------
    //  resize_with
    //
    //  Dequeをリサイズする。
    //
    //  new_lenが現在の長さよりも小さかった場合、new_lenに合わせて末尾の要素が切
    //  り捨てられる。new_lenが現在の長さよりも大きかった場合、Dequeは拡張され、
    //  valueで埋められる。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/deque/resize_with.md")]
    #[inline]
    pub fn resize_with<F>( &mut self, new_len: usize, f: F )
        where
            F: FnMut() -> T,
    {
        self.deq.resize_with(new_len, f)
    }

    //--------------------------------------------------------------------------
    //  truncate
    //
    //  指定されたlenまでDequeを縮小する。lenが現在の要素数よりも小さい場合、
    //  末尾の要素を削除する（ただし容量は変わらない）。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/deque/truncate.md")]
    #[inline]
    pub fn truncate( &mut self, len: usize )
    {
        self.deq.truncate(len);
    }

    //--------------------------------------------------------------------------
    //  clear
    //
    //  全ての要素を削除する（ただし容量は変わらない）。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/deque/clear.md")]
    #[inline]
    pub fn clear( &mut self )
    {
        self.deq.clear();
    }

    //--------------------------------------------------------------------------
    //  split_off
    //
    //  指定したインデックスでDequeを分割し、末尾のDequeを返す。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/deque/split_off.md")]
    #[inline]
    pub fn split_off( &mut self, at: usize ) -> Deque<T>
    {
        Deque { deq: self.deq.split_off(at) }
    }

    //--------------------------------------------------------------------------
    //  append
    //
    //  別のDequeを末尾に移動する。otherは空になる。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/deque/append.md")]
    #[inline]
    pub fn append( &mut self, other: &mut Deque<T> )
    {
        self.deq.append(other)
    }

    //--------------------------------------------------------------------------
    //  drain
    //
    //  指定された範囲を一括で削除し、削除された要素をイテレータとして返す。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/deque/drain.md")]
    #[inline]
    pub fn drain<R>( &mut self, range: R ) -> Drain<T>
        where
            R: RangeBounds<usize>,
    {
        self.deq.drain(range)
    }

    //--------------------------------------------------------------------------
    //  retain
    //
    //  各要素に対して引数で与えられたクロージャを実行し、falseを返す全ての要素
    //  を削除する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/deque/retain.md")]
    #[inline]
    pub fn retain<F>( &mut self, f: F )
        where
            F: FnMut(&T) -> bool,
    {
        self.deq.retain(f)
    }
}

impl<T: Clone> Deque<T>
{
    //--------------------------------------------------------------------------
    //  resize
    //
    //  Dequeをリサイズする。
    //
    //  new_lenが現在の長さよりも小さかった場合、new_lenに合わせて末尾の要素が切
    //  り捨てられる。new_lenが現在の長さよりも大きかった場合、Dequeは拡張され、
    //  valueで埋められる。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/deque/resize.md")]
    #[inline]
    pub fn resize( &mut self, new_len: usize, value: T )
    {
        self.deq.resize(new_len, value)
    }
}


//------------------------------------------------------------------------------
//  入れ替え
//------------------------------------------------------------------------------
impl<T> Deque<T>
{
    //--------------------------------------------------------------------------
    //  swap
    //
    //  2つのインデックスを指定し、それらの要素を入れ替える。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/deque/swap.md")]
    #[inline]
    pub fn swap( &mut self, a: usize, b: usize )
    {
        self.deq.swap(a, b);
    }

    //--------------------------------------------------------------------------
    //  swap_remove_front
    //
    //  要素を削除して返す。削除された要素はVectorの最初の要素に置き換えられる。
    //  順番は保持しないがremoveより高速。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/deque/swap_remove_front.md")]
    #[inline]
    pub fn swap_remove_front( &mut self, index: usize ) -> Option<T>
    {
        self.deq.swap_remove_front(index)
    }

    //--------------------------------------------------------------------------
    //  swap_remove_back
    //
    //  要素を削除して返す。削除された要素はVectorの最後の要素に置き換えられる。
    //  順番は保持しないがremoveより高速。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/deque/swap_remove_back.md")]
    #[inline]
    pub fn swap_remove_back( &mut self, index: usize ) -> Option<T>
    {
        self.deq.swap_remove_back(index)
    }
}


//------------------------------------------------------------------------------
//  ソートと検索
//------------------------------------------------------------------------------
impl<T> Deque<T>
{
    //--------------------------------------------------------------------------
    //  binary_search_by_key
    //
    //  Vectorをキー抽出関数でバイナリ検索する。ソート済みのVectorであれば、
    //  containsと同様に動作する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/binary_search_by_key.md")]
    #[inline]
    pub fn binary_search_by_key<'a, F, B>(
        &'a self,
        b: &B,
        f: F
    ) -> Result<usize, usize>
        where
            F: FnMut( &'a T ) -> B,
            B: Ord,
    {
        self.deq.binary_search_by_key(b, f)
    }
}


//------------------------------------------------------------------------------
//  トレイトの実装
//------------------------------------------------------------------------------

impl<T> Deref for Deque<T>
{
    type Target = VecDeque<T>;

    //--------------------------------------------------------------------------
    //  deref
    //--------------------------------------------------------------------------
    #[inline]
    fn deref( &self ) -> &VecDeque<T>
    {
        &self.deq
    }
}

impl<T> DerefMut for Deque<T>
{
    //--------------------------------------------------------------------------
    //  deref_mut
    //--------------------------------------------------------------------------
    #[inline]
    fn deref_mut( &mut self ) -> &mut VecDeque<T>
    {
        &mut self.deq
    }
}

impl<T, U> PartialEq<U> for Deque<T>
    where
        VecDeque<T>: PartialEq<U>,
{
    //--------------------------------------------------------------------------
    //  eq
    //--------------------------------------------------------------------------
    #[inline]
    fn eq( &self, other: &U ) -> bool
    {
        self.deq.eq(other)
    }
}

impl<T> PartialEq<Deque<T>> for VecDeque<T>
    where
        T: PartialEq,
{
    //--------------------------------------------------------------------------
    //  eq
    //--------------------------------------------------------------------------
    #[inline]
    fn eq( &self, other: &Deque<T> ) -> bool
    {
        VecDeque::eq(self, &other.deq)
    }
}

impl<T> FromIterator<T> for Deque<T>
{
    //--------------------------------------------------------------------------
    //  from_iter
    //--------------------------------------------------------------------------
    #[inline]
    fn from_iter<I: IntoIterator<Item=T>>( iter: I ) -> Self
    {
        let mut deq = Deque::new();

        for elem in iter
        {
            deq.push_back(elem);
        }

        deq
    }
}


//------------------------------------------------------------------------------
//  macro
//------------------------------------------------------------------------------
#[macro_export]
macro_rules! deque
{
    () =>
    {
        Deque { deq: ::std::collections::VecDeque::new() }
    };

    ( $elem: expr; $n: expr ) =>
    {
        {
            let mut deq = ::std::collections::VecDeque::new();
            deq.resize_with($n, || $elem);
            Deque { deq }
        }
    };

    ( $( $elem: expr ),+ $(,)? ) =>
    {
        {
            const CAP: usize = $crate::count!($($elem),*);
            let mut deq = ::std::collections::VecDeque::with_capacity(CAP);
            $(
                deq.push_back($elem);
            )+
            Deque { deq }
        }
    };
}
