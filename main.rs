// use vector2::Vector2;

// // fn main() {
// //     // Create a Vector2
// //     // let mut vector2 = Vector2::new(1.0, 0.5);
// //     let vector2 = Vector2::new(1.0, 0.5);
// //     let b=let a = 8;
    
// //     // Assuming Vector2::RIGHT is a predefined constant or function in the Vector2 module
// //     // let other = Vector2::RIGHT;

// //     // Add two Vector2s
// //     // let added = vector2 + other;
// //     let added2 = vector2 - Vector2::LEFT;
// //     // Print the result
// //     println!("({}, {})", added2.x, added2.y);
// // }

// trait Drawable {
//     fn draw(&self);
// }

// struct Circle;
// struct Square;

// impl Drawable for Circle {
//     fn draw(&self) {
//         println!("Drawing a circle");
//     }
// }

// impl Drawable for Square {
//     fn draw(&self) {
//         println!("Drawing a square");
//     }
// }

// fn main() {
//     let shapes: Vec<Box<dyn Drawable>> = vec![
//         Box::new(Circle),
//         Box::new(Square),
//     ];

//     for shape in &shapes {
//         shape.draw(); // 杩愯鏃跺喅瀹氳皟鐢ㄥ摢涓叿浣撶殑 draw 鏂规硶
//     }
// }

// trait Component {
//     fn operation(&self);
// }

// struct ConcreteComponent;

// impl Component for ConcreteComponent {
//     fn operation(&self) {
//         println!("ConcreteComponent operation");
//     }
// }

// struct ColorDecorator {
//     component: Box<dyn Component>,
// }

// impl Component for ColorDecorator {
//     fn operation(&self) {
//         println!("ColorDecorator operation");
//         self.component.operation();
//     }
// }

// struct BorderDecorator {
//     component: Box<dyn Component>,
// }

// impl Component for BorderDecorator {
//     fn operation(&self) {
//         println!("BorderDecorator operation");
//         self.component.operation();
//     }
// }

// fn decorate_tb() {
//     let component: Box<dyn Component> = Box::new(ConcreteComponent);

//     let component = Box::new(ColorDecorator { component });
//     let component = Box::new(BorderDecorator { component });

//     component.operation();
// }

// fn main() {
//     decorate_tb();
// }

use std::cell::RefCell;

// 定义 Config 结构体
struct Config {
    value: RefCell<i32>,
}

// 定义 Object 结构体
struct Object<'a> {
    config: &'a Config,
}

impl<'a> Object<'a> {
    fn new(config: &'a Config) -> Self {
        Object { config }
    }

    fn get_value(&self) -> i32 {
        *self.config.value.borrow()
    }
}

// 定义 Guide 结构体
struct Guide<'a> {
    config: &'a Config,
}

impl<'a> Guide<'a> {
    fn new(config: &'a Config) -> Self {
        Guide { config }
    }

    fn set_value(&self, value: i32) {
        *self.config.value.borrow_mut() = value;
    }
}

fn main() {
    // 创建 Config 实例
    let config = Config { value: RefCell::new(0) };

    // 创建 Object 实例
    let object = Object::new(&config);

    // 创建 Guide 实例
    let guide = Guide::new(&config);

    // 修改 Config 的值
    guide.set_value(10);

    // 读取 Config 的值
    println!("Value in Object: {}", object.get_value());
}