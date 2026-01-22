//
//! # Haskell 风格语法糖宏
//!
//! 本模块提供了三个宏，用于在 Rust 中模拟 Haskell 的 Guards 和模式匹配语法。
//!
//! - [`guard!`] - Haskell 风格的 guard 表达式
//! - [`fn_guard!`] - 带 guard 语法的函数定义
//! - [`case!`] - Haskell 风格的 case 表达式
#[allow(dead_code)]
#[allow(non_upper_case_globals)]
const otherwise: bool = true;
/// Haskell 风格的 Guard 表达式宏
///
/// 模拟 Haskell 中的 guards 语法，允许使用 `| condition => result` 的形式
/// 进行条件分支判断，类似于 Haskell 的：
///
/// ```haskell
/// | condition1 = result1
/// | condition2 = result2
/// | otherwise  = defaultResult
/// ```
///
/// # 语法
///
/// ```text
/// guard!(
///     | condition1 => result1,
///     | condition2 => result2,
///     | otherwise => default_result,
///     where binding1 = value1, binding2 = value2,  // 可选的 where 子句
/// )
/// ```
///
/// # 特性
///
/// - 支持 `otherwise` 关键字作为默认分支（类似 Haskell）
/// - 支持 `where` 子句定义局部绑定
/// - 如果没有 `otherwise` 且所有条件都不满足，将会 panic
///
/// # Examples
///
/// ## 基本用法
///
/// ```
/// # macro_rules! guard {
/// #     (
/// #       $(| $cond:expr => $result:expr),+ $(,)?,
/// #       where $($binding:ident = $val:expr),+ $(,)?
/// #     ) => {{
/// #       $(let $binding = $val;)+
/// #        guard!($(| $cond => $result),+)
/// #     }};
/// #     (| otherwise => $result:expr $(,)?) => {
/// #        $result
/// #     };
/// #     (| $cond:expr => $result:expr,$($rest:tt)+) => {
/// #       if $cond {
/// #            $result
/// #        }
/// #       else{
/// #            guard!($($rest)+)
/// #        }
/// #     };
/// #     (| $cond:expr => $result:expr $(,)?) => {
/// #         if $cond {$result} else {panic!("Non-exhaustive guards")}
/// #     };
/// # }
/// let x = 10;
/// let result = guard!(
///     | x < 0 => "negative",
///     | x == 0 => "zero",
///     | x > 0 => "positive",
///     | otherwise => "unreachable",
/// );
/// assert_eq!(result, "positive");
/// ```
///
/// ## 使用 where 子句
///
/// ```
/// # const otherwise:bool = true;
/// # macro_rules! guard {
/// #     (
/// #       $(| $cond:expr => $result:expr),+ $(,)?,
/// #       where $($binding:ident = $val:expr),+ $(,)?
/// #     ) => {{
/// #       $(let $binding = $val;)+
/// #        guard!($(| $cond => $result),+)
/// #     }};
/// #     (| otherwise => $result:expr $(,)?) => {
/// #        $result
/// #     };
/// #     (| $cond:expr => $result:expr,$($rest:tt)+) => {
/// #       if $cond {
/// #            $result
/// #        }
/// #       else{
/// #            guard!($($rest)+)
/// #        }
/// #     };
/// #     (| $cond:expr => $result:expr $(,)?) => {
/// #         if $cond {$result} else {panic!("Non-exhaustive guards")}
/// #     };
/// # }
/// let a = 3;
/// let b = 4;
/// let result = guard!(
///     | hyp < 5.0 => "small",
///     | hyp < 10.0 => "medium",
///     | otherwise => "large",
///     where hyp = ((a * a + b * b) as f64).sqrt(),
/// );
/// assert_eq!(result, "medium");
/// ```
///
/// # Panics
///
/// 当所有条件都不满足且没有 `otherwise` 分支时，会触发 panic：
///
/// ```should_panic
/// # const otherwise:bool =  true;
/// # macro_rules! guard {
/// #     (
/// #       $(| $cond:expr => $result:expr),+ $(,)?,
/// #       where $($binding:ident = $val:expr),+ $(,)?
/// #     ) => {{
/// #       $(let $binding = $val;)+
/// #        guard!($(| $cond => $result),+)
/// #     }};
/// #     (| otherwise => $result:expr $(,)?) => {
/// #        $result
/// #     };
/// #     (| $cond:expr => $result:expr,$($rest:tt)+) => {
/// #       if $cond {
/// #            $result
/// #        }
/// #       else{
/// #            guard!($($rest)+)
/// #        }
/// #     };
/// #     (| $cond:expr => $result:expr $(,)?) => {
/// #         if $cond {$result} else {panic!("Non-exhaustive guards")}
/// #     };
/// # }
/// let x = 0;
/// guard!(
///     | x > 0 => "positive",
///     | x < 0 => "negative",
///     // 缺少 otherwise，x == 0 时会 panic
/// );
/// ```
#[macro_export]
macro_rules! guard {
    (
      $(| $cond:expr => $result:expr),+ $(,)?,
      where $($binding:ident = $val:expr),+ $(,)?
    ) => {{
      $(let $binding = $val;)+
       guard!($(| $cond => $result),+)
    }};
    (| otherwise => $result:expr $(,)?) => {
       $result
    };
    (| $cond:expr => $result:expr,$($rest:tt)+) => {
      if $cond {
           $result
       }
      else{
           guard!($($rest)+)
       }
    };
    (| $cond:expr => $result:expr $(,)?) => {
        if $cond {$result} else {panic!("Non-exhaustive guards")}
    };
}

/// Haskell 风格的 Guard 函数定义宏
///
/// 允许使用 Haskell guards 语法直接定义函数，将函数签名与 guard 条件
/// 结合在一起，类似于 Haskell 的：
///
/// ```haskell
/// abs :: Int -> Int
/// abs n
///     | n < 0     = -n
///     | otherwise = n
/// ```
///
/// # 语法
///
/// ```text
/// fn_guard!(
///     #[attributes]           // 可选的属性
///     pub fn name(arg1: Type1, arg2: Type2) -> ReturnType
///     | condition1 => result1,
///     | condition2 => result2,
///     | otherwise => default,
///     where binding = value,  // 可选的 where 子句
/// );
/// ```
///
/// # 特性
///
/// - 支持可见性修饰符（`pub`, `pub(crate)` 等）
/// - 支持函数属性（`#[inline]`, `#[must_use]` 等）
/// - 支持有返回值和无返回值（`-> ()`）的函数
/// - 支持 `where` 子句定义局部绑定
///
/// # Examples
///
/// ## 基本函数定义
///
/// ```
/// # const otherwise:bool = true;
/// # macro_rules! guard {
/// #     (
/// #       $(| $cond:expr => $result:expr),+ $(,)?,
/// #       where $($binding:ident = $val:expr),+ $(,)?
/// #     ) => {{
/// #       $(let $binding = $val;)+
/// #        guard!($(| $cond => $result),+)
/// #     }};
/// #     (| otherwise => $result:expr $(,)?) => {
/// #        $result
/// #     };
/// #     (| $cond:expr => $result:expr,$($rest:tt)+) => {
/// #       if $cond {
/// #            $result
/// #        }
/// #       else{
/// #            guard!($($rest)+)
/// #        }
/// #     };
/// #     (| $cond:expr => $result:expr $(,)?) => {
/// #         if $cond {$result} else {panic!("Non-exhaustive guards")}
/// #     };
/// # }
/// # macro_rules! fn_guard {
/// #     (
/// #         $(#[$attr:meta])*
/// #         $vis:vis fn $name:ident($($param:ident : $ptype:ty),* $(,)?) -> $ret:ty
/// #         $(| $cond:expr => $result:expr),+ $(,)?,
/// #         $(where $($binding:ident = $val:expr),+ $(,)?)?
/// #     ) => {
/// #         $(#[$attr])*
/// #         $vis fn $name($($param: $ptype),*) -> $ret {
/// #             $($(let $binding = $val;)+)?
/// #             guard!($(| $cond => $result),+)
/// #         }
/// #     };
/// #     (
/// #         $(#[$attr:meta])*
/// #         $vis:vis fn $name:ident($($param:ident : $ptype:ty),* $(,)?)
/// #         $(| $cond:expr => $result:expr),+ $(,)?,
/// #         $(where $($binding:ident = $val:expr),+ $(,)?)?
/// #     ) => {
/// #         $(#[$attr])*
/// #         $vis fn $name($($param: $ptype),*) {
/// #             $($(let $binding = $val;)+)?
/// #             guard!($(| $cond => $result),+)
/// #         }
/// #     };
/// # }
/// fn_guard!(
///     fn abs(n: i32) -> i32
///     | n < 0 => -n,
///     | otherwise => n,
/// );
///
/// assert_eq!(abs(-5), 5);
/// assert_eq!(abs(3), 3);
/// ```
///
/// ## 带属性和 where 子句
///
/// ```
/// # const otherwise:bool = true;
/// # macro_rules! guard {
/// #     (
/// #       $(| $cond:expr => $result:expr),+ $(,)?,
/// #       where $($binding:ident = $val:expr),+ $(,)?
/// #     ) => {{
/// #       $(let $binding = $val;)+
/// #        guard!($(| $cond => $result),+)
/// #     }};
/// #     (| otherwise => $result:expr $(,)?) => {
/// #        $result
/// #     };
/// #     (| $cond:expr => $result:expr,$($rest:tt)+) => {
/// #       if $cond {
/// #            $result
/// #        }
/// #       else{
/// #            guard!($($rest)+)
/// #        }
/// #     };
/// #     (| $cond:expr => $result:expr $(,)?) => {
/// #         if $cond {$result} else {panic!("Non-exhaustive guards")}
/// #     };
/// # }
/// # macro_rules! fn_guard {
/// #     (
/// #         $(#[$attr:meta])*
/// #         $vis:vis fn $name:ident($($param:ident : $ptype:ty),* $(,)?) -> $ret:ty
/// #         $(| $cond:expr => $result:expr),+ $(,)?,
/// #         $(where $($binding:ident = $val:expr),+ $(,)?)?
/// #     ) => {
/// #         $(#[$attr])*
/// #         $vis fn $name($($param: $ptype),*) -> $ret {
/// #             $($(let $binding = $val;)+)?
/// #             guard!($(| $cond => $result),+)
/// #         }
/// #     };
/// #     (
/// #         $(#[$attr:meta])*
/// #         $vis:vis fn $name:ident($($param:ident : $ptype:ty),* $(,)?)
/// #         $(| $cond:expr => $result:expr),+ $(,)?,
/// #         $(where $($binding:ident = $val:expr),+ $(,)?)?
/// #     ) => {
/// #         $(#[$attr])*
/// #         $vis fn $name($($param: $ptype),*) {
/// #             $($(let $binding = $val;)+)?
/// #             guard!($(| $cond => $result),+)
/// #         }
/// #     };
/// # }
/// fn_guard!(
///     #[inline]
///     pub fn bmi_category(weight: f64, height: f64) -> &'static str
///     | bmi < 18.5 => "Underweight",
///     | bmi < 25.0 => "Normal",
///     | bmi < 30.0 => "Overweight",
///     | otherwise => "Obese",
///     where bmi = weight / (height * height),
/// );
///
/// assert_eq!(bmi_category(70.0, 1.75), "Normal");
/// ```
///
/// ## 无返回值函数
///
/// ```
/// # const otherwise:bool = true;
/// # macro_rules! guard {
/// #     (
/// #       $(| $cond:expr => $result:expr),+ $(,)?,
/// #       where $($binding:ident = $val:expr),+ $(,)?
/// #     ) => {{
/// #       $(let $binding = $val;)+
/// #        guard!($(| $cond => $result),+)
/// #     }};
/// #     (| otherwise => $result:expr $(,)?) => {
/// #        $result
/// #     };
/// #     (| $cond:expr => $result:expr,$($rest:tt)+) => {
/// #       if $cond {
/// #            $result
/// #        }
/// #       else{
/// #            guard!($($rest)+)
/// #        }
/// #     };
/// #     (| $cond:expr => $result:expr $(,)?) => {
/// #         if $cond {$result} else {panic!("Non-exhaustive guards")}
/// #     };
/// # }
/// # macro_rules! fn_guard {
/// #     (
/// #         $(#[$attr:meta])*
/// #         $vis:vis fn $name:ident($($param:ident : $ptype:ty),* $(,)?) -> $ret:ty
/// #         $(| $cond:expr => $result:expr),+ $(,)?,
/// #         $(where $($binding:ident = $val:expr),+ $(,)?)?
/// #     ) => {
/// #         $(#[$attr])*
/// #         $vis fn $name($($param: $ptype),*) -> $ret {
/// #             $($(let $binding = $val;)+)?
/// #             guard!($(| $cond => $result),+)
/// #         }
/// #     };
/// #     (
/// #         $(#[$attr:meta])*
/// #         $vis:vis fn $name:ident($($param:ident : $ptype:ty),* $(,)?)
/// #         $(| $cond:expr => $result:expr),+ $(,)?,
/// #         $(where $($binding:ident = $val:expr),+ $(,)?)?
/// #     ) => {
/// #         $(#[$attr])*
/// #         $vis fn $name($($param: $ptype),*) {
/// #             $($(let $binding = $val;)+)?
/// #             guard!($(| $cond => $result),+)
/// #         }
/// #     };
/// # }
/// fn_guard!(
///     fn log_level(level: u8)
///     | level == 0 => println!("DEBUG"),
///     | level == 1 => println!("INFO"),
///     | level == 2 => println!("WARN"),
///     | otherwise => println!("ERROR"),
/// );
/// ```
#[macro_export]
macro_rules! fn_guard {
    // 带 where 子句
    (
        $(#[$attr:meta])*
        $vis:vis fn $name:ident($($param:ident : $ptype:ty),* $(,)?) -> $ret:ty
        $(| $cond:expr => $result:expr),+ $(,)?,
        $(where $($binding:ident = $val:expr),+ $(,)?)?
    ) => {
        $(#[$attr])*
        $vis fn $name($($param: $ptype),*) -> $ret {
            $($(let $binding = $val;)+)?
            guard!($(| $cond => $result),+)
        }
    };

    // 无返回值 (-> ())
    (
        $(#[$attr:meta])*
        $vis:vis fn $name:ident($($param:ident : $ptype:ty),* $(,)?)
        $(| $cond:expr => $result:expr),+ $(,)?,
        $(where $($binding:ident = $val:expr),+ $(,)?)?
    ) => {
        $(#[$attr])*
        $vis fn $name($($param: $ptype),*) {
            $($(let $binding = $val;)+)?
            guard!($(| $cond => $result),+)
        }
    };
}

/// Haskell 风格的 Case 表达式宏
///
/// 模拟 Haskell 中的 case 表达式语法，使用 `| pattern => result` 的形式
/// 进行模式匹配，类似于 Haskell 的：
///
/// ```haskell
/// case x of
///     pattern1 -> result1
///     pattern2 | guard -> result2
///     _        -> defaultResult
/// ```
///
/// # 语法
///
/// ```text
/// case!(expression =>
///     | pattern1 => result1,
///     | pattern2 if guard => result2,  // 支持 guard 条件
///     | _ => default,
/// )
/// ```
///
/// # 特性
///
/// - 使用 `|` 前缀使语法更接近 Haskell
/// - 支持模式守卫（pattern guards）：`| pattern if condition => result`
/// - 底层编译为标准 Rust `match` 表达式，保持完整的模式匹配能力
/// - 支持所有 Rust 模式语法（结构体解构、元组、枚举等）
///
/// # Examples
///
/// ## 基本模式匹配
///
/// ```
/// # macro_rules! case {
/// #     (@acc $x:expr => [$($arms:tt)*]) => {
/// #         match $x {$($arms)*}
/// #     };
/// #     (@acc $x:expr => [$($arms:tt)*] | $pat:pat if $guard:expr => $result:expr $(,$($rest:tt)*)?) => {
/// #         case!(@acc $x => [$($arms)* $pat if $guard => $result,] $($($rest)*)?)
/// #     };
/// #     (@acc $x:expr => [$($arms:tt)*] | $pat:pat => $result:expr $(,$($rest:tt)*)?) => {
/// #         case!(@acc $x => [$($arms)* $pat => $result,] $($($rest)*)?)
/// #     };
/// #     ($x:expr=>$($rest:tt)+) => {
/// #         case!(@acc $x =>[] $($rest)+)
/// #     };
/// # }
/// let x = Some(42);
/// let result = case!(x =>
///     | None => "nothing",
///     | Some(0) => "zero",
///     | Some(_) => "something",
/// );
/// assert_eq!(result, "something");
/// ```
///
/// ## 使用模式守卫（Pattern Guards）
///
/// ```
/// # macro_rules! case {
/// #     (@acc $x:expr => [$($arms:tt)*]) => {
/// #         match $x {$($arms)*}
/// #     };
/// #     (@acc $x:expr => [$($arms:tt)*] | $pat:pat if $guard:expr => $result:expr $(,$($rest:tt)*)?) => {
/// #         case!(@acc $x => [$($arms)* $pat if $guard => $result,] $($($rest)*)?)
/// #     };
/// #     (@acc $x:expr => [$($arms:tt)*] | $pat:pat => $result:expr $(,$($rest:tt)*)?) => {
/// #         case!(@acc $x => [$($arms)* $pat => $result,] $($($rest)*)?)
/// #     };
/// #     ($x:expr=>$($rest:tt)+) => {
/// #         case!(@acc $x =>[] $($rest)+)
/// #     };
/// # }
/// let pair = (3, 5);
/// let result = case!(pair =>
///     | (x, y) if x == y => "equal",
///     | (x, y) if x > y => "first is larger",
///     | (_, _) => "second is larger",
/// );
/// assert_eq!(result, "second is larger");
/// ```
///
/// ## 枚举匹配
///
/// ```
/// # macro_rules! case {
/// #     (@acc $x:expr => [$($arms:tt)*]) => {
/// #         match $x {$($arms)*}
/// #     };
/// #     (@acc $x:expr => [$($arms:tt)*] | $pat:pat if $guard:expr => $result:expr $(,$($rest:tt)*)?) => {
/// #         case!(@acc $x => [$($arms)* $pat if $guard => $result,] $($($rest)*)?)
/// #     };
/// #     (@acc $x:expr => [$($arms:tt)*] | $pat:pat => $result:expr $(,$($rest:tt)*)?) => {
/// #         case!(@acc $x => [$($arms)* $pat => $result,] $($($rest)*)?)
/// #     };
/// #     ($x:expr=>$($rest:tt)+) => {
/// #         case!(@acc $x =>[] $($rest)+)
/// #     };
/// # }
/// enum Color { Red, Green, Blue, Rgb(u8, u8, u8) }
///
/// let color = Color::Rgb(255, 128, 0);
/// let name = case!(color =>
///     | Color::Red => "red",
///     | Color::Green => "green",
///     | Color::Blue => "blue",
///     | Color::Rgb(r, _, _) if r > 200 => "reddish",
///     | Color::Rgb(_, _, _) => "custom",
/// );
/// assert_eq!(name, "reddish");
/// ```
///
/// ## 与标准 match 对比
///
/// ```
/// # macro_rules! case {
/// #     (@acc $x:expr => [$($arms:tt)*]) => {
/// #         match $x {$($arms)*}
/// #     };
/// #     (@acc $x:expr => [$($arms:tt)*] | $pat:pat if $guard:expr => $result:expr $(,$($rest:tt)*)?) => {
/// #         case!(@acc $x => [$($arms)* $pat if $guard => $result,] $($($rest)*)?)
/// #     };
/// #     (@acc $x:expr => [$($arms:tt)*] | $pat:pat => $result:expr $(,$($rest:tt)*)?) => {
/// #         case!(@acc $x => [$($arms)* $pat => $result,] $($($rest)*)?)
/// #     };
/// #     ($x:expr=>$($rest:tt)+) => {
/// #         case!(@acc $x =>[] $($rest)+)
/// #     };
/// # }
/// let n = 5;
///
/// // 使用 case! 宏 (Haskell 风格)
/// let a = case!(n =>
///     | 0 => "zero",
///     | x if x < 0 => "negative",
///     | _ => "positive",
/// );
///
/// // 等价的标准 Rust match
/// let b = match n {
///     0 => "zero",
///     x if x < 0 => "negative",
///     _ => "positive",
/// };
///
/// assert_eq!(a, b);
/// ```
#[macro_export]
macro_rules! case {
    (@acc $x:expr => [$($arms:tt)*]) => {
        match $x {$($arms)*}
    };
    (@acc $x:expr => [$($arms:tt)*] | $pat:pat if $guard:expr => $result:expr $(,$($rest:tt)*)?) => {
        case!(@acc $x => [$($arms)* $pat if $guard => $result,] $($($rest)*)?)
    };
    (@acc $x:expr => [$($arms:tt)*] | $pat:pat => $result:expr $(,$($rest:tt)*)?) => {
        case!(@acc $x => [$($arms)* $pat => $result,] $($($rest)*)?)
    };
    ($x:expr=>$($rest:tt)+) => {
        case!(@acc $x =>[] $($rest)+)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guard() {
        let x = 10;
        let result = guard!(
            | x < 0 => "negative",
            | x == 0 => "zero",
            | otherwise => "positive",
        );
        assert_eq!(result, "positive");
    }

    #[test]
    fn test_case() {
        let opt = Some(42);
        let result = case!(opt =>
            | None => 0,
            | Some(x) if x > 50 => 1,
            | Some(_) => 2,
        );
        assert_eq!(result, 2);
    }
}

#[cfg(test)]
mod hgm {
    use super::*;

    #[test]
    fn macros_guard() {
        let weight = 70.0;
        let height = 1.75;
        let result = guard! {
            | bmi <= 18.5 => "Underweight",
            | otherwise => "Unknown",
            where bmi = weight / height
        };
        assert_eq!(result, "Unknown");
    }
    fn_guard! {
        #[inline]
        pub fn bmi_tell(weight:f64,height:f64) -> &'static str
        | bmi <= 18.5 => "Underweight",
        | otherwise => "Unknown",
        where bmi = weight / height
    }
    #[test]
    fn macros_fn_guard() {
        let result = bmi_tell(70.0, 1.75);
        assert_eq!(result, "Unknown");
    }
}
