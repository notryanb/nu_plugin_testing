
use nu_errors::ShellError;
use nu_plugin::{serve_plugin, Plugin};
use nu_protocol::{
    CallInfo,  ReturnSuccess, ReturnValue, Signature, SyntaxShape, Value,
};

struct TestPlayground;

impl TestPlayground {
    fn new() -> TestPlayground {
        TestPlayground
    }
}

impl Plugin for TestPlayground {
    fn config(&mut self) -> Result<Signature, ShellError> {
        dbg!("Called config");
        Ok(
            Signature::build("testing")
            .desc("description for the signature goes here")
            .required(
                "required_arg",
                SyntaxShape::Any,
                "required_arg description goes here"
            )
            .optional(
                "optional_arg",
                SyntaxShape::Any,
                "optional_arg description goes here"
            )
            .filter()
        )
    }

    fn begin_filter(&mut self, call_info: CallInfo) -> Result<Vec<ReturnValue>, ShellError> {
        dbg!("Called begin_filter");
        //dbg!(call_info);
        Ok(vec![])
    }

    fn filter(&mut self, input: Value) -> Result<Vec<ReturnValue>, ShellError> {
        dbg!("Called filter");
        //dbg!(input);
        Ok(vec![])
    }

    fn end_filter(&mut self) -> Result<Vec<ReturnValue>, ShellError> {
        dbg!("Called end_filter");
        Ok(vec![])
    }

    fn sink(&mut self, call_info: CallInfo, _input: Vec<Value>) {
        //dbg!(call_info);
        dbg!("Called sink");
    }

    fn quit(&mut self) {
        dbg!("Called quit");
    }
}

fn main() {
    serve_plugin(&mut TestPlayground::new());
}