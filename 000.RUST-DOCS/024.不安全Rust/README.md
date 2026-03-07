# 不安全Rust

|摘要|说明|备注|
|-|-|-|
|- 不安全 Rust（unsafe Rust）|- 目前为止讨论过的代码都有 Rust 在编译时会强制执行的内存安全保证。然而，Rust 还隐藏有第二种语言，它不会强制执行这类内存安全保证：这被称为 不安全 Rust（unsafe Rust）|- 场景： <br/> - Rust 编译器没有足够的信息来确定代码是合法的 <br/> -  另一个 Rust 存在不安全一面的原因是底层计算机硬件固有的不安全性。(直接与操作系统交互等底层系统编程)|
|-|-|-|
|- unsafe 并不会关闭借用检查器或禁用任何其他 Rust 安全检查：如果在不安全代码中使用引用，它仍会被检查。|-|-|
|-|-|-|
|- 裸指针|-|- 使用场景: <br/> + 用 C 代码接口  <br/> + 构建借用检查器无法理解的安全抽象???|
|-|-|-|
|-  全局变量（global variables）|- 全局变量在 Rust 中被称为 静态（static）变量|-|
|-|-|-|


## 不安全的超能力（unsafe superpowers）
要切换到 unsafe Rust，可以使用 unsafe 关键字，然后开启一个包含不安全代码的新块, 意图是由程序员来确保 unsafe 块中的代码以有效的方式访问内存

### 解引用裸指针
不安全 Rust 有两个被称为 裸指针（raw pointers）的类似于引用的新类型。和引用一样，裸指针是不可变或可变的，分别写作 *const T 和 *mut T。（星号不是解引用运算符；它是类型名称的一部分）

裸指针与引用和智能指针的区别在于：
- 允许忽略借用规则，可以同时拥有不可变和可变的指针，或多个指向相同位置的可变指针
- 不保证指向有效的内存
- 允许为空
- 不能实现任何自动清理功能
- ```rust
      let mut num = 5;

      let r1 = &raw const num; // 不可变裸指针
      let r2 = &raw mut num;  // 可变裸指针
      // 这段代码中没有引入 unsafe 关键字。可以在安全代码中创建裸指针；只是不能在不安全块之外解引用裸指针
  ```
- 解引用 裸指针和读取其指向的数据
  ```rust
         let mut num = 5;

         let r1 = &raw const num;
         let r2 = &raw mut num;
     
         unsafe {
             println!("r1 is: {}", *r1);
             println!("r2 is: {}", *r2);
         }

  ```

### 调用不安全函数或方法
通过 unsafe 块，我们向 Rust 断言我们已经阅读过函数的文档，理解如何正确使用它，并核实我们履行了该函数的契约

```rust
unsafe fn dangerous() {}

fn main() {
    unsafe {
        dangerous();
    }
}
```

#### 创建不安全代码的安全抽象
```rust
use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    // as_mut_ptr 返回一个 *mut i32 类型的裸指针，并将其存储在 ptr 变量中
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            // slice::from_raw_parts_mut 函数获取一个裸指针和一个长度来创建一个 slice
            // slice::from_raw_parts_mut 函数是不安全的因为它获取一个裸指针，并必须确信这个指针是有效的。
            slice::from_raw_parts_mut(ptr, mid),
            
            // 裸指针上的 add 方法也是不安全的，因为其必须确信此地址偏移量也是有效的指针
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
``` 

#### 使用 extern 函数调用外部代码
有时你的 Rust 代码可能需要与其他语言编写的代码交互。为此 Rust 有一个关键字，extern，有助于创建和使用 外部函数接口（Foreign Function Interface，FFI）

```rust
/*
 * Rust 默认会自动链接这些标准库:glibc ... ,所以就不用引入头文件
 * 
 * 只要你没显式禁用默认库（#![no_std] 且没配置链接器），链接器会自动去系统路径下找 libc
 */
unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}


# 使用 safe 关键字来表明这个特定的函数即便是在 unsafe extern 块中也是可以安全调用的: 调用它不再需要 unsafe 块
unsafe extern "C" {
    safe fn abs(input: i32) -> i32;
}

fn main() {
    println!("Absolute value of -3 according to C: {}", abs(-3));
}
```

#### 从其它语言调用 Rust 函数

```rust
# 可以使用 extern 来创建一个允许其它语言调用 Rust 函数的接口

# 需增加 #[no_mangle] 注解来告诉 Rust 编译器不要 mangle 此函数的名称
# 每一个编程语言的编译器都会以稍微不同的方式 mangle 函数名，所以为了使 Rust 函数能在其他语言中指定，必须禁用 Rust 编译器的 name mangling。

#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

```

### 访问或修改可变静态变量
静态变量只能储存拥有 'static 生命周期的引用，这意味着 Rust 编译器可以自己计算出其生命周期而无需显式标注。访问不可变静态变量是安全的

访问和修改可变静态变量都是 不安全 的

```rust
static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("value is: {HELLO_WORLD}");
}



# 访问和修改可变静态变量都是 不安全 的
static mut COUNTER: u32 = 0;

/// SAFETY: 同时在多个线程调用这个方法是未定义的行为，所以你*必须*保证同一时间只
/// 有一个线程在调用它。
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    unsafe {
        // SAFETY: 它只在 `main` 这一个线程被调用。
        add_to_count(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }
}
```

### 实现不安全 trait
使用 unsafe 来实现一个不安全 trait。当 trait 中至少有一个方法中包含编译器无法验证的不变式（invariant）时该 trait 就是不安全的。

可以在 trait 之前增加 unsafe 关键字将 trait 声明为 unsafe，同时 trait 的实现也必须标记为 unsafe
```rust
unsafe trait Foo {
    // 方法在这里
}

unsafe impl Foo for i32 {
    // 方法实现在这里
}

```

### 访问联合体中的字段
联合体主要用于和 C 代码中的联合体进行交互。访问联合体的字段是不安全的，因为 Rust 无法保证当前存储在联合体实例中数据的类型。


## 使用 miri 检查不安全代码
借用检查器是一个在编译时工作的静态工具，Miri 是一个在运行时工作的动态工具。它通过运行程序，或者测试集来检查代码，并检测你是否违反了它理解的 Rust 应该如何工作的规则。

使用 Miri 要求使用 nightly 版本的 Rust

```bash
rustup +nightly component add miri 来同时安装 nightly 版本的 Rust 和 Miri

cargo +nightly miri run or cargo +nightly miri test 在项目中使用 Miri
```