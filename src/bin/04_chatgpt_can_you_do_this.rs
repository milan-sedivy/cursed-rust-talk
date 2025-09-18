// 04 - Hey ChatGPT can you do this?

// I disabled ChatGPT from gathering my data
// This way it will never learn from its mistakes
// the only issue is, it tried to convince me from the start 
// Back to slides - for a while.

fn main() {
    println!("Hey ChatGPT can you do this!?");
    // This compiles and is absolutely valid...
    {{{{5;}()}()}()}();
    
    match { {{{{5;}()}()}()}() } {
        () => println!("Huzzah?")
    }
    // This one won't work
    //match /*{*/ {{{{5;}()}()}()}() /*}*/ {
    //         () => println!("Huzzah?")
    //     }
    
    // Why?
    // ChatGPT wrongly assumes it's a function ... after all why wouldn't it
    // But Rust parses {}() as a BlockExpression that returns () and a () unit type
    // However notice that ()() doesn't compile, this is because it's not 2 unit types
    // This is however
    ;()
    // And so is this
    ;;;;;;;  ;;;;;;;  ;     ;  ;;;;;;;  ;;;;;;;  ;;;;;;;  ;        ;;;;;;;  ;     ;
    ;        ;        ;;   ;;    ;      ;        ;     ;  ;        ;     ;  ;;    ;
    ;        ;        ; ; ; ;    ;      ;        ;     ;  ;        ;     ;  ; ;   ;
    ;;;;;;;  ;;;;;;;  ;  ;  ;    ;      ;        ;     ;  ;        ;     ;  ;  ;  ;
          ;  ;        ;     ;    ;      ;        ;     ;  ;        ;     ;  ;   ; ;
          ;  ;        ;     ;    ;      ;        ;     ;  ;        ;     ;  ;    ;;
    ;;;;;;;  ;;;;;;;  ;     ;  ;;;;;;;  ;;;;;;;  ;;;;;;;  ;;;;;;;; ;;;;;;;  ;     ;
}

// 04 - End of Hey ChatGPT can you do this?