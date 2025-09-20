// 05 - Exercise in gaslighting!

// For simplicity I didn't implement what tokio::main actually does
// hence this doesn't really create an async runtime.
// It's just a regular fn main() {}
#[tokio::main]
async fn main() {
    // DON'T REMOVE THIS IT WON'T COMPILE OTHERWISE!!!
    {5;}()
}

// 05 - End of Exercise in gaslighting!