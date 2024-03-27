pub mod exercise_0;
pub mod exercise_1;

async fn get_data(){
    println!("Hello Wrold");
}


#[tokio::main]
async fn main() {
    let data = get_data();
    println!("Hello, world!");
    data.await;

    exercise_0::main().await;
    exercise_1::main().await;
}
