//! cargo testでドキュメントテストを実行
//! cargo doc --no-deps --openでドキュメントをブラウズ

/// Add the two values together.
/// 
/// ```
/// let result = tests::add(1, 2);
/// assert_eq!(result, 3);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}