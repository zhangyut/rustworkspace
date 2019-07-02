fn area(rect:(u32, u32)) -> u32 {
    rect.0*rect.1
}

struct Test {
    xi:i32,
    yi:i32,
}

impl Test {
    fn hold(&self, other:Test) -> bool {
        self.xi > other.xi && self.yi > other.yi
    }
}

fn main() {
    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![2; 20];
    let index : usize = 3;
    println!("v2[{}] = {}", index, v2[index]);
    println!("len(v2) = {}", v2.len());

    let init_arr = [2;20];
    println!("init_arr[{}] = {}", index, init_arr[index]);
    println!("len(init_arr) = {}", init_arr.len());
    for i in &v1 { // 为什么用&v1而不是直接v1，关乎所有权问题，下次再学习
        println!("{}", i);
    }
    println!("Hello, world!");
    let v3 = String::from("hello world");
    {
        let v4 = v3; 
        println!("{}", v4);
    }

    let a = "hello world";
    let b = &a[..];
    println!("{}", b);

    let rect1 = (10,20);
    area(rect1);
    println!("{}, {}", rect1.0, rect1.1);

    let test1 = Test{xi:10, yi:20};
    let test2 = Test{xi:5, yi:10};
    let test3 = Test{xi:30, yi:40};

    println!("can test1 hold test2: {}", test1.hold(test2));
    println!("can test1 hold test3: {}", test1.hold(test3));

    println!("test2: {:?}", test2.xi);
}
