
// 03 - Wait a minute, what?

// From the rust reference book:
/*
    CallExpression → Expression ( CallParams? )

    Expression →
        ExpressionWithoutBlock
        | ExpressionWithBlock

    ExpressionWithoutBlock →
        OuterAttribute*
        (
           LiteralExpression
         | PathExpression
         | OperatorExpression
         | GroupedExpression
         | ArrayExpression
         | AwaitExpression
         | IndexExpression
         | TupleExpression
         | TupleIndexingExpression
         | StructExpression
         | CallExpression
         | MethodCallExpression
         | FieldExpression
         | ClosureExpression
         | AsyncBlockExpression
         | ContinueExpression
         | BreakExpression
         | RangeExpression
         | ReturnExpression
         | UnderscoreExpression
         | MacroInvocation
        )

    ExpressionWithBlock →
        OuterAttribute*
        (
           BlockExpression
         | ConstBlockExpression
         | UnsafeBlockExpression
         | LoopExpression
         | IfExpression
         | MatchExpression
        )
 */

fn wait_a_minute_what() {
    {}(); // <-- explained in next example file ;)
    (|x: i32| println!("{}", x))(5);
    ({
        let x= |x: i32| println!("{x}");
        x
    })
        (5);
    // This won't work:
    // (if true {|x| println!("x: {}", x)})(5);
    // But this will:
    (if true {|x| println!("x: {}", x)} else {return})(5);


    // And this will also build:
    (if(if(||{(||{(||{let g:Box<dyn Fn(i32)->bool>=Box::new(|x|x==5);g(5)})()})()})(){|x|x}else{return})(true){|_,_,_,_|println!(";-)")}else{return})(1,3,3,7);
}


fn main() {
    println!("Wait a minute what!");
}

// 03 - End of Wait a minute, what?