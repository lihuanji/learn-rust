mod letter;
mod alphabet;

fn main() {
    println!("开始打印 a~Z");
    letter::print_letter();
    println!("结束打印 a~Z");

    println!("开始打印 A~z");
    alphabet::letter::print();
    println!("结束打印 A~z");
}
