# Đề bài : Implement trait Iterator (của thư viện Rust) cho kiểu dữ liệu Struct sau
*https://doc.rust-lang.org/std/iter/trait.Iterator.html*

```rust
struct Fibonacci {
    a: u32,
    b: u32,
}
```
---
Như mọi người đã biết Dãy Fibonacci có quy luật như sau
Dãy Fibonacci là dãy số bắt đầu bằng 0 với 1. Mọi số tiếp theo
đều là tổng của 2 số trước đó.

Ví dụ: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, ...

Trong trường hợp bài toán này, khởi tạo ban đầu sẽ là
```rust
struct Fibonacci {
    a = 1,
    b = 0,
}
```

Một số kiến thức để làm dc bài này: 
- **[Trait](https://doc.rust-lang.org/book/ch10-02-traits.html)**

- **[Generic Type](https://doc.rust-lang.org/book/ch10-01-syntax.html)**

- **[Associated type](https://doc.rust-lang.org/rust-by-example/generics/assoc_items/types.html)**


Gợi ý có sườn như sau:
```rust
#[derive(Debug)]
struct Fibonacci {
    a: u32,
    b: u32,
}

impl Iterator for Fibonacci {
    type Item = TODO!;

    fn next(&mut self) -> Option<u32> {
        todo!()
    }
}

fn fibonacci_numbers() -> Fibonacci {
    Fibonacci { a: 1, b: 0 }
}

fn main() {
    for number in fibonacci_numbers() {
        println!("{}", number);
    }
}
```
---
**Output**
```
1
1
2
3
5
8
13
21
34
55
89
144
233
377
...
```


