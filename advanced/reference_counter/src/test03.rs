use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Logger {
    logs: RefCell<Vec<String>>,
    logger: Logger
}

impl Logger {
    fn log(&self, msg: &str) {
        self.logs.borrow_mut().push(msg.to_string());
    }

    fn show_logs(&self) {
        for log in self.logs.borrow().iter() {
            println!("- {}", log);
        }
    }
}
pub fn test() {
    let logger = Rc::new(Logger {
        logs: RefCell::new(vec![]),
        logger: Logger {},
    });

    logger.log("start");
    logger.log("Processing");
    logger.log("Done");

    logger.show_logs();
}