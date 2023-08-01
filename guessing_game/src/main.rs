use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number =rand::thread_rng().gen_range(0..=100);
    println!("the secret number is {secret_number}");

    println!("Please enter a number");

    let mut guess=String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read");

    println!("Your guesses: {guess}");

    //note:
    // std: 标准库  std::io： 输出输入流
    //fn main() {}：申明了一个新函数，没有参数
    // println!: 宏
    //let mut guess: 申明一个可变参数，如果没有mut是不可变，即let guess=5; :guess的值等于5，且不可变
    //String::new(): ::new 那一行的 :: 语法表明 new 是 String 类型的一个 关联函数（associated function）。String类型的静态方法new方法
    //io::stdin(): 调用 io 库中的函数 stdin();stdin 函数返回一个 std::io::Stdin 的实例，这代表终端标准输入句柄的类型。
    //.read_line(&mut guess)，调用 read_line 方法从标准输入句柄获取用户输入。我们还将 &mut guess 作为参数传递给 read_line() 函数，让其将用户输入储存到这个字符串中。
    //read_line 的工作是，无论用户在标准输入中键入什么内容，都将其追加（不会覆盖其原有内容）到一个字符串中，因此它需要字符串作为参数。
    //这个字符串参数应该是可变的，以便 read_line 将用户输入附加上去。
    //&mut guess: 指向guess的引用,& 表示这个参数是一个 引用（reference），它允许多处代码访问同一处数据，而无需在内存中多次拷贝。

    //read_line 会将用户输入附加到传递给它的字符串中，不过它也会返回一个类型为 Result 的值。 
    //1.Result 是一种枚举类型，通常也写作 enum。枚举类型变量的值可以是多种可能状态中的一个。我们把每种可能的状态称为一种 枚举成员（variant）。
    //2.Result 的成员是 Ok 和 Err，Ok 成员表示操作成功，内部包含成功时产生的值。Err 成员则意味着操作失败，并且包含失败的前因后果。
    //3.Result 的实例拥有 expect 方法。如果 io::Result 实例的值是 Err，expect 会导致程序崩溃，并显示当做参数传递给 expect 的信息。
    //  如果 read_line 方法返回 Err，则可能是来源于底层操作系统错误的结果。如果 Result 实例的值是 Ok，expect 会获取 Ok 中的值并原样返回。
    // println!("You guessed: {guess}");里面的 {} 是预留在特定位置的占位符

    //use rand::Rng;Rng 是一个 trait，它定义了随机数生成器应实现的方法，想使用这些方法的话，此 trait 必须在作用域中
    //rand::thread_rng 函数提供实际使用的随机数生成器：它位于当前执行线程的本地环境中，并从操作系统获取 seed。接着调用随机数生成器的 gen_range 方法。这个方法由 use rand::Rng 语句引入到作用域的 Rng trait 定义。gen_range 方法获取一个范围表达式（range expression）作为参数，并生成一个在此范围之间的随机数。这里使用的这类范围表达式使用了 start..=end 这样的形式，也就是说包含了上下端点，所以需要指定 1..=100 来请求一个 1 和 100 之间的数。
    //运行 cargo doc --open 命令来构建所有本地依赖提供的文档，并在浏览器中打开。例如，假设你对 rand crate 中的其他功能感兴趣，你可以运行 cargo doc --open 并点击左侧导航栏中的 rand。
    match guess.cmp(&secret_number){
        Ordering::less => println!("Too small"),
        Ordering::greater => println!("Too big"),
        Ordering::equal => println!("You win!"),
    }
    //标准库引入了一个叫做 std::cmp::Ordering 的类型到作用域中。 Ordering 也是一个枚举，不过它的成员是 Less、Greater 和 Equal。这是比较两个值时可能出现的三种结果。
    //Ordering 类型，cmp 方法用来比较两个值并可以在任何可比较的值上调用。它获取一个被比较值的引用：这里是把 guess 与 secret_number 做比较。然后它会返回一个刚才通过 use 引入作用域的 Ordering 枚举的成员。使用一个 match 表达式，根据对 guess 和 secret_number 调用 cmp 返回的 Ordering 成员来决定接下来做什么。
    






}
