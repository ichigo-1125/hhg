/*

    Vector

    ----------------------------------------------------------------------------

    # 目次

    - Vector

    - コンストラクタ
    - 要素へのアクセス
    - 状態の取得
    - Vectorの伸縮
    - 分割
    - 入れ替え
    - フィル
    - ソートと検索

    - トレイトの実装

    - macro
    - test


    # TODO

    - Iteratorの実装
    - Extendの実装
    - concat, joinの実装はsliceにゆだねる

*/

#![doc = include_str!("../../doc/collections/vector/vector.md")]

use std::cmp::Ordering;
use std::ops::{ Deref, DerefMut, RangeBounds };
use std::slice::{
    Split, SplitMut,
    SplitInclusive, SplitInclusiveMut,
    RSplit, RSplitMut,
    Chunks, ChunksMut,
    RChunks, RChunksMut,
    ChunksExact, ChunksExactMut,
    RChunksExact, RChunksExactMut,
    Windows,
    SliceIndex
};
use std::vec::Drain;


//------------------------------------------------------------------------------
//  Vector
//------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Vector<T>
{
    pub v: Vec<T>
}


//------------------------------------------------------------------------------
//  コンストラクタ
//------------------------------------------------------------------------------
impl<T> Vector<T>
{
    //--------------------------------------------------------------------------
    //  new
    //
    //  空のVectorを生成する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/new.md")]
    #[inline]
    #[must_use]
    pub fn new() -> Self
    {
        Self { v: Vec::new() }
    }

    //--------------------------------------------------------------------------
    //  with_capacity
    //
    //  容量を指定して空のVectorを生成する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/with_capacity.md")]
    #[inline]
    #[must_use]
    pub fn with_capacity( capacity: usize ) -> Self
    {
        Self { v: Vec::with_capacity(capacity) }
    }
}


//------------------------------------------------------------------------------
//  要素へのアクセス
//------------------------------------------------------------------------------
impl<T> Vector<T>
{
    //--------------------------------------------------------------------------
    //  first
    //
    //  先頭の要素の参照を取得する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/first.md")]
    #[inline]
    pub fn first( &self ) -> Option<&T>
    {
        self.v.first()
    }

    //--------------------------------------------------------------------------
    //  first_mut
    //
    //  先頭の要素の可変参照を取得する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/first_mut.md")]
    #[inline]
    pub fn first_mut( &mut self ) -> Option<&mut T>
    {
        self.v.first_mut()
    }

    //--------------------------------------------------------------------------
    //  last
    //
    //  末尾の要素の参照を取得する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/last.md")]
    #[inline]
    pub fn last( &self ) -> Option<&T>
    {
        self.v.last()
    }

    //--------------------------------------------------------------------------
    //  last_mut
    //
    //  末尾の要素の可変参照を取得する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/last.md")]
    #[inline]
    pub fn last_mut( &mut self ) -> Option<&mut T>
    {
        self.v.last_mut()
    }

    //--------------------------------------------------------------------------
    //  get
    //
    //  指定したインデックスの要素の参照を取得する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/get.md")]
    #[inline]
    pub fn get<I>( &self, index: I ) -> Option<&<I as SliceIndex<[T]>>::Output>
        where
            I: SliceIndex<[T]>,
    {
        self.v.get(index)
    }

    //--------------------------------------------------------------------------
    //  get_mut
    //
    //  指定したインデックスの要素の可変参照を取得する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/get_mut.md")]
    #[inline]
    pub fn get_mut<I>(
        &mut self,
        index: I
    ) -> Option<&mut <I as SliceIndex<[T]>>::Output>
        where
            I: SliceIndex<[T]>,
    {
        self.v.get_mut(index)
    }
}

impl<T: Clone> Vector<T>
{
    //--------------------------------------------------------------------------
    //  to_vec
    //
    //  スライス全体を複製して新しいVecを返す。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/to_vec.md")]
    #[inline]
    pub fn to_vec( &self ) -> Vec<T>
    {
        self.v.to_vec()
    }
}


//------------------------------------------------------------------------------
//  状態の取得
//------------------------------------------------------------------------------
impl<T> Vector<T>
{
    //--------------------------------------------------------------------------
    //  len
    //
    //  要素数を取得する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/len.md")]
    #[inline]
    pub fn len( &self ) -> usize
    {
        self.v.len()
    }

    //--------------------------------------------------------------------------
    //  is_empty
    //
    //  Vectorが空であるか確認する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/is_empty.md")]
    #[inline]
    pub fn is_empty( &self ) -> bool
    {
        self.v.is_empty()
    }

    //--------------------------------------------------------------------------
    //  capacity
    //
    //  容量を取得する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/capacity.md")]
    #[inline]
    pub fn capacity( &self ) -> usize
    {
        self.v.capacity()
    }
}


//------------------------------------------------------------------------------
//  Vectorの伸縮
//------------------------------------------------------------------------------
impl<T> Vector<T>
{
    //--------------------------------------------------------------------------
    //  reserve
    //
    //  容量を確保する。将来の再確保に備えて余分に容量を確保する可能性がある。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/reserve.md")]
    #[inline]
    pub fn reserve( &mut self, additional: usize )
    {
        self.v.reserve(additional);
    }

    //--------------------------------------------------------------------------
    //  reserve_exact
    //
    //  容量を確保する。将来の再確保に備えて余分に容量を確保することはない。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/reserve_exact.md")]
    #[inline]
    pub fn reserve_exact( &mut self, additional: usize )
    {
        self.v.reserve_exact(additional);
    }

    //--------------------------------------------------------------------------
    //  shrink_to_fit
    //
    //  要素数に合わせて容量を縮小する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/shrink_to_fit.md")]
    #[inline]
    pub fn shrink_to_fit( &mut self )
    {
        self.v.shrink_to_fit();
    }

    //--------------------------------------------------------------------------
    //  push
    //
    //  末尾に要素を追加する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/push.md")]
    #[inline]
    pub fn push( &mut self, value: T )
    {
        self.v.push(value);
    }

    //--------------------------------------------------------------------------
    //  pop
    //
    //  末尾の要素を取得する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/pop.md")]
    #[inline]
    pub fn pop( &mut self ) -> Option<T>
    {
        self.v.pop()
    }

    //--------------------------------------------------------------------------
    //  insert
    //
    //  指定したインデックスに要素を挿入する。それ以降の要素は右シフトする。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/insert.md")]
    #[inline]
    pub fn insert( &mut self, index: usize, element: T )
    {
        self.v.insert(index, element);
    }

    //--------------------------------------------------------------------------
    //  remove
    //
    //  指定したインデックスの要素を削除して返す。それ以降の要素は左シフトする。
    //  順番を保持しなくてよい場合はswap_removeの方が高速。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/remove.md")]
    #[inline]
    pub fn remove( &mut self, index: usize ) -> T
    {
        self.v.remove(index)
    }

    //--------------------------------------------------------------------------
    //  resize_with
    //
    //  Vectorをリサイズする。
    //
    //  new_lenが現在の長さよりも小さかった場合、new_lenに合わせて末尾の要素が切
    //  り捨てられる。new_lenが現在の長さよりも大きかった場合、Vectorは拡張され、
    //  valueで埋められる。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/resize_with.md")]
    #[inline]
    pub fn resize_with<F>( &mut self, new_len: usize, f: F )
        where
            F: FnMut() -> T,
    {
        self.v.resize_with(new_len, f)
    }

    //--------------------------------------------------------------------------
    //  truncate
    //
    //  指定されたlenまでVectorを縮小する。lenが現在の要素数よりも小さい場合、
    //  末尾の要素を削除する（ただし容量は変わらない）。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/truncate.md")]
    #[inline]
    pub fn truncate( &mut self, len: usize )
    {
        self.v.truncate(len);
    }

    //--------------------------------------------------------------------------
    //  clear
    //
    //  全ての要素を削除する（ただし容量は変わらない）。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/clear.md")]
    #[inline]
    pub fn clear( &mut self )
    {
        self.v.clear();
    }

    //--------------------------------------------------------------------------
    //  split_off
    //
    //  指定したインデックスでVectorを分割し、末尾のVectorを返す。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/split_off.md")]
    #[inline]
    pub fn split_off( &mut self, at: usize ) -> Vector<T>
    {
        Vector{ v: self.v.split_off(at) }
    }

    //--------------------------------------------------------------------------
    //  append
    //
    //  別のVectorを末尾に移動する。otherは空になる。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/append.md")]
    #[inline]
    pub fn append( &mut self, other: &mut Vector<T> )
    {
        self.v.append(other)
    }

    //--------------------------------------------------------------------------
    //  drain
    //
    //  指定された範囲を一括で削除し、削除された要素をイテレータとして返す。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/drain.md")]
    #[inline]
    pub fn drain<R>( &mut self, range: R ) -> Drain<T>
        where
            R: RangeBounds<usize>,
    {
        self.v.drain(range)
    }

    //--------------------------------------------------------------------------
    //  retain
    //
    //  各要素に対して引数で与えられたクロージャを実行し、falseを返す全ての要素
    //  を削除する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/retain.md")]
    #[inline]
    pub fn retain<F>( &mut self, f: F )
        where
            F: FnMut(&T) -> bool,
    {
        self.v.retain(f)
    }

    //--------------------------------------------------------------------------
    //  dedup_by
    //
    //  隣り合う要素のうち、指定された等値関係を満たす要素を削除する。same_bucket
    //  には、ベクトルから2つの要素への参照が渡され、要素が等しいかどうかを判定
    //  した結果を返す必要がある。要素はスライス内の順と逆の順序で渡されるため、
    //  same_bucket(a, b)がtrueを返す場合は、aが削除される点に注意。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/dedup_by.md")]
    #[inline]
    pub fn dedup_by<F>( &mut self, same_bucket: F )
        where
            F: FnMut(&mut T, &mut T) -> bool,
    {
        self.v.dedup_by(same_bucket);
    }

    //--------------------------------------------------------------------------
    //  dedup_by_key
    //
    //  隣り合う要素のうち、keyを適用した結果が重複する要素（後ろのもの）を削除
    //  する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/dedup_by_key.md")]
    #[inline]
    pub fn dedup_by_key<F, K>( &mut self, key: F )
        where
            F: FnMut(&mut T) -> K,
            K: PartialEq,
    {
        self.v.dedup_by_key(key);
    }
}

impl<T: Clone> Vector<T>
{
    //--------------------------------------------------------------------------
    //  resize
    //
    //  Vectorをリサイズする。
    //
    //  new_lenが現在の長さよりも小さかった場合、new_lenに合わせて末尾の要素が切
    //  り捨てられる。new_lenが現在の長さよりも大きかった場合、Vectorは拡張され、
    //  valueで埋められる。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/resize.md")]
    #[inline]
    pub fn resize( &mut self, new_len: usize, value: T )
    {
        self.v.resize(new_len, value)
    }
}

impl<T: PartialEq> Vector<T>
{
    //--------------------------------------------------------------------------
    //  dedup
    //
    //  隣り合う要素のうち、等しい要素を削除する。Vectorがソートされていれば、
    //  重複する全ての要素が削除される。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/dedup.md")]
    #[inline]
    pub fn dedup( &mut self )
    {
        self.v.dedup();
    }
}


//------------------------------------------------------------------------------
//  分割
//------------------------------------------------------------------------------
impl<T> Vector<T>
{
    //--------------------------------------------------------------------------
    //  split_at
    //
    //  指定したインデックスでVectorを2つのスライスに分ける。元のVectorは変更さ
    //  れない。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/split_at.md")]
    #[inline]
    pub fn split_at( &self, mid: usize ) -> (&[T], &[T])
    {
        self.v.split_at(mid)
    }

    //--------------------------------------------------------------------------
    //  split_at_mut
    //
    //  指定したインデックスでVectorを2つの可変スライスに分ける。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/split_at_mut.md")]
    #[inline]
    pub fn split_at_mut( &mut self, mid: usize ) -> (&mut [T], &mut [T])
    {
        self.v.split_at_mut(mid)
    }

    //--------------------------------------------------------------------------
    //  split_first
    //
    //  Vectorを最初の要素と残りの要素のスライスに分ける。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/split_first.md")]
    #[inline]
    pub fn split_first( &self ) -> Option<(&T, &[T])>
    {
        self.v.split_first()
    }

    //--------------------------------------------------------------------------
    //  split_first_mut
    //
    //  Vectorを最初の要素と残りの要素の可変スライスに分ける。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/split_first_mut.md")]
    #[inline]
    pub fn split_first_mut( &mut self ) -> Option<(&mut T, &mut [T])>
    {
        self.v.split_first_mut()
    }

    //--------------------------------------------------------------------------
    //  split_last
    //
    //  Vectorを最後の要素と残りの要素のスライスに分ける。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/split_last.md")]
    #[inline]
    pub fn split_last( &self ) -> Option<(&T, &[T])>
    {
        self.v.split_last()
    }

    //--------------------------------------------------------------------------
    //  split_last_mut
    //
    //  Vectorを最後の要素と残りの要素の可変スライスに分ける。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/split_last_mut.md")]
    #[inline]
    pub fn split_last_mut( &mut self ) -> Option<(&mut T, &mut [T])>
    {
        self.v.split_last_mut()
    }

    //--------------------------------------------------------------------------
    //  split
    //
    //  各要素に対して引数で与えられたクロージャを実行し、trueを返す要素で区切ら
    //  れたスライスのイテレータを返す。trueを返した要素は含まれない。
    //
    //  最初の要素および最後の要素に対する実行結果がtrueを返した場合、空のスライ
    //  スが挿入される。また、2つの隣り合う要素に対する実行結果がどちらもtrueで
    //  あった場合には、その間に空のスライスが挿入される。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/split.md")]
    #[inline]
    pub fn split<F>( &self, pred: F ) -> Split<'_, T, F>
        where
            F: FnMut( &T ) -> bool,
    {
        self.v.split(pred)
    }

    //--------------------------------------------------------------------------
    //  split_mut
    //
    //  各要素に対して引数で与えられたクロージャを実行し、trueを返す要素で区切ら
    //  れた可変スライスのイテレータを返す。trueを返した要素は含まれない。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/split_mut.md")]
    #[inline]
    pub fn split_mut<F>( &mut self, pred: F ) -> SplitMut<'_, T, F>
        where
            F: FnMut( &T ) -> bool,
    {
        self.v.split_mut(pred)
    }

    //--------------------------------------------------------------------------
    //  split_inclusive
    //
    //  各要素に対して引数で与えられたクロージャを実行し、trueを返す要素で区切ら
    //  れたスライスのイテレータを返す。trueを返した要素は、前のスライスに含ま
    //  れる。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/split_inclusive.md")]
    #[inline]
    pub fn split_inclusive<F>( &self, pred: F ) -> SplitInclusive<'_, T, F>
        where
            F: FnMut( &T ) -> bool,
    {
        self.v.split_inclusive(pred)
    }

    //--------------------------------------------------------------------------
    //  split_inclusive_mut
    //
    //  各要素に対して引数で与えられたクロージャを実行し、trueを返す要素で区切ら
    //  れた可変スライスのイテレータを返す。trueを返した要素は、前のスライスに
    //  含まれる。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/split_inclusive_mut.md")]
    #[inline]
    pub fn split_inclusive_mut<F>( &mut self, pred: F ) -> SplitInclusiveMut<'_, T, F>
        where
            F: FnMut( &T ) -> bool,
    {
        self.v.split_inclusive_mut(pred)
    }

    //--------------------------------------------------------------------------
    //  rsplit
    //
    //  splitを末尾から実行する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/rsplit.md")]
    #[inline]
    pub fn rsplit<F>( &self, pred: F ) -> RSplit<'_, T, F>
        where
            F: FnMut( &T ) -> bool,
    {
        self.v.rsplit(pred)
    }

    //--------------------------------------------------------------------------
    //  rsplit_mut
    //
    //  split_mutを末尾から実行する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/rsplit_mut.md")]
    #[inline]
    pub fn rsplit_mut<F>( &mut self, pred: F ) -> RSplitMut<'_, T, F>
        where
            F: FnMut( &T ) -> bool,
    {
        self.v.rsplit_mut(pred)
    }

    //--------------------------------------------------------------------------
    //  chunks
    //
    //  与えられたchunk_sizeごとに要素を分割し、そのスライスのイテレータを返す。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/chunks.md")]
    #[inline]
    pub fn chunks( &self, chunk_size: usize ) -> Chunks<'_, T>
    {
        self.v.chunks(chunk_size)
    }

    //--------------------------------------------------------------------------
    //  chunks_mut
    //
    //  与えられたchunk_sizeごとに要素を分割し、その可変スライスのイテレータを
    //  返す。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/chunks_mut.md")]
    #[inline]
    pub fn chunks_mut( &mut self, chunk_size: usize ) -> ChunksMut<'_, T>
    {
        self.v.chunks_mut(chunk_size)
    }

    //--------------------------------------------------------------------------
    //  rchunks
    //
    //  chunksを末尾から実行する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/rchunks.md")]
    #[inline]
    pub fn rchunks( &self, chunk_size: usize ) -> RChunks<'_, T>
    {
        self.v.rchunks(chunk_size)
    }

    //--------------------------------------------------------------------------
    //  rchunks_mut
    //
    //  chunks_mutを末尾から実行する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/rchunks_mut.md")]
    #[inline]
    pub fn rchunks_mut( &mut self, chunk_size: usize ) -> RChunksMut<'_, T>
    {
        self.v.rchunks_mut(chunk_size)
    }

    //--------------------------------------------------------------------------
    //  chunks_exact
    //
    //  与えられたchunk_sizeごとに要素を分割し、そのスライスのイテレータを返す。
    //  chunk_sizeに満たないチャンクはイテレータには含まれず、remainder()で取得
    //  できる。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/chunks_exact.md")]
    #[inline]
    pub fn chunks_exact( &self, chunk_size: usize ) -> ChunksExact<'_, T>
    {
        self.v.chunks_exact(chunk_size)
    }

    //--------------------------------------------------------------------------
    //  chunks_exact_mut
    //
    //  与えられたchunk_sizeごとに要素を分割し、その可変スライスのイテレータを
    //  返す。chunk_sizeに満たないチャンクはイテレータには含まれず、remainder()
    //  で取得できる。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/chunks_exact_mut.md")]
    #[inline]
    pub fn chunks_exact_mut( &mut self, chunk_size: usize ) -> ChunksExactMut<'_, T>
    {
        self.v.chunks_exact_mut(chunk_size)
    }

    //--------------------------------------------------------------------------
    //  rchunks_exact
    //
    //  chunks_exactを末尾から実行する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/rchunks_exact.md")]
    #[inline]
    pub fn rchunks_exact( &self, chunk_size: usize ) -> RChunksExact<'_, T>
    {
        self.v.rchunks_exact(chunk_size)
    }

    //--------------------------------------------------------------------------
    //  rchunks_exact_mut
    //
    //  chunks_exactを末尾から実行する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/rchunks_exact_mut.md")]
    #[inline]
    pub fn rchunks_exact_mut( &mut self, chunk_size: usize ) -> RChunksExactMut<'_, T>
    {
        self.v.rchunks_exact_mut(chunk_size)
    }

    //--------------------------------------------------------------------------
    //  windows
    //
    //  与えられたsizeごとに連続するスライスのイテレータを返す。ウィンドウは重
    //  なる。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/windows.md")]
    #[inline]
    pub fn windows( &self, size: usize ) -> Windows<'_, T>
    {
        self.v.windows(size)
    }
}


//------------------------------------------------------------------------------
//  入れ替え
//------------------------------------------------------------------------------
impl<T> Vector<T>
{
    //--------------------------------------------------------------------------
    //  swap
    //
    //  2つのインデックスを指定し、それらの要素を入れ替える。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/swap.md")]
    #[inline]
    pub fn swap( &mut self, a: usize, b: usize )
    {
        self.v.swap(a, b);
    }

    //--------------------------------------------------------------------------
    //  swap_remove
    //
    //  要素を削除して返す。削除された要素はVectorの最後の要素に置き換えられる。
    //  順番は保持しないがremoveより高速。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/swap_remove.md")]
    #[inline]
    pub fn swap_remove( &mut self, index: usize ) -> T
    {
        self.v.swap_remove(index)
    }
}


//------------------------------------------------------------------------------
//  フィル
//------------------------------------------------------------------------------
impl<T> Vector<T>
{
    //--------------------------------------------------------------------------
    //  fill_with
    //
    //  クロージャの返り値でVectorを埋める。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/fill_with.md")]
    #[inline]
    pub fn fill_with<F>( &mut self, f: F )
        where
            F: FnMut() -> T,
    {
        self.v.fill_with(f);
    }
}

impl<T: Clone> Vector<T>
{
    //--------------------------------------------------------------------------
    //  fill
    //
    //  値でVectorを埋める。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/fill.md")]
    #[inline]
    pub fn fill( &mut self, value: T )
    {
        self.v.fill(value);
    }
}


//------------------------------------------------------------------------------
//  ソートと検索
//------------------------------------------------------------------------------
impl<T> Vector<T>
{
    //--------------------------------------------------------------------------
    //  sort_by
    //
    //  与えられたクロージャによりVectorをソートする。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/sort_by.md")]
    #[inline]
    pub fn sort_by<F>( &mut self, compare: F )
        where
            F: FnMut( &T, &T ) -> Ordering,
    {
        self.v.sort_by(compare);
    }

    //--------------------------------------------------------------------------
    //  sort_by_key
    //
    //  キー抽出関数によりVectorをソートする。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/sort_by_key.md")]
    #[inline]
    pub fn sort_by_key<K, F>( &mut self, f: F )
        where
            F: FnMut( &T ) -> K,
            K: Ord,
    {
        self.v.sort_by_key(f);
    }

    //--------------------------------------------------------------------------
    //  reverse
    //
    //  Vectorの要素の順序を逆にする。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/reverse.md")]
    #[inline]
    pub fn reverse( &mut self )
    {
        self.v.reverse();
    }

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
        self.v.binary_search_by_key(b, f)
    }
}

impl<T: Ord> Vector<T>
{
    //--------------------------------------------------------------------------
    //  sort
    //
    //  Vectorをソートする。アルゴリズムはtimsortベースの適応型反復マージソート。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/sort.md")]
    #[inline]
    pub fn sort( &mut self )
    {
        self.v.sort();
    }

    //--------------------------------------------------------------------------
    //  binary_search
    //
    //  特定の要素を検索する。要素が見つかった場合はそのインデックスを返し、見
    //  つからなかった場合は、ソートの順番を維持した場合に検索値が挿入されるべき
    //  インデックスを返す。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/binary_search.md")]
    #[inline]
    pub fn binary_search( &self, x: &T ) -> Result<usize, usize>
    {
        self.v.binary_search(x)
    }
}

impl<T: PartialEq> Vector<T>
{
    //--------------------------------------------------------------------------
    //  contains
    //
    //  特定の要素がVectorに含まれているか検索する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/contains.md")]
    #[inline]
    pub fn contains( &self, x: &T ) -> bool
    {
        self.v.contains(x)
    }

    //--------------------------------------------------------------------------
    //  starts_with
    //
    //  Vectorの先頭の要素がneedleで始まるかどうかを調べる。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/starts_with.md")]
    #[inline]
    pub fn starts_with( &self, needle: &[T] ) -> bool
    {
        self.v.starts_with(needle)
    }

    //--------------------------------------------------------------------------
    //  ends_with
    //
    //  Vectorの末尾の要素がneedleで終わるかどうかを調べる。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/vector/ends_with.md")]
    #[inline]
    pub fn ends_with( &self, needle: &[T] ) -> bool
    {
        self.v.ends_with(needle)
    }
}


//------------------------------------------------------------------------------
//  トレイトの実装
//------------------------------------------------------------------------------

impl<T> Deref for Vector<T>
{
    type Target = Vec<T>;

    //--------------------------------------------------------------------------
    //  deref
    //--------------------------------------------------------------------------
    #[inline]
    fn deref( &self ) -> &Vec<T>
    {
        self.v.as_ref()
    }
}

impl<T> DerefMut for Vector<T>
{
    //--------------------------------------------------------------------------
    //  deref_mut
    //--------------------------------------------------------------------------
    #[inline]
    fn deref_mut( &mut self ) -> &mut Vec<T>
    {
        self.v.as_mut()
    }
}

impl<T, U> PartialEq<U> for Vector<T>
    where
        Vec<T>: PartialEq<U>
{
    //--------------------------------------------------------------------------
    //  eq
    //--------------------------------------------------------------------------
    #[inline]
    fn eq( &self, other: &U ) -> bool
    {
        self.v.eq(other)
    }
}

impl<T> PartialEq<Vector<T>> for Vec<T>
    where
        T: PartialEq
{
    //--------------------------------------------------------------------------
    //  eq
    //--------------------------------------------------------------------------
    #[inline]
    fn eq( &self, other: &Vector<T> ) -> bool
    {
        Vec::eq(self, &other.v)
    }
}

impl<T> FromIterator<T> for Vector<T>
{
    //--------------------------------------------------------------------------
    //  from_iter
    //--------------------------------------------------------------------------
    #[inline]
    fn from_iter<I: IntoIterator<Item=T>>( iter: I ) -> Self
    {
        let mut v = Vector::new();

        for elem in iter
        {
            v.push(elem);
        }

        v
    }
}


//------------------------------------------------------------------------------
//  macro
//------------------------------------------------------------------------------
#[macro_export]
macro_rules! vector
{
    ( $elem:expr ; $n:expr ) =>
    {
        Vector { v: ::std::vec::from_elem($elem, $n) }
    };

    ( $( $x:expr ),* ) =>
    {
        Vector { v: <[_]>::into_vec(Box::new([ $( $x ),* ])) }
    };

    ( $( $x:expr ),+, ) =>
    {
        Vector { v: vec![ $( $x ),* ] }
    };
}
