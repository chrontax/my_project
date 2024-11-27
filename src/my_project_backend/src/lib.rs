use std::cell::RefCell;

thread_local! {
    static MSG: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

#[ic_cdk::update]
fn save_msg(msg: String) {
    MSG.with_borrow_mut(|msgs| msgs.push(msg));
}

#[ic_cdk::query]
fn get_chat() -> Vec<String> {
    MSG.with_borrow(Clone::clone)
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
